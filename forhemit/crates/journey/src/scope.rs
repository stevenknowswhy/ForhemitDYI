//! Storage Scope — where a question's answer belongs (Journey Builder
//! Architecture doc §6: "Storage Scope Is Part of the Journey Definition").
//!
//! Every question declares its scope, and the runtime stores the answer
//! under it — placement is the definition's decision, never the caller's.
//! In v1 every bucket is physically local (nothing leaves the machine),
//! but the declaration is honored structurally so the hybrid layer can
//! later move only the scopes the definition says it may.

use serde::{Deserialize, Serialize};

/// Where an answer is stored — the Journey Builder doc §6 list, verbatim.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageScope {
    /// Safe to display publicly.
    Public,
    /// Stored in the user's account but not public (the hybrid online
    /// layer — unused in v1, which has no account).
    OnlinePrivate,
    /// Stored only on the user's computer unless explicitly shared.
    LocalPrivate,
    /// Uploaded only after the user authorizes sharing.
    SharedTransactionData,
    /// Information entered or submitted by a professional.
    ProfessionalDetermination,
}
