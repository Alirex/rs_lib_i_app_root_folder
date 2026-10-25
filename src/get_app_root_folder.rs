//! The root folder of the app.

use crate::get_executable_path::{ExecutablePath, get_executable_path};
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

/// Returns the app root folder for the given executable.
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
    let main_folder_candidate: PathBuf = executable
        .parent()
        .unwrap_or_else(|| {
            panic!(
                "The executable has no parent folder: {}",
                executable.display()
            )
        })
        .into();

    // TODO: ? Make better indicator?
    let build_folder_indicator = main_folder_candidate.join(".cargo-lock");

    let root = if build_folder_indicator.exists() {
        // The executable is in the target/..., we need to go up.
        main_folder_candidate
            .ancestors()
            .find(|folder| folder.join("target").exists())
            .unwrap_or_else(|| {
                panic!(
                    "No parent folder with a 'target' folder for: {}",
                    executable.display()
                )
            })
            .into()
    } else {
        // The parent of the binary is already the app root folder.
        main_folder_candidate
    };

    assert!(
        root.exists(),
        "The main folder does not exist: {}",
        root.display()
    );

    root
}

/// Returns the app root folder of the running executable.
///
/// See [`get_app_root_folder_by_executable`] for the rule.
///
/// # Panics
///
/// Panics in the same cases as [`get_executable_path()`] and [`get_app_root_folder_by_executable()`].
///
/// # Examples
///
/// ```rust
/// let root = app_root_folder::get_app_root_folder();
/// assert!(root.is_dir());
/// ```
#[must_use]
pub fn get_app_root_folder() -> AppRootFolderPath {
    get_app_root_folder_by_executable(&get_executable_path())
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
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
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
        let _ = get_app_root_folder_by_executable(&executable);
    }

    #[test]
    fn running_app_has_an_existing_root_folder() {
        assert!(get_app_root_folder().is_dir());
    }
}
