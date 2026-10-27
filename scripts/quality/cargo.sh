#!/usr/bin/env bash
# cargo with ~/.cargo/bin on PATH, for the prek hooks and recipes: a shell that never read the rustup profile line
# (an agent session, a GUI git client) has no cargo, and the hook failure would block the commit.
#
# Usage: scripts/quality/cargo.sh <cargo arguments...>
set -euo pipefail

export PATH="${HOME}/.cargo/bin:${PATH}"
exec cargo "$@"
