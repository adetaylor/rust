//! This module contains paths to types and functions Clippy needs to know
//! about.
//!
//! Whenever possible, please consider diagnostic items over hardcoded paths.
//! See <https://github.com/rust-lang/rust-clippy/issues/5393> for more information.

pub const APPLICABILITY: [&str; 2] = ["rustc_lint_defs", "Applicability"];
pub const APPLICABILITY_VALUES: [[&str; 3]; 4] = [
    ["rustc_lint_defs", "Applicability", "Unspecified"],
    ["rustc_lint_defs", "Applicability", "HasPlaceholders"],
    ["rustc_lint_defs", "Applicability", "MaybeIncorrect"],
    ["rustc_lint_defs", "Applicability", "MachineApplicable"],
];
pub const DIAG: [&str; 2] = ["rustc_errors", "Diag"];
pub const EARLY_CONTEXT: [&str; 2] = ["rustc_lint", "EarlyContext"];
pub const EARLY_LINT_PASS: [&str; 3] = ["rustc_lint", "passes", "EarlyLintPass"];
pub const FILE_OPTIONS: [&str; 4] = ["std", "fs", "File", "options"];
#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const FUTURES_IO_ASYNCREADEXT: [&str; 3] = ["futures_util", "io", "AsyncReadExt"];
#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const FUTURES_IO_ASYNCWRITEEXT: [&str; 3] = ["futures_util", "io", "AsyncWriteExt"];
pub const HASHMAP_ITER: [&str; 5] = ["std", "collections", "hash", "map", "Iter"];
pub const HASHMAP_ITER_MUT: [&str; 5] = ["std", "collections", "hash", "map", "IterMut"];
pub const HASHMAP_KEYS: [&str; 5] = ["std", "collections", "hash", "map", "Keys"];
pub const HASHMAP_VALUES: [&str; 5] = ["std", "collections", "hash", "map", "Values"];
pub const HASHMAP_DRAIN: [&str; 5] = ["std", "collections", "hash", "map", "Drain"];
pub const HASHMAP_VALUES_MUT: [&str; 5] = ["std", "collections", "hash", "map", "ValuesMut"];
pub const HASHSET_ITER_TY: [&str; 5] = ["std", "collections", "hash", "set", "Iter"];
pub const HASHSET_DRAIN: [&str; 5] = ["std", "collections", "hash", "set", "Drain"];
pub const IDENT: [&str; 3] = ["rustc_span", "symbol", "Ident"];
pub const IDENT_AS_STR: [&str; 4] = ["rustc_span", "symbol", "Ident", "as_str"];
pub const ITERTOOLS_NEXT_TUPLE: [&str; 3] = ["itertools", "Itertools", "next_tuple"];
pub const KW_MODULE: [&str; 3] = ["rustc_span", "symbol", "kw"];
pub const LATE_CONTEXT: [&str; 2] = ["rustc_lint", "LateContext"];
pub const LATE_LINT_PASS: [&str; 3] = ["rustc_lint", "passes", "LateLintPass"];
pub const LINT: [&str; 2] = ["rustc_lint_defs", "Lint"];
pub const MSRV: [&str; 3] = ["clippy_config", "msrvs", "Msrv"];
pub const OPEN_OPTIONS_NEW: [&str; 4] = ["std", "fs", "OpenOptions", "new"];
pub const PARKING_LOT_MUTEX_GUARD: [&str; 3] = ["lock_api", "mutex", "MutexGuard"];
pub const PARKING_LOT_RWLOCK_READ_GUARD: [&str; 3] = ["lock_api", "rwlock", "RwLockReadGuard"];
pub const PARKING_LOT_RWLOCK_WRITE_GUARD: [&str; 3] = ["lock_api", "rwlock", "RwLockWriteGuard"];
#[cfg_attr(not(unix), allow(clippy::invalid_paths))]
pub const PERMISSIONS_FROM_MODE: [&str; 6] = ["std", "os", "unix", "fs", "PermissionsExt", "from_mode"];
pub const REGEX_BUILDER_NEW: [&str; 3] = ["regex", "RegexBuilder", "new"];
pub const REGEX_BYTES_BUILDER_NEW: [&str; 4] = ["regex", "bytes", "RegexBuilder", "new"];
pub const REGEX_BYTES_NEW: [&str; 4] = ["regex", "bytes", "Regex", "new"];
pub const REGEX_BYTES_SET_NEW: [&str; 4] = ["regex", "bytes", "RegexSet", "new"];
pub const REGEX_NEW: [&str; 3] = ["regex", "Regex", "new"];
pub const REGEX_SET_NEW: [&str; 3] = ["regex", "RegexSet", "new"];
pub const SERDE_DESERIALIZE: [&str; 3] = ["serde", "de", "Deserialize"];
pub const SERDE_DE_VISITOR: [&str; 3] = ["serde", "de", "Visitor"];
pub const STD_IO_SEEK_FROM_CURRENT: [&str; 4] = ["std", "io", "SeekFrom", "Current"];
pub const STD_IO_SEEKFROM_START: [&str; 4] = ["std", "io", "SeekFrom", "Start"];
pub const STRING_NEW: [&str; 4] = ["alloc", "string", "String", "new"];
pub const STR_ENDS_WITH: [&str; 4] = ["core", "str", "<impl str>", "ends_with"];
pub const STR_LEN: [&str; 4] = ["core", "str", "<impl str>", "len"];
pub const STR_STARTS_WITH: [&str; 4] = ["core", "str", "<impl str>", "starts_with"];
pub const SYMBOL: [&str; 3] = ["rustc_span", "symbol", "Symbol"];
pub const SYMBOL_AS_STR: [&str; 4] = ["rustc_span", "symbol", "Symbol", "as_str"];
pub const SYMBOL_INTERN: [&str; 4] = ["rustc_span", "symbol", "Symbol", "intern"];
pub const SYMBOL_TO_IDENT_STRING: [&str; 4] = ["rustc_span", "symbol", "Symbol", "to_ident_string"];
pub const SYM_MODULE: [&str; 3] = ["rustc_span", "symbol", "sym"];
pub const SYNTAX_CONTEXT: [&str; 3] = ["rustc_span", "hygiene", "SyntaxContext"];
pub const STRING_FROM_UTF8: [&str; 4] = ["alloc", "string", "String", "from_utf8"];
#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const TOKIO_FILE_OPTIONS: [&str; 5] = ["tokio", "fs", "file", "File", "options"];
#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const TOKIO_IO_ASYNCREADEXT: [&str; 5] = ["tokio", "io", "util", "async_read_ext", "AsyncReadExt"];
#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const TOKIO_IO_ASYNCWRITEEXT: [&str; 5] = ["tokio", "io", "util", "async_write_ext", "AsyncWriteExt"];
#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const TOKIO_IO_OPEN_OPTIONS: [&str; 4] = ["tokio", "fs", "open_options", "OpenOptions"];
#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const TOKIO_IO_OPEN_OPTIONS_NEW: [&str; 5] = ["tokio", "fs", "open_options", "OpenOptions", "new"];
pub const INSTANT_NOW: [&str; 4] = ["std", "time", "Instant", "now"];
pub const WAKER: [&str; 4] = ["core", "task", "wake", "Waker"];
pub const BOOL_THEN: [&str; 4] = ["core", "bool", "<impl bool>", "then"];
