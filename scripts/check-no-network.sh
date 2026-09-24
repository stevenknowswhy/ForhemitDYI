#!/usr/bin/env bash
# Zero-network gate (spec art_fL7Z2ate; HYBRID LOCAL-FIRST ARCHITECTURE.md):
# v1 is local-only — no engine crate may reach for a network stack.
#
# Checks every engine crate under forhemit/crates/ (the Tauri shell at
# forhemit/app is exempt — it is the adapter layer, and future shells may
# legitimately host the online layer) for:
#   1. known network/TLS crate dependencies in Cargo.toml
#   2. socket-level APIs in source (std::net, tokio::net, …)
# The denylist covers the crates a Rust project reaches for when adding
# HTTP or raw sockets; anything novel still has to arrive as a
# dependency, so the Cargo.toml sweep is the load-bearing check.
set -euo pipefail

cd "$(dirname "$0")/../forhemit"

fail=0

# Network client / transport / TLS / async-runtime crates, by Cargo.toml
# dependency key (underscores normalized; whole-key match).
NETWORK_CRATES='reqwest|ureq|surf|isahc|curl|hyper|h2|quinn|tokio|mio|socket2|native_tls|rustls|openssl|tungstenite|tokio_tungstenite|tonic|tarpc|libp2p|igd|local_ip_address|trust_dns|hickory|ssh2|ftp|smtp|imap|pop3|ldap3|coap|quinn_proto'

# Source-level socket APIs — a violation even if the dependency arrives
# transitively.
SOURCE_PATTERNS='std::net|tokio::net|UdpSocket|TcpListener|TcpStream|SocketAddr'

for crate_toml in crates/*/Cargo.toml; do
  crate_dir=${crate_toml%/Cargo.toml}

  # Dependency keys: line-start `name =` and rename entries
  # `name = { package = "…" }`.
  found_deps=$(grep -oE '^[a-z0-9_-]+[[:space:]]*=' "$crate_toml" \
    | sed -E 's/[[:space:]]*=.*//;s/-/_/g' \
    | grep -E "^($NETWORK_CRATES)$" || true)
  found_deps+=$(grep -oE 'package = "[a-z0-9_-]+"' "$crate_toml" \
    | sed -E 's/package = "//;s/"//' | sed 's/-/_/g' \
    | grep -E "^($NETWORK_CRATES)$" || true)

  if [[ -n $found_deps ]]; then
    echo "VIOLATION: $crate_dir depends on network crate(s): $(echo "$found_deps" | sort -u | tr '\n' ' ')"
    fail=1
  fi

  found_src=$(grep -rnE "$SOURCE_PATTERNS" "$crate_dir/src" 2>/dev/null || true)
  if [[ -n $found_src ]]; then
    echo "VIOLATION: $crate_dir source uses socket APIs:"
    echo "$found_src" | head -10
    fail=1
  fi
done

if [[ $fail -ne 0 ]]; then
  echo "zero-network rule: FAILED"
  exit 1
fi
echo "zero-network rule: OK"
