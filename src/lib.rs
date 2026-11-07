//! Read-only OneLibrary encrypted database adapter, compatible with pinned TS1.1.6.
mod adapter;
pub mod connection;
mod encryption;
pub mod schema;
pub mod types;
pub use adapter::{DatabaseAdapter, DatePolicy, OneLibraryAdapter, Options};
pub use connection::{engine_version, open_one_library_db};
pub use encryption::get_encryption_key;
pub use schema::*;
pub use types::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    UnsupportedEngine,
    OpenFailed,
    InvalidDatabase,
    InvalidSchema,
    QueryFailed,
    Closed,
    InvalidKeyDerivation,
    ResultLimit,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::UnsupportedEngine => "Unsupported OneLibrary SQLite engine",
            Self::OpenFailed => "Cannot open OneLibrary database read-only",
            Self::InvalidDatabase => "Cannot read encrypted OneLibrary database",
            Self::InvalidSchema => "OneLibrary schema is invalid",
            Self::QueryFailed => "OneLibrary query failed",
            Self::Closed => "OneLibrary adapter is closed",
            Self::InvalidKeyDerivation => "Cannot derive OneLibrary key",
            Self::ResultLimit => "OneLibrary query exceeds configured safety limit",
        })
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;
