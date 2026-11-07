# onelibrary-connect-rs

Read-only Rust port of the reusable TypeScript `onelibrary-connect`1.1.6 library,
pinned at `ac3edd3b8a18f7649d336ebd56d28b15923a05d6`. It reads encrypted
`exportLibrary.db` files using SQLite3 Multiple Ciphers SQLCipher compatibility4.
Public source: https://github.com/chrisle/onelibrary-connect-rs.
Application integration and live-software acceptance remain in progress.
This is not an AlphaTheta application/protocol port.

```rust,no_run
use onelibrary_connect_rs::OneLibraryAdapter;
let db = OneLibraryAdapter::open("/explicit/path/exportLibrary.db")?;
let track = db.find_track(1)?;
let root = db.find_playlist(None)?;
let cues = db.find_cues(1)?;
# Ok::<(), onelibrary_connect_rs::Error>(())
```

No database discovery, personal options, credential stores, network access,
rekeying or writes occur in the adapter. Opens use READ_ONLY and do not force
immutable mode. Keys pass through `sqlite3_key` as bytes, never interpolated SQL.
A real encrypted schema read plus preparation of all adapter tables/columns
succeeds before `open` returns. Errors omit paths, SQL and key material.

## One engine is required

The standalone workspace patches `libsqlite3-sys0.35.0` with the included genuine
upstream fork, maintaining its `links=sqlite3` ownership, Rust bindings and API.
The fork compiles checksum-pinned SQLite3MC2.3.5/SQLite3.53.2 through Cargo/cc,
without shell prebuilds or system SQLite lookup. No OpenSSL installation is
needed. `libsqlite3-sys/FORK.md` and its provenance/license files describe the
reviewed build change. The reader asserts the actual linked codec/version.

**Consumers must repeat the patch at their workspace root.** Cargo does not
apply dependency-local patch sections:

```toml
[dependencies]
onelibrary-connect-rs = { path = "vendor/onelibrary-connect-rs" }
# Other readers may use the same rusqlite0.37 instance.

[patch.crates-io]
libsqlite3-sys = { path = "vendor/onelibrary-connect-rs/libsqlite3-sys" }
```

Keep all SQLite consumers on compatible rusqlite0.37/libsqlite3-sys0.35 versions.
Do not introduce another SQLite native link owner. Bundled feature unification
still selects this MC build. Incompatible SQLCipher/Gecko/loadable-extension
modes fail explicitly. This is a source dependency integration candidate,
not a crates.io release or approved app dependency change.

## API coverage

All public TS adapter read methods have snake_case Rust equivalents. Every
query returns `Result`; missing single items return `Ok(None)`, missing lists
return empty lists. The TS default relation form is `Track`; `TrackFks` and
`PlaylistEntryFks` represent explicit foreign-key forms. Named reference entities,
cue union, configuration entities, raw schema row structs and constants are
exported from the crate root. `DatabaseAdapter`, `DatabaseType`,
`DatabasePreference` and `EntityFK` preserve the reusable interface concepts.
See [EXPORTS.md](EXPORTS.md) for the complete mapping.

IDs and sequence indices are integer Rust types; metadata numeric fields and cue
milliseconds allow fractional values. Cue colors/buttons retain raw numeric
values so unknown source values are not discarded. Serde uses the original
camelCase keys, preserves required null relations, omits undefined optionals,
and serializes invalid dates as null. `analyzePath` removes the final four UTF16
code units like the TS substring operation when the resulting prefix is valid
UTF16, including short strings. If truncation splits a surrogate pair (for
example `😀abc`), Rust deliberately omits the invalid native path; it never
manufactures a different path containing U+FFFD. Valid Unicode `.DAT` paths
retain their exact prefix.

`Options { date_policy: DatePolicy::Local }` is the default: ISO date-only values
are UTC, offset datetime values keep their instant, and naive datetime values
use the host timezone like JS Date. Overlaps choose the earlier instant and
nonexistent local times advance by the timezone gap. `DatePolicy::Utc` makes
naive datetimes deterministic for fixture tools. Supported date inputs are ISO
calendar dates and ISO/SQLite datetime forms with optional fractional seconds;
ISO day values1..31 normalize excess month days like JavaScript (including
February30 and April31); month0/13 and day0/32 are invalid. ISO24:00 advances a
day only with zero minutes/seconds/fraction; invalid24-hour forms and leap seconds
are rejected. Engine-specific JavaScript permissive parsing of other nonstandard
date strings is not claimed. `date_added`/`analyze_date` expose optional JSON ISO date values;
`analyze_date` remains unset because the original adapter never sets it.

Deliberate safety differences: `open` verifies schema eagerly rather than waiting
for the first query; repeated `close()` is harmless; individual SQLite values
are limited to16MiB, each query to1million rows and128MiB aggregate text; invalid
UTF8 and BLOBs in the documented scalar schema return an error. Row IDs beyond
JavaScript's safe integer range retain Rust integer precision. These bounds and
strict schema errors are not silent truncation. Queries project only mapped
scalar fields, so unknown schema extension columns (including BLOBs) are ignored
without decoding or allocating their payloads. BLOBs in mapped scalar fields
still return an explicit query error. TS queries without ORDER BY keep
their original SQLite query order; only explicitly ordered queries promise order.

## Verification

```sh
cargo test --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo fmt --check
npm ci --prefix tools --no-audit --no-fund
node tools/regenerate.cjs
```

Thirteen tests pass locally on macOS ARM64. They include two actual pinned-TS
comparisons covering81 query cases each, read-only hash/write-denial, missing/
wrong-key/corrupt/truncated/plaintext/schema failures, encrypted WAL commit and
rollback visibility, synthetic reproduction, UTC/local DST date behavior, and
actual-TS oracle coverage for23 calendar/24-hour cases and7 UTF16 path cases.
The latter records JS code units to make the intentional invalid-path correction
explicit rather than declaring false parity on a lone surrogate.
The Unix timezone regression runs in an isolated child process; Windows still
runs the core ISO/UTC and encrypted database suites.

The original TypeScript adapter runs with a fresh native addon12.11.1 built
inside `tools/node_modules` for Node24; no original checkout dependencies were
modified, and no ABI-skipped test run is counted. `tools/node-evidence.json`
records the actual addon engine. Both original encrypted fixture and nullable/
fractional/orphan/Unicode/date/cue edge mutations match Rust.
Actual pinned TypeScript also reads disposable encrypted schema extensions with
BLOB columns in every adapter table and retains all81 base query outputs; Rust
matches those outputs and separately rejects known scalar BLOB corruption. A
Rust-generated encrypted fixture is read by original TS and matches all81 base
query cases. Only numeric JSON representation is normalized (`128` versus
`128.0`); field presence, nulls, values and array order remain compared.

`fixtures/synthetic.sql` is the checked redistributable mock's schema/data dump;
`cargo run --example generate_fixture -- NEW_DISPOSABLE_PATH` recreates an
encrypted synthetic database and refuses to overwrite an existing file. The
example is a fixture tool, not a writable connector API. The original fixture
hash remains `7a47f162edaef42fe03fad836013ad4fe3e1b1cc8f5604173a97ece74d8f08bb`.
All engine tests use disposable copies; reference source checksums are recorded.

CI covers macOS/Windows/Linux and rebuilds the original TS oracle without skips.
Those remote jobs have not run for this unpublished candidate. The precursor
engine proof compiled/executed x86_64 macOS binaries under translation; this
library has not yet been exercised on physical Intel hardware. Windows SDK was
absent on the local host, so Windows compilation/runtime remains an explicit
release gate. No live DJ app, personal database or physical hardware verification
is claimed.

License: MIT, retaining the original library and native engine/component notices.
