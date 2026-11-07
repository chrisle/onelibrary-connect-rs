# Private SQLite3MC candidate fork

Base is the complete crates.io libsqlite3-sys0.35.0, MIT, with archive SHA256 in
FORK-PROVENANCE.json. It retains upstream Rust FFI/error modules, generated
bindings, feature declarations and original native/build source. It is not a
handwritten shim. `links="sqlite3"` remains unchanged.

Changes: Cargo's cc build dependency is unconditional; sha2 verifies the pinned
MC C source/header files at build time. Existing build main was renamed
`upstream_main` and retained for review; the new included build_sqlite3mc.rs
owns the selected build. No system/pkg-config/environment engine fallback is
reachable. bundled/bundled-windows flags select this same MC engine, so feature
unification cannot switch to ordinary SQLite. SQLCipher, Gecko,
loadable-extension and WASI-VFS modes fail explicitly rather than silently
changing semantics. The original optional features still drive column metadata,
unlock notifications, preupdate hooks/session, bindgen and ASan compilation.

This candidate supports desktop macOS/Linux/Windows builds only. MSVC uses cc's
native compiler detection, no Unix-only flags; static CRT is based on the TARGET
feature list rather than build-host cfg. Windows links advapi32 for the included
RtlGenRandom branch. Unix adds localtime/usleep definitions and math; Linux adds
dl/pthread. SQLite3MC crypto is self-contained, without OpenSSL/Security framework.
Host-side SHA validation does not execute target code during cross-builds.

Existing SQLITE_MAX_VARIABLE_NUMBER/MAX_EXPR_DEPTH/MAX_COLUMN overrides remain
supported with numeric validation. External LIBSQLITE3_FLAGS/SQLITE3_LIB_DIR/
LIBSQLITE3_SYS_USE_PKG_CONFIG overrides do not switch this engine. General
compiler CC/CFLAGS variables are still cc behavior; production build isolation
must control these as for any native dependency.

Five normal tests and six extended-api tests pass on macOS ARM64; five tests also
execute successfully as x86_64 macOS binaries through this host's translation
support. This is not physical Intel machine evidence. Windows cross-build was
attempted and fails compiling the pinned C source because windows.h/SDK is
absent; no Windows success claim. Session/preupdate/buildtime-bindgen features
are retained but need their own actual compatibility checks before using them.
