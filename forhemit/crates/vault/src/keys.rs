//! Key material and where the vault key lives (Vault doc §9).
//!
//! ```text
//! User Authentication
//!         ↓
//! Workspace Authorization
//!         ↓
//! Vault Key          ← stored here: the OS keychain (keyring crate)
//!         ↓
//! Document Encryption Key
//!         ↓
//! Encrypted Document
//! ```
//!
//! The vault key is generated once at vault creation and stored only in
//! the OS keychain — never in the vault database, never on disk in
//! plaintext. Per-document data-encryption keys (DEKs) are random 256-bit
//! keys, generated at import and stored only in wrapped form (the vault
//! key AES-256-GCM-seals each DEK), so the keychain is the single root of
//! trust and rotating a DEK never touches the vault key.

use crate::error::VaultError;
use base64::Engine as _;
use zeroize::Zeroize;

/// Length of the vault key and of every document key — 256 bits.
pub const KEY_LEN: usize = 32;

/// The vault key, or a document data-encryption key, in memory.
///
/// Key material is wiped when the value drops (`zeroize`), and `Debug`
/// never reveals the bytes.
#[derive(Clone, PartialEq, Eq, zeroize::Zeroize)]
#[zeroize(drop)]
pub struct VaultKey([u8; KEY_LEN]);

impl VaultKey {
    /// Wraps existing key bytes (length must be exactly [`KEY_LEN`]).
    pub fn from_bytes(bytes: [u8; KEY_LEN]) -> Self {
        Self(bytes)
    }

    /// Borrows the key material.
    pub fn as_bytes(&self) -> &[u8; KEY_LEN] {
        &self.0
    }
}

impl std::fmt::Debug for VaultKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never render key material, not even in test output.
        f.write_str("VaultKey([REDACTED])")
    }
}

/// Storage for the vault key.
///
/// The production store is the OS keychain ([`OsKeyRing`]); tests use
/// [`MemoryKeyStore`]. Vault doc §9 puts the vault key behind "User
/// Authentication → Workspace Authorization" — this trait is the OS's
/// half of that chain.
pub trait VaultKeyStore: std::fmt::Debug + Send + Sync {
    /// Persists (or replaces) the vault key.
    fn store(
        &self,
        vault_id: &forhemit_contracts::VaultId,
        key: &VaultKey,
    ) -> Result<(), VaultError>;
    /// Loads the vault key, or `None` when the key store holds none for
    /// this vault (the vault-locked state, Vault doc §50).
    fn load(&self, vault_id: &forhemit_contracts::VaultId) -> Result<Option<VaultKey>, VaultError>;
    /// Removes the stored key — device revocation, or re-escrow through
    /// the recovery path.
    fn delete(&self, vault_id: &forhemit_contracts::VaultId) -> Result<(), VaultError>;
}

/// OS-keychain storage for the vault key (the `keyring` crate): the macOS
/// Keychain, Windows Credential Manager, or the Linux Secret Service.
#[derive(Debug, Clone)]
pub struct OsKeyRing;

impl VaultKeyStore for OsKeyRing {
    fn store(
        &self,
        vault_id: &forhemit_contracts::VaultId,
        key: &VaultKey,
    ) -> Result<(), VaultError> {
        let entry = keyring_entry(vault_id)?;
        entry
            .set_password(&encode_key(key))
            .map_err(|error| VaultError::KeyStore(error.to_string()))
    }

    fn load(&self, vault_id: &forhemit_contracts::VaultId) -> Result<Option<VaultKey>, VaultError> {
        let entry = keyring_entry(vault_id)?;
        match entry.get_password() {
            Ok(password) => Ok(Some(decode_key(&password)?)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(error) => Err(VaultError::KeyStore(error.to_string())),
        }
    }

    fn delete(&self, vault_id: &forhemit_contracts::VaultId) -> Result<(), VaultError> {
        let entry = keyring_entry(vault_id)?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            // Already gone is the state the caller asked for.
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(error) => Err(VaultError::KeyStore(error.to_string())),
        }
    }
}

fn keyring_entry(vault_id: &forhemit_contracts::VaultId) -> Result<keyring::Entry, VaultError> {
    keyring::Entry::new("forhemit-vault", vault_id.as_str())
        .map_err(|error| VaultError::KeyStore(error.to_string()))
}

fn encode_key(key: &VaultKey) -> String {
    base64::engine::general_purpose::STANDARD.encode(key.as_bytes())
}

fn decode_key(encoded: &str) -> Result<VaultKey, VaultError> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|error| {
            VaultError::KeyStore(format!("stored vault key is not valid base64: {error}"))
        })?;
    let bytes: [u8; KEY_LEN] = bytes
        .try_into()
        .map_err(|_| VaultError::KeyStore("stored vault key is not 32 bytes".to_owned()))?;
    Ok(VaultKey::from_bytes(bytes))
}

/// In-memory key store — for tests and fixtures only.
///
/// The shipped app must use [`OsKeyRing`]: a key held only in memory is
/// exactly the vault-locked-on-restart state the recovery path exists for.
/// Exposed at crate level so integration tests can share it.
#[derive(Debug, Default)]
pub struct MemoryKeyStore {
    keys: std::sync::Mutex<std::collections::HashMap<String, VaultKey>>,
}

impl VaultKeyStore for MemoryKeyStore {
    fn store(
        &self,
        vault_id: &forhemit_contracts::VaultId,
        key: &VaultKey,
    ) -> Result<(), VaultError> {
        self.keys
            .lock()
            .map_err(poisoned)?
            .insert(vault_id.to_string(), key.clone());
        Ok(())
    }

    fn load(&self, vault_id: &forhemit_contracts::VaultId) -> Result<Option<VaultKey>, VaultError> {
        Ok(self
            .keys
            .lock()
            .map_err(poisoned)?
            .get(vault_id.as_str())
            .cloned())
    }

    fn delete(&self, vault_id: &forhemit_contracts::VaultId) -> Result<(), VaultError> {
        self.keys
            .lock()
            .map_err(poisoned)?
            .remove(vault_id.as_str());
        Ok(())
    }
}

fn poisoned<T>(_: T) -> VaultError {
    VaultError::KeyStore("key store lock poisoned".to_owned())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;
    use crate::crypto::generate_key;
    use forhemit_contracts::VaultId;

    #[test]
    fn memory_keystore_round_trips_keys_by_vault() {
        let store = MemoryKeyStore::default();
        let vault_a = VaultId::new("vault-a").unwrap();
        let vault_b = VaultId::new("vault-b").unwrap();
        let key = VaultKey::from_bytes([7u8; KEY_LEN]);

        assert_eq!(store.load(&vault_a).unwrap(), None);
        store.store(&vault_a, &key).unwrap();
        assert_eq!(store.load(&vault_a).unwrap().unwrap(), key);
        assert_eq!(store.load(&vault_b).unwrap(), None);

        store.delete(&vault_a).unwrap();
        assert_eq!(store.load(&vault_a).unwrap(), None);
    }

    #[test]
    fn vault_key_debug_never_reveals_material() {
        let key = VaultKey::from_bytes([9u8; KEY_LEN]);
        assert_eq!(format!("{key:?}"), "VaultKey([REDACTED])");
        assert!(!format!("{key:?}").contains("0909"));
    }

    #[test]
    fn key_encoding_round_trips_through_base64() {
        let key = VaultKey::from_bytes(generate_key());
        let decoded = decode_key(&encode_key(&key)).unwrap();
        assert_eq!(decoded, key);
    }

    #[test]
    fn decoding_rejects_wrong_lengths() {
        let short = base64::engine::general_purpose::STANDARD.encode([1u8; 8]);
        assert!(matches!(decode_key(&short), Err(VaultError::KeyStore(_))));
    }
}
