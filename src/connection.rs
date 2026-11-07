use rusqlite::{Connection, OpenFlags};
use std::{
    ffi::{c_char, c_int, c_void, CStr},
    path::Path,
};
unsafe extern "C" {
    fn sqlite3_key(db: *mut rusqlite::ffi::sqlite3, key: *const c_void, len: c_int) -> c_int;
    fn sqlite3mc_version() -> *const c_char;
}

pub fn engine_version() -> &'static str {
    // SQLite3MC returns a static, NUL-terminated string.
    unsafe { CStr::from_ptr(sqlite3mc_version()) }
        .to_str()
        .expect("engine version UTF8")
}

/// Configure before the first schema/page access. Key bytes never enter SQL.
/// This does not validate a key: the caller must perform an actual page read.
pub fn set_key(connection: &Connection, key: &[u8]) -> rusqlite::Result<()> {
    let len = c_int::try_from(key.len()).map_err(|_| rusqlite::Error::InvalidQuery)?;
    connection.execute_batch("PRAGMA cipher=sqlcipher; PRAGMA legacy=4;")?;
    // The key API copies/derives synchronously; connection owns a valid handle.
    let rc = unsafe { sqlite3_key(connection.handle(), key.as_ptr().cast(), len) };
    if rc != rusqlite::ffi::SQLITE_OK {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rc),
            None,
        ));
    }
    Ok(())
}

pub fn open_one_library_db(path: impl AsRef<Path>) -> crate::Result<Connection> {
    if engine_version() != "SQLite3 Multiple Ciphers 2.3.5" || rusqlite::version() != "3.53.2" {
        return Err(crate::Error::UnsupportedEngine);
    }
    let db = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|_| crate::Error::OpenFailed)?;
    db.set_limit(
        rusqlite::limits::Limit::SQLITE_LIMIT_LENGTH,
        16 * 1024 * 1024,
    )
    .map_err(|_| crate::Error::OpenFailed)?;
    db.busy_timeout(std::time::Duration::from_millis(2000))
        .map_err(|_| crate::Error::OpenFailed)?;
    let key = crate::get_encryption_key()?;
    set_key(&db, &key).map_err(|_| crate::Error::InvalidDatabase)?;
    db.query_row("SELECT count(*) FROM sqlite_schema", [], |r| {
        r.get::<_, i64>(0)
    })
    .map_err(|_| crate::Error::InvalidDatabase)?;
    Ok(db)
}
