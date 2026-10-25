# app_root_folder: instructions for AI agents

`app_root_folder` (crates.io) is a small Rust library with no dependencies: the root folder of the running app (the
executable's folder, or the folder with `target` when run from a cargo build folder). Public API:
`get_app_root_folder`, `get_app_root_folder_by_executable`, `get_executable_path`, their `try_` variants and
`AppRootFolderError` (`src/`). Edition 2024, MSRV in `Cargo.toml`, toolchain pinned in `rust-toolchain.toml`.

## Commands

[just](https://just.systems) is the entry point (`just` lists recipes).

- `just q-check`: the full gate. It must pass before committing: prek hooks (rustfmt, clippy with `-D warnings`, tests and
  doc tests, rustdoc with `-D warnings`, markdownlint, prettier, zizmor, gitleaks, justfile format), then `cargo deny
check` and `cargo package`. `just q-fast`: hooks only, no network. `just q-test`: tests.
- `just dev-hooks` installs the git hook. `just q-secrets`: gitleaks over the whole history.
- Prek fixers rewrite files; run the gate again until it passes. Never commit a bump of the `pre-commit-update` hook unasked.

## Rules

- Lints are `warn` in `Cargo.toml [lints]` (pedantic, no `unwrap`/`expect`/`dbg!`/`print!`) and the gate makes them errors.
  Fix the finding; suppress only with `#[expect(lint, reason = "...")]`, never `#[allow]`.
  [lint-strictness](https://github.com/evocaslab/hq/blob/main/practices/lint-strictness.md)
- Every public function documents `# Panics` (or `# Errors`) and has a doc test; the gate builds docs with warnings as errors.
  Tests must pass on Linux, macOS and Windows (CI runs all three).
- Dependencies: caret requirements, `cargo update` in its own commit with the lock diff read; `cargo deny check` is clean.
  Dependabot has a 7-day cooldown. [tooling-and-deps](https://github.com/evocaslab/hq/blob/main/practices/tooling-and-deps.md),
  [supply-chain-security](https://github.com/evocaslab/hq/blob/main/practices/supply-chain-security.md)
- GitHub Actions are pinned to a full commit SHA with a `# vX.Y.Z` comment; workflows start with `permissions: contents: read`.
- `.idea/` is tracked selectively (see `.gitignore`).
  [gitignore-and-ide](https://github.com/evocaslab/hq/blob/main/practices/gitignore-and-ide.md)
- Every user-visible change gets a line under `[Unreleased]` in `CHANGELOG.md`.
  [quality-gate](https://github.com/evocaslab/hq/blob/main/practices/quality-gate.md)

## Release

Version lives only in `Cargo.toml`. SemVer (while `0.x`: breaking or feature = minor, fix = patch). Steps: move
`[Unreleased]` in `CHANGELOG.md` to the new version, bump `Cargo.toml` and `Cargo.lock`, land on `main` with green CI,
then `git tag -a vX.Y.Z -m vX.Y.Z && git push origin vX.Y.Z`. The tag runs `release.yml`: `just rel-check`, then
`cargo publish` by crates.io trusted publishing (environment `release`), then the GitHub release from the changelog
section. Never publish locally (`just rel-dry-run` only).
[releases-and-versions](https://github.com/evocaslab/hq/blob/main/practices/releases-and-versions.md)

## Reasoning

Shared best practices and the reasons behind these rules: <https://github.com/evocaslab/hq/tree/main/practices>
