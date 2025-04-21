use std::env;
use std::path::PathBuf;

/// Path to the executable.
pub type ExecutablePath = PathBuf;

/// Returns the path to the executable.
///
/// Fails if the executable does not exist.
pub fn get_executable_path() -> ExecutablePath {
    let executable = env::current_exe().unwrap();

    if !executable.exists() {
        panic!("The executable does not exist: {:?}", executable);
    }

    executable
}
