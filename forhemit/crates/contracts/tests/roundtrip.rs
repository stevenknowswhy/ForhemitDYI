//! Round-trip and rejection tests over every public contract type (spec
//! verification matrix, "Contracts crate" row: "serde round-trip tests on
//! every public type; unknown fields rejected").
#![allow(clippy::unwrap_used, clippy::expect_used)] // test code: malformed input must fail the test loudly

use forhemit_contracts::{
    ActionOrigination, ActorClassification, ActorId, ActorKind, ActorRecord, AuditEvent,
    AuditEventDraft, AuditEventType, CausationId, CorrelationId, DecisionLayer, DestinationId,
    EngineId, EventId, FactId, FactVersionId, NonnegotiableState, ObjectId, PayloadRef, Provenance,
    Sha256Hex, TransactionId, Verification, WorkspaceId,
};

/// Serializes to JSON and back, asserting the value survives unchanged.
fn roundtrip<T>(value: T) -> T
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let json = serde_json::to_string(&value).expect("serialization cannot fail here");
    serde_json::from_str(&json).expect("deserialization cannot fail here")
}

const HEX: &str = "3f2a1b0c9d8e7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e9f8a7b6c5d4e3f2a";

#[test]
fn ids_roundtrip_as_bare_strings() {
    let event = EventId::new("evt_01J9X").unwrap();
    assert_eq!(roundtrip(event.clone()), event);
    assert_eq!(serde_json::to_string(&event).unwrap(), "\"evt_01J9X\"");
}

#[test]
fn ids_reject_empty_values() {
    for empty in ["", "   "] {
        assert!(EventId::new(empty).is_err());
        assert!(WorkspaceId::new(empty).is_err());
    }
}

#[test]
fn every_id_type_roundtrips() {
    macro_rules! id_case {
        ($ty:ty, $value:expr) => {{
            let id = <$ty>::new($value).unwrap();
            assert_eq!(roundtrip(id.clone()), id);
            assert_eq!(id.as_str(), $value);
            assert_eq!(id.to_string(), $value);
        }};
    }
    id_case!(EventId, "evt_1");
    id_case!(CorrelationId, "COR-882");
    id_case!(CausationId, "evt_cause");
    id_case!(WorkspaceId, "ws_main");
    id_case!(ObjectId, "obj_dest_1");
    id_case!(DestinationId, "dest_1");
    id_case!(TransactionId, "txn_1");
    id_case!(ActorId, "owner_stefano");
    id_case!(FactId, "fact_1");
    id_case!(FactVersionId, "factver_1");
}

#[test]
fn sha256_hex_rejects_malformed_digests() {
    assert!(Sha256Hex::parse(HEX).is_ok());
    assert!(Sha256Hex::parse(&HEX[..63]).is_err()); // too short
    assert!(Sha256Hex::parse(&format!("{}G", &HEX[..63])).is_err()); // not hex
    assert!(Sha256Hex::parse(&HEX.to_uppercase()).is_err()); // not lowercase
}

#[test]
fn sha256_hex_validates_on_deserialization() {
    let json = format!("\"{HEX}\"");
    assert!(serde_json::from_str::<Sha256Hex>(&json).is_ok());
    let bad = "\"not-a-digest\"";
    assert!(serde_json::from_str::<Sha256Hex>(bad).is_err());
}

#[test]
fn payload_ref_roundtrips() {
    let reference = PayloadRef {
        digest: Sha256Hex::parse(HEX).unwrap(),
    };
    assert_eq!(roundtrip(reference.clone()), reference);
}

#[test]
fn payload_ref_rejects_unknown_fields() {
    let json = format!(r#"{{"digest":"{HEX}","sneaky":true}}"#);
    let result = serde_json::from_str::<PayloadRef>(&json);
    assert!(result.is_err(), "unknown fields must be rejected");
}

#[test]
fn engine_id_covers_all_variants() {
    let all = [
        (EngineId::Audit, "\"audit\""),
        (EngineId::Vault, "\"vault\""),
        (EngineId::Destination, "\"destination\""),
        (EngineId::Journey, "\"journey\""),
        (EngineId::Reality, "\"reality\""),
        (EngineId::Scenario, "\"scenario\""),
        (EngineId::Package, "\"package\""),
        (EngineId::App, "\"app\""),
    ];
    for (variant, wire) in all {
        assert_eq!(serde_json::to_string(&variant).unwrap(), wire);
        assert_eq!(roundtrip(variant.clone()), variant);
    }
}

#[test]
fn actor_kind_covers_all_variants() {
    let all = [
        ActorKind::Owner,
        ActorKind::CoOwner,
        ActorKind::Professional,
        ActorKind::Employee,
        ActorKind::Lender,
        ActorKind::Trustee,
        ActorKind::Buyer,
        ActorKind::PlatformAdministrator,
        ActorKind::ServiceAccount,
        ActorKind::Automation,
        ActorKind::AiAgent,
        ActorKind::Integration,
        ActorKind::SystemProcess,
    ];
    for variant in all {
        assert_eq!(roundtrip(variant.clone()), variant);
    }
    // Spot-check the snake_case wire format.
    assert_eq!(
        serde_json::to_string(&ActorKind::PlatformAdministrator).unwrap(),
        "\"platform_administrator\""
    );
    assert_eq!(
        serde_json::to_string(&ActorKind::AiAgent).unwrap(),
        "\"ai_agent\""
    );
}

#[test]
fn action_origination_covers_all_variants() {
    let all = [
        ActionOrigination::HumanInitiated,
        ActionOrigination::HumanApproved,
        ActionOrigination::SystemGenerated,
        ActionOrigination::AutomatedWorkflow,
        ActionOrigination::ExternalIntegration,
        ActionOrigination::AiAssisted,
        ActionOrigination::AiGenerated,
    ];
    for variant in all {
        assert_eq!(roundtrip(variant.clone()), variant);
    }
    assert_eq!(
        serde_json::to_string(&ActionOrigination::HumanInitiated).unwrap(),
        "\"human_initiated\""
    );
}

#[test]
fn decision_layer_covers_all_variants() {
    let all = [
        DecisionLayer::OwnerObjective,
        DecisionLayer::PlatformScenario,
        DecisionLayer::ProfessionalDetermination,
    ];
    for variant in all {
        assert_eq!(roundtrip(variant.clone()), variant);
    }
    assert_eq!(
        serde_json::to_string(&DecisionLayer::ProfessionalDetermination).unwrap(),
        "\"professional_determination\""
    );
}

#[test]
fn provenance_covers_all_variants() {
    let all = [
        Provenance::OwnerReported,
        Provenance::DocumentSupported,
        Provenance::ResearchSupported,
        Provenance::ProfessionallySupplied,
        Provenance::ScenarioAssumed,
        Provenance::SystemDerived,
        Provenance::ModelDerived,
    ];
    for variant in all {
        assert_eq!(roundtrip(variant.clone()), variant);
    }
    assert_eq!(
        serde_json::to_string(&Provenance::OwnerReported).unwrap(),
        "\"owner_reported\""
    );
}

#[test]
fn verification_covers_all_variants() {
    let all = [
        Verification::Unknown,
        Verification::Unverified,
        Verification::PartiallyVerified,
        Verification::Verified,
        Verification::ProfessionallyVerified,
        Verification::Contested,
        Verification::Rejected,
    ];
    for variant in all {
        assert_eq!(roundtrip(variant.clone()), variant);
    }
    assert_eq!(
        serde_json::to_string(&Verification::PartiallyVerified).unwrap(),
        "\"partially_verified\""
    );
}

#[test]
fn nonnegotiable_state_defaults_to_unspecified() {
    assert_eq!(
        NonnegotiableState::default(),
        NonnegotiableState::Unspecified
    );
    let all = [
        NonnegotiableState::Unspecified,
        NonnegotiableState::Preference,
        NonnegotiableState::StrongPreference,
        NonnegotiableState::Nonnegotiable,
    ];
    for variant in all {
        assert_eq!(roundtrip(variant.clone()), variant);
    }
    assert_eq!(
        serde_json::to_string(&NonnegotiableState::StrongPreference).unwrap(),
        "\"strong_preference\""
    );
}

/// The versioned-tag pattern from the crate docs, pinned as a test so the
/// audit task can rely on it (and so serde behavior changes surface here).
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "contract_version", rename_all = "snake_case")]
enum VersionedPattern {
    /// Version 1 — the only version in this fixture.
    V1 { object_id: String },
}

#[test]
fn versioned_tag_pattern_roundtrips() {
    let value = VersionedPattern::V1 {
        object_id: "obj_1".to_owned(),
    };
    let json = serde_json::to_string(&value).unwrap();
    assert_eq!(json, r#"{"contract_version":"v1","object_id":"obj_1"}"#);
    assert_eq!(roundtrip(value.clone()), value);
}

// --- Audit event contract (contracts/src/audit.rs) ---

/// A fully-detailed actor record for fixtures.
fn sample_actor() -> ActorRecord {
    ActorRecord {
        actor_id: ActorId::new("owner_stefano").unwrap(),
        classification: ActorClassification::Human,
        kind: Some(ActorKind::Owner),
        origination: Some(ActionOrigination::HumanInitiated),
    }
}

/// A minimal-but-complete V1 audit event for fixtures.
fn sample_event() -> AuditEvent {
    AuditEvent::V1 {
        event_id: EventId::new("01JD1WS9S4XQ8V7S9G2H6XJ1WV").unwrap(),
        event_type: AuditEventType::OwnerDecisionRecorded,
        source_engine: EngineId::Destination,
        source_object: ObjectId::new("obj_dest_1").unwrap(),
        actor: sample_actor(),
        timestamp: time::OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap(),
        workspace_id: WorkspaceId::new("ws_main").unwrap(),
        transaction_id: None,
        correlation_id: CorrelationId::new("COR-882").unwrap(),
        causation_id: None,
        payload_reference: PayloadRef {
            digest: Sha256Hex::parse(HEX).unwrap(),
        },
        previous_event_hash: Sha256Hex::parse(HEX).unwrap(),
    }
}

#[test]
fn actor_record_roundtrips_with_full_detail() {
    let actor = sample_actor();
    assert_eq!(roundtrip(actor.clone()), actor);
    assert_eq!(
        serde_json::to_string(&actor.classification).unwrap(),
        "\"human\""
    );
}

#[test]
fn actor_record_rejects_unknown_fields() {
    let json = r#"{"actor_id":"a_1","classification":"human","sneaky":true}"#;
    let result = serde_json::from_str::<ActorRecord>(json);
    assert!(result.is_err(), "unknown fields must be rejected");
}

#[test]
fn audit_event_type_covers_all_variants() {
    let all = [
        (
            AuditEventType::OwnerDecisionRecorded,
            "\"owner_decision_recorded\"",
        ),
        (AuditEventType::ScenarioChanged, "\"scenario_changed\""),
        (
            AuditEventType::CorrectionRecorded,
            "\"correction_recorded\"",
        ),
        (
            AuditEventType::RealityFactRecorded,
            "\"reality_fact_recorded\"",
        ),
        (
            AuditEventType::RealityFactRevised,
            "\"reality_fact_revised\"",
        ),
    ];
    for (variant, wire) in all {
        assert_eq!(serde_json::to_string(&variant).unwrap(), wire);
        assert_eq!(roundtrip(variant.clone()), variant);
    }
}

#[test]
fn audit_event_v1_roundtrips_with_version_tag() {
    let event = sample_event();
    let json = serde_json::to_string(&event).unwrap();
    assert!(
        json.contains(r#""event_version":"v1""#),
        "wire form must carry the version tag: {json}"
    );
    assert_eq!(roundtrip(event.clone()), event);
}

#[test]
fn audit_event_rejects_unknown_or_missing_version_tag() {
    // Unknown version: an older reader must refuse rather than reinterpret.
    let event = sample_event();
    let mut json = serde_json::to_value(&event).unwrap();
    json["event_version"] = serde_json::json!("v999");
    let result = serde_json::from_value::<AuditEvent>(json);
    assert!(result.is_err(), "unknown event_version must be rejected");

    // Missing version tag cannot identify the contract version.
    let event = sample_event();
    let mut json = serde_json::to_value(&event).unwrap();
    json.as_object_mut().unwrap().remove("event_version");
    let result = serde_json::from_value::<AuditEvent>(json);
    assert!(result.is_err(), "missing event_version must be rejected");
}

#[test]
fn audit_event_draft_roundtrips_and_rejects_unknown_fields() {
    let draft = AuditEventDraft {
        event_type: AuditEventType::ScenarioChanged,
        source_engine: EngineId::Scenario,
        source_object: ObjectId::new("obj_scn_1").unwrap(),
        actor: ActorRecord {
            actor_id: ActorId::new("svc_journey").unwrap(),
            classification: ActorClassification::Service,
            kind: Some(ActorKind::ServiceAccount),
            origination: Some(ActionOrigination::SystemGenerated),
        },
        workspace_id: WorkspaceId::new("ws_main").unwrap(),
        transaction_id: Some(TransactionId::new("txn_1").unwrap()),
        correlation_id: CorrelationId::new("COR-882").unwrap(),
        causation_id: None,
        payload: serde_json::json!({ "previous_value": 1, "new_value": 2 }),
    };
    assert_eq!(roundtrip(draft.clone()), draft);

    let mut json = serde_json::to_value(&draft).unwrap();
    json.as_object_mut()
        .unwrap()
        .insert("sneaky".to_owned(), serde_json::json!(true));
    let result = serde_json::from_value::<AuditEventDraft>(json);
    assert!(result.is_err(), "unknown fields must be rejected");
}
