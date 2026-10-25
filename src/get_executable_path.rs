//! The path of the running executable.

use std::env;
use std::path::PathBuf;

/// Path to the executable.
pub type ExecutablePath = PathBuf;

/// Returns the path to the executable.
///
/// # Panics
///
/// Panics if the path cannot be determined ([`std::env::current_exe`] fails) or the executable does not exist (for
/// example, it was deleted while running).
///
/// # Examples
///
/// ```rust
/// let executable = app_root_folder::get_executable_path();
/// assert!(executable.exists());
/// ```
#[must_use]
pub fn get_executable_path() -> ExecutablePath {
    #[expect(
        clippy::expect_used,
        reason = "documented panic; the app cannot find its files without its own path"
    )]
    let executable = env::current_exe().expect("the path of the executable is unknown");

    assert!(
        executable.exists(),
        "The executable does not exist: {}",
        executable.display()
    );

    executable
}
