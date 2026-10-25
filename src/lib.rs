//! Get the root folder of the app.
//!
//! Useful when the current working directory is not enough and `CARGO_MANIFEST_DIR` (a compile-time value of one
//! crate, not of the workspace or the installed app) is the wrong answer. Like `BASE_DIR` / `ROOT_DIR` in Python
//! projects, the path can be used to find files that live next to the app.
//!
//! The rule: take the folder of the running executable. If it is a cargo build folder (`target/debug`,
//! `target/release`, `target/<triple>/debug`, ...; recognised by the `.cargo-lock` file cargo puts there), go up to the
//! folder that contains `target`. So `cargo run` and a binary copied out of `target` both give the folder where the
//! app's files are.
//!
//! ```rust
//! let root = app_root_folder::get_app_root_folder();
//! assert!(root.exists());
//! ```
//!
//! Functions:
//!
//! - [`get_app_root_folder()`]: the root folder of the running app.
//! - [`get_app_root_folder_by_executable()`]: the same for a given executable path.
//! - [`get_executable_path()`]: the path of the running executable.

pub use get_app_root_folder::{
    AppRootFolderPath, get_app_root_folder, get_app_root_folder_by_executable,
};
pub use get_executable_path::{ExecutablePath, get_executable_path};

pub mod get_app_root_folder;
pub mod get_executable_path;
