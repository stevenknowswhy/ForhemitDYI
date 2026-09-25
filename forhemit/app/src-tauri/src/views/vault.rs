//! Vault display projections — pure functions over the vault engine's
//! state. The vault's honesty rules shape the view set: locked is a
//! first-class state surfaced, never worked around (Vault doc §50), and
//! the sharing wording states plainly what v1 does and does not do.

use base64::Engine as _;
use forhemit_vault::{SearchHit, StoredVersion, VaultEngine};
use serde::Serialize;

/// The vault's state, as the UI's opening screen renders it.
#[derive(Clone, Debug, Serialize)]
pub struct VaultStatusView {
    /// `not_set_up` (no vault yet) | `ready` (open and usable) |
    /// `locked` (database exists but the key is unavailable — recovery
    /// goes through the recovery passphrase).
    pub state: &'static str,
    /// The vault id.
    pub vault_id: Option<String>,
    /// How many documents the vault holds.
    pub document_count: Option<usize>,
    /// Where exports and backups are saved — shown so the owner knows
    /// exactly where files land.
    pub exports_dir: String,
}

/// The honest v1 sharing wording, exactly as the docs prescribe: the
/// vault keeps everything on this device; the owner exports the package
/// file themselves.
pub const SHARING_WORDING: &str = "v1 keeps everything on this device. Importing a document does not upload it — nothing leaves this machine. When you are ready to share, you export the package file yourself.";

/// The vault's open state, the commands' vocabulary for the status view.
#[derive(Clone, Copy, Debug)]
pub enum VaultState {
    /// The vault is open and usable.
    Ready,
}

/// The status view for a vault whose engine is open.
///
/// # Errors
///
/// The document count read refuses (poisoned lock or a store failure).
pub fn status_view(
    engine: &VaultEngine,
    state: VaultState,
    exports_dir: &str,
) -> Result<VaultStatusView, String> {
    let document_count = engine
        .list_documents()
        .map_err(|error| error.to_string())?
        .len();
    Ok(VaultStatusView {
        state: match state {
            VaultState::Ready => "ready",
        },
        vault_id: Some(engine.vault_id().as_str().to_owned()),
        document_count: Some(document_count),
        exports_dir: exports_dir.to_owned(),
    })
}

/// The status view when no vault exists yet.
#[must_use]
pub fn not_set_up_view(exports_dir: &str) -> VaultStatusView {
    VaultStatusView {
        state: "not_set_up",
        vault_id: None,
        document_count: None,
        exports_dir: exports_dir.to_owned(),
    }
}

/// The status view when the vault exists but its key is unavailable —
/// surfaced, never worked around (Vault doc §50).
#[must_use]
pub fn locked_view(exports_dir: &str) -> VaultStatusView {
    VaultStatusView {
        state: "locked",
        vault_id: None,
        document_count: None,
        exports_dir: exports_dir.to_owned(),
    }
}

/// One document in the vault's list view, as its latest version shows it.
#[derive(Clone, Debug, Serialize)]
pub struct VaultDocumentView {
    /// The document's id.
    pub document_id: String,
    /// The filename.
    pub filename: String,
    /// The file's size in bytes.
    pub byte_count: i64,
    /// The version count.
    pub version_count: usize,
    /// The latest version id.
    pub latest_version_id: String,
    /// The latest created at.
    pub latest_created_at: String,
    /// The note.
    pub note: Option<String>,
}

/// A document's full version history — its latest state plus every
/// version row, oldest first, never rewritten (Vault doc §14).
#[derive(Clone, Debug, Serialize)]
pub struct VaultDocumentHistoryView {
    /// The document's id.
    pub document_id: String,
    /// The version rows, oldest first.
    pub versions: Vec<VaultVersionView>,
}

/// One row of a document's version history — oldest first, never
/// rewritten (Vault doc §14).
#[derive(Clone, Debug, Serialize)]
pub struct VaultVersionView {
    /// The version's id.
    pub version_id: String,
    /// The document's id.
    pub document_id: String,
    /// 1-based position in the document's history (display order).
    pub version_number: usize,
    /// The filename.
    pub filename: String,
    /// The content hash.
    pub content_hash: String,
    /// The file's size in bytes.
    pub byte_count: i64,
    /// The note.
    pub note: Option<String>,
    /// The restored from.
    pub restored_from: Option<String>,
    /// The previous version id.
    pub previous_version_id: Option<String>,
    /// When the record was created (RFC 3339).
    pub created_at: String,
}

/// One search hit with its content excerpt (Vault doc §47 `searchLocal`).
#[derive(Clone, Debug, Serialize)]
pub struct VaultSearchHitView {
    /// The document's id.
    pub document_id: String,
    /// The version's id.
    pub version_id: String,
    /// The filename.
    pub filename: String,
    /// The matching excerpt, in words.
    pub snippet: String,
}

/// One version's decrypted content — base64 for download, plus the text
/// when the bytes are valid UTF-8 so the UI can preview it.
#[derive(Clone, Debug, Serialize)]
pub struct VaultVersionContentView {
    /// The version's id.
    pub version_id: String,
    /// The document's id.
    pub document_id: String,
    /// The filename.
    pub filename: String,
    /// The file's size in bytes.
    pub byte_count: i64,
    /// The file's bytes, base64-encoded for the frontend.
    pub content_base64: String,
    /// The content text.
    pub content_text: Option<String>,
}

/// Formats the stored nanosecond timestamp as RFC 3339; an unformattable
/// value renders empty rather than fabricating a time.
fn iso(unix_nanos: i128) -> String {
    match time::OffsetDateTime::from_unix_timestamp_nanos(unix_nanos) {
        Ok(at) => at
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_default(),
        Err(_) => String::new(),
    }
}

/// The list view of one document.
pub fn document_view(
    engine: &VaultEngine,
    document_id: &forhemit_contracts::DocumentId,
) -> Result<VaultDocumentView, String> {
    let latest = engine
        .read_document(document_id)
        .map_err(|error| error.to_string())?;
    let history = engine
        .version_history(document_id)
        .map_err(|error| error.to_string())?;
    Ok(VaultDocumentView {
        document_id: document_id.as_str().to_owned(),
        filename: latest.version.filename,
        byte_count: latest.version.byte_count,
        version_count: history.len(),
        latest_version_id: latest.version.version_id.as_str().to_owned(),
        latest_created_at: iso(latest.version.created_at_unix_nanos),
        note: latest.version.note,
    })
}

/// A document's full version history.
pub fn version_views(history: &[StoredVersion]) -> Vec<VaultVersionView> {
    history
        .iter()
        .enumerate()
        .map(|(index, version)| VaultVersionView {
            version_id: version.version_id.as_str().to_owned(),
            document_id: version.document_id.as_str().to_owned(),
            version_number: index + 1,
            filename: version.filename.clone(),
            content_hash: version.content_hash.as_str().to_owned(),
            byte_count: version.byte_count,
            note: version.note.clone(),
            restored_from: version
                .restored_from
                .as_ref()
                .map(|id| id.as_str().to_owned()),
            previous_version_id: version
                .previous_version_id
                .as_ref()
                .map(|id| id.as_str().to_owned()),
            created_at: iso(version.created_at_unix_nanos),
        })
        .collect()
}

/// A search hit row.
pub fn search_hit_view(hit: &SearchHit) -> VaultSearchHitView {
    VaultSearchHitView {
        document_id: hit.document_id.as_str().to_owned(),
        version_id: hit.version_id.as_str().to_owned(),
        filename: hit.filename.clone(),
        snippet: hit.snippet.clone(),
    }
}

/// A version's decrypted content view — the `getDocument()` contract
/// (Vault doc §47), base64-wrapped for the wire.
pub fn version_content_view(
    full: &forhemit_vault::StoredVersionContent,
) -> VaultVersionContentView {
    VaultVersionContentView {
        version_id: full.version.version_id.as_str().to_owned(),
        document_id: full.version.document_id.as_str().to_owned(),
        filename: full.version.filename.clone(),
        byte_count: full.version.byte_count,
        content_base64: base64::engine::general_purpose::STANDARD.encode(&full.content),
        content_text: String::from_utf8(full.content.clone()).ok(),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // tests: failures must panic the test

    use super::*;

    #[test]
    fn nanosecond_timestamps_render_as_rfc3339() {
        let rendered = iso(0);
        assert!(rendered.starts_with("1970-01-01T"), "got: {rendered}");
        assert!(iso(i128::MAX).is_empty());
    }
}
