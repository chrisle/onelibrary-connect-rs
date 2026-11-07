// Private candidate fork: retain upstream bindings/API and the links=sqlite3 owner.
fn main() {
    use sha2::{Digest, Sha256};
    for (file, expected) in [
        ("sqlite3.c", "670d8d053176b53a68073b168f8e68fb72db67bdf964a0eb130338e9391198b9"),
        ("sqlite3.h", "8270c30673c9dccb08f0516ae63b64f898a09bc92b76d850cd912b0c1461dbe5"),
        ("sqlite3ext.h", "a3ca6e430c8e97edf8cbd66867ac178ab179a41d85c04cad48889a8b84806dcd"),
    ] {
        let path = format!("sqlite3mc/{file}");
        println!("cargo:rerun-if-changed={path}");
        let actual = format!("{:x}", Sha256::digest(std::fs::read(&path).expect("pinned MC source missing")));
        assert_eq!(actual, expected, "pinned SQLite3MC checksum mismatch: {file}");
    }
    println!("cargo:rerun-if-changed=build_sqlite3mc.rs");
    assert!(!cfg!(any(feature = "sqlcipher", feature = "bundled-sqlcipher", feature = "in_gecko", feature = "loadable_extension", feature = "wasm32-wasi-vfs")), "SQLite3MC fork does not support external SQLCipher/Gecko/loadable-extension/WASI-VFS modes");
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("target OS");
    assert!(matches!(target_os.as_str(), "macos" | "linux" | "windows"), "SQLite3MC candidate supports desktop targets only");
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("bindgen.rs");
    #[cfg(feature = "buildtime_bindgen")]
    bindings::write_to_out_dir(HeaderLocation::FromPath("sqlite3mc".to_owned()), &out_path);
    #[cfg(not(feature = "buildtime_bindgen"))]
    copy_bindings("sqlite3", "bindgen_bundled_version", &out_path);

    let mut build = cc::Build::new();
    build.file("sqlite3mc/sqlite3.c").include("sqlite3mc").warnings(false);
    for flag in ["SQLITE_CORE", "SQLITE_DEFAULT_FOREIGN_KEYS=1", "SQLITE_ENABLE_API_ARMOR", "SQLITE_ENABLE_COLUMN_METADATA", "SQLITE_ENABLE_DBSTAT_VTAB", "SQLITE_ENABLE_FTS3", "SQLITE_ENABLE_FTS3_PARENTHESIS", "SQLITE_ENABLE_FTS5", "SQLITE_ENABLE_JSON1", "SQLITE_ENABLE_LOAD_EXTENSION=1", "SQLITE_ENABLE_MEMORY_MANAGEMENT", "SQLITE_ENABLE_RTREE", "SQLITE_ENABLE_STAT4", "SQLITE_SOUNDEX", "SQLITE_THREADSAFE=1", "SQLITE_USE_URI=1"] {
        let (name,value) = flag.split_once('=').map_or((flag,None), |(k,v)|(k,Some(v)));
        build.define(name,value);
    }
    if target_os != "windows" {
        build.flag_if_supported("-std=c99").define("HAVE_USLEEP", "1").define("HAVE_LOCALTIME_R",None).define("HAVE_ISNAN",None).define("_POSIX_THREAD_SAFE_FUNCTIONS",None);
        println!("cargo:rustc-link-lib=m");
        if target_os == "linux" { println!("cargo:rustc-link-lib=dl"); println!("cargo:rustc-link-lib=pthread"); }
    } else {
        // SQLite3MC's included crypto random generator imports WinCrypt APIs.
        // MSVC has a matching pragma, GNU needs the explicit import library too.
        println!("cargo:rustc-link-lib=advapi32");
        if is_compiler("msvc") {
            build.define("_CRT_SECURE_NO_WARNINGS",None);
            let static_crt = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default().split(',').any(|f|f=="crt-static");
            build.static_crt(static_crt);
        }
    }
    for (enabled,name) in [(cfg!(feature="unlock_notify"),"SQLITE_ENABLE_UNLOCK_NOTIFY"),(cfg!(feature="preupdate_hook"),"SQLITE_ENABLE_PREUPDATE_HOOK"),(cfg!(feature="session"),"SQLITE_ENABLE_SESSION")] {
        if enabled { build.define(name,None); }
    }
    if cfg!(feature="with-asan") { build.flag("-fsanitize=address"); }
    for name in ["SQLITE_MAX_VARIABLE_NUMBER","SQLITE_MAX_EXPR_DEPTH","SQLITE_MAX_COLUMN"] {
        println!("cargo:rerun-if-env-changed={name}");
        if let Ok(value)=env::var(name) {
            assert!(!value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()), "SQLite build limit must be numeric");
            build.define(name,value.as_str());
        }
    }
    // Avoid silent engine changes through inherited upstream override env vars.
    // Build this engine regardless of SQLITE3_LIB_DIR/pkg-config/bundled features.
    build.compile("sqlite3");
    println!("cargo:include={}/sqlite3mc", env!("CARGO_MANIFEST_DIR"));
    println!("cargo:link-target=sqlite3");
}
