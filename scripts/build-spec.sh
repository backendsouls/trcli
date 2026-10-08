#!/usr/bin/env bash
# Assemble specs/<feature>/spec.md from the part files in specs/<feature>/spec-src/.
#
# The part files are the source of truth; spec.md is a generated product and must
# not be edited by hand. Parts are concatenated in path order, one blank line apart.
#
# Usage:
#   scripts/build-spec.sh                 build every feature that has a spec-src/
#   scripts/build-spec.sh <feature-dir>   build one feature (e.g. specs/001-research-workspace)
#   scripts/build-spec.sh --check [...]   build nothing; exit 1 if any spec.md is out of date
set -euo pipefail

NOTICE='<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->'

usage() {
  sed -n '2,10p' "$0" | sed 's/^# \{0,1\}//'
}

# Print the assembled spec for the feature directory given as $1.
assemble() {
  local src="$1/spec-src" first=1 part
  printf '%s\n\n' "$NOTICE"
  while IFS= read -r part; do
    if [ "$first" -eq 0 ]; then
      printf '\n'
    fi
    first=0
    cat "$part"
  done < <(find "$src" -type f -name '*.md' | LC_ALL=C sort)
}

check=0
features=()
for arg in "$@"; do
  case "$arg" in
    --check) check=1 ;;
    -h | --help) usage; exit 0 ;;
    -*) echo "error: unknown option '$arg'" >&2; usage >&2; exit 2 ;;
    *) features+=("${arg%/}") ;;
  esac
done

root="$(cd "$(dirname "$0")/.." && pwd)"
if [ "${#features[@]}" -eq 0 ]; then
  while IFS= read -r dir; do
    features+=("${dir%/spec-src}")
  done < <(find "$root/specs" -mindepth 2 -maxdepth 2 -type d -name spec-src | LC_ALL=C sort)
fi

if [ "${#features[@]}" -eq 0 ]; then
  echo "error: no feature with a spec-src/ directory was found under specs/" >&2
  exit 2
fi

status=0
for feature in "${features[@]}"; do
  if [ ! -d "$feature/spec-src" ]; then
    echo "error: '$feature' has no spec-src/ directory" >&2
    exit 2
  fi
  if [ -z "$(find "$feature/spec-src" -type f -name '*.md' -print -quit)" ]; then
    echo "error: '$feature/spec-src' contains no .md parts" >&2
    exit 2
  fi
  if [ "$check" -eq 1 ]; then
    if [ -f "$feature/spec.md" ] && assemble "$feature" | cmp -s - "$feature/spec.md"; then
      echo "up to date: $feature/spec.md"
    else
      echo "out of date: $feature/spec.md (run scripts/build-spec.sh)" >&2
      status=1
    fi
  else
    assemble "$feature" > "$feature/spec.md.tmp"
    mv "$feature/spec.md.tmp" "$feature/spec.md"
    echo "built: $feature/spec.md"
  fi
done
exit "$status"
