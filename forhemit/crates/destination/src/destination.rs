//! The Destination aggregate — the journey's persistent North Star.
//!
//! Destination First - Locked Core Architecture: the destination is "the
//! first object created by every journey and the persistent North Star."
//! The aggregate is an append-only list of immutable
//! [`DestinationVersion`]s plus the destination's current process status:
//! no method exposes `&mut` access to stored versions, so history cannot
//! be overwritten through the type system, only extended (Destination
//! Engine doc §26, §46 rule 9).

use crate::content::DestinationContent;
use crate::status::DestinationStatus;
use crate::version::DestinationVersion;
use forhemit_contracts::{
    ActorRecord, DestinationId, DestinationVersionId, NonnegotiableState, ObjectiveId, WorkspaceId,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// A DesiredOutcome / Destination: the owner's desired end state as a
/// whole, versioned, with every historical version retained.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Destination {
    /// The destination's identifier — the object ID audit events for this
    /// destination reference.
    pub destination_id: DestinationId,
    /// The workspace the destination belongs to.
    pub workspace_id: WorkspaceId,
    /// Process state of the current (head) version — doc §6 statuses.
    /// Superseded is never stored here; see
    /// [`Destination::status_of_version`].
    pub status: DestinationStatus,
    /// When the destination was created.
    pub created_at: OffsetDateTime,
    /// Who created the destination.
    pub created_by: ActorRecord,
    /// The version chain, oldest first — append-only by construction:
    /// no accessor returns a mutable reference into it.
    versions: Vec<DestinationVersion>,
}

impl Destination {
    /// Creates a destination with its initial (immutable) version.
    pub(crate) fn create(
        destination_id: DestinationId,
        workspace_id: WorkspaceId,
        content: DestinationContent,
        created_by: ActorRecord,
        created_at: OffsetDateTime,
    ) -> Self {
        let initial = DestinationVersion::initial(
            destination_id.clone(),
            content,
            created_by.clone(),
            created_at,
        );
        Self {
            destination_id,
            workspace_id,
            status: DestinationStatus::Draft,
            created_at,
            created_by,
            versions: vec![initial],
        }
    }

    /// The head version — the destination's current content and the
    /// version audit events for edits reference.
    pub fn head_version(&self) -> &DestinationVersion {
        self.versions
            .last()
            .unwrap_or_else(|| unreachable!("a destination always has its initial version"))
    }

    /// The current content.
    pub fn head_content(&self) -> &DestinationContent {
        &self.head_version().content
    }

    /// Every retained version, oldest first. The slice is immutable —
    /// history is queryable, never editable.
    pub fn history(&self) -> &[DestinationVersion] {
        &self.versions
    }

    /// One retained version by ID.
    pub fn version(&self, version_id: &DestinationVersionId) -> Option<&DestinationVersion> {
        self.versions
            .iter()
            .find(|version| &version.version_id == version_id)
    }

    /// The process status of a version as seen from the destination's
    /// current state: the head version carries the stored status; every
    /// earlier version is derived as Superseded (doc §6) — never stored,
    /// always computed.
    pub fn status_of_version(&self, version_id: &DestinationVersionId) -> DestinationStatus {
        if &self.head_version().version_id == version_id {
            self.status.clone()
        } else {
            DestinationStatus::Superseded
        }
    }

    /// Appends a successor version. The engine is the only caller; the
    /// method takes the new version as a value and cannot rewrite any
    /// stored one.
    pub(crate) fn append_version(&mut self, version: DestinationVersion) {
        self.versions.push(version);
    }

    /// Sets the process status of the head version (engine-only).
    pub(crate) fn set_status(&mut self, status: DestinationStatus) {
        self.status = status;
    }

    /// Every objective in the head content that carries a preference
    /// designation, as stable reference IDs paired with their state.
    ///
    /// This is the machinery the implementation spec points at for
    /// nonnegotiable constraints: "constraints carry their state across
    /// versions and expose reference IDs so later scenario conflicts can
    /// back-reference them." A later `NonnegotiableConflict` cites one of
    /// these `ObjectiveId`s.
    pub fn designated_objectives(&self) -> Vec<(ObjectiveId, NonnegotiableState)> {
        self.head_content().designated_objectives()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)] // test code: failures must panic the test

    use super::*;
    use crate::answer::Answer;
    use crate::content::{Avoidance, DestinationContent};
    use crate::objective::Objective;
    use crate::version::new_version_id;
    use forhemit_contracts::{ActionOrigination, ActorClassification, ActorId, ActorKind};

    fn actor() -> ActorRecord {
        ActorRecord {
            actor_id: ActorId::new("owner-1").unwrap(),
            classification: ActorClassification::Human,
            kind: Some(ActorKind::Owner),
            origination: Some(ActionOrigination::HumanInitiated),
        }
    }

    fn destination() -> Destination {
        Destination::create(
            DestinationId::new("dst-1").unwrap(),
            WorkspaceId::new("ws-1").unwrap(),
            DestinationContent::default(),
            actor(),
            time::OffsetDateTime::UNIX_EPOCH,
        )
    }

    #[test]
    fn a_new_destination_has_one_retained_version() {
        let destination = destination();
        assert_eq!(destination.history().len(), 1);
        assert_eq!(destination.history()[0].version_number, 1);
        assert_eq!(
            destination.head_version().version_id,
            destination.history()[0].version_id
        );
        assert_eq!(destination.status, DestinationStatus::Draft);
    }

    #[test]
    fn appended_versions_do_not_touch_the_original() {
        let mut destination = destination();
        let original_id = destination.head_version().version_id.clone();
        let original_content = destination.head_content().clone();

        let successor = DestinationVersion::successor(
            destination.head_version(),
            DestinationContent::default(),
            crate::version::ChangeReason::PrioritiesChanged,
            None,
            Vec::new(),
            actor(),
            time::OffsetDateTime::UNIX_EPOCH,
        );
        let successor_id = successor.version_id.clone();
        destination.append_version(successor);

        assert_eq!(destination.history().len(), 2);
        assert_eq!(destination.head_version().version_id, successor_id);
        // The original version is intact.
        let retained = destination.version(&original_id).unwrap();
        assert_eq!(retained.content, original_content);
        assert_eq!(retained.version_number, 1);
        assert_eq!(retained.previous_version_id, None);
    }

    #[test]
    fn non_head_versions_read_as_superseded() {
        let mut destination = destination();
        let first_id = destination.head_version().version_id.clone();
        destination.append_version(DestinationVersion::successor(
            destination.head_version(),
            DestinationContent::default(),
            crate::version::ChangeReason::BusinessChanged,
            None,
            Vec::new(),
            actor(),
            time::OffsetDateTime::UNIX_EPOCH,
        ));

        assert_eq!(
            destination.status_of_version(&first_id),
            DestinationStatus::Superseded
        );
        assert_eq!(
            destination.status_of_version(&destination.head_version().version_id),
            DestinationStatus::Draft
        );
    }

    #[test]
    fn designated_objectives_expose_reference_ids() {
        let mut content = DestinationContent::default();
        // One nonnegotiable financial objective and one avoidance — the
        // two shapes objectives appear in (single field, list entries).
        content.financial_objective.preference = NonnegotiableState::Nonnegotiable;
        content.avoidances = Answer::Answered(vec![Objective::empty()
            .with_value(Answer::Answered(Avoidance::LosingEmployeeOwnership))
            .with_preference(NonnegotiableState::Nonnegotiable)]);

        let destination = Destination::create(
            DestinationId::new("dst-2").unwrap(),
            WorkspaceId::new("ws-1").unwrap(),
            content,
            actor(),
            time::OffsetDateTime::UNIX_EPOCH,
        );

        let designated = destination.designated_objectives();
        assert_eq!(designated.len(), 2);
        assert!(designated
            .iter()
            .all(|(_, state)| *state == NonnegotiableState::Nonnegotiable));
    }

    #[test]
    fn destination_round_trips_through_serde_with_history() {
        let mut destination = destination();
        destination.append_version(DestinationVersion::successor(
            destination.head_version(),
            DestinationContent::default(),
            crate::version::ChangeReason::ExploringDifferentOutcome,
            Some("want to see the numbers".to_owned()),
            Vec::new(),
            actor(),
            time::OffsetDateTime::UNIX_EPOCH,
        ));
        destination.set_status(DestinationStatus::Confirmed);

        let json = serde_json::to_string(&destination).unwrap();
        let back: Destination = serde_json::from_str(&json).unwrap();
        assert_eq!(back, destination);
        assert_eq!(back.history().len(), 2);
        assert_eq!(back.status, DestinationStatus::Confirmed);

        // Unknown fields rejected.
        let mut tampered: serde_json::Value = serde_json::from_str(&json).unwrap();
        tampered["rank"] = serde_json::Value::Null;
        assert!(serde_json::from_str::<Destination>(&tampered.to_string()).is_err());
    }

    #[test]
    fn new_version_id_is_unique() {
        let a = new_version_id();
        let b = new_version_id();
        assert_ne!(a, b);
    }
}
