# App root folder

---

[![pre-commit](https://github.com/Alirex/rs_lib_i_app_root_folder/actions/workflows/pre-commit.yaml/badge.svg)](https://github.com/Alirex/rs_lib_i_path_as_uri/actions/workflows/pre-commit.yaml)

Allows you to get the root folder path of the app.

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

### Register pre-commit hooks

```shell
pre-commit install
```

### Run pre-commit hooks

```shell
pre-commit run --all-files
```

### Upgrade dependencies

```shell
cargo upgrade
```

- [cargo upgrade](https://crates.io/crates/cargo-edit#cargo-upgrade)
  - `cargo install cargo-edit --no-default-features --features "upgrade"`

## Other

Maybe you will be interested in:

- Ways to get similar paths

  - Get the CWD (current working directory)

    - https://doc.rust-lang.org/stable/std/env/fn.current_dir.html

  - Path to the folder with the Cargo.toml file of the crate (not the workspace)
    - https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    - https://gist.github.com/Alirex/db9655d8d6f2ddc27ce0cf8f195f0cba

- Ways to get other useful paths:
  - Platform-specific, user-accessible locations
    - https://crates.io/crates/directories
