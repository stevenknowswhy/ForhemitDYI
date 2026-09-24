//! Integrity primitives — validated hash digests and hash-addressed payload
//! references.
//!
//! The architectural requirement is tamper detection (Audit doc §16: "The
//! system must be able to detect tampering with historical audit records").
//! The event hash chain and payload addressing build on these types.

use serde::{Deserialize, Serialize};

/// A lowercase hex-encoded SHA-256 digest (exactly 64 hex characters).
///
/// Validation runs on construction *and* on deserialization, so a value read
/// from disk can never hold a malformed digest.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, schemars::JsonSchema, Serialize)]
#[serde(try_from = "String")]
pub struct Sha256Hex(String);

/// Returned when a string is not a valid lowercase hex SHA-256 digest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidSha256Hex;

impl std::fmt::Display for InvalidSha256Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("expected a 64-character lowercase hex SHA-256 digest")
    }
}

impl std::error::Error for InvalidSha256Hex {}

impl Sha256Hex {
    /// Parses and validates a hex-encoded SHA-256 digest.
    pub fn parse(value: &str) -> Result<Self, InvalidSha256Hex> {
        let valid = value.len() == 64
            && value
                .bytes()
                .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
        if valid {
            Ok(Self(value.to_owned()))
        } else {
            Err(InvalidSha256Hex)
        }
    }

    /// Borrows the underlying hex string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Sha256Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for Sha256Hex {
    type Error = InvalidSha256Hex;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

/// Hash-addressed reference to bulk data (Audit doc §63 field
/// `payload_reference`; spec: "hash-addressed, never inline bulk data").
///
/// Payloads live behind their digest — that is what keeps audit events small
/// and history verifiable.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, schemars::JsonSchema, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PayloadRef {
    /// Digest of the referenced payload bytes.
    pub digest: Sha256Hex,
}
