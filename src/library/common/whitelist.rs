#[cfg(target_os = "windows")]
use crate::library::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::library::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::library::platforms::macos as platform;
use crate::library::common::db;
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
    for env_var in env {
        if ENV_BLACKLIST.contains(&env_var.0.to_str().unwrap()) {
            unsafe_env = true;
            break;
        }
    }
    // Introduced limitation:
    // WhiteBeam is permissive for up to 5 minutes after boot to avoid interfering with the boot
    // process. While attackers should not be able to reboot a system due to whitelisting policy,
    // this is a weakness while WhiteBeam is actively developed. Alternatives include:
    // 1. Whitelisting all binaries by default, including malware (other EDR software use
    //    this approach, maintaining a large database of permitted executables)
    // 2. Require a reboot to baseline systems (which may interfere with production systems)
    // Feedback/ideas welcome: https://github.com/noproto/WhiteBeam/issues
    if platform::get_uptime().unwrap().as_secs() < (60*5) {
        allow_exec = true;
    }
    for (allowed_program, allow_unsafe, allowed_hash) in WHITELIST.iter() {
        if  (&program == allowed_program) &&
            (&unsafe_env == allow_unsafe) &&
            ((&hexdigest == allowed_hash) || (allowed_hash == &"ANY")) {
            allow_exec = true;
            break;
        }
    }
    let conn = db::open();
    for dyn_result in db::get_dyn_whitelist(&conn).iter() {
        if  (&program == &dyn_result.program) &&
            (&unsafe_env == &dyn_result.allow_unsafe) &&
            ((&hexdigest == &dyn_result.hash) || (&dyn_result.hash == &"ANY")) {
            allow_exec = true;
            break;
        }
    }
    allow_exec
}
