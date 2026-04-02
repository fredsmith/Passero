use std::env;
use std::ffi::OsString;

/// Return an augmented PATH that includes common tool directories.
/// macOS GUI apps don't inherit the user's shell PATH, so tools like
/// pass, gpg, and git in /opt/homebrew/bin or /usr/local/bin aren't found.
pub fn augmented_path() -> OsString {
    let extra_dirs = [
        "/opt/homebrew/bin",
        "/opt/homebrew/sbin",
        "/usr/local/bin",
        "/usr/local/sbin",
    ];

    let current = env::var_os("PATH").unwrap_or_default();
    let mut path = OsString::new();
    for dir in &extra_dirs {
        path.push(dir);
        path.push(":");
    }
    path.push(&current);
    path
}
