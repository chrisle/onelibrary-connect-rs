use onelibrary_connect_rs::{get_encryption_key, open_one_library_db, Error, OneLibraryAdapter};
use rusqlite::{Connection, OpenFlags};
use sha2::{Digest, Sha256};
fn fixture(dir: &std::path::Path) -> std::path::PathBuf {
    let p = dir.join("fixture.db");
    std::fs::write(&p, include_bytes!("../fixtures/test-onelibrary.db")).unwrap();
    p
}
fn writer(path: &std::path::Path) -> Connection {
    let db = Connection::open(path).unwrap();
    onelibrary_connect_rs::connection::set_key(&db, &get_encryption_key().unwrap()).unwrap();
    db
}
#[test]
fn encrypted_readonly_queries_preserve_bytes_and_close_idempotently() {
    let dir = tempfile::tempdir().unwrap();
    let path = fixture(dir.path());
    let before = std::fs::read(&path).unwrap();
    assert_ne!(&before[..16], b"SQLite format 3\0");
    assert_eq!(
        format!("{:x}", Sha256::digest(&before)),
        "7a47f162edaef42fe03fad836013ad4fe3e1b1cc8f5604173a97ece74d8f08bb"
    );
    let db = open_one_library_db(&path).unwrap();
    assert!(db.execute("DELETE FROM content", []).is_err());
    assert_eq!(
        db.query_row("PRAGMA query_only", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        0
    );
    drop(db);
    let mut adapter = OneLibraryAdapter::open(&path).unwrap();
    assert_eq!(adapter.find_all_tracks().unwrap().len(), 5);
    adapter.close();
    adapter.close();
    assert_eq!(adapter.find_cues(1), Err(Error::Closed));
    assert_eq!(std::fs::read(&path).unwrap(), before);
}
#[test]
fn missing_wrongkey_corrupt_truncated_plaintext_and_wrong_schema_fail() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("missing");
    assert!(matches!(
        OneLibraryAdapter::open(&missing),
        Err(Error::OpenFailed)
    ));
    assert!(!missing.exists());
    let path = fixture(dir.path());
    let db = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    onelibrary_connect_rs::connection::set_key(&db, b"synthetic wrong key").unwrap();
    assert!(db
        .query_row("SELECT count(*) FROM sqlite_schema", [], |r| r
            .get::<_, i64>(0))
        .is_err());
    drop(db);
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[64] ^= 0xff;
    std::fs::write(&path, &bytes).unwrap();
    assert!(matches!(
        OneLibraryAdapter::open(&path),
        Err(Error::InvalidDatabase)
    ));
    std::fs::write(&path, &bytes[..512]).unwrap();
    assert!(OneLibraryAdapter::open(&path).is_err());
    let plain = dir.path().join("plain");
    let db = Connection::open(&plain).unwrap();
    db.execute_batch("CREATE TABLE t(x)").unwrap();
    drop(db);
    assert!(matches!(
        OneLibraryAdapter::open(&plain),
        Err(Error::InvalidDatabase)
    ));
    let wrong = dir.path().join("wrong_schema");
    let db = writer(&wrong);
    db.execute_batch("CREATE TABLE t(x)").unwrap();
    drop(db);
    assert!(matches!(
        OneLibraryAdapter::open(&wrong),
        Err(Error::InvalidSchema)
    ));
}
#[test]
fn encrypted_wal_refreshes_queries_without_checkpoint_and_hides_rollbacks() {
    let dir = tempfile::tempdir().unwrap();
    let path = fixture(dir.path());
    let w = writer(&path);
    w.execute_batch("PRAGMA journal_mode=WAL;PRAGMA wal_autocheckpoint=0")
        .unwrap();
    let db = OneLibraryAdapter::open(&path).unwrap();
    let before = db.find_track(1).unwrap().unwrap().title;
    w.execute_batch("BEGIN;UPDATE content SET title='Pending' WHERE content_id=1")
        .unwrap();
    assert_eq!(db.find_track(1).unwrap().unwrap().title, before);
    w.execute_batch("COMMIT").unwrap();
    assert!(path.with_extension("db-wal").exists());
    assert_eq!(db.find_track(1).unwrap().unwrap().title, "Pending");
    w.execute_batch("BEGIN;UPDATE content SET title='Rolled back' WHERE content_id=1;ROLLBACK")
        .unwrap();
    assert_eq!(db.find_track(1).unwrap().unwrap().title, "Pending");
    drop(db);
    assert_eq!(
        OneLibraryAdapter::open(&path)
            .unwrap()
            .find_track(1)
            .unwrap()
            .unwrap()
            .title,
        "Pending"
    );
}
#[test]
fn independent_synthetic_generator_reproduces_track_reference_cue_and_browse_data() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("synthetic.db");
    let w = writer(&path);
    w.execute_batch(include_str!("../fixtures/synthetic.sql"))
        .unwrap();
    drop(w);
    let db = OneLibraryAdapter::open(&path).unwrap();
    let original = fixture(dir.path());
    let reference = OneLibraryAdapter::open(original).unwrap();
    assert_eq!(
        db.find_all_tracks().unwrap(),
        reference.find_all_tracks().unwrap()
    );
    assert_eq!(db.find_cues(1).unwrap(), reference.find_cues(1).unwrap());
    assert_eq!(
        db.find_playlist(None).unwrap(),
        reference.find_playlist(None).unwrap()
    );
    assert_eq!(
        db.find_my_tags(None).unwrap(),
        reference.find_my_tags(None).unwrap()
    );
    assert_eq!(
        db.get_property().unwrap(),
        reference.get_property().unwrap()
    );
}
#[test]
fn adapter_normalizes_calendar_rollover_and_omits_broken_utf16_paths() {
    let dir = tempfile::tempdir().unwrap();
    let path = fixture(dir.path());
    let w = writer(&path);
    w.execute(
        "UPDATE content SET dateAdded=?,analysisDataFilePath=? WHERE content_id=1",
        ["2024-02-30", "😀abc"],
    )
    .unwrap();
    let db = OneLibraryAdapter::open(&path).unwrap();
    let track = db.find_track(1).unwrap().unwrap();
    assert_eq!(
        track.date_added,
        Some(serde_json::json!("2024-03-01T00:00:00.000Z"))
    );
    assert!(track.analyze_path.is_none());
    assert!(!serde_json::to_value(track)
        .unwrap()
        .as_object()
        .unwrap()
        .contains_key("analyzePath"));
    w.execute(
        "UPDATE content SET analysisDataFilePath=? WHERE content_id=1",
        ["/USB/音楽/😀.DAT"],
    )
    .unwrap();
    assert_eq!(
        db.find_track(1).unwrap().unwrap().analyze_path.as_deref(),
        Some("/USB/音楽/😀")
    );
}
