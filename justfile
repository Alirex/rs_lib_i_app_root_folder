# app_root_folder recipes. Run `just` for the list. Prefixes: q- quality, dev- setup, rel- release.

set shell := ["bash", "-euo", "pipefail", "-c"]

[private]
default:
    @just --list

# Full gate: prek hooks (format, lint, tests, docs), then the supply-chain check and the package check.
[group('quality')]
q-check:
    SKIP=pre-commit-update prek run --all-files
    cargo deny check
    cargo package --locked --allow-dirty

# Fast loop: prek hooks only (no network).
[group('quality')]
q-fast:
    SKIP=pre-commit-update prek run --all-files

# Run the unit tests and doc tests.
[group('quality')]
q-test *args:
    cargo test --locked {{ args }}

# Supply chain: RustSec advisories, licenses, bans, sources (fetches the advisory database).
[group('quality')]
q-deny:
    cargo deny check

# Scan the whole git history for leaked secrets (gitleaks; monthly and when adopting).
[group('quality')]
q-secrets:
    prek run gitleaks-history --hook-stage manual --all-files

# Install the git hook (once per clone).
[group('dev')]
dev-hooks:
    prek install

# Upgrade dependencies in Cargo.lock (read the diff; run q-check after).
[group('dev')]
dev-update:
    cargo update

# Fail unless the tag (default: HEAD's tag) equals the Cargo.toml version and the changelog has that section.
[group('release')]
rel-check tag="":
    #!/usr/bin/env bash
    set -euo pipefail
    tag="{{ tag }}"
    [[ -n "$tag" ]] || tag="$(git describe --exact-match --tags HEAD)"
    version="$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')"
    if [[ "$tag" != "v$version" ]]; then
        echo "tag $tag differs from Cargo.toml version $version" >&2
        exit 1
    fi
    grep -q "^## \[$version\]" CHANGELOG.md || { echo "CHANGELOG.md has no section for $version" >&2; exit 1; }
    echo "ok: $tag"

# Dry-run of the publish (no upload). The real publish runs from the tag in CI (release.yml).
[group('release')]
rel-dry-run:
    cargo publish --dry-run --locked
