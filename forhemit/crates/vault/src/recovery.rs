//! The recovery path (Vault doc §36: "The better we protect the vault,
//! the more carefully we must design legitimate recovery. … There should
//! never be a hidden universal backdoor").
//!
//! At vault creation the owner's recovery passphrase derives — via
//! Argon2id, with a per-vault random salt — a recovery key; that key
//! AES-256-GCM-wraps a copy of the vault key, and the wrapping is escrowed
//! in the vault database. The passphrase itself is never stored. If the
//! OS keychain entry is lost (lost device, failed drive, reinstall), the
//! owner re-derives the key from the passphrase, unwraps the vault key,
//! and installs it into a new key store. Without the passphrase there is
//! no path in — that is the design, not a defect.

use crate::crypto;
use crate::error::VaultError;
use crate::keys::{VaultKey, KEY_LEN};
use forhemit_contracts::VaultId;
use zeroize::Zeroizing;

/// Length of the per-vault Argon2id salt — 128 bits.
const SALT_LEN: usize = 16;

/// AAD binding for the recovery escrow: a wrapped vault key is only
/// openable as the recovery escrow of its own vault.
fn escrow_aad(vault_id: &VaultId) -> Vec<u8> {
    format!("forhemit-vault:recovery-escrow:{vault_id}").into_bytes()
}

/// Minimum accepted recovery-passphrase length. The escrow and the `age`
/// backups are offline brute-force targets, so the floor is engine
/// policy, not UI advice (security review M1). It gates only creation:
/// existing escrows keep opening with whatever passphrase made them —
/// a wrong passphrase already fails authenticated decryption, so
/// tightening recovery would add no security and could lock owners out.
pub const MIN_RECOVERY_PASSPHRASE_LEN: usize = 12;

/// Enforces the recovery-passphrase policy: never blank, and at least
/// [`MIN_RECOVERY_PASSPHRASE_LEN`] characters. Public: the shell
/// pre-flights it before creating a vault database, so a refused setup
/// leaves no file behind.
pub fn validate_recovery_passphrase(passphrase: &str) -> Result<(), VaultError> {
    if passphrase.trim().is_empty() {
        return Err(VaultError::InvalidInput(
            "recovery passphrase must not be blank".into(),
        ));
    }
    if passphrase.chars().count() < MIN_RECOVERY_PASSPHRASE_LEN {
        return Err(VaultError::InvalidInput(format!(
            "recovery passphrase must be at least {MIN_RECOVERY_PASSPHRASE_LEN} characters — it guards an offline brute-force target"
        )));
    }
    Ok(())
}

/// The escrowed material stored in the vault database at creation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryEscrow {
    /// Per-vault random salt for the Argon2id derivation (hex).
    pub salt_hex: String,
    /// Nonce of the vault-key wrapping (hex).
    pub wrapped_nonce_hex: String,
    /// The vault key, AES-256-GCM-wrapped under the Argon2id-derived
    /// recovery key (hex).
    pub wrapped_vault_key_hex: String,
}

impl RecoveryEscrow {
    /// Creates the escrow for a vault key from a recovery passphrase,
    /// generating the per-vault salt. The passphrase must meet the
    /// [`MIN_RECOVERY_PASSPHRASE_LEN`] policy.
    pub fn create(
        vault_id: &VaultId,
        vault_key: &VaultKey,
        passphrase: &str,
    ) -> Result<Self, VaultError> {
        validate_recovery_passphrase(passphrase)?;
        Self::wrap_vault_key(vault_id, vault_key, passphrase)
    }

    /// Seals the vault key under the passphrase-derived key — the escrow
    /// format itself, shared by `create` and the test-only pre-policy
    /// constructor.
    fn wrap_vault_key(
        vault_id: &VaultId,
        vault_key: &VaultKey,
        passphrase: &str,
    ) -> Result<Self, VaultError> {
        let salt = crypto::random_bytes(SALT_LEN);
        let recovery_key = crypto::derive_recovery_key(passphrase, &salt)?;
        let nonce = crypto::random_nonce();
        let wrapped = crypto::seal(
            &recovery_key,
            &nonce,
            &escrow_aad(vault_id),
            vault_key.as_bytes(),
            "escrow vault key for recovery",
        )?;
        Ok(Self {
            salt_hex: hex_encode(&salt),
            wrapped_nonce_hex: hex_encode(&nonce),
            wrapped_vault_key_hex: hex_encode(&wrapped),
        })
    }

    /// Test-only: builds an escrow without the passphrase policy, to
    /// simulate escrows created before the floor existed.
    #[cfg(test)]
    pub(crate) fn create_unchecked(
        vault_id: &VaultId,
        vault_key: &VaultKey,
        passphrase: &str,
    ) -> Result<Self, VaultError> {
        Self::wrap_vault_key(vault_id, vault_key, passphrase)
    }

    /// Opens the escrow with the recovery passphrase and returns the
    /// vault key it protects. A wrong passphrase is
    /// [`VaultError::RecoveryFailed`] — no other signal is exposed.
    pub fn open(
        &self,
        vault_id: &VaultId,
        passphrase: &str,
    ) -> Result<Zeroizing<VaultKey>, VaultError> {
        let salt = hex_decode_fixed::<SALT_LEN>(&self.salt_hex, "escrow salt")?;
        let nonce = hex_decode_fixed::<12>(&self.wrapped_nonce_hex, "escrow nonce")?;
        let wrapped = hex_decode_any(&self.wrapped_vault_key_hex)?;
        let recovery_key = crypto::derive_recovery_key(passphrase, &salt)?;
        let vault_key_bytes = crypto::open(
            &recovery_key,
            &nonce,
            &escrow_aad(vault_id),
            &wrapped,
            "open recovery escrow",
        )
        .map_err(|_| VaultError::RecoveryFailed)?;
        let bytes: [u8; KEY_LEN] = vault_key_bytes
            .as_slice()
            .try_into()
            .map_err(|_| VaultError::Internal("escrowed vault key was not 32 bytes".to_owned()))?;
        Ok(Zeroizing::new(VaultKey::from_bytes(bytes)))
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode_fixed<const N: usize>(hex: &str, what: &str) -> Result<[u8; N], VaultError> {
    let bytes = hex_decode_any(hex)?;
    let length = bytes.len();
    bytes
        .try_into()
        .map_err(|_| VaultError::Internal(format!("{what} had the wrong length ({length} bytes)")))
}

fn hex_decode_any(hex: &str) -> Result<Vec<u8>, VaultError> {
    (0..hex.len() / 2)
        .map(|index| {
            u8::from_str_radix(&hex[2 * index..2 * index + 2], 16)
                .map_err(|error| VaultError::Internal(format!("escrow hex decode: {error}")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;
    use crate::crypto::generate_key;

    fn vault() -> VaultId {
        VaultId::new("vault-escrow-test").unwrap()
    }

    #[test]
    fn escrow_round_trips_with_the_right_passphrase() {
        let vault = vault();
        let key = VaultKey::from_bytes(generate_key());
        let escrow = RecoveryEscrow::create(&vault, &key, "paper clip horizon").unwrap();
        let opened = escrow.open(&vault, "paper clip horizon").unwrap();
        assert_eq!(opened.as_bytes(), key.as_bytes());
    }

    #[test]
    fn wrong_passphrase_is_recovery_failed() {
        let vault = vault();
        let key = VaultKey::from_bytes(generate_key());
        let escrow = RecoveryEscrow::create(&vault, &key, "right passphrase").unwrap();
        assert!(matches!(
            escrow.open(&vault, "wrong passphrase"),
            Err(VaultError::RecoveryFailed)
        ));
    }

    #[test]
    fn escrow_is_bound_to_its_vault() {
        let key = VaultKey::from_bytes(generate_key());
        let vault = vault();
        let escrow = RecoveryEscrow::create(&vault, &key, "binding passphrase").unwrap();
        let other_vault = VaultId::new("another-vault").unwrap();
        // The AAD binding makes an escrow blob transferred to a different
        // vault's record fail rather than yield the key.
        assert!(matches!(
            escrow.open(&other_vault, "binding passphrase"),
            Err(VaultError::RecoveryFailed)
        ));
    }

    #[test]
    fn create_refuses_passphrases_below_the_policy_floor() {
        let vault = vault();
        let key = VaultKey::from_bytes(generate_key());
        for weak in [
            "",
            "   ",
            "short",
            "0123456789a",  // 11 characters
            "            ", // long enough, but blank
        ] {
            assert!(
                matches!(
                    RecoveryEscrow::create(&vault, &key, weak),
                    Err(VaultError::InvalidInput(_))
                ),
                "expected refusal for {weak:?}"
            );
        }
    }

    #[test]
    fn create_accepts_the_twelve_character_floor() {
        let vault = vault();
        let key = VaultKey::from_bytes(generate_key());
        let escrow = RecoveryEscrow::create(&vault, &key, "twelve chars")
            .expect("a 12-character passphrase meets the policy");
        let opened = escrow.open(&vault, "twelve chars").unwrap();
        assert_eq!(opened.as_bytes(), key.as_bytes());
    }

    #[test]
    fn pre_policy_escrows_still_open_with_short_passphrases() {
        // The policy gates creation, not recovery: an escrow made before
        // the floor existed keeps opening with its original passphrase.
        let vault = vault();
        let key = VaultKey::from_bytes(generate_key());
        let escrow = RecoveryEscrow::create_unchecked(&vault, &key, "pre-policy").unwrap();
        let opened = escrow.open(&vault, "pre-policy").unwrap();
        assert_eq!(opened.as_bytes(), key.as_bytes());
    }

    #[test]
    fn salts_are_unique_per_escrow() {
        let vault = vault();
        let key = VaultKey::from_bytes(generate_key());
        let first = RecoveryEscrow::create(&vault, &key, "same passphrase").unwrap();
        let second = RecoveryEscrow::create(&vault, &key, "same passphrase").unwrap();
        assert_ne!(first.salt_hex, second.salt_hex);
        assert_ne!(first.wrapped_vault_key_hex, second.wrapped_vault_key_hex);
    }
}
