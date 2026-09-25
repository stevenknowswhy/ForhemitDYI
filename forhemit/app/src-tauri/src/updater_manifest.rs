//! Pre-flight verdict over a Tauri static update manifest (`latest.json`).
//!
//! The updater's trust story has two layers, both failing loudly:
//!
//! 1. **This module** (unit-tested here) derives the refusal decision from
//!    the raw manifest the updater plugin fetched — before any update bytes
//!    are downloaded. It refuses unsigned or stale manifests with a precise,
//!    named reason the UI can show the owner.
//! 2. **The plugin** (tauri-plugin-updater) verifies the downloaded
//!    artifact's minisign signature against the public key embedded at build
//!    time. That verification cannot be disabled; a refused artifact is
//!    never installed.
//!
//! Refusal rules, in the order [`verdict`] applies them:
//!
//! - the manifest must be well-formed JSON with a semver `version`
//!   (a leading `v` is accepted);
//! - the offered version must be **strictly newer** than the running app —
//!   an equal or older version is a stale manifest, refused (no downgrades,
//!   no re-installing the same build);
//! - `pub_date`, when present, must parse as RFC 3339 and must not sit in
//!   the future beyond clock tolerance (a future date signals a corrupted
//!   or hostile manifest; an old date is normal — a stable release is
//!   served for months, so age alone is not staleness);
//! - the running platform must have an entry (`target` is the updater's
//!   `OS-ARCH` key, e.g. `linux-x86_64`);
//! - that entry must carry a non-empty signature, decodable exactly the way
//!   the plugin decodes it (base64 → UTF-8 → minisign signature text): an
//!   unsigned entry is refused before any bytes move.
//!
//! Cryptographic verification of the artifact itself happens at download
//! time in the plugin (layer 2); the tests below pin that contract with a
//! real keypair fixture.

use std::collections::BTreeMap;

use base64::Engine as _;
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// How far past `now` a manifest's `pub_date` may sit before it is treated
/// as corrupted rather than clock skew.
const FUTURE_CLOCK_TOLERANCE: time::Duration = time::Duration::minutes(30);

/// A Tauri static update manifest — the `latest.json` attached to a release.
///
/// Unknown top-level fields are ignored (forward compatibility); required
/// content is validated by [`verdict`], not by the parser, so each failure
/// gets its own precise [`Refusal`].
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UpdateManifest {
    /// The offered version — a valid semver, with or without a leading `v`.
    pub version: String,
    /// Release notes shown to the owner, if the release carries any.
    #[serde(default)]
    pub notes: Option<String>,
    /// When the release was published (RFC 3339), if present.
    #[serde(default)]
    pub pub_date: Option<String>,
    /// Per-platform updater artifacts, keyed `OS-ARCH` (e.g.
    /// `linux-x86_64`).
    #[serde(default)]
    pub platforms: BTreeMap<String, PlatformUpdate>,
}

/// One platform's updater artifact entry.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PlatformUpdate {
    /// URL the updater downloads the artifact from.
    #[serde(default)]
    pub url: String,
    /// The minisign signature of the artifact — the content of the
    /// generated `.sig` file. Empty means the release was built unsigned.
    #[serde(default)]
    pub signature: String,
}

/// Why a manifest was refused. Serialized to the UI with the `reason` tag;
/// the variant names are part of the wire contract.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum Refusal {
    /// The manifest could not be parsed or required content is missing.
    Malformed {
        /// What was wrong.
        detail: String,
    },
    /// The offered version is not strictly newer than the running app —
    /// a stale manifest (downgrade or reinstall of the same version).
    NotNewer {
        /// The running app's version.
        current: String,
        /// The version the manifest offered.
        offered: String,
    },
    /// No entry for the running platform.
    MissingPlatform {
        /// The updater target key that had no entry.
        target: String,
    },
    /// The platform entry carries no signature — an unsigned release.
    Unsigned {
        /// The updater target key of the unsigned entry.
        target: String,
    },
    /// The signature is not decodable minisign signature content.
    BadSignature {
        /// The updater target key of the malformed entry.
        target: String,
        /// What was wrong.
        detail: String,
    },
    /// `pub_date` is present but not RFC 3339, or sits in the future
    /// beyond clock tolerance.
    BadPubDate {
        /// What was wrong.
        detail: String,
    },
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Refusal::Malformed { detail } => write!(f, "update manifest is malformed: {detail}"),
            Refusal::NotNewer { current, offered } => write!(
                f,
                "update manifest is stale: it offers {offered}, but this app runs {current}"
            ),
            Refusal::MissingPlatform { target } => {
                write!(
                    f,
                    "update manifest has no entry for this platform ({target})"
                )
            }
            Refusal::Unsigned { target } => write!(
                f,
                "update manifest entry for {target} is unsigned — refusing before any download"
            ),
            Refusal::BadSignature { target, detail } => {
                write!(
                    f,
                    "update signature for {target} is not valid minisign content: {detail}"
                )
            }
            Refusal::BadPubDate { detail } => {
                write!(f, "update manifest has an invalid publish date: {detail}")
            }
        }
    }
}

/// What the UI learns from an update check. Serialized with an `outcome`
/// tag; the variant names are part of the wire contract.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum UpdateCheckView {
    /// No update available.
    UpToDate {
        /// The running app's version.
        current_version: String,
    },
    /// A signed, strictly newer release is offered.
    Available {
        /// The offered version.
        version: String,
        /// Release notes, if any.
        notes: Option<String>,
        /// Publish date (RFC 3339), if present.
        pub_date: Option<String>,
        /// Where the updater will download from.
        download_url: String,
    },
    /// The pre-flight verdict refused the manifest — nothing was downloaded.
    Refused {
        /// The precise refusal, shown to the owner as-is.
        refusal: Refusal,
    },
    /// The check could not run (endpoint unreachable, no release yet).
    Unavailable {
        /// The underlying reason.
        detail: String,
    },
}

/// The full pre-flight verdict: parse the manifest text and apply every
/// refusal rule. `current_version` is the running app's version, `target`
/// the updater's `OS-ARCH` key, `now` the clock the staleness check runs
/// against. On success the parsed manifest is returned.
pub fn verdict(
    manifest_text: &str,
    current_version: &str,
    target: &str,
    now: OffsetDateTime,
) -> Result<UpdateManifest, Refusal> {
    let manifest: UpdateManifest =
        serde_json::from_str(manifest_text).map_err(|error| Refusal::Malformed {
            detail: error.to_string(),
        })?;

    let offered = parse_version(&manifest.version).ok_or(Refusal::Malformed {
        detail: format!("version {:?} is not a valid semver", manifest.version),
    })?;
    let current = parse_version(current_version).ok_or(Refusal::Malformed {
        detail: format!("the running version {current_version:?} is not a valid semver"),
    })?;
    if offered <= current {
        return Err(Refusal::NotNewer {
            current: current_version.to_string(),
            offered: manifest.version.clone(),
        });
    }

    if let Some(pub_date) = &manifest.pub_date {
        let parsed =
            OffsetDateTime::parse(pub_date, &Rfc3339).map_err(|error| Refusal::BadPubDate {
                detail: format!("{} is not RFC 3339: {error}", pub_date),
            })?;
        if parsed > now + FUTURE_CLOCK_TOLERANCE {
            return Err(Refusal::BadPubDate {
                detail: format!("{pub_date} is in the future"),
            });
        }
    }

    let platform = manifest
        .platforms
        .get(target)
        .ok_or_else(|| Refusal::MissingPlatform {
            target: target.to_string(),
        })?;
    if platform.url.trim().is_empty() {
        return Err(Refusal::Malformed {
            detail: format!("platform entry {target} has no download url"),
        });
    }
    if platform.signature.trim().is_empty() {
        return Err(Refusal::Unsigned {
            target: target.to_string(),
        });
    }
    if let Err(error) = decode_signature(&platform.signature) {
        return Err(Refusal::BadSignature {
            target: target.to_string(),
            detail: error,
        });
    }

    Ok(manifest)
}

/// Parses a semver, accepting the optional leading `v` the manifest format
/// allows ("both `1.0.0` and `v1.0.0` are valid").
fn parse_version(text: &str) -> Option<semver::Version> {
    let stripped = text.trim().strip_prefix('v').unwrap_or(text.trim());
    semver::Version::parse(stripped).ok()
}

/// Decodes a `signature` field exactly the way the updater plugin does:
/// base64, then UTF-8, then minisign signature text. The plugin applies the
/// same pipeline before verifying; a field that does not survive it can
/// never verify, so it is refused here.
fn decode_signature(signature: &str) -> Result<(), String> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(signature.trim())
        .map_err(|error| format!("not base64: {error}"))?;
    let text =
        String::from_utf8(decoded).map_err(|error| format!("not utf-8 after base64: {error}"))?;
    minisign_verify::Signature::decode(text.trim())
        .map(|_| ())
        .map_err(|error| format!("not minisign signature text: {error}"))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    /// "Now" for the verdict tests: a fixed past instant, so fixtures stay
    /// stable. RFC 3339: 2026-09-25T00:00:00Z.
    fn test_now() -> OffsetDateTime {
        OffsetDateTime::parse("2026-09-25T00:00:00Z", &Rfc3339).unwrap()
    }

    /// The base64 public-key line embedded in tauri.conf.json — the dev/test
    /// updater key (see distribution/SIGNING.md; rotate before public
    /// distribution).
    const FIXTURE_PUBKEY: &str = "RWQcppctTHfjatQgKtXwQBXPvIQohBtJFXraye3ZVK6eYJ2qDyiyTr1g";

    /// Fixed artifact bytes the fixture signature covers.
    const FIXTURE_ARTIFACT: &[u8] = b"Forhemit fixture update artifact bytes v1";

    /// The `signature` field for FIXTURE_ARTIFACT: the base64 content of the
    /// `.sig` file produced by `tauri signer sign` with the fixture key —
    /// the exact form a release's `latest.json` carries.
    const FIXTURE_SIGNATURE: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVRY3BwY3RUSGZqYXFMcnFqM1hKa2M4dHBLUGJrMnlBbTBmNTAwcHB5aGR3WnN3RkNzeHkvcUF0MWRpb3BPb0RvNTVzUFdZS2hHMnZGWm04aU04QldMTkV4WGFDdmp5T2dRPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzkwMzExNjc1CWZpbGU6Zm9yaGVtaXQtZml4dHVyZS5iaW4KQTN6V2xaazlOTzFEUmFrdkhhby9vb1RpNEo0dGY5UFd4Q1BRbDR6bXk0dnRJdVRld1gwVlpnK1VjSjRkdVhRa1dHYmFpU3hUMDRuYVRNWUpheDdxQ3c9PQo=";

    /// A well-formed, signed, strictly-newer manifest for the fixture key.
    fn fixture_manifest() -> String {
        format!(
            r#"{{
                "version": "0.2.0",
                "notes": "test release",
                "pub_date": "2026-09-20T12:00:00Z",
                "platforms": {{
                    "linux-x86_64": {{
                        "url": "https://example.com/Forhemit_0.2.0_amd64.AppImage",
                        "signature": "{FIXTURE_SIGNATURE}"
                    }}
                }}
            }}"#
        )
    }

    #[test]
    fn fixture_signature_verifies_against_the_embedded_pubkey() {
        // Pins the layer-2 contract the plugin enforces: the committed
        // public key accepts the fixture signature over the fixture bytes,
        // and rejects the same signature over tampered bytes.
        let public_key = minisign_verify::PublicKey::from_base64(FIXTURE_PUBKEY).unwrap();
        let signature_text = String::from_utf8(
            base64::engine::general_purpose::STANDARD
                .decode(FIXTURE_SIGNATURE)
                .unwrap(),
        )
        .unwrap();
        let signature = minisign_verify::Signature::decode(&signature_text).unwrap();

        let verified = public_key.verify(FIXTURE_ARTIFACT, &signature, false);
        assert!(
            verified.is_ok(),
            "fixture signature must verify: {verified:?}"
        );

        let tampered = b"Forhemit fixture update artifact bytes v2";
        assert!(public_key.verify(tampered, &signature, false).is_err());
    }

    #[test]
    fn accepts_well_formed_newer_signed_manifest() {
        let manifest = verdict(&fixture_manifest(), "0.1.0", "linux-x86_64", test_now()).unwrap();
        assert_eq!(manifest.version, "0.2.0");
        assert_eq!(manifest.notes.as_deref(), Some("test release"));
        assert_eq!(
            manifest.platforms["linux-x86_64"].url,
            "https://example.com/Forhemit_0.2.0_amd64.AppImage"
        );
    }

    #[test]
    fn accepts_leading_v_in_versions() {
        let manifest = fixture_manifest().replace("\"0.2.0\"", "\"v0.2.0\"");
        let parsed = verdict(&manifest, "0.1.0", "linux-x86_64", test_now()).unwrap();
        assert_eq!(parsed.version, "v0.2.0");
    }

    #[test]
    fn refuses_unsigned_manifest_before_any_download() {
        let unsigned = fixture_manifest().replace(
            &format!("\"signature\": \"{FIXTURE_SIGNATURE}\""),
            "\"signature\": \"\"",
        );
        assert_eq!(
            verdict(&unsigned, "0.1.0", "linux-x86_64", test_now()),
            Err(Refusal::Unsigned {
                target: "linux-x86_64".to_string()
            })
        );

        let whitespace = fixture_manifest().replace(
            &format!("\"signature\": \"{FIXTURE_SIGNATURE}\""),
            "\"signature\": \"   \"",
        );
        assert_eq!(
            verdict(&whitespace, "0.1.0", "linux-x86_64", test_now()),
            Err(Refusal::Unsigned {
                target: "linux-x86_64".to_string()
            })
        );
    }

    #[test]
    fn refuses_stale_manifest_equal_version() {
        assert_eq!(
            verdict(&fixture_manifest(), "0.2.0", "linux-x86_64", test_now()),
            Err(Refusal::NotNewer {
                current: "0.2.0".to_string(),
                offered: "0.2.0".to_string()
            })
        );
    }

    #[test]
    fn refuses_stale_manifest_downgrade() {
        assert_eq!(
            verdict(&fixture_manifest(), "0.3.0", "linux-x86_64", test_now()),
            Err(Refusal::NotNewer {
                current: "0.3.0".to_string(),
                offered: "0.2.0".to_string()
            })
        );
    }

    #[test]
    fn refuses_stale_manifest_prerelease_of_same_version() {
        // Semver ordering: 0.2.0-rc.1 < 0.2.0 — a prerelease manifest is
        // older than the running release, so it is stale.
        let prerelease = fixture_manifest().replace("\"0.2.0\"", "\"0.2.0-rc.1\"");
        assert_eq!(
            verdict(&prerelease, "0.2.0", "linux-x86_64", test_now()),
            Err(Refusal::NotNewer {
                current: "0.2.0".to_string(),
                offered: "0.2.0-rc.1".to_string()
            })
        );
    }

    #[test]
    fn refuses_manifest_without_the_running_platform() {
        assert_eq!(
            verdict(&fixture_manifest(), "0.1.0", "darwin-aarch64", test_now()),
            Err(Refusal::MissingPlatform {
                target: "darwin-aarch64".to_string()
            })
        );
    }

    #[test]
    fn refuses_malformed_manifests() {
        assert!(matches!(
            verdict("not json", "0.1.0", "linux-x86_64", test_now()),
            Err(Refusal::Malformed { .. })
        ));
        assert!(matches!(
            verdict(r#"{"platforms": {}}"#, "0.1.0", "linux-x86_64", test_now()),
            Err(Refusal::Malformed { .. })
        ));
        let bad_version = fixture_manifest().replace("\"0.2.0\"", "\"not-a-version\"");
        assert!(matches!(
            verdict(&bad_version, "0.1.0", "linux-x86_64", test_now()),
            Err(Refusal::Malformed { .. })
        ));
    }

    #[test]
    fn refuses_signature_that_cannot_decode() {
        let not_base64 = fixture_manifest().replace(
            &format!("\"signature\": \"{FIXTURE_SIGNATURE}\""),
            "\"signature\": \"!!!not base64!!!\"",
        );
        assert!(matches!(
            verdict(&not_base64, "0.1.0", "linux-x86_64", test_now()),
            Err(Refusal::BadSignature { .. })
        ));

        // Valid base64, but not minisign signature text.
        let not_minisign = fixture_manifest().replace(
            &format!("\"signature\": \"{FIXTURE_SIGNATURE}\""),
            "\"signature\": \"aGVsbG8=\"",
        );
        assert!(matches!(
            verdict(&not_minisign, "0.1.0", "linux-x86_64", test_now()),
            Err(Refusal::BadSignature { .. })
        ));
    }

    #[test]
    fn refuses_future_pub_date_but_accepts_old_and_missing() {
        let future = fixture_manifest().replace("2026-09-20T12:00:00Z", "2026-10-01T00:00:00Z");
        assert!(matches!(
            verdict(&future, "0.1.0", "linux-x86_64", test_now()),
            Err(Refusal::BadPubDate { .. })
        ));

        let old = fixture_manifest().replace("2026-09-20T12:00:00Z", "2025-01-01T00:00:00Z");
        assert!(verdict(&old, "0.1.0", "linux-x86_64", test_now()).is_ok());

        let missing = fixture_manifest().replace("\"pub_date\": \"2026-09-20T12:00:00Z\",", "");
        assert!(verdict(&missing, "0.1.0", "linux-x86_64", test_now()).is_ok());
    }

    #[test]
    fn refuses_unparseable_pub_date() {
        let bad = fixture_manifest().replace("2026-09-20T12:00:00Z", "yesterday");
        assert!(matches!(
            verdict(&bad, "0.1.0", "linux-x86_64", test_now()),
            Err(Refusal::BadPubDate { .. })
        ));
    }
}
