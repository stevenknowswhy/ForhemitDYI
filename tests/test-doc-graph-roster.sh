#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

cp -R "$ROOT/." "$TMP/repo/"
cd "$TMP/repo"

sed -E -i 's/[0-9]+ documents in thirteen groups/999 documents in thirteen groups/' DOCUMENT-INDEX.md

set +e
output="$(python3 scripts/doc-graph.py --check 2>&1)"
status=$?
set -e

if [ "$status" -eq 0 ]; then
  printf 'Expected stale roster prose to fail validation.\n%s\n' "$output" >&2
  exit 1
fi

printf '%s\n' "$output" | grep -Eq 'DOCUMENT-INDEX prose says 999 docs; index lists [0-9]+'
printf 'document roster regression test passed\n'