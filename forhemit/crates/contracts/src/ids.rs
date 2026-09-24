//! Typed identifier newtypes.
//!
//! The audit event contract names every object it touches through
//! identifiers (Audit doc §6: "Audit Event ID … Organization/workspace …
//! Source object … Correlation ID … Causation ID"); typed newtypes keep an
//! [`EventId`] from being passed where a [`WorkspaceId`] is expected. All
//! IDs are serde-transparent newtypes over `String`; ULID generation for
//! [`EventId`]/[`CorrelationId`] lands with the audit store task.

use serde::{Deserialize, Serialize};

/// Returned when an ID is constructed from an empty string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmptyIdError;

impl std::fmt::Display for EmptyIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("identifier must be a non-empty string")
    }
}

impl std::error::Error for EmptyIdError {}

macro_rules! define_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(
            Clone,
            Debug,
            Deserialize,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            schemars::JsonSchema,
            Serialize,
        )]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates a new identifier, rejecting empty values.
            pub fn new(value: impl Into<String>) -> Result<Self, EmptyIdError> {
                let value = value.into();
                if value.trim().is_empty() {
                    Err(EmptyIdError)
                } else {
                    Ok(Self(value))
                }
            }

            /// Borrows the underlying string.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

define_id!(
    /// Identifies one audit event. ULID generation lands with the audit store.
    EventId
);

define_id!(
    /// Groups related activity under one identifier (Audit doc §18: "Show me
    /// everything that happened because of this change").
    CorrelationId
);

define_id!(
    /// Names the immediate event that caused this one (Audit doc §19) — the
    /// direct causal chain, distinct from correlation.
    CausationId
);

define_id!(
    /// Identifies the workspace ("Organization/workspace", Audit doc §6) an
    /// event or object belongs to.
    WorkspaceId
);

define_id!(
    /// Identifies the source object an event is about (Audit doc §6: "Source
    /// object"); version lineage per Audit doc §21.
    ObjectId
);

define_id!(
    /// Identifies a DesiredOutcome — the destination is "the first object
    /// created by every journey" (Destination First - Locked Core
    /// Architecture).
    DestinationId
);
