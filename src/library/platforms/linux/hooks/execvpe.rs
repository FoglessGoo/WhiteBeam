use libc::{c_char, c_int};
use crate::platforms::linux;
use crate::common::whitelist;
use crate::common::event;
use crate::common::hash;

/*
       int execvpe(const char *path, char *const argv[],
                       char *const envp[]);
*/
#[no_mangle]
pub unsafe extern "C" fn execvpe(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int {
    let program = linux::c_char_to_osstring(path);
    let env = linux::parse_env_collection(envp);
    let which_abs_pathbuf = match linux::search_path(&program) {
        Some(prog_path) => prog_path,
        None => {
			*linux::errno_location() = libc::ENOENT;
			return -1 }
    };
	let abs_prog_str = which_abs_pathbuf.as_os_str();
    let hexdigest = hash::common_hash_file(abs_prog_str);
    let uid = linux::get_current_uid();
    // Permit/deny execution
    if whitelist::is_whitelisted(abs_prog_str, &env, &hexdigest) {
        event::send_exec_event(uid, abs_prog_str, &hexdigest, true);
        // Pass through
        static mut REAL: *const u8 = 0 as *const u8;
        static mut ONCE: ::std::sync::Once = ::std::sync::Once::new();
        ONCE.call_once(|| {
            REAL = crate::platforms::linux::dlsym_next("execvpe\u{0}");
        });
        let execvpe_real: unsafe extern "C" fn(path: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int = std::mem::transmute(REAL);
        execvpe_real(path, argv, envp)
    } else {
        event::send_exec_event(uid, abs_prog_str, &hexdigest, false);
        *linux::errno_location() = libc::EACCES;
        return -1
    }
}
