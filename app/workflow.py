"""Deterministic Release 0 application service.

This module deliberately has no Jev adapter or model dependency. All protected
transitions are controlled by explicit actor, consent, contract, and domain
invariants.
"""

from __future__ import annotations

import copy
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable

from .contracts import ContractValidator
from .errors import AuthorizationError, ConflictError, InvariantError
from .persistence import CollaborationStore, WorkspaceStore


REQUIRED_FACT_FIELDS = {
    "business_name",
    "entity_type",
    "revenue_range",
    "operating_cash_flow_range",
    "employee_count",
    "ownership_summary",
    "management_summary",
    "debt_range",
}


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


def new_id(prefix: str) -> str:
    return f"{prefix}-{uuid.uuid4()}"


def reference(object_type: str, object_id: str, version: int) -> dict[str, Any]:
    return {
        "object_type": object_type,
        "object_id": object_id,
        "version": version,
    }


def parse_time(value: str) -> datetime:
    return datetime.fromisoformat(value.replace("Z", "+00:00"))


class Release0Workflow:
    """Coordinates the private workspace and professional collaboration stores."""

    def __init__(
        self,
        workspace_path: str | Path,
        collaboration_path: str | Path,
        clock: Callable[[], str] = utc_now,
    ):
        self.workspace = WorkspaceStore(workspace_path)
        self.collaboration = CollaborationStore(collaboration_path)
        self.clock = clock
        self.contracts = ContractValidator()

    def close(self) -> None:
        self.workspace.close()
        self.collaboration.close()

    def _event(
        self,
        *,
        event_type: str,
        actor_type: str,
        actor_reference: str,
        source_module: str,
        subject_reference: dict[str, Any],
        idempotency_key: str,
        payload: dict[str, Any] | None = None,
        occurred_at: str | None = None,
    ) -> dict[str, Any]:
        event = {
            "schema_version": "1.1.0",
            "event_id": new_id("event"),
            "event_type": event_type,
            "occurred_at": occurred_at or self.clock(),
            "actor_type": actor_type,
            "actor_reference": actor_reference,
            "source_module": source_module,
            "subject_reference": subject_reference,
            "correlation_id": new_id("correlation"),
            "causation_id": new_id("command"),
            "idempotency_key": idempotency_key,
            "payload": payload or {},
        }
        self.contracts.validate("EventEnvelope", event)
        return event

    @staticmethod
    def _require_owner(actor_id: str, owner_id: str) -> None:
        if actor_id != owner_id:
            raise AuthorizationError("only the owner may perform this command")

    def confirm_destination(
        self,
        *,
        actor_id: str,
        owner_id: str,
        business_id: str,
        destination_id: str,
        transition_horizon: str,
        employee_ownership_intent: str,
        owner_involvement: str,
        objectives: list[dict[str, Any]],
        nonnegotiables: list[dict[str, Any]],
        preferences: list[str] | None = None,
        unknowns: list[str] | None = None,
        contradictions: list[str] | None = None,
        idempotency_key: str,
    ) -> dict[str, Any]:
        self._require_owner(actor_id, owner_id)
        if contradictions:
            raise InvariantError(
                "unresolved destination contradictions block confirmation"
            )
        if not objectives:
            raise InvariantError("at least one owner-authored objective is required")
        if transition_horizon not in {
            "within_1_year",
            "1_to_3_years",
            "3_to_5_years",
            "flexible",
            "unknown",
        }:
            raise InvariantError("invalid transition horizon")
        if employee_ownership_intent not in {
            "essential",
            "very_important",
            "explore",
            "unknown",
        }:
            raise InvariantError("invalid employee-ownership intent")
        if owner_involvement not in {
            "exit",
            "transition_period",
            "ongoing",
            "unknown",
        }:
            raise InvariantError("invalid owner-involvement value")

        now = self.clock()
        scope = "destination.confirm"
        with self.workspace.transaction() as conn:
            cached = self.workspace.get_idempotent(conn, scope, idempotency_key)
            if cached:
                return cached
            version = self.workspace.next_version(
                conn, "DesiredOutcome", destination_id
            )
            normalized_nonnegotiables = []
            for item in nonnegotiables:
                if item.get("source_authority") != "owner_stated":
                    raise InvariantError(
                        "every nonnegotiable must be explicitly owner-authored"
                    )
                if item.get("status", "active") != "active":
                    raise InvariantError(
                        "newly confirmed nonnegotiables must be active"
                    )
                normalized_nonnegotiables.append(
                    {
                        "nonnegotiable_id": item["nonnegotiable_id"],
                        "statement": item["statement"],
                        "status": "active",
                        "source_authority": "owner_stated",
                        "owner_confirmed_at": now,
                    }
                )
            result = {
                "schema_version": "1.1.0",
                "destination_id": destination_id,
                "version": version,
                "owner_id": owner_id,
                "business_id": business_id,
                "transition_horizon": transition_horizon,
                "employee_ownership_intent": employee_ownership_intent,
                "owner_involvement": owner_involvement,
                "objectives": copy.deepcopy(objectives),
                "nonnegotiables": normalized_nonnegotiables,
                "preferences": preferences or [],
                "unknowns": unknowns or [],
                "source_authority": "owner_stated",
                "sensitivity": "confidential",
                "confirmed_at": now,
            }
            self.contracts.validate("DesiredOutcome", result)
            self.workspace.store_version(
                conn,
                "DesiredOutcome",
                destination_id,
                version,
                result,
                now,
            )
            invalidated = self.workspace.invalidate_older_dependents(
                conn, "DesiredOutcome", destination_id, version, now
            )
            event = self._event(
                event_type="DestinationConfirmed",
                actor_type="human",
                actor_reference=owner_id,
                source_module="destination",
                subject_reference=reference(
                    "DesiredOutcome", destination_id, version
                ),
                idempotency_key=idempotency_key,
                payload={"invalidated_dependents": invalidated},
                occurred_at=now,
            )
            self.workspace.append_event(conn, event)
            self.workspace.save_idempotent(
                conn, scope, idempotency_key, result, now
            )
            return result

    def confirm_business_snapshot(
        self,
        *,
        actor_id: str,
        owner_id: str,
        business_id: str,
        current_state_id: str,
        primary_jurisdiction: str,
        facts: list[dict[str, Any]],
        idempotency_key: str,
    ) -> dict[str, Any]:
        self._require_owner(actor_id, owner_id)
        if primary_jurisdiction != "US-CA":
            raise InvariantError(
                "Release 0 is limited to owner-stated California jurisdiction"
            )
        fields = {fact.get("field") for fact in facts}
        missing = sorted(REQUIRED_FACT_FIELDS - fields)
        if missing:
            raise InvariantError(
                f"required snapshot fields must be present or explicitly unknown: {missing}"
            )
        identifiers = [fact.get("fact_id") for fact in facts]
        if len(identifiers) != len(set(identifiers)):
            raise InvariantError("fact stable IDs must be unique")
        for fact in facts:
            status = fact.get("value_status")
            if status not in {"known", "range", "unknown", "conflicted", "stale"}:
                raise InvariantError(f"invalid fact status for {fact.get('field')}")
            if status == "unknown" and fact.get("value") is not None:
                raise InvariantError(
                    f"unknown fact {fact.get('fact_id')} cannot carry a value"
                )
            if status != "unknown" and fact.get("value") is None:
                raise InvariantError(
                    f"{status} fact {fact.get('fact_id')} must carry its stated value"
                )
            if not fact.get("provenance"):
                raise InvariantError(
                    f"material fact {fact.get('fact_id')} requires provenance"
                )

        now = self.clock()
        scope = "business_snapshot.confirm"
        with self.workspace.transaction() as conn:
            cached = self.workspace.get_idempotent(conn, scope, idempotency_key)
            if cached:
                return cached
            version = self.workspace.next_version(
                conn, "BusinessCurrentState", current_state_id
            )
            result = {
                "schema_version": "1.1.0",
                "current_state_id": current_state_id,
                "version": version,
                "business_id": business_id,
                "primary_jurisdiction": primary_jurisdiction,
                "facts": copy.deepcopy(facts),
                "source_authority": "owner_stated",
                "sensitivity": "restricted",
                "confirmed_at": now,
            }
            self.contracts.validate("BusinessCurrentState", result)
            self.workspace.store_version(
                conn,
                "BusinessCurrentState",
                current_state_id,
                version,
                result,
                now,
            )
            invalidated = self.workspace.invalidate_older_dependents(
                conn, "BusinessCurrentState", current_state_id, version, now
            )
            event = self._event(
                event_type="BusinessCurrentStateConfirmed",
                actor_type="human",
                actor_reference=owner_id,
                source_module="business_reality",
                subject_reference=reference(
                    "BusinessCurrentState", current_state_id, version
                ),
                idempotency_key=idempotency_key,
                payload={"invalidated_dependents": invalidated},
                occurred_at=now,
            )
            self.workspace.append_event(conn, event)
            self.workspace.save_idempotent(
                conn, scope, idempotency_key, result, now
            )
            return result

    def create_scenario(
        self,
        *,
        actor_id: str,
        owner_id: str,
        scenario_id: str,
        destination_reference: dict[str, Any],
        current_state_reference: dict[str, Any],
        assumptions: list[str],
        unknowns: list[str],
        conflicts: list[str],
        professional_questions: list[str],
        idempotency_key: str,
    ) -> dict[str, Any]:
        self._require_owner(actor_id, owner_id)
        if destination_reference["object_type"] != "DesiredOutcome":
            raise InvariantError("scenario requires a DesiredOutcome reference")
        if current_state_reference["object_type"] != "BusinessCurrentState":
            raise InvariantError(
                "scenario requires a BusinessCurrentState reference"
            )
        destination = self.workspace.get_version(
            "DesiredOutcome",
            destination_reference["object_id"],
            destination_reference["version"],
        )
        current = self.workspace.get_version(
            "BusinessCurrentState",
            current_state_reference["object_id"],
            current_state_reference["version"],
        )
        if destination["owner_id"] != owner_id:
            raise AuthorizationError("destination belongs to another owner")
        if destination["business_id"] != current["business_id"]:
            raise InvariantError("destination and snapshot business IDs differ")
        if self.workspace.is_invalidated(
            "DesiredOutcome",
            destination["destination_id"],
            destination["version"],
        ) or self.workspace.is_invalidated(
            "BusinessCurrentState",
            current["current_state_id"],
            current["version"],
        ):
            raise InvariantError("scenario sources are invalidated")
        if conflicts:
            raise InvariantError(
                "unresolved hard conflicts prevent professional-review readiness"
            )

        now = self.clock()
        scope = "scenario.create"
        with self.workspace.transaction() as conn:
            cached = self.workspace.get_idempotent(conn, scope, idempotency_key)
            if cached:
                return cached
            version = self.workspace.next_version(
                conn, "ScenarioVersion", scenario_id
            )
            scenario = {
                "schema_version": "1.1.0",
                "scenario_id": scenario_id,
                "version": version,
                "scenario_type": "esop_exploration",
                "destination_reference": copy.deepcopy(destination_reference),
                "current_state_reference": copy.deepcopy(current_state_reference),
                "assumptions": assumptions,
                "unknowns": unknowns,
                "conflicts": conflicts,
                "professional_questions": professional_questions,
                "exploratory_only": True,
                "source_authority": "platform_derived",
                "sensitivity": "restricted",
                "created_at": now,
            }
            readiness = {
                "scenario_reference": reference(
                    "ScenarioVersion", scenario_id, version
                ),
                "state": "ready_for_professional_review",
                "deterministic_gates_passed": True,
                "professionally_validated": False,
                "source_authority": "platform_derived",
                "evaluated_at": now,
            }
            dependencies = [
                destination_reference,
                current_state_reference,
            ]
            self.contracts.validate("ScenarioVersion", scenario)
            self.contracts.validate("ScenarioReadiness", readiness)
            self.workspace.store_version(
                conn,
                "ScenarioVersion",
                scenario_id,
                version,
                scenario,
                now,
                dependencies=dependencies,
            )
            self.workspace.store_version(
                conn,
                "ScenarioReadiness",
                scenario_id,
                version,
                readiness,
                now,
                dependencies=[
                    reference("ScenarioVersion", scenario_id, version)
                ],
            )
            event = self._event(
                event_type="ScenarioVersionCreated",
                actor_type="human",
                actor_reference=owner_id,
                source_module="scenario",
                subject_reference=reference(
                    "ScenarioVersion", scenario_id, version
                ),
                idempotency_key=idempotency_key,
                payload={"deterministic_readiness": readiness},
                occurred_at=now,
            )
            self.workspace.append_event(conn, event)
            result = {"scenario": scenario, "readiness": readiness}
            self.workspace.save_idempotent(
                conn, scope, idempotency_key, result, now
            )
            return result

    def grant_professional_consent(
        self,
        *,
        actor_id: str,
        owner_id: str,
        consent_id: str,
        recipient_id: str,
        purpose: str,
        resource_references: list[dict[str, Any]],
        permissions: list[str],
        expires_at: str,
        idempotency_key: str,
    ) -> dict[str, Any]:
        self._require_owner(actor_id, owner_id)
        if not recipient_id:
            raise InvariantError("a named professional recipient is required")
        if purpose != "california_esop_exploration_review":
            raise InvariantError("unsupported professional-sharing purpose")
        if not set(permissions) & {"full_view", "download"}:
            raise InvariantError(
                "professional consent must permit full_view or download"
            )
        now = self.clock()
        if parse_time(expires_at) <= parse_time(now):
            raise InvariantError("consent expiry must be in the future")
        for resource in resource_references:
            self.workspace.get_version(
                resource["object_type"],
                resource["object_id"],
                resource["version"],
            )

        scope = "professional_consent.grant"
        with self.workspace.transaction() as conn:
            cached = self.workspace.get_idempotent(conn, scope, idempotency_key)
            if cached:
                return cached
            version = self.workspace.next_version(
                conn, "ConsentRecord", consent_id
            )
            result = {
                "schema_version": "1.1.0",
                "consent_id": consent_id,
                "version": version,
                "owner_id": owner_id,
                "recipient_id": recipient_id,
                "recipient_type": "professional",
                "purpose": purpose,
                "resource_references": copy.deepcopy(resource_references),
                "permissions": permissions,
                "status": "active",
                "granted_at": now,
                "expires_at": expires_at,
                "revoked_at": None,
                "source_authority": "owner_stated",
                "sensitivity": "confidential",
            }
            self.contracts.validate("ConsentRecord", result)
            self.workspace.store_version(
                conn, "ConsentRecord", consent_id, version, result, now
            )
            event = self._event(
                event_type="ProfessionalSharingConsentGranted",
                actor_type="human",
                actor_reference=owner_id,
                source_module="consent_access",
                subject_reference=reference(
                    "ConsentRecord", consent_id, version
                ),
                idempotency_key=idempotency_key,
                occurred_at=now,
            )
            self.workspace.append_event(conn, event)
            self.workspace.save_idempotent(
                conn, scope, idempotency_key, result, now
            )
            return result

    def revoke_professional_consent(
        self,
        *,
        actor_id: str,
        owner_id: str,
        consent_id: str,
        idempotency_key: str,
    ) -> dict[str, Any]:
        self._require_owner(actor_id, owner_id)
        current = self.workspace.get_version("ConsentRecord", consent_id)
        if current["owner_id"] != owner_id:
            raise AuthorizationError("consent belongs to another owner")
        if current["recipient_type"] != "professional":
            raise InvariantError("only professional-sharing consent is handled here")

        now = self.clock()
        scope = "professional_consent.revoke"
        with self.workspace.transaction() as conn:
            cached = self.workspace.get_idempotent(conn, scope, idempotency_key)
            if cached:
                return cached
            live = self.workspace.get_version(
                "ConsentRecord", consent_id, conn=conn
            )
            if live["status"] == "revoked":
                raise ConflictError("professional consent is already revoked")
            version = self.workspace.next_version(
                conn, "ConsentRecord", consent_id
            )
            revoked = {
                **live,
                "version": version,
                "status": "revoked",
                "revoked_at": now,
            }
            self.contracts.validate("ConsentRecord", revoked)
            self.workspace.store_version(
                conn, "ConsentRecord", consent_id, version, revoked, now
            )
            invalidated = self.workspace.invalidate_older_dependents(
                conn, "ConsentRecord", consent_id, version, now
            )
            event = self._event(
                event_type="ProfessionalSharingConsentRevoked",
                actor_type="human",
                actor_reference=owner_id,
                source_module="consent_access",
                subject_reference=reference(
                    "ConsentRecord", consent_id, version
                ),
                idempotency_key=idempotency_key,
                payload={"invalidated_dependents": invalidated},
                occurred_at=now,
            )
            self.workspace.append_event(conn, event)
            self.workspace.save_idempotent(
                conn, scope, idempotency_key, revoked, now
            )
            return revoked

    def authorize_review_package(
        self,
        *,
        actor_id: str,
        owner_id: str,
        business_id: str,
        package_id: str,
        recipient_id: str,
        professional_role: str,
        purpose: str,
        destination_reference: dict[str, Any],
        current_state_reference: dict[str, Any],
        scenario_reference: dict[str, Any],
        consent_reference: dict[str, Any],
        included_categories: list[str],
        excluded_categories: list[str],
        idempotency_key: str,
    ) -> dict[str, Any]:
        self._require_owner(actor_id, owner_id)
        if set(included_categories) & set(excluded_categories):
            raise InvariantError(
                "package categories cannot be included and excluded"
            )
        destination = self.workspace.get_version(
            destination_reference["object_type"],
            destination_reference["object_id"],
            destination_reference["version"],
        )
        current = self.workspace.get_version(
            current_state_reference["object_type"],
            current_state_reference["object_id"],
            current_state_reference["version"],
        )
        scenario = self.workspace.get_version(
            scenario_reference["object_type"],
            scenario_reference["object_id"],
            scenario_reference["version"],
        )
        consent = self.workspace.get_version(
            consent_reference["object_type"],
            consent_reference["object_id"],
            consent_reference["version"],
        )
        if destination["owner_id"] != owner_id:
            raise AuthorizationError("destination belongs to another owner")
        if not (
            destination["business_id"]
            == current["business_id"]
            == business_id
        ):
            raise InvariantError("business IDs differ across package sources")
        if scenario["destination_reference"] != destination_reference:
            raise InvariantError("scenario uses another destination version")
        if scenario["current_state_reference"] != current_state_reference:
            raise InvariantError("scenario uses another snapshot version")
        if self.workspace.is_invalidated(
            "ScenarioVersion", scenario["scenario_id"], scenario["version"]
        ):
            raise InvariantError("scenario has been invalidated")
        readiness = self.workspace.get_version(
            "ScenarioReadiness", scenario["scenario_id"], scenario["version"]
        )
        if not readiness["deterministic_gates_passed"]:
            raise InvariantError("deterministic scenario gates failed")
        if consent["recipient_type"] != "professional":
            raise InvariantError("package requires professional consent")
        if consent["owner_id"] != owner_id:
            raise AuthorizationError("consent belongs to another owner")
        if consent["recipient_id"] != recipient_id:
            raise InvariantError("package recipient differs from consent recipient")
        if consent["purpose"] != purpose:
            raise InvariantError("package purpose differs from consent purpose")
        current_consent = self.workspace.get_version(
            "ConsentRecord", consent["consent_id"]
        )
        if current_consent["version"] != consent["version"]:
            raise InvariantError("consent reference is not the current version")
        if consent["status"] != "active":
            raise InvariantError("professional consent is not active")
        now = self.clock()
        if not (
            parse_time(consent["granted_at"])
            <= parse_time(now)
            <= parse_time(consent["expires_at"])
        ):
            raise InvariantError("professional consent is outside its validity")
        if scenario_reference not in consent["resource_references"]:
            raise InvariantError("consent does not cover this scenario version")
        if not set(consent["permissions"]) & {"full_view", "download"}:
            raise InvariantError("consent does not permit package disclosure")

        scope = "review_package.authorize"
        with self.workspace.transaction() as conn:
            cached = self.workspace.get_idempotent(conn, scope, idempotency_key)
            if cached:
                return cached
            version = self.workspace.next_version(
                conn, "ProfessionalReviewPackage", package_id
            )
            package = {
                "schema_version": "1.1.0",
                "package_id": package_id,
                "version": version,
                "owner_id": owner_id,
                "business_id": business_id,
                "recipient_id": recipient_id,
                "professional_role": professional_role,
                "purpose": purpose,
                "destination_reference": copy.deepcopy(destination_reference),
                "current_state_reference": copy.deepcopy(
                    current_state_reference
                ),
                "scenario_reference": copy.deepcopy(scenario_reference),
                "included_categories": included_categories,
                "excluded_categories": excluded_categories,
                "consent_reference": copy.deepcopy(consent_reference),
                "sharing_status": "authorized",
                "source_authority": "system_recorded",
                "sensitivity": "restricted",
                "created_at": now,
            }
            disclosure_id = new_id("disclosure")
            disclosure = {
                "schema_version": "1.1.0",
                "disclosure_id": disclosure_id,
                "consent_reference": copy.deepcopy(consent_reference),
                "package_reference": reference(
                    "ProfessionalReviewPackage", package_id, version
                ),
                "recipient_id": recipient_id,
                "purpose": purpose,
                "disclosed_categories": included_categories,
                "source_authority": "system_recorded",
                "disclosed_at": now,
            }
            dependencies = [
                destination_reference,
                current_state_reference,
                scenario_reference,
                consent_reference,
            ]
            self.contracts.validate("ProfessionalReviewPackage", package)
            self.contracts.validate("DisclosureRecord", disclosure)
            self.workspace.store_version(
                conn,
                "ProfessionalReviewPackage",
                package_id,
                version,
                package,
                now,
                dependencies=dependencies,
            )
            self.workspace.store_version(
                conn,
                "DisclosureRecord",
                disclosure_id,
                1,
                disclosure,
                now,
                dependencies=[
                    reference(
                        "ProfessionalReviewPackage", package_id, version
                    ),
                    consent_reference,
                ],
            )
            event = self._event(
                event_type="ProfessionalReviewPackageAuthorized",
                actor_type="human",
                actor_reference=owner_id,
                source_module="review_package",
                subject_reference=reference(
                    "ProfessionalReviewPackage", package_id, version
                ),
                idempotency_key=idempotency_key,
                payload={
                    "recipient_id": recipient_id,
                    "purpose": purpose,
                    "disclosure_id": disclosure_id,
                },
                occurred_at=now,
            )
            self.workspace.append_event(conn, event)
            message_id = new_id("message")
            self.workspace.enqueue(
                conn,
                message_id,
                "review_package.authorized",
                {
                    "package": package,
                    "disclosure": disclosure,
                    # Deliberately excludes the complete private workspace.
                },
                now,
            )
            result = {
                "package": package,
                "disclosure": disclosure,
                "message_id": message_id,
            }
            self.workspace.save_idempotent(
                conn, scope, idempotency_key, result, now
            )
            return result

    def deliver_packages(self) -> int:
        """Deliver only while the exact authorization remains current and active."""
        delivered = 0
        for message in self.workspace.pending_messages():
            if message["topic"] != "review_package.authorized":
                continue
            package = message["payload"]["package"]
            consent_ref = package["consent_reference"]
            referenced = self.workspace.get_version(
                "ConsentRecord",
                consent_ref["object_id"],
                consent_ref["version"],
            )
            current = self.workspace.get_version(
                "ConsentRecord", consent_ref["object_id"]
            )
            now = self.clock()
            authorized = (
                current["version"] == referenced["version"]
                and current["status"] == "active"
                and current["recipient_id"] == package["recipient_id"]
                and current["purpose"] == package["purpose"]
                and parse_time(current["granted_at"])
                <= parse_time(now)
                <= parse_time(current["expires_at"])
            )
            if not authorized:
                self.workspace.cancel_message(message["message_id"])
                continue
            self.collaboration.receive_package(
                message["message_id"], message["payload"], now
            )
            self.workspace.mark_delivered(message["message_id"], now)
            delivered += 1
        return delivered

    def submit_professional_response(
        self,
        *,
        actor_id: str,
        package_id: str,
        package_version: int,
        professional_role: str,
        response_text: str,
        category: str,
        idempotency_key: str,
    ) -> dict[str, Any]:
        package = self.collaboration.get_package(package_id, package_version)
        if actor_id != package["recipient_id"]:
            raise AuthorizationError(
                "only the assigned professional may submit this response"
            )
        if professional_role != package["professional_role"]:
            raise InvariantError(
                "professional response role differs from package assignment"
            )
        if not response_text.strip():
            raise InvariantError("professional response text is required")
        now = self.clock()
        response = {
            "schema_version": "1.1.0",
            "response_id": new_id("response"),
            "review_id": new_id("review"),
            "package_id": package_id,
            "package_version": package_version,
            "professional_id": actor_id,
            "professional_role": professional_role,
            "scenario_reference": package["scenario_reference"],
            "response_text": response_text,
            "category": category,
            "credential_status": "unverified",
            "submitted_at": now,
            "owner_acknowledged_at": None,
            "idempotency_key": idempotency_key,
        }
        return self.collaboration.submit_response(
            response, new_id("message"), now
        )

    def import_professional_responses(self) -> int:
        """Copy exact attributed responses into the private owner workspace."""
        imported = 0
        for message in self.collaboration.pending_messages():
            if message["topic"] != "professional.response.submitted":
                continue
            response = message["payload"]
            now = self.clock()
            with self.workspace.transaction() as conn:
                existing = conn.execute(
                    """
                    SELECT 1 FROM aggregate_versions
                    WHERE object_type = 'ProfessionalDetermination'
                      AND object_id = ? AND version = 1
                    """,
                    (response["response_id"],),
                ).fetchone()
                if not existing:
                    determination = {
                        "schema_version": "1.1.0",
                        "determination_id": response["response_id"],
                        "review_id": response["review_id"],
                        "professional_id": response["professional_id"],
                        "professional_role": response["professional_role"],
                        "scenario_reference": response["scenario_reference"],
                        "determination_text": response["response_text"],
                        "category": response["category"],
                        "supporting_references": [],
                        "owner_acknowledged_at": None,
                        "source_authority": "professional_attributed",
                        "sensitivity": "restricted",
                        "submitted_at": response["submitted_at"],
                        "professional_identity_reference": response[
                            "professional_id"
                        ],
                        "credential_status": response["credential_status"],
                    }
                    self.contracts.validate(
                        "ProfessionalDetermination", determination
                    )
                    self.workspace.store_version(
                        conn,
                        "ProfessionalDetermination",
                        response["response_id"],
                        1,
                        determination,
                        now,
                        dependencies=[response["scenario_reference"]],
                    )
                    event = self._event(
                        event_type="ProfessionalResponseReceived",
                        actor_type="professional",
                        actor_reference=response["professional_id"],
                        source_module="professional_review",
                        subject_reference=reference(
                            "ProfessionalDetermination",
                            response["response_id"],
                            1,
                        ),
                        idempotency_key=response["idempotency_key"],
                        occurred_at=response["submitted_at"],
                    )
                    self.workspace.append_event(conn, event)
                    imported += 1
            self.collaboration.mark_delivered(message["message_id"], now)
        return imported

    def acknowledge_professional_response(
        self,
        *,
        actor_id: str,
        owner_id: str,
        determination_id: str,
        statement: str,
        idempotency_key: str,
    ) -> dict[str, Any]:
        self._require_owner(actor_id, owner_id)
        determination = self.workspace.get_version(
            "ProfessionalDetermination", determination_id, 1
        )
        now = self.clock()
        scope = "professional_response.acknowledge"
        with self.workspace.transaction() as conn:
            cached = self.workspace.get_idempotent(conn, scope, idempotency_key)
            if cached:
                return cached
            acknowledgment = {
                "acknowledgment_id": new_id("ack"),
                "owner_id": owner_id,
                "determination_id": determination_id,
                "acknowledged_at": now,
                "statement": statement,
                "agreement_inferred": False,
                "automatic_handoff_created": False,
            }
            conn.execute(
                """
                INSERT INTO acknowledgments (
                    acknowledgment_id, owner_id, determination_id,
                    acknowledged_at, statement
                ) VALUES (?, ?, ?, ?, ?)
                """,
                (
                    acknowledgment["acknowledgment_id"],
                    owner_id,
                    determination_id,
                    now,
                    statement,
                ),
            )
            event = self._event(
                event_type="ProfessionalResponseAcknowledged",
                actor_type="human",
                actor_reference=owner_id,
                source_module="professional_review",
                subject_reference=reference(
                    "ProfessionalDetermination", determination_id, 1
                ),
                idempotency_key=idempotency_key,
                payload={
                    "agreement_inferred": False,
                    "automatic_handoff_created": False,
                },
                occurred_at=now,
            )
            self.workspace.append_event(conn, event)
            self.workspace.save_idempotent(
                conn, scope, idempotency_key, acknowledgment, now
            )
            return acknowledgment