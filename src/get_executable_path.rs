//! The path of the running executable.

use std::env;
use std::fmt;
use std::io;
use std::path::PathBuf;

/// Path to the executable.
pub type ExecutablePath = PathBuf;

/// Why the `try_` functions could not find the executable or the app root folder.
///
/// The panicking functions panic with the same messages.
#[derive(Debug)]
#[non_exhaustive]
pub enum AppRootFolderError {
    /// [`std::env::current_exe`] failed.
    ExecutableUnknown(io::Error),
    /// The executable path does not exist (for example, it was deleted while running).
    ExecutableMissing {
        /// The path [`std::env::current_exe`] returned.
        path: PathBuf,
    },
    /// The executable path has no parent folder.
    NoParent {
        /// The executable path.
        path: PathBuf,
    },
    /// The executable is in a cargo build folder, but no parent folder holds a `target` folder.
    NoTargetFolder {
        /// The executable path.
        path: PathBuf,
    },
    /// The app root folder found does not exist.
    RootMissing {
        /// The app root folder.
        path: PathBuf,
    },
}

impl fmt::Display for AppRootFolderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutableUnknown(error) => {
                write!(f, "the path of the executable is unknown: {error}")
            }
            Self::ExecutableMissing { path } => {
                write!(f, "The executable does not exist: {}", path.display())
            }
            Self::NoParent { path } => {
                write!(f, "The executable has no parent folder: {}", path.display())
            }
            Self::NoTargetFolder { path } => {
                write!(
                    f,
                    "No parent folder with a 'target' folder for: {}",
                    path.display()
                )
            }
            Self::RootMissing { path } => {
                write!(f, "The main folder does not exist: {}", path.display())
            }
        }
    }
}

impl std::error::Error for AppRootFolderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ExecutableUnknown(error) => Some(error),
            _ => None,
        }
    }
}

/// Returns the path to the executable, or why it is unknown.
///
/// # Errors
///
/// [`AppRootFolderError::ExecutableUnknown`] if [`std::env::current_exe`] fails,
/// [`AppRootFolderError::ExecutableMissing`] if the executable does not exist.
///
/// # Examples
///
/// ```rust
/// let executable = app_root_folder::try_get_executable_path()?;
/// assert!(executable.exists());
/// # Ok::<(), app_root_folder::AppRootFolderError>(())
/// ```
pub fn try_get_executable_path() -> Result<ExecutablePath, AppRootFolderError> {
    let executable = env::current_exe().map_err(AppRootFolderError::ExecutableUnknown)?;
    if executable.exists() {
        Ok(executable)
    } else {
        Err(AppRootFolderError::ExecutableMissing { path: executable })
    }
}

/// Returns the path to the executable.
///
/// # Panics
///
/// Panics if the path cannot be determined ([`std::env::current_exe`] fails) or the executable does not exist (for
/// example, it was deleted while running). [`try_get_executable_path`] returns these as errors.
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
        clippy::panic,
        reason = "the panicking twin of the try_ function, documented under # Panics"
    )]
    try_get_executable_path().unwrap_or_else(|error| panic!("{error}"))
}
