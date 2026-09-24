#!/usr/bin/env bash
# Enforces the workspace rule (spec art_fL7Z2ate): engine crates may depend
# only on `forhemit-contracts` and `forhemit-enginekit` — never on each
# other's internals. The Tauri shell (`forhemit/app`) is exempt: it is the
# adapter layer that wires engines together.
#
# Every crate under forhemit/crates/ must be classified below; an
# unclassified crate fails the check so new engines cannot dodge the rule
# by being forgotten.
set -euo pipefail

cd "$(dirname "$0")/../forhemit"

fail=0

# Greps for forhemit-* dependencies without matching the crate's own
# package name: dependency keys at line start, or `package = "forhemit-…"`
# rename entries.
find_deps() {
  local toml=$1
  {
    grep -oE '^forhemit-[a-z_-]+' "$toml" || true
    grep -oE 'package = "forhemit-[a-z_-]+"' "$toml" | sed -E 's/package = "(forhemit-[a-z_-]+)"/\1/' || true
  } | sort -u
}

# check_crate <dir> <space-separated allowed forhemit deps>
check_crate() {
  local dir=$1 allowed=" $2 " toml dep
  toml=$dir/Cargo.toml
  if [[ ! -f $toml ]]; then
    echo "MISSING: $toml"
    fail=1
    return
  fi
  for dep in $(find_deps "$toml"); do
    if [[ $allowed != *" $dep "* ]]; then
      echo "VIOLATION: $dir depends on $dep (allowed shared deps: ${allowed:-none})"
      fail=1
    fi
  done
}

# The foundation: contracts is the shared vocabulary and depends on no
# workspace crate; enginekit may sit on contracts alone.
check_crate crates/contracts ""
check_crate crates/enginekit "forhemit-contracts"

# Engine crates — add each new engine crate here with its allowed shared
# deps, e.g.:
#   check_crate crates/audit "forhemit-contracts forhemit-enginekit"
check_crate crates/audit "forhemit-contracts forhemit-enginekit"
check_crate crates/reality "forhemit-contracts forhemit-enginekit"
check_crate crates/destination "forhemit-contracts forhemit-enginekit"

# Any crate directory not classified above fails loudly:
for crate_dir in crates/*/; do
  crate_dir=${crate_dir%/}
  case $crate_dir in
    crates/contracts | crates/enginekit | crates/audit | crates/reality | crates/destination) ;;
    *)
      if [[ -f $crate_dir/Cargo.toml ]]; then
        echo "UNCLASSIFIED: $crate_dir is not in scripts/check-engine-deps.sh — classify it with its allowed shared deps"
        fail=1
      fi
      ;;
  esac
done

if [[ $fail -ne 0 ]]; then
  echo "engine dependency rule: FAILED"
  exit 1
fi
echo "engine dependency rule: OK"
