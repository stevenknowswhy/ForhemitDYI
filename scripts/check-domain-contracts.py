#!/usr/bin/env python3
"""Validate vertical-slice schemas plus cross-object workflow invariants."""

from __future__ import annotations

import copy
import json
import sys
from datetime import datetime
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "contracts" / "vertical-slice-contracts.schema.json"
EXAMPLE_PATH = ROOT / "contracts" / "examples" / "valid-slice.json"


def load(path: Path) -> Any:
    return json.loads(path.read_text())


def parse_time(value: str) -> datetime:
    return datetime.fromisoformat(value.replace("Z", "+00:00"))


def schema_errors(validator: Draft202012Validator, instance: Any) -> list[str]:
    return [
        f"{'.'.join(str(part) for part in error.path) or '<root>'}: {error.message}"
        for error in sorted(
            validator.iter_errors(instance),
            key=lambda error: [str(part) for part in error.path],
        )
    ]


def ref_key(reference: dict[str, Any]) -> tuple[str, str, int]:
    return (
        reference["object_type"],
        reference["object_id"],
        reference["version"],
    )


def semantic_errors(instance: dict[str, Any]) -> list[str]:
    """Check invariants JSON Schema cannot express across objects and time."""

    errors: list[str] = []
    outcome = instance["desired_outcome"]
    current = instance["business_current_state"]
    scenario = instance["scenario_version"]
    readiness = instance["scenario_readiness"]
    package = instance["professional_review_package"]
    determination = instance["professional_determination"]
    consents = instance["consent_records"]
    disclosure = instance["disclosure_record"]
    ai_disclosure = instance["external_ai_disclosure_record"]
    event = instance["event_envelope"]
    jev = instance["jev_evaluation_reference"]

    objects = {
        ("DesiredOutcome", outcome["destination_id"], outcome["version"]),
        ("BusinessCurrentState", current["current_state_id"], current["version"]),
        ("ScenarioVersion", scenario["scenario_id"], scenario["version"]),
        ("ProfessionalReviewPackage", package["package_id"], package["version"]),
        (
            "ExternalAIDisclosureRecord",
            ai_disclosure["disclosure_id"],
            ai_disclosure["version"],
        ),
        *{
            ("ConsentRecord", consent["consent_id"], consent["version"])
            for consent in consents
        },
    }

    for label, reference in [
        ("scenario.destination_reference", scenario["destination_reference"]),
        ("scenario.current_state_reference", scenario["current_state_reference"]),
        ("readiness.scenario_reference", readiness["scenario_reference"]),
        ("package.destination_reference", package["destination_reference"]),
        ("package.current_state_reference", package["current_state_reference"]),
        ("package.scenario_reference", package["scenario_reference"]),
        ("determination.scenario_reference", determination["scenario_reference"]),
        ("disclosure.package_reference", disclosure["package_reference"]),
        ("event.subject_reference", event["subject_reference"]),
        ("package.consent_reference", package["consent_reference"]),
        ("disclosure.consent_reference", disclosure["consent_reference"]),
        ("jev.consent_reference", jev["consent_reference"]),
        (
            "jev.external_ai_disclosure_reference",
            jev["external_ai_disclosure_reference"],
        ),
        (
            "ai_disclosure.consent_reference",
            ai_disclosure["consent_reference"],
        ),
    ]:
        if ref_key(reference) not in objects:
            errors.append(f"{label} does not resolve to this aggregate")

    if not (
        outcome["business_id"]
        == current["business_id"]
        == package["business_id"]
    ):
        errors.append("business_id differs across destination, current state, and package")

    if outcome["owner_id"] != package["owner_id"]:
        errors.append("owner_id differs between destination and package")

    consent_by_ref = {
        ("ConsentRecord", consent["consent_id"], consent["version"]): consent
        for consent in consents
    }
    professional_consent = consent_by_ref.get(ref_key(package["consent_reference"]))
    ai_consent = consent_by_ref.get(ref_key(jev["consent_reference"]))

    if professional_consent:
        if professional_consent["recipient_type"] != "professional":
            errors.append("package consent is not professional-recipient consent")
        if professional_consent["recipient_id"] != package["recipient_id"]:
            errors.append("package recipient differs from professional consent recipient")
        if professional_consent["purpose"] != package["purpose"]:
            errors.append("package purpose differs from professional consent purpose")
        if professional_consent["status"] != "active":
            errors.append("package consent is not active")
        if ref_key(package["scenario_reference"]) not in {
            ref_key(reference)
            for reference in professional_consent["resource_references"]
        } and (
            "ProfessionalReviewPackage",
            package["package_id"],
            package["version"],
        ) not in {
            ref_key(reference)
            for reference in professional_consent["resource_references"]
        }:
            errors.append("professional consent does not cover the package or scenario")
        if ref_key(disclosure["consent_reference"]) != ref_key(
            package["consent_reference"]
        ):
            errors.append("package and disclosure use different consent versions")
        if disclosure["recipient_id"] != package["recipient_id"]:
            errors.append("disclosure recipient differs from package recipient")
        if disclosure["purpose"] != package["purpose"]:
            errors.append("disclosure purpose differs from package purpose")
        if not (
            parse_time(professional_consent["granted_at"])
            <= parse_time(disclosure["disclosed_at"])
            <= parse_time(professional_consent["expires_at"])
        ):
            errors.append("disclosure falls outside professional consent validity")
        if professional_consent.get("revoked_at") and (
            parse_time(disclosure["disclosed_at"])
            >= parse_time(professional_consent["revoked_at"])
        ):
            errors.append("disclosure occurred after professional consent revocation")

    if ai_consent:
        if ai_consent["recipient_type"] != "external_ai":
            errors.append("Jev consent is not external-AI consent")
        if ai_consent["recipient_id"] != "typesafe-jev":
            errors.append("Jev consent recipient is not typesafe-jev")
        if ai_consent["status"] != "active":
            errors.append("Jev consent is not active")
        if ref_key(readiness["scenario_reference"]) not in {
            ref_key(reference)
            for reference in ai_consent["resource_references"]
        }:
            errors.append("external-AI consent does not cover the evaluated scenario")
        if not (
            parse_time(ai_consent["granted_at"])
            <= parse_time(readiness["evaluated_at"])
            <= parse_time(ai_consent["expires_at"])
        ):
            errors.append("Jev evaluation falls outside external-AI consent validity")
        if ai_consent.get("revoked_at") and (
            parse_time(readiness["evaluated_at"]) >= parse_time(ai_consent["revoked_at"])
        ):
            errors.append("Jev evaluation occurred after external-AI consent revocation")

    for consent in consents:
        if parse_time(consent["expires_at"]) <= parse_time(consent["granted_at"]):
            errors.append(
                f"consent {consent['consent_id']} must expire after it is granted"
            )
        if consent.get("revoked_at") and parse_time(
            consent["revoked_at"]
        ) < parse_time(consent["granted_at"]):
            errors.append(
                f"consent {consent['consent_id']} is revoked before it is granted"
            )

    if ai_disclosure["evaluation_id"] != jev["evaluation_id"]:
        errors.append("external-AI disclosure and evaluation IDs differ")
    if ai_disclosure["recipient_id"] != "typesafe-jev":
        errors.append("external-AI disclosure recipient is not typesafe-jev")
    if ai_consent and ai_disclosure["purpose"] != ai_consent["purpose"]:
        errors.append("external-AI disclosure purpose differs from consent purpose")
    if ref_key(ai_disclosure["consent_reference"]) != ref_key(
        jev["consent_reference"]
    ):
        errors.append("external-AI disclosure and evaluation use different consent")
    if ref_key(jev["external_ai_disclosure_reference"]) != (
        "ExternalAIDisclosureRecord",
        ai_disclosure["disclosure_id"],
        ai_disclosure["version"],
    ):
        errors.append("Jev evaluation references a different external-AI disclosure")
    if ref_key(readiness["scenario_reference"]) not in {
        ref_key(reference) for reference in ai_disclosure["source_references"]
    }:
        errors.append("external-AI disclosure omits the evaluated scenario source")
    for field in (
        "state_hash",
        "request_hash",
        "criteria_hash",
        "minimization_policy_id",
        "minimization_policy_version",
    ):
        if ai_disclosure[field] != jev[field]:
            errors.append(
                f"external-AI disclosure {field} differs from evaluation reference"
            )
    if not (
        parse_time(ai_consent["granted_at"])
        <= parse_time(ai_disclosure["disclosed_at"])
        <= parse_time(readiness["evaluated_at"])
        <= parse_time(ai_consent["expires_at"])
    ):
        errors.append(
            "external-AI disclosure/evaluation chronology falls outside consent"
        )

    included = set(package["included_categories"])
    excluded = set(package["excluded_categories"])
    disclosed = set(disclosure["disclosed_categories"])
    if included & excluded:
        errors.append("package categories cannot be both included and excluded")
    if not disclosed <= included:
        errors.append("disclosed categories exceed package included categories")
    if disclosed & excluded:
        errors.append("disclosed categories include explicitly excluded categories")

    advanced_states = {
        "ready_for_comparison",
        "ready_for_professional_review",
        "under_professional_review",
        "revised",
    }
    if readiness["state"] in advanced_states and not readiness[
        "deterministic_gates_passed"
    ]:
        errors.append("advanced readiness state requires deterministic gates to pass")

    if determination["professional_id"] != determination[
        "professional_identity_reference"
    ]:
        errors.append("determination professional identity reference is inconsistent")
    if determination["professional_id"] != package["recipient_id"]:
        errors.append("determination professional differs from package recipient")
    if determination["professional_role"] != package["professional_role"]:
        errors.append("determination role differs from assigned package role")
    if parse_time(determination["submitted_at"]) < parse_time(
        disclosure["disclosed_at"]
    ):
        errors.append("professional determination predates package disclosure")
    if (
        determination["credential_status"] == "platform_verified"
        and not determination.get("credential_verification_reference")
    ):
        errors.append("verified credential status requires verification evidence")
    if (
        determination["credential_status"] != "platform_verified"
        and determination.get("credential_verification_reference") is not None
    ):
        errors.append("unverified credential status cannot cite verification evidence")

    for label, values, key in [
        ("objectives", outcome["objectives"], "objective_id"),
        ("nonnegotiables", outcome["nonnegotiables"], "nonnegotiable_id"),
        ("facts", current["facts"], "fact_id"),
    ]:
        identifiers = [value[key] for value in values]
        if len(identifiers) != len(set(identifiers)):
            errors.append(f"{label} contain duplicate stable IDs")

    for fact in current["facts"]:
        if fact["value_status"] == "unknown" and fact["value"] is not None:
            errors.append(f"unknown fact {fact['fact_id']} carries a concrete value")

    if readiness.get("jev_evaluation_reference") != jev:
        errors.append("readiness and aggregate Jev references differ")

    if event["actor_type"] == "jev":
        errors.append("Jev cannot be attributed as actor for package authorization")
    if event["event_type"] == "ProfessionalReviewPackageAuthorized":
        if event["actor_type"] != "human":
            errors.append("package authorization must be attributed to a human")
        if event["actor_reference"] != package["owner_id"]:
            errors.append("package authorization actor must be the package owner")
        if ref_key(event["subject_reference"]) != (
            "ProfessionalReviewPackage",
            package["package_id"],
            package["version"],
        ):
            errors.append("package authorization event references the wrong subject")
        if parse_time(event["occurred_at"]) > parse_time(
            disclosure["disclosed_at"]
        ):
            errors.append("package disclosure predates its authorization event")

    return errors


def rejected(
    validator: Draft202012Validator, instance: dict[str, Any]
) -> bool:
    return bool(schema_errors(validator, instance) or semantic_errors(instance))


def main() -> int:
    schema = load(SCHEMA_PATH)
    example = load(EXAMPLE_PATH)
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())

    structural = schema_errors(validator, example)
    semantic = semantic_errors(example) if not structural else []
    if structural or semantic:
        print("Valid synthetic slice failed validation:")
        for error in structural + semantic:
            print(f"- {error}")
        return 1

    negative_cases: list[tuple[str, dict[str, Any]]] = []

    def case(name: str, mutate) -> None:
        value = copy.deepcopy(example)
        mutate(value)
        negative_cases.append((name, value))

    case(
        "non-California jurisdiction",
        lambda x: x["business_current_state"].__setitem__(
            "primary_jurisdiction", "US-NV"
        ),
    )
    case(
        "non-exploratory scenario",
        lambda x: x["scenario_version"].__setitem__("exploratory_only", False),
    )
    case(
        "platform-authored professional determination",
        lambda x: x["professional_determination"].__setitem__(
            "source_authority", "platform_derived"
        ),
    )
    case(
        "unversioned source reference",
        lambda x: x["scenario_version"]["destination_reference"].pop("version"),
    )
    case(
        "unrecognized event actor",
        lambda x: x["event_envelope"].__setitem__("actor_type", "model"),
    )
    case(
        "recipient mismatch",
        lambda x: x["professional_review_package"].__setitem__(
            "recipient_id", "professional-OTHER"
        ),
    )
    case(
        "expired-before-grant consent",
        lambda x: x["consent_records"][0].__setitem__(
            "expires_at", "2026-09-19T20:00:00Z"
        ),
    )
    case(
        "included/excluded category overlap",
        lambda x: x["professional_review_package"][
            "excluded_categories"
        ].append("owner_objectives"),
    )
    case(
        "advanced state with failed hard gates",
        lambda x: x["scenario_readiness"].__setitem__(
            "deterministic_gates_passed", False
        ),
    )
    case(
        "duplicate objective IDs",
        lambda x: x["desired_outcome"]["objectives"][1].__setitem__(
            "objective_id", "objective-001"
        ),
    )
    case(
        "unknown fact with concrete value",
        lambda x: x["business_current_state"]["facts"][0].__setitem__(
            "value_status", "unknown"
        ),
    )
    case(
        "professional and AI consent conflated",
        lambda x: x["jev_evaluation_reference"].__setitem__(
            "consent_reference",
            copy.deepcopy(
                x["professional_review_package"]["consent_reference"]
            ),
        ),
    )
    case(
        "disclosure exceeds package",
        lambda x: x["disclosure_record"]["disclosed_categories"].append(
            "customer_records"
        ),
    )
    case(
        "unverified credential claims evidence",
        lambda x: x["professional_determination"].__setitem__(
            "credential_verification_reference", "verification-001"
        ),
    )
    case(
        "external-AI disclosure hash mismatch",
        lambda x: x["external_ai_disclosure_record"].__setitem__(
            "state_hash", "sha256:" + "d" * 64
        ),
    )
    case(
        "external-AI disclosure after evaluation",
        lambda x: x["external_ai_disclosure_record"].__setitem__(
            "disclosed_at", "2026-09-20T20:00:01Z"
        ),
    )
    case(
        "external-AI disclosure evaluation mismatch",
        lambda x: x["external_ai_disclosure_record"].__setitem__(
            "evaluation_id", "jev-eval-OTHER"
        ),
    )
    case(
        "professional determination by different recipient",
        lambda x: x["professional_determination"].__setitem__(
            "professional_id", "professional-OTHER"
        ),
    )

    failures = [
        name for name, instance in negative_cases if not rejected(validator, instance)
    ]
    if failures:
        print("Boundary cases were incorrectly accepted:")
        for failure in failures:
            print(f"- {failure}")
        return 1

    print(
        "Domain contract validation passed "
        f"(1 valid conformance aggregate, {len(negative_cases)} rejected boundary cases; "
        "structural + referential + temporal invariants)"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())