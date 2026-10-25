# App root folder

[![CI](https://github.com/Alirex/rs_lib_i_app_root_folder/actions/workflows/ci.yml/badge.svg)](https://github.com/Alirex/rs_lib_i_app_root_folder/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/app_root_folder.svg)](https://crates.io/crates/app_root_folder)
[![docs.rs](https://img.shields.io/docsrs/app_root_folder)](https://docs.rs/app_root_folder)
[![license: ISC](https://img.shields.io/crates/l/app_root_folder.svg)](LICENSE.txt)

Allows you to get the root folder path of the app.

## Install

```shell
cargo add app_root_folder
```

## Example

```rust
fn main() {
    let root = app_root_folder::get_app_root_folder();
    println!("App root folder: {}", root.display());
}
```

The rule: the folder of the running executable. If the executable is in a cargo build folder (`target/debug`,
`target/release`, `target/<triple>/debug`, ...), the result is the folder that contains `target`. The functions panic
when the folder cannot be found (see the `# Panics` sections on docs.rs).

The minimum supported Rust version is in `Cargo.toml` (`rust-version`) and is checked in CI.

## 🤔 Why?

### Why do I need this?

This can be useful when CWD (current working directory) is not enough.

And the `manifest directory` is different.

You can see the same thing in Python as:

- BASE_DIR
- ROOT_DIR

So, this can be useful, also, for projects that moving from Python (with this approach) to Rust.

This path can be used to access files from the repository, and that does not really depend on the implementation of the
app.

### Why do I need this library? And not implement it by myself?

Less boilerplate code. Call the function and get the result.

## 🛠️ Dev

Tools: [just](https://just.systems), [prek](https://github.com/j178/prek), Rust via rustup (the toolchain is pinned in
`rust-toolchain.toml`), [cargo-deny](https://github.com/EmbarkStudios/cargo-deny).

```shell
just dev-hooks   # install the git hook (once per clone)
just q-check     # the full gate: hooks (fmt, clippy, tests, docs), cargo deny, cargo package
just             # all recipes
```

Release: [CHANGELOG.md](CHANGELOG.md), then a `vX.Y.Z` tag runs `.github/workflows/release.yml`, which publishes to
crates.io by trusted publishing. Rules for contributors and agents: [AGENTS.md](AGENTS.md).

## Other

Maybe you will be interested in:

- Ways to get similar paths

  - Get the CWD (current working directory)

    - <https://doc.rust-lang.org/stable/std/env/fn.current_dir.html>

  - Path to the folder with the Cargo.toml file of the crate (not the workspace)
    - <https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates>
    - <https://gist.github.com/Alirex/db9655d8d6f2ddc27ce0cf8f195f0cba>

- Ways to get other useful paths:
  - Platform-specific, user-accessible locations
    - <https://crates.io/crates/directories>
