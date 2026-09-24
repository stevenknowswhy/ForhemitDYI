//! Cryptographic primitives of the vault engine.
//!
//! AES-256-GCM with 96-bit random nonces (spec locked decision) and a
//! strict key hierarchy (Vault doc §9):
//!
//! ```text
//! recovery passphrase →(Argon2id)→ recovery key ⇒ wraps vault key
//! vault key (OS key store)                      ⇒ wraps per-document keys
//! document key                                  ⇒ AES-256-GCM ciphertext
//! ```
//!
//! Every sealed buffer carries additional authenticated data (AAD) that
//! binds the ciphertext to its database row — a ciphertext or wrapped key
//! moved to another row fails authentication and surfaces as
//! [`VaultError::Crypto`], never as garbage plaintext.
//!
//! Only established, audited primitives are used (Vault doc §9: "The exact
//! cryptographic implementation should use established, audited primitives
//! rather than custom encryption") — no custom modes, no homegrown KDFs.
//! `unsafe_code = "forbid"` holds workspace-wide.

use crate::error::VaultError;
use aes_gcm::{
    aead::consts::U12,
    aead::{rand_core::RngCore, AeadInPlace},
    Aes256Gcm, Key, KeyInit, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use base64::Engine as _;
use forhemit_contracts::Sha256Hex;
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use zeroize::Zeroizing;

/// Symmetric key length in bytes — 256 bits.
pub const KEY_LEN: usize = 32;

/// AEAD nonce length in bytes — 96 bits, the GCM-recommended size.
pub const NONCE_LEN: usize = 12;

/// Generates a fresh 256-bit symmetric key from the OS entropy source.
///
/// This is a fresh per-document DEK (Vault doc §9: one "Document
/// Encryption Key" per protected resource, rotatable independently of the
/// vault key).
pub fn generate_key() -> [u8; KEY_LEN] {
    let key: Key<Aes256Gcm> = Aes256Gcm::generate_key(&mut aes_gcm::aead::rand_core::OsRng);
    let mut bytes = [0u8; KEY_LEN];
    bytes.copy_from_slice(key.as_slice());
    bytes
}

/// Generates a fresh 96-bit AEAD nonce from the OS entropy source.
pub fn random_nonce() -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    aes_gcm::aead::rand_core::OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Generates `len` random bytes from the OS entropy source (salts and
/// other non-nonce randomness).
pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    aes_gcm::aead::rand_core::OsRng.fill_bytes(&mut bytes);
    bytes
}

/// Encrypts `plaintext` under `key` with the given `nonce`, binding the
/// result to `aad` (authenticated, unencrypted context).
///
/// Key sizes are enforced by construction ([`KEY_LEN`], [`NONCE_LEN`] are
/// fixed-length arrays at every call site).
pub fn seal(
    key: &[u8; KEY_LEN],
    nonce: &[u8; NONCE_LEN],
    aad: &[u8],
    plaintext: &[u8],
    operation: &'static str,
) -> Result<Vec<u8>, VaultError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut buffer = plaintext.to_vec();
    cipher
        .encrypt_in_place(Nonce::<U12>::from_slice(nonce), aad, &mut buffer)
        .map_err(|_| VaultError::Crypto { operation })?;
    Ok(buffer)
}

/// Decrypts `ciphertext` under `key`, verifying the AEAD tag and the `aad`
/// binding.
///
/// The returned buffer is [`Zeroizing`] so the plaintext is wiped when the
/// last handle drops — decrypted document content never lingers.
pub fn open(
    key: &[u8; KEY_LEN],
    nonce: &[u8; NONCE_LEN],
    aad: &[u8],
    ciphertext: &[u8],
    operation: &'static str,
) -> Result<Zeroizing<Vec<u8>>, VaultError> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut buffer = ciphertext.to_vec();
    cipher
        .decrypt_in_place(Nonce::<U12>::from_slice(nonce), aad, &mut buffer)
        .map_err(|_| VaultError::Crypto { operation })?;
    Ok(Zeroizing::new(buffer))
}

/// Derives the 32-byte recovery key from the owner's recovery passphrase
/// and a per-vault random salt, with Argon2id (spec locked decision:
/// "Argon2id recovery").
///
/// Parameters: 19 MiB of memory (OWASP Argon2id baseline for interactive
/// secrets), 2 iterations, degree 1. Deterministic by design — the same
/// passphrase and salt must reproduce the key that opens the escrowed
/// vault-key wrapping on any device.
pub fn derive_recovery_key(
    passphrase: &str,
    salt: &[u8],
) -> Result<Zeroizing<[u8; KEY_LEN]>, VaultError> {
    let params = Params::new(19_456, 2, 1, None)
        .map_err(|error| VaultError::Internal(format!("argon2 parameters: {error}")))?;
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    argon
        .hash_password_into(passphrase.as_bytes(), salt, key.as_mut())
        .map_err(|error| VaultError::Internal(format!("argon2 derivation: {error}")))?;
    Ok(key)
}

/// Lowercase-hex SHA-256 of `bytes` — the document content hash recorded
/// per version (Vault doc §13: "Hash/checksum"; §47: `source_hash`).
pub fn sha256_hex(bytes: &[u8]) -> Sha256Hex {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut hex = String::with_capacity(2 * digest.len());
    for byte in digest {
        let _ = write!(
            hex,
            "{}{}",
            HEX[(byte >> 4) as usize] as char,
            HEX[(byte & 0x0f) as usize] as char
        );
    }
    Sha256Hex::parse(&hex)
        .unwrap_or_else(|_| unreachable!("a SHA-256 digest is always 64 lowercase hex chars"))
}

pub(crate) fn encode_b64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

pub(crate) fn decode_b64(encoded: &str) -> Result<Vec<u8>, VaultError> {
    base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|error| VaultError::Backup(format!("invalid base64: {error}")))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    #[test]
    fn seal_and_open_round_trip() {
        let key = generate_key();
        let nonce = random_nonce();
        let sealed = seal(&key, &nonce, b"context", b"EBITDA 8,240,000", "test").unwrap();
        assert_ne!(sealed, b"EBITDA 8,240,000");
        let opened = open(&key, &nonce, b"context", &sealed, "test").unwrap();
        assert_eq!(opened.as_slice(), b"EBITDA 8,240,000");
    }

    #[test]
    fn tampered_ciphertext_fails_authentication() {
        let key = generate_key();
        let nonce = random_nonce();
        let mut sealed = seal(&key, &nonce, b"context", b"secret", "test").unwrap();
        let last = sealed.len() - 1;
        sealed[last] ^= 0x01;
        assert!(matches!(
            open(&key, &nonce, b"context", &sealed, "test"),
            Err(VaultError::Crypto { .. })
        ));
    }

    #[test]
    fn mismatched_aad_fails_authentication() {
        let key = generate_key();
        let nonce = random_nonce();
        let sealed = seal(&key, &nonce, b"row-1", b"secret", "test").unwrap();
        assert!(matches!(
            open(&key, &nonce, b"row-2", &sealed, "test"),
            Err(VaultError::Crypto { .. })
        ));
    }

    #[test]
    fn wrong_key_fails_authentication() {
        let nonce = random_nonce();
        let sealed = seal(&generate_key(), &nonce, b"aad", b"secret", "test").unwrap();
        assert!(matches!(
            open(&generate_key(), &nonce, b"aad", &sealed, "test"),
            Err(VaultError::Crypto { .. })
        ));
    }

    #[test]
    fn recovery_key_is_deterministic_per_passphrase() {
        // Argon2 requires salts of at least 8 bytes; production escrows use
        // SALT_LEN random bytes.
        let first = derive_recovery_key("correct horse battery staple", b"salt-A-long").unwrap();
        let again = derive_recovery_key("correct horse battery staple", b"salt-A-long").unwrap();
        let other = derive_recovery_key("incorrect horse", b"salt-A-long").unwrap();
        let other_salt =
            derive_recovery_key("correct horse battery staple", b"salt-B-long").unwrap();
        assert_eq!(first.as_ref(), again.as_ref());
        assert_ne!(first.as_ref(), other.as_ref());
        assert_ne!(first.as_ref(), other_salt.as_ref());
    }

    #[test]
    fn content_hash_is_stable_and_well_formed() {
        let a = sha256_hex(b"forhemit");
        let b = sha256_hex(b"forhemit");
        let c = sha256_hex(b"Forhemit");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.as_str().len(), 64);
    }
}
