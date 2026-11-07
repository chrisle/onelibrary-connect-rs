use onelibrary_connect_rs::{Error, OneLibraryAdapter};
use serde_json::{json, Value};
fn canonical(value: &mut Value) {
    match value {
        Value::Number(n) => *value = json!(n.as_f64().unwrap()),
        Value::Array(a) => a.iter_mut().for_each(canonical),
        Value::Object(m) => m.values_mut().for_each(canonical),
        _ => {}
    }
}
#[test]
fn all_adapter_queries_match_actual_pinned_typescript_encrypted_fixture_outputs() {
    run(false)
}
#[test]
fn mutated_nullable_edge_queries_match_actual_typescript() {
    run(true)
}
fn run(edges: bool) {
    let scratch = tempfile::tempdir().unwrap();
    let file = scratch.path().join("fixture.db");
    std::fs::write(&file, include_bytes!("../fixtures/test-onelibrary.db")).unwrap();
    if edges {
        let writer = rusqlite::Connection::open(&file).unwrap();
        onelibrary_connect_rs::connection::set_key(
            &writer,
            &onelibrary_connect_rs::get_encryption_key().unwrap(),
        )
        .unwrap();
        writer
            .execute_batch(include_str!("../fixtures/mutations.sql"))
            .unwrap();
    }
    let mut db = OneLibraryAdapter::open_with_options(
        &file,
        onelibrary_connect_rs::Options {
            date_policy: onelibrary_connect_rs::DatePolicy::Utc,
        },
    )
    .unwrap();
    let cases: Vec<Value> = serde_json::from_str(if edges {
        include_str!("../fixtures/expected-edges.json")
    } else {
        include_str!("../fixtures/expected.json")
    })
    .unwrap();
    assert_eq!(cases.len(), 81);
    for case in cases {
        let id = case["args"][0].as_i64();
        let required = id.unwrap_or(0);
        let mut actual = match case["method"].as_str().unwrap() {
            "findTrack" => json!(db.find_track(required).unwrap()),
            "findAllTracks" => json!(db.find_all_tracks().unwrap()),
            "findCues" => json!(db.find_cues(required).unwrap()),
            "findPlaylistById" => json!(db.find_playlist_by_id(required).unwrap()),
            "findPlaylist" => json!(db.find_playlist(id).unwrap()),
            "findPlaylistContents" => json!(db.find_playlist_contents(required).unwrap()),
            "findArtist" => json!(db.find_artist(required).unwrap()),
            "findAlbum" => json!(db.find_album(required).unwrap()),
            "findGenre" => json!(db.find_genre(required).unwrap()),
            "findKey" => json!(db.find_key(required).unwrap()),
            "findColor" => json!(db.find_color(required).unwrap()),
            "findLabel" => json!(db.find_label(required).unwrap()),
            "findArtwork" => json!(db.find_artwork(required).unwrap()),
            "findMyTags" => json!(db.find_my_tags(id).unwrap()),
            "findMyTagById" => json!(db.find_my_tag_by_id(required).unwrap()),
            "findMyTagContents" => json!(db.find_my_tag_contents(required).unwrap()),
            "findMyTagsForTrack" => json!(db.find_my_tags_for_track(required).unwrap()),
            "findHistorySessions" => json!(db.find_history_sessions().unwrap()),
            "findHistoryContents" => json!(db.find_history_contents(required).unwrap()),
            "findHotCueBankLists" => json!(db.find_hot_cue_bank_lists().unwrap()),
            "findHotCueBankListCues" => json!(db.find_hot_cue_bank_list_cues(required).unwrap()),
            "findMenuItems" => json!(db.find_menu_items().unwrap()),
            "findVisibleCategories" => json!(db.find_visible_categories().unwrap()),
            "findVisibleSortOptions" => json!(db.find_visible_sort_options().unwrap()),
            "getProperty" => json!(db.get_property().unwrap()),
            _ => panic!("unknown reference query"),
        };
        let mut expected = case["result"].clone();
        canonical(&mut expected);
        canonical(&mut actual);
        assert_eq!(
            actual, expected,
            "method={} args={}",
            case["method"], case["args"]
        );
    }
    db.close();
    db.close();
    assert_eq!(db.find_track(1), Err(Error::Closed));
}
