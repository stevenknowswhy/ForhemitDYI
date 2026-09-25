//! The package engine's error surface.
//!
//! Assembly is a pure read of the snapshot — the failures are shapes the
//! caller must not paper over: a workspace mismatch, or an export attempt
//! that the audit store rejected (audit first: the caller never receives
//! bytes whose event was not recorded).

use forhemit_contracts::WorkspaceId;

/// What can go wrong while assembling or exporting a package.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PackageError {
    /// The workspace id carried by the snapshot was empty.
    EmptyWorkspace,
    /// The snapshot names a workspace the engine was not built for —
    /// the shell mapped state across workspaces, which would forge
    /// provenance.
    WorkspaceMismatch {
        /// The engine's workspace id.
        expected: String,
        /// The snapshot's workspace id.
        got: String,
    },
    /// The audit store rejected the export event — no bytes are returned
    /// (Implementation Roadmap Phase 1: an unrecorded change is a silent
    /// history rewrite by another name).
    AuditRejected(String),
}

impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyWorkspace => f.write_str("workspace id must be a non-empty string"),
            Self::WorkspaceMismatch { expected, got } => write!(
                f,
                "snapshot workspace {got:?} does not match the engine's workspace {expected:?}"
            ),
            Self::AuditRejected(reason) => {
                write!(f, "the audit store rejected the export event: {reason}")
            }
        }
    }
}

impl std::error::Error for PackageError {}

impl From<forhemit_enginekit::AuditError> for PackageError {
    fn from(error: forhemit_enginekit::AuditError) -> Self {
        Self::AuditRejected(error.0)
    }
}

/// Validates the workspace id a snapshot carries.
pub(crate) fn validate_workspace(workspace_id: &WorkspaceId) -> Result<(), PackageError> {
    if workspace_id.as_str().trim().is_empty() {
        Err(PackageError::EmptyWorkspace)
    } else {
        Ok(())
    }
}
