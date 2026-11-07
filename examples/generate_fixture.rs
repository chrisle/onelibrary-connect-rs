//! Disposable fixture tool only; refuses to overwrite an existing file.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::PathBuf::from(
        std::env::args_os()
            .nth(1)
            .ok_or("provide a NEW disposable fixture path")?,
    );
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    drop(file);
    let db = rusqlite::Connection::open(&path)?;
    onelibrary_connect_rs::connection::set_key(&db, &onelibrary_connect_rs::get_encryption_key()?)?;
    db.execute_batch(include_str!("../fixtures/synthetic.sql"))?;
    Ok(())
}
