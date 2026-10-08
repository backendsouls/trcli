#!/usr/bin/env bash
# Turns on this repository's git hooks for your clone.
#
# The hooks are kept in `.githooks/`, under version control, so that everyone runs the
# same ones. Git does not use a repository's own hooks until told to; this tells it.
#
# Usage: scripts/install-hooks.sh            turn the hooks on
#        scripts/install-hooks.sh --remove   turn them off again
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

if [[ "${1:-}" == "--remove" ]]; then
    git config --unset core.hooksPath || true
    echo "Git hooks are off for this clone."
    exit 0
fi

git config core.hooksPath .githooks
# On systems that keep the executable bit, make sure the hooks have it.
chmod +x .githooks/* 2>/dev/null || true

echo "Git hooks are on for this clone:"
for hook in .githooks/*; do
    echo "  $(basename "$hook")"
done
echo "Before every push, every test is run. Skip once with: git push --no-verify"
