use crate::get_executable_path::{ExecutablePath, get_executable_path};
use std::path::PathBuf;

/// Path to the app root folder.
///
/// This is the folder where the main files of the project are.
///
/// This folder is a root of a "workspace" or "project" in the sense that it contains
/// the main files of the project.
///
/// And, if binary is moved out of the build folder, it will
pub type AppRootFolderPath = PathBuf;

pub fn get_app_root_folder_by_executable(executable: &ExecutablePath) -> AppRootFolderPath {
    let mut main_folder_candidate: PathBuf = executable.parent().unwrap().into();

    // If the path-indicator exists, then expected something like this:
    // - target/debug
    // - target/release
    // - target/{target}/debug
    // - target/{target}/release
    // Else:
    // - parent of binary is already the app root folder

    // TODO: ? Make better indicator?
    let build_folder_indicator = main_folder_candidate.join(".cargo-lock");

    if build_folder_indicator.exists() {
        // If the executable is in the target/..., we need to go up.

        while !main_folder_candidate.join("target").exists() {
            main_folder_candidate = main_folder_candidate.parent().unwrap().into();
        }
    }

    if !main_folder_candidate.exists() {
        panic!(
            "The main folder does not exist: {:?}",
            main_folder_candidate
        );
    }

    main_folder_candidate
}

pub fn get_app_root_folder() -> AppRootFolderPath {
    get_app_root_folder_by_executable(&get_executable_path())
}
