//! Backup export and restore (Vault doc §37 "Exportable encrypted
//! backups"; spec locked decision: "`age` for exportable backups").
//!
//! A backup is the vault's SQLite database — which contains only
//! ciphertext and the recovery escrow, never the vault key — wrapped in an
//! `age` file encrypted with the owner's **recovery passphrase**. One
//! credential therefore does two jobs: restoring the backup file, and
//! re-deriving the vault key from the escrow inside it (via the recovery
//! path). A backup without the passphrase is inert bytes, exactly as the
//! Vault doc requires: "Encrypted Backups — encrypted by the user's keys
//! and readable only by the user."
//!
//! `age` brings its own hardened passphrase KDF (scrypt), so the backup
//! layer does not re-derive anything itself.

use crate::error::VaultError;
use age::secrecy::SecretString;
use std::io::{Read, Write};

/// Marker string identifying a Forhemit vault backup payload.
pub const BACKUP_MARKER: &[u8] = b"FORHEMIT-VAULT-BACKUP-v1\n";

/// Exports an encrypted backup of the vault database to `output_path`.
///
/// `database_bytes` is the raw SQLite file (ciphertext at rest — it may be
/// read while the vault is open only if the caller guarantees
/// transactional consistency; the store module snapshots it in a
/// transaction). The payload is prefixed with [`BACKUP_MARKER`] so a
/// corrupted or foreign `.age` file is diagnosed as a backup-format error
/// instead of a crypto error.
pub fn export_backup(
    database_bytes: &[u8],
    passphrase: &str,
    output_path: &std::path::Path,
) -> Result<(), VaultError> {
    if passphrase.is_empty() {
        return Err(VaultError::Backup(
            "recovery passphrase must not be empty".to_owned(),
        ));
    }
    let encryptor = age::Encryptor::with_user_passphrase(SecretString::from(passphrase.to_owned()));
    let mut output = std::fs::File::create(output_path)
        .map_err(|error| VaultError::Backup(format!("create backup file: {error}")))?;
    let mut writer = encryptor
        .wrap_output(&mut output)
        .map_err(|error| VaultError::Backup(format!("age wrap: {error}")))?;
    writer
        .write_all(BACKUP_MARKER)
        .and_then(|()| writer.write_all(database_bytes))
        .map_err(|error| VaultError::Backup(format!("write backup payload: {error}")))?;
    writer
        .finish()
        .map_err(|error| VaultError::Backup(format!("age finish: {error}")))?;
    output
        .sync_all()
        .map_err(|error| VaultError::Backup(format!("sync backup file: {error}")))?;
    Ok(())
}

/// Restores a backup: decrypts it with the recovery passphrase and
/// returns the raw SQLite database bytes (still all-ciphertext — restoring
/// *into* a workspace and re-keying happens through the store's recovery
/// path, not here).
pub fn import_backup(
    backup_path: &std::path::Path,
    passphrase: &str,
) -> Result<Vec<u8>, VaultError> {
    let file = std::fs::File::open(backup_path)
        .map_err(|error| VaultError::Backup(format!("open backup file: {error}")))?;
    let decryptor = age::Decryptor::new(file)
        .map_err(|error| VaultError::Backup(format!("not an age file: {error}")))?;
    if !decryptor.is_scrypt() {
        return Err(VaultError::Backup(
            "backup is not passphrase-encrypted".to_owned(),
        ));
    }
    let identity = age::scrypt::Identity::new(SecretString::from(passphrase.to_owned()));
    let mut reader = decryptor
        .decrypt(std::iter::once(&identity as &dyn age::Identity))
        .map_err(|_| {
            VaultError::Backup(
                "backup decryption failed: wrong passphrase or corrupt file".to_owned(),
            )
        })?;
    let mut payload = Vec::new();
    reader
        .read_to_end(&mut payload)
        .map_err(|error| VaultError::Backup(format!("read backup payload: {error}")))?;
    let marker_len = BACKUP_MARKER.len();
    if payload.len() < marker_len || !payload.starts_with(BACKUP_MARKER) {
        return Err(VaultError::Backup(
            "decrypted payload is not a Forhemit vault backup".to_owned(),
        ));
    }
    Ok(payload.split_off(marker_len))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    fn temp_path(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join("forhemit-vault-tests");
        std::fs::create_dir_all(&dir).unwrap();
        dir.join(name)
    }

    #[test]
    fn backup_round_trips_the_database() {
        let path = temp_path("round-trip.age");
        let database = b"sqlite-ciphertext-bytes-not-plaintext";
        export_backup(database, "recovery passphrase", &path).unwrap();
        let restored = import_backup(&path, "recovery passphrase").unwrap();
        assert_eq!(restored, database);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn wrong_passphrase_fails_the_restore() {
        let path = temp_path("wrong-pass.age");
        export_backup(b"database", "right", &path).unwrap();
        assert!(matches!(
            import_backup(&path, "wrong"),
            Err(VaultError::Backup(_))
        ));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn backup_file_on_disk_is_not_the_database() {
        let path = temp_path("ciphertext-on-disk.age");
        let database = b"obvious sqlite database content";
        export_backup(database, "pass", &path).unwrap();
        let on_disk = std::fs::read(&path).unwrap();
        assert!(on_disk.starts_with(b"age-encryption.org"));
        assert!(!on_disk.windows(database.len()).any(|w| w == database));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn non_age_file_is_reported_as_backup_error() {
        let path = temp_path("not-age.age");
        std::fs::write(&path, b"totally not an age file").unwrap();
        assert!(matches!(
            import_backup(&path, "pass"),
            Err(VaultError::Backup(_))
        ));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn empty_passphrase_is_rejected_at_export() {
        let path = temp_path("empty-pass.age");
        assert!(matches!(
            export_backup(b"database", "", &path),
            Err(VaultError::Backup(_))
        ));
        let _ = std::fs::remove_file(&path);
    }
}
