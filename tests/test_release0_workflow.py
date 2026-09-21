from __future__ import annotations

import tempfile
import unittest
from datetime import datetime, timedelta, timezone
from pathlib import Path

from app import AuthorizationError, InvariantError, Release0Workflow


class MutableClock:
    def __init__(self):
        self.value = datetime(2026, 9, 21, 16, 0, tzinfo=timezone.utc)

    def __call__(self) -> str:
        return self.value.isoformat().replace("+00:00", "Z")

    def advance(self, seconds: int = 1) -> None:
        self.value += timedelta(seconds=seconds)


def ref(object_type, object_id, version=1):
    return {
        "object_type": object_type,
        "object_id": object_id,
        "version": version,
    }


class Release0WorkflowTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.clock = MutableClock()
        root = Path(self.temp.name)
        self.workflow = Release0Workflow(
            root / "workspace.sqlite",
            root / "collaboration.sqlite",
            clock=self.clock,
        )
        self.owner = "owner-001"
        self.business = "business-001"
        self.professional = "professional-001"

    def tearDown(self):
        self.workflow.close()
        self.temp.cleanup()

    def facts(self):
        result = []
        values = {
            "business_name": "Example Manufacturing",
            "entity_type": "corporation",
            "revenue_range": {"min": 5_000_000, "max": 8_000_000},
            "operating_cash_flow_range": {"min": 700_000, "max": 1_200_000},
            "employee_count": 42,
            "ownership_summary": "Owner controls 100%",
            "management_summary": "Management team exists",
            "debt_range": None,
        }
        for index, (field, value) in enumerate(values.items(), 1):
            result.append(
                {
                    "fact_id": f"fact-{index:03d}",
                    "field": field,
                    "value": value,
                    "value_status": "unknown" if value is None else (
                        "range" if isinstance(value, dict) else "known"
                    ),
                    "source_authority": "owner_stated",
                    "sensitivity": "restricted",
                    "provenance": [
                        {
                            "source_id": "owner-entry-001",
                            "source_type": "owner_input",
                            "source_version": "1",
                            "information_status": "stated",
                            "observed_at": self.clock(),
                            "excerpt_hash": None,
                        }
                    ],
                }
            )
        return result

    def create_destination(self, key="destination-1"):
        return self.workflow.confirm_destination(
            actor_id=self.owner,
            owner_id=self.owner,
            business_id=self.business,
            destination_id="dest-001",
            transition_horizon="1_to_3_years",
            employee_ownership_intent="explore",
            owner_involvement="transition_period",
            objectives=[
                {
                    "objective_id": "objective-001",
                    "statement": "Explore meaningful employee ownership",
                    "priority": 1,
                }
            ],
            nonnegotiables=[
                {
                    "nonnegotiable_id": "nonnegotiable-001",
                    "statement": "No path may proceed without professional review",
                    "source_authority": "owner_stated",
                }
            ],
            idempotency_key=key,
        )

    def create_snapshot(self, key="snapshot-1"):
        return self.workflow.confirm_business_snapshot(
            actor_id=self.owner,
            owner_id=self.owner,
            business_id=self.business,
            current_state_id="state-001",
            primary_jurisdiction="US-CA",
            facts=self.facts(),
            idempotency_key=key,
        )

    def create_scenario(self, destination_version=1, snapshot_version=1):
        return self.workflow.create_scenario(
            actor_id=self.owner,
            owner_id=self.owner,
            scenario_id="scenario-001",
            destination_reference=ref(
                "DesiredOutcome", "dest-001", destination_version
            ),
            current_state_reference=ref(
                "BusinessCurrentState", "state-001", snapshot_version
            ),
            assumptions=["Employee ownership remains an exploration goal"],
            unknowns=["Independent valuation"],
            conflicts=[],
            professional_questions=[
                "What additional facts are required to assess ESOP feasibility?"
            ],
            idempotency_key=f"scenario-{destination_version}-{snapshot_version}",
        )

    def grant_consent(self):
        expiry = (
            self.clock.value + timedelta(days=30)
        ).isoformat().replace("+00:00", "Z")
        return self.workflow.grant_professional_consent(
            actor_id=self.owner,
            owner_id=self.owner,
            consent_id="consent-001",
            recipient_id=self.professional,
            purpose="california_esop_exploration_review",
            resource_references=[
                ref("ScenarioVersion", "scenario-001", 1)
            ],
            permissions=["full_view"],
            expires_at=expiry,
            idempotency_key="consent-1",
        )

    def authorize_package(self, key="package-1", recipient=None):
        return self.workflow.authorize_review_package(
            actor_id=self.owner,
            owner_id=self.owner,
            business_id=self.business,
            package_id="package-001",
            recipient_id=recipient or self.professional,
            professional_role="ESOP attorney",
            purpose="california_esop_exploration_review",
            destination_reference=ref("DesiredOutcome", "dest-001", 1),
            current_state_reference=ref(
                "BusinessCurrentState", "state-001", 1
            ),
            scenario_reference=ref("ScenarioVersion", "scenario-001", 1),
            consent_reference=ref("ConsentRecord", "consent-001", 1),
            included_categories=[
                "destination",
                "business_snapshot",
                "exploratory_scenario",
                "professional_questions",
            ],
            excluded_categories=["raw_documents", "external_ai_records"],
            idempotency_key=key,
        )

    def prepare_through_consent(self):
        self.create_destination()
        self.clock.advance()
        self.create_snapshot()
        self.clock.advance()
        self.create_scenario()
        self.clock.advance()
        self.grant_consent()
        self.clock.advance()

    def test_complete_manual_slice_without_jev(self):
        self.prepare_through_consent()
        result = self.authorize_package()

        # Disclosure exists in the private workspace before boundary delivery.
        disclosure = self.workflow.workspace.get_version(
            "DisclosureRecord",
            result["disclosure"]["disclosure_id"],
            1,
        )
        self.assertEqual(disclosure["recipient_id"], self.professional)
        self.assertEqual(self.workflow.deliver_packages(), 1)

        response = self.workflow.submit_professional_response(
            actor_id=self.professional,
            package_id="package-001",
            package_version=1,
            professional_role="ESOP attorney",
            response_text="Additional feasibility work and qualified valuation are required.",
            category="information_request",
            idempotency_key="response-1",
        )
        self.clock.advance()
        self.assertEqual(self.workflow.import_professional_responses(), 1)

        acknowledgment = self.workflow.acknowledge_professional_response(
            actor_id=self.owner,
            owner_id=self.owner,
            determination_id=response["response_id"],
            statement="I acknowledge this feedback and have not selected a transaction.",
            idempotency_key="ack-1",
        )
        self.assertFalse(acknowledgment["agreement_inferred"])
        self.assertFalse(acknowledgment["automatic_handoff_created"])
        self.assertEqual(
            self.workflow.workspace.event_count(
                "ProfessionalResponseAcknowledged"
            ),
            1,
        )

    def test_only_owner_can_confirm_owner_intent(self):
        with self.assertRaises(AuthorizationError):
            self.workflow.confirm_destination(
                actor_id="advisor-001",
                owner_id=self.owner,
                business_id=self.business,
                destination_id="dest-001",
                transition_horizon="1_to_3_years",
                employee_ownership_intent="explore",
                owner_involvement="unknown",
                objectives=[{"objective_id": "o1"}],
                nonnegotiables=[],
                idempotency_key="unauthorized",
            )

    def test_unresolved_destination_contradiction_blocks_confirmation(self):
        with self.assertRaisesRegex(InvariantError, "contradictions"):
            self.workflow.confirm_destination(
                actor_id=self.owner,
                owner_id=self.owner,
                business_id=self.business,
                destination_id="dest-001",
                transition_horizon="1_to_3_years",
                employee_ownership_intent="explore",
                owner_involvement="unknown",
                objectives=[{"objective_id": "o1"}],
                nonnegotiables=[],
                contradictions=["exit immediately vs remain indefinitely"],
                idempotency_key="contradiction",
            )

    def test_unknown_fact_cannot_become_zero(self):
        facts = self.facts()
        debt = next(f for f in facts if f["field"] == "debt_range")
        debt["value"] = 0
        with self.assertRaisesRegex(InvariantError, "cannot carry a value"):
            self.workflow.confirm_business_snapshot(
                actor_id=self.owner,
                owner_id=self.owner,
                business_id=self.business,
                current_state_id="state-001",
                primary_jurisdiction="US-CA",
                facts=facts,
                idempotency_key="unknown-zero",
            )

    def test_package_recipient_must_match_consent(self):
        self.prepare_through_consent()
        with self.assertRaisesRegex(InvariantError, "recipient"):
            self.authorize_package(recipient="another-professional")

    def test_authorization_is_idempotent_and_does_not_duplicate_outbox(self):
        self.prepare_through_consent()
        first = self.authorize_package(key="same-command")
        second = self.authorize_package(key="same-command")
        self.assertEqual(first, second)
        self.assertEqual(len(self.workflow.workspace.pending_messages()), 1)
        self.assertEqual(
            self.workflow.workspace.event_count(
                "ProfessionalReviewPackageAuthorized"
            ),
            1,
        )

    def test_collaboration_receives_package_not_private_workspace(self):
        self.prepare_through_consent()
        self.authorize_package()
        self.workflow.deliver_packages()
        copied = self.workflow.collaboration.get_package("package-001", 1)
        self.assertIn("scenario_reference", copied)
        self.assertNotIn("facts", copied)
        self.assertNotIn("objectives", copied)
        self.assertEqual(
            self.workflow.collaboration.disclosed_package_count(), 1
        )

    def test_only_assigned_professional_can_respond(self):
        self.prepare_through_consent()
        self.authorize_package()
        self.workflow.deliver_packages()
        with self.assertRaises(AuthorizationError):
            self.workflow.submit_professional_response(
                actor_id="professional-999",
                package_id="package-001",
                package_version=1,
                professional_role="ESOP attorney",
                response_text="Should not be accepted",
                category="feedback",
                idempotency_key="wrong-professional",
            )

    def test_new_destination_version_invalidates_existing_scenario(self):
        self.create_destination()
        self.create_snapshot()
        self.create_scenario()
        self.clock.advance()
        updated = self.create_destination(key="destination-2")
        self.assertEqual(updated["version"], 2)
        self.assertTrue(
            self.workflow.workspace.is_invalidated(
                "ScenarioVersion", "scenario-001", 1
            )
        )

    def test_scenario_with_hard_conflict_cannot_advance(self):
        self.create_destination()
        self.create_snapshot()
        with self.assertRaisesRegex(InvariantError, "hard conflicts"):
            self.workflow.create_scenario(
                actor_id=self.owner,
                owner_id=self.owner,
                scenario_id="scenario-001",
                destination_reference=ref("DesiredOutcome", "dest-001", 1),
                current_state_reference=ref(
                    "BusinessCurrentState", "state-001", 1
                ),
                assumptions=[],
                unknowns=[],
                conflicts=["Owner nonnegotiable conflicts with scenario"],
                professional_questions=[],
                idempotency_key="scenario-conflict",
            )

    def test_revocation_before_delivery_prevents_boundary_crossing(self):
        self.prepare_through_consent()
        self.authorize_package()
        self.clock.advance()
        revoked = self.workflow.revoke_professional_consent(
            actor_id=self.owner,
            owner_id=self.owner,
            consent_id="consent-001",
            idempotency_key="revoke-1",
        )
        self.assertEqual(revoked["version"], 2)
        self.assertEqual(revoked["status"], "revoked")
        self.assertEqual(self.workflow.deliver_packages(), 0)
        self.assertEqual(
            self.workflow.collaboration.disclosed_package_count(), 0
        )
        status = self.workflow.workspace.connection.execute(
            "SELECT status FROM outbox"
        ).fetchone()["status"]
        self.assertEqual(status, "cancelled")

    def test_stale_consent_version_cannot_authorize_a_package(self):
        self.prepare_through_consent()
        self.workflow.revoke_professional_consent(
            actor_id=self.owner,
            owner_id=self.owner,
            consent_id="consent-001",
            idempotency_key="revoke-stale",
        )
        with self.assertRaisesRegex(InvariantError, "current version"):
            self.authorize_package()

    def test_professional_submission_is_idempotent(self):
        self.prepare_through_consent()
        self.authorize_package()
        self.workflow.deliver_packages()
        arguments = dict(
            actor_id=self.professional,
            package_id="package-001",
            package_version=1,
            professional_role="ESOP attorney",
            response_text="Further qualified review is required.",
            category="information_request",
            idempotency_key="professional-same-command",
        )
        first = self.workflow.submit_professional_response(**arguments)
        second = self.workflow.submit_professional_response(**arguments)
        self.assertEqual(first, second)
        response_count = self.workflow.collaboration.connection.execute(
            "SELECT COUNT(1) AS n FROM professional_responses"
        ).fetchone()["n"]
        outbox_count = self.workflow.collaboration.connection.execute(
            "SELECT COUNT(1) AS n FROM outbox"
        ).fetchone()["n"]
        self.assertEqual(response_count, 1)
        self.assertEqual(outbox_count, 1)

    def test_persisted_runtime_objects_conform_to_canonical_definitions(self):
        self.prepare_through_consent()
        result = self.authorize_package()
        self.workflow.deliver_packages()
        response = self.workflow.submit_professional_response(
            actor_id=self.professional,
            package_id="package-001",
            package_version=1,
            professional_role="ESOP attorney",
            response_text="Independent feasibility analysis is still required.",
            category="information_request",
            idempotency_key="schema-response",
        )
        self.workflow.import_professional_responses()

        objects = [
            ("DesiredOutcome", "dest-001", 1),
            ("BusinessCurrentState", "state-001", 1),
            ("ScenarioVersion", "scenario-001", 1),
            ("ScenarioReadiness", "scenario-001", 1),
            ("ConsentRecord", "consent-001", 1),
            ("ProfessionalReviewPackage", "package-001", 1),
            (
                "DisclosureRecord",
                result["disclosure"]["disclosure_id"],
                1,
            ),
            (
                "ProfessionalDetermination",
                response["response_id"],
                1,
            ),
        ]
        for definition, object_id, version in objects:
            with self.subTest(definition=definition):
                value = self.workflow.workspace.get_version(
                    definition, object_id, version
                )
                self.workflow.contracts.validate(definition, value)

    def test_audit_chain_is_hash_linked(self):
        self.create_destination()
        self.clock.advance()
        self.create_snapshot()
        rows = self.workflow.workspace.connection.execute(
            """
            SELECT sequence, previous_hash, event_hash
            FROM audit_events ORDER BY sequence
            """
        ).fetchall()
        self.assertEqual(len(rows), 2)
        self.assertIsNone(rows[0]["previous_hash"])
        self.assertEqual(rows[1]["previous_hash"], rows[0]["event_hash"])


if __name__ == "__main__":
    unittest.main()