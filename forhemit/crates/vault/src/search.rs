//! Local full-text search — an in-memory FTS5 index (Vault doc §16 "Local
//! search"; §56 item 4 "Local search"; §57 lock 11: "Local search does not
//! require cloud indexing").
//!
//! The index is **derived data kept strictly inside the encrypted
//! boundary**: it lives in memory only and is rebuilt from the encrypted
//! documents whenever the vault opens. No plaintext index ever touches
//! disk — Vault doc §50 names the rebuild as the designed answer to a
//! broken index, not a fallback ("Broken local index → The Vault can
//! rebuild it from the underlying encrypted data").
//!
//! Text extraction ("Document Intelligence at the Edge", Vault doc §11) is
//! a separate engine concern. The index treats content as text when it is
//! valid UTF-8 and indexes the filename for everything; binary formats
//! (PDF, XLSX) stay findable by name without pretending to have parsed
//! them.

use crate::error::VaultError;
use forhemit_contracts::{DocumentId, DocumentVersionId};
use rusqlite::Connection;

/// One search result: the matching document version and where it matched.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchHit {
    /// The document that matched.
    pub document_id: DocumentId,
    /// The version whose content or filename matched.
    pub version_id: DocumentVersionId,
    /// The document's (decrypted) filename.
    pub filename: String,
    /// A short excerpt of the matching body, with `…` at the cut edges.
    pub snippet: String,
}

/// The in-memory full-text index over the vault's decrypted documents.
#[derive(Debug)]
pub struct SearchIndex {
    /// In-memory connection — the index never persists itself.
    conn: Connection,
}

impl SearchIndex {
    /// Creates an empty index.
    pub fn new() -> Result<Self, VaultError> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            "CREATE VIRTUAL TABLE document_fts USING fts5(
                document_id UNINDEXED,
                version_id UNINDEXED,
                filename,
                body
            );",
        )?;
        Ok(Self { conn })
    }

    /// Indexes one document version: its filename always, and its content
    /// when the bytes are valid UTF-8 (binary formats are filename-only —
    /// text extraction is a different engine, Vault doc §11).
    pub fn index_version(
        &self,
        document_id: &DocumentId,
        version_id: &DocumentVersionId,
        filename: &str,
        plaintext: &[u8],
    ) -> Result<(), VaultError> {
        self.remove_version(document_id, version_id)?;
        let body = std::str::from_utf8(plaintext).unwrap_or("");
        self.conn.execute(
            "INSERT INTO document_fts (document_id, version_id, filename, body) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![document_id.as_str(), version_id.as_str(), filename, body],
        )?;
        Ok(())
    }

    /// Drops one version from the index (used when a version is
    /// re-indexed, and by the full rebuild).
    pub fn remove_version(
        &self,
        document_id: &DocumentId,
        version_id: &DocumentVersionId,
    ) -> Result<(), VaultError> {
        self.conn.execute(
            "DELETE FROM document_fts WHERE document_id = ?1 AND version_id = ?2",
            rusqlite::params![document_id.as_str(), version_id.as_str()],
        )?;
        Ok(())
    }

    /// Empties the index — the starting point of the §50 rebuild.
    pub fn clear(&self) -> Result<(), VaultError> {
        self.conn.execute("DELETE FROM document_fts", [])?;
        Ok(())
    }

    /// Full-text search over filenames and indexed content.
    ///
    /// User input is never passed to FTS5 raw (its query syntax would
    /// turn stray quotes and operators into errors): each whitespace term
    /// is escaped and quoted, terms combine with an implicit AND.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, VaultError> {
        let match_query = fts_query(query);
        if match_query.is_empty() {
            return Ok(Vec::new());
        }
        let mut statement = self.conn.prepare_cached(
            "SELECT document_id, version_id, filename,
                    snippet(document_fts, 3, '→', '←', '…', 12) AS excerpt
             FROM document_fts
             WHERE document_fts MATCH ?1
             ORDER BY rank
             LIMIT ?2",
        )?;
        let hits = statement
            .query_map(rusqlite::params![match_query, limit as i64], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(document_id, version_id, filename, snippet)| {
                Ok(SearchHit {
                    document_id: DocumentId::new(document_id).map_err(|_| {
                        VaultError::Internal("indexed document_id was empty".into())
                    })?,
                    version_id: DocumentVersionId::new(version_id)
                        .map_err(|_| VaultError::Internal("indexed version_id was empty".into()))?,
                    filename,
                    snippet,
                })
            })
            .collect::<Result<Vec<_>, VaultError>>()?;
        Ok(hits)
    }

    /// Number of indexed versions — used by tests and health reporting.
    pub fn len(&self) -> Result<usize, VaultError> {
        Ok(self
            .conn
            .query_row("SELECT count(*) FROM document_fts", [], |row| {
                row.get::<_, i64>(0)
            })? as usize)
    }

    /// Whether the index holds no versions.
    pub fn is_empty(&self) -> Result<bool, VaultError> {
        Ok(self.len()? == 0)
    }
}

/// Builds a safe FTS5 MATCH expression from user input: every
/// whitespace-separated term is quoted (embedded quotes doubled), terms
/// combine with an implicit AND. Empty input yields an empty expression —
/// callers return no hits rather than "match everything".
fn fts_query(query: &str) -> String {
    query
        .split_whitespace()
        .map(|term| format!("\"{}\"", term.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;

    fn hit(index: &SearchIndex, query: &str) -> Vec<SearchHit> {
        index.search(query, 50).unwrap()
    }

    fn sample_index() -> SearchIndex {
        let index = SearchIndex::new().unwrap();
        index
            .index_version(
                &DocumentId::new("doc-1").unwrap(),
                &DocumentVersionId::new("ver-1").unwrap(),
                "2025 P&L.txt",
                b"Q3 EBITDA was 8,240,000. Revenue grew across every line.",
            )
            .unwrap();
        index
            .index_version(
                &DocumentId::new("doc-2").unwrap(),
                &DocumentVersionId::new("ver-2").unwrap(),
                "payroll.xlsx",
                &[0xFF, 0xFE, 0x00, 0x92], // not valid UTF-8
            )
            .unwrap();
        index
    }

    #[test]
    fn finds_content_matches_with_snippets() {
        let index = sample_index();
        let hits = hit(&index, "EBITDA");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].document_id.as_str(), "doc-1");
        assert_eq!(hits[0].filename, "2025 P&L.txt");
        assert!(hits[0].snippet.contains("EBITDA"));
    }

    #[test]
    fn binary_documents_are_found_by_filename_only() {
        let index = sample_index();
        let hits = hit(&index, "payroll");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].document_id.as_str(), "doc-2");
        // And the binary bytes were not indexed as text.
        assert!(hit(&index, "0x92").is_empty());
    }

    #[test]
    fn miss_returns_no_hits() {
        let index = sample_index();
        assert!(hit(&index, "zebra").is_empty());
    }

    #[test]
    fn user_query_syntax_is_escaped_not_interpreted() {
        let index = sample_index();
        // Raw FTS5 would error on a bare quote or operator; these must be
        // treated as literal terms instead.
        assert!(hit(&index, "\"").is_empty());
        assert!(hit(&index, "AND OR NOT (").is_empty());
        assert!(hit(&index, "").is_empty());
    }

    #[test]
    fn clear_and_reindex_round_trip() {
        let index = sample_index();
        assert_eq!(index.len().unwrap(), 2);
        index.clear().unwrap();
        assert!(index.is_empty().unwrap());
        assert!(hit(&index, "EBITDA").is_empty());
        index
            .index_version(
                &DocumentId::new("doc-1").unwrap(),
                &DocumentVersionId::new("ver-1").unwrap(),
                "2025 P&L.txt",
                b"Q3 EBITDA was 8,240,000.",
            )
            .unwrap();
        assert_eq!(hit(&index, "EBITDA").len(), 1);
    }

    #[test]
    fn reindexing_a_version_does_not_duplicate_it() {
        let index = sample_index();
        index
            .index_version(
                &DocumentId::new("doc-1").unwrap(),
                &DocumentVersionId::new("ver-1").unwrap(),
                "renamed P&L.txt",
                b"EBITDA revised.",
            )
            .unwrap();
        let hits = hit(&index, "EBITDA");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].filename, "renamed P&L.txt");
        assert_eq!(index.len().unwrap(), 2);
    }
}
