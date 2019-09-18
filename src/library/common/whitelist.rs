#[cfg(target_os = "windows")]
use crate::library::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::library::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::library::platforms::macos as platform;
use std::{ffi::OsString};

// Hardcoded whitelist data for setup
static ENV_BLACKLIST: &'static [&str] = &[
    "LD_PRELOAD",
    "LD_AUDIT",
    "LD_LIBRARY_PATH"
];

#[cfg(not(feature = "whitelist_test"))]
static WHITELIST: &'static [(&str, bool, &str)] = &[
    // Tuple of (permitted program, allow unsafe environment variables, SHA3-256 hexdigest)
    // Shells
    ("/bin/bash", false, "ANY"),
    ("/bin/sh", false, "ANY"),
    // WhiteBeam
    ("/opt/WhiteBeam/whitebeam", false, "ANY"),
    ("/usr/local/bin/whitebeam", false, "ANY")
];
#[cfg(feature = "whitelist_test")]
static WHITELIST: &'static [(&str, bool, &str)] = &[
    ("/usr/bin/whoami", true, "ANY")
];

pub fn is_whitelisted(program: &str, env: &Vec<(OsString, OsString)>, hexdigest: &str) -> bool {
    let mut unsafe_env = false;
    let mut allow_exec = false;
    // TODO: Parse cache file
    let _cache_file = platform::get_cache_file();
    for env_var in env {
        if ENV_BLACKLIST.contains(&env_var.0.to_str().unwrap()) {
            unsafe_env = true;
            break;
        }
    }
    for (allowed_program, allow_unsafe, allowed_hash) in WHITELIST.iter() {
        if  (&program == allowed_program) &&
            (&unsafe_env == allow_unsafe) &&
            ((&hexdigest == allowed_hash) || (allowed_hash == &"ANY")) {
            allow_exec = true;
            break;
        }
    }
    allow_exec
}