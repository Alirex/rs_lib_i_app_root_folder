//! The root folder of the app.

use crate::get_executable_path::{AppRootFolderError, ExecutablePath, try_get_executable_path};
use std::path::PathBuf;

/// Path to the app root folder.
///
/// This is the folder where the main files of the project are.
///
/// This folder is a root of a "workspace" or "project" in the sense that it contains
/// the main files of the project.
///
/// If the binary is moved out of the build folder, it is the folder of the binary.
pub type AppRootFolderPath = PathBuf;

/// Returns the app root folder for the given executable, or why it cannot be found.
///
/// If the executable's folder is a cargo build folder (it holds a `.cargo-lock` file), the result is the nearest parent
/// folder that contains a `target` folder, for example:
///
/// - `target/debug`
/// - `target/release`
/// - `target/{target}/debug`
/// - `target/{target}/release`
///
/// Otherwise the folder of the executable already is the app root folder.
///
/// # Errors
///
/// [`AppRootFolderError::NoParent`] if the executable has no parent folder, [`AppRootFolderError::NoTargetFolder`] if
/// no parent folder with a `target` folder is found while going up from a build folder,
/// [`AppRootFolderError::RootMissing`] if the resulting folder does not exist.
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
///
/// // Not a cargo build folder: the folder of the executable is the root.
/// let executable = std::env::current_exe()?;
/// let root = app_root_folder::try_get_app_root_folder_by_executable(&executable)?;
/// assert_eq!(root, executable.parent().unwrap_or(Path::new("/")));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn try_get_app_root_folder_by_executable(
    executable: &ExecutablePath,
) -> Result<AppRootFolderPath, AppRootFolderError> {
    let main_folder_candidate =
        executable
            .parent()
            .ok_or_else(|| AppRootFolderError::NoParent {
                path: executable.clone(),
            })?;

    // TODO: ? Make better indicator?
    let build_folder_indicator = main_folder_candidate.join(".cargo-lock");

    let root: PathBuf = if build_folder_indicator.exists() {
        // The executable is in the target/..., we need to go up.
        main_folder_candidate
            .ancestors()
            .find(|folder| folder.join("target").exists())
            .ok_or_else(|| AppRootFolderError::NoTargetFolder {
                path: executable.clone(),
            })?
            .into()
    } else {
        // The parent of the binary is already the app root folder.
        main_folder_candidate.into()
    };

    if root.exists() {
        Ok(root)
    } else {
        Err(AppRootFolderError::RootMissing { path: root })
    }
}

/// Returns the app root folder for the given executable.
///
/// The rule is in [`try_get_app_root_folder_by_executable`].
///
/// # Panics
///
/// Panics if the executable has no parent folder, if no parent folder with a `target` folder is found while going up
/// from a build folder, or if the resulting folder does not exist.
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
///
/// // Not a cargo build folder: the folder of the executable is the root.
/// let root = app_root_folder::get_app_root_folder_by_executable(&std::env::current_exe().unwrap());
/// assert_eq!(root, Path::new(&std::env::current_exe().unwrap()).parent().unwrap());
/// ```
#[must_use]
pub fn get_app_root_folder_by_executable(executable: &ExecutablePath) -> AppRootFolderPath {
    #[expect(
        clippy::panic,
        reason = "the panicking twin of the try_ function, documented under # Panics"
    )]
    try_get_app_root_folder_by_executable(executable).unwrap_or_else(|error| panic!("{error}"))
}

/// Returns the app root folder of the running executable, or why it cannot be found.
///
/// The rule is in [`try_get_app_root_folder_by_executable`].
///
/// # Errors
///
/// The errors of [`try_get_executable_path`] and [`try_get_app_root_folder_by_executable`].
///
/// # Examples
///
/// ```rust
/// let root = app_root_folder::try_get_app_root_folder()?;
/// assert!(root.is_dir());
/// # Ok::<(), app_root_folder::AppRootFolderError>(())
/// ```
pub fn try_get_app_root_folder() -> Result<AppRootFolderPath, AppRootFolderError> {
    try_get_app_root_folder_by_executable(&try_get_executable_path()?)
}

/// Returns the app root folder of the running executable.
///
/// The rule is in [`try_get_app_root_folder_by_executable`].
///
/// # Panics
///
/// Panics in the cases where [`try_get_app_root_folder`] returns an error.
///
/// # Examples
///
/// ```rust
/// let root = app_root_folder::get_app_root_folder();
/// assert!(root.is_dir());
/// ```
#[must_use]
pub fn get_app_root_folder() -> AppRootFolderPath {
    #[expect(
        clippy::panic,
        reason = "the panicking twin of the try_ function, documented under # Panics"
    )]
    try_get_app_root_folder().unwrap_or_else(|error| panic!("{error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    /// A scratch folder under the system temp dir, removed on drop.
    struct Scratch(PathBuf);

    impl Scratch {
        fn new(name: &str) -> Self {
            let path =
                std::env::temp_dir().join(format!("app_root_folder_{}_{name}", std::process::id()));
            if let Err(error) = fs::remove_dir_all(&path) {
                assert_eq!(error.kind(), std::io::ErrorKind::NotFound, "{error}");
            }
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            #[expect(
                clippy::let_underscore_must_use,
                reason = "cleanup on drop; a leftover temp folder is harmless"
            )]
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn executable_in(folder: &Path) -> ExecutablePath {
        let executable = folder.join("app");
        fs::write(&executable, "").unwrap();
        executable
    }

    #[test]
    fn executable_outside_target_gives_its_own_folder() {
        let scratch = Scratch::new("plain");
        let executable = executable_in(&scratch.0);
        assert_eq!(get_app_root_folder_by_executable(&executable), scratch.0);
    }

    #[test]
    fn executable_in_target_debug_gives_the_project_folder() {
        let scratch = Scratch::new("debug");
        let build = scratch.0.join("target").join("debug");
        fs::create_dir_all(&build).unwrap();
        fs::write(build.join(".cargo-lock"), "").unwrap();
        let executable = executable_in(&build);
        assert_eq!(get_app_root_folder_by_executable(&executable), scratch.0);
    }

    #[test]
    fn executable_in_target_triple_release_gives_the_project_folder() {
        let scratch = Scratch::new("triple");
        let build = scratch
            .0
            .join("target")
            .join("x86_64-unknown-linux-gnu")
            .join("release");
        fs::create_dir_all(&build).unwrap();
        fs::write(build.join(".cargo-lock"), "").unwrap();
        let executable = executable_in(&build);
        assert_eq!(get_app_root_folder_by_executable(&executable), scratch.0);
    }

    #[test]
    #[should_panic(expected = "No parent folder with a 'target' folder")]
    fn build_folder_without_target_parent_panics() {
        let scratch = Scratch::new("no_target");
        let build = scratch.0.join("build");
        fs::create_dir_all(&build).unwrap();
        fs::write(build.join(".cargo-lock"), "").unwrap();
        let executable = executable_in(&build);
        let _root = get_app_root_folder_by_executable(&executable);
    }

    #[test]
    fn running_app_has_an_existing_root_folder() {
        assert!(get_app_root_folder().is_dir());
    }

    #[test]
    fn try_without_parent_is_no_parent() {
        let executable = ExecutablePath::from("/");
        let error = try_get_app_root_folder_by_executable(&executable).unwrap_err();
        assert!(
            matches!(&error, AppRootFolderError::NoParent { path } if path == Path::new("/")),
            "{error:?}"
        );
    }

    #[test]
    fn try_build_folder_without_target_parent_is_no_target_folder() {
        let scratch = Scratch::new("try_no_target");
        let build = scratch.0.join("build");
        fs::create_dir_all(&build).unwrap();
        fs::write(build.join(".cargo-lock"), "").unwrap();
        let executable = executable_in(&build);
        let error = try_get_app_root_folder_by_executable(&executable).unwrap_err();
        assert!(
            matches!(&error, AppRootFolderError::NoTargetFolder { path } if *path == executable),
            "{error:?}"
        );
    }

    #[test]
    fn try_missing_folder_is_root_missing() {
        let scratch = Scratch::new("try_missing");
        let missing = scratch.0.join("gone");
        let error = try_get_app_root_folder_by_executable(&missing.join("app")).unwrap_err();
        assert!(
            matches!(&error, AppRootFolderError::RootMissing { path } if *path == missing),
            "{error:?}"
        );
    }

    #[test]
    fn try_running_app_has_an_existing_root_folder() {
        assert!(try_get_app_root_folder().unwrap().is_dir());
    }
}
