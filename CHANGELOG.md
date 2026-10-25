# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project follows
[Semantic Versioning](https://semver.org/) (while `0.x`: a breaking change bumps the minor version).

## [Unreleased]

### Added

- `try_get_executable_path`, `try_get_app_root_folder_by_executable` and `try_get_app_root_folder`: the same
  lookups returning `Result<_, AppRootFolderError>` instead of panicking. The error enum (`#[non_exhaustive]`, no new
  dependency) has one variant per former panic, with the same message; the panicking functions are unchanged.

## [0.1.1] - 2026-10-25

### Changed

- Crate documentation on docs.rs: an overview, `# Panics` sections and tested examples; the unfinished sentence on
  `AppRootFolderPath` is completed.
- The functions are `#[must_use]`; panic messages print the path with `Display`.
- Going up from a build folder no longer loops on `parent().unwrap()`: it walks the ancestors and panics with a clear
  message when no folder with `target` is found.
- `rust-version = "1.85"` (edition 2024) is now declared in `Cargo.toml`; keywords and description are more specific.
- The package contains only the sources, README, LICENSE and CHANGELOG.

### Added

- Unit tests for the plain, `target/debug`, `target/<triple>/release` and missing-`target` cases.

## [0.1.0] - 2025-04-21

First release on crates.io (the source is the commit `e561dc0`; there was no git tag, `v0.1.0` now points to it).

[Unreleased]: https://github.com/Alirex/rs_lib_i_app_root_folder/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/Alirex/rs_lib_i_app_root_folder/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Alirex/rs_lib_i_app_root_folder/releases/tag/v0.1.0
