#!/usr/bin/env python3
"""Validate Jev schemas, registries, examples, and safety invariants."""

from __future__ import annotations

import copy
import json
import math
import re
import sys
from pathlib import Path
from typing import Any

try:
    from jsonschema import Draft202012Validator, FormatChecker
except ImportError:
    print(
        "Jev contract validation requires jsonschema; install requirements-dev.txt",
        file=sys.stderr,
    )
    raise SystemExit(2)

ROOT = Path(__file__).resolve().parents[1]
JEV = ROOT / "jev"
REGISTRY = JEV / "pilot-questions.json"
REGISTRY_SCHEMA = JEV / "pilot-question-registry.schema.json"
POLICIES = JEV / "threshold-policies.json"
POLICY_SCHEMA = JEV / "threshold-policy.schema.json"
RECORD_SCHEMA = JEV / "evaluation-record.schema.json"
VALID_RECORD = JEV / "examples" / "valid-evaluation.json"

ALLOWED_AUTHORITIES = {
    "signal_only",
    "human_confirmed",
    "reversible_route",
    "professional_only",
    "no_role",
}
PROHIBITED_OWNERS_FOR_AUTOMATION = {
    "Identity & Access",
    "Consent & Access",
    "Policy",
    "Financial Modeling",
    "Valuation",
    "Professional Determination",
}
POLICY_MAX_AUTHORITY = {
    "shadow_only": "signal_only",
    "display_only": "signal_only",
    "human_confirmation": "human_confirmed",
    "reversible_route": "reversible_route",
}
AUTHORITY_RANK = {
    "no_role": 0,
    "signal_only": 1,
    "human_confirmed": 2,
    "reversible_route": 3,
    "professional_only": 3,
}
EFFECT_RANK = {
    "no_effect": 0,
    "displayed": 1,
    "human_confirmed": 2,
    "reversible_route": 3,
    "professional_only": 3,
}
SEMVER = re.compile(r"^[0-9]+\.[0-9]+\.[0-9]+$")


def load(path: Path) -> Any:
    return json.loads(path.read_text())


def fail(errors: list[str], message: str) -> None:
    errors.append(message)


def validate_with_schema(
    errors: list[str],
    instance_path: Path,
    schema_path: Path,
) -> tuple[dict[str, Any], dict[str, Any]]:
    instance = load(instance_path)
    schema = load(schema_path)
    try:
        Draft202012Validator.check_schema(schema)
    except Exception as exc:
        fail(errors, f"{schema_path.name}: invalid JSON Schema: {exc}")
        return instance, schema

    validator = Draft202012Validator(
        schema,
        format_checker=FormatChecker(),
    )
    for error in sorted(
        validator.iter_errors(instance),
        key=lambda item: [str(part) for part in item.path],
    ):
        location = ".".join(str(part) for part in error.path) or "<root>"
        fail(errors, f"{instance_path.name}:{location}: {error.message}")
    return instance, schema


def validate_registry(
    errors: list[str],
) -> tuple[dict[str, dict[str, Any]], dict[str, dict[str, Any]]]:
    data, _ = validate_with_schema(errors, REGISTRY, REGISTRY_SCHEMA)
    policies_data, _ = validate_with_schema(errors, POLICIES, POLICY_SCHEMA)
    questions = data.get("questions", [])
    policies = policies_data.get("policies", {})

    if data.get("threshold_policy_registry") != POLICIES.name:
        fail(errors, "question registry must reference threshold-policies.json")

    seen_ids: set[str] = set()
    questions_by_id: dict[str, dict[str, Any]] = {}

    for index, question in enumerate(questions):
        label = f"questions[{index}]"
        qid = question.get("id", "")
        if qid in seen_ids:
            fail(errors, f"{label}: duplicate active question id {qid!r}")
        seen_ids.add(qid)
        questions_by_id[qid] = question

        authority = question.get("maximum_authority")
        if authority not in ALLOWED_AUTHORITIES:
            fail(errors, f"{label}: invalid maximum_authority {authority!r}")

        fallback = question.get("fallback")
        if not isinstance(fallback, str) or not fallback:
            fail(errors, f"{label}: explicit question fallback is required")

        policy_id = question.get("threshold_policy")
        policy = policies.get(policy_id)
        if policy is None:
            fail(errors, f"{label}: unknown threshold_policy {policy_id!r}")
            continue

        action = policy.get("action")
        policy_ceiling = POLICY_MAX_AUTHORITY.get(action)
        if policy_ceiling is None:
            fail(errors, f"{label}: unsupported policy action {action!r}")
        elif AUTHORITY_RANK[policy_ceiling] > AUTHORITY_RANK.get(authority, -1):
            fail(
                errors,
                f"{label}: threshold policy {policy_id!r} exceeds "
                f"maximum authority {authority!r}",
            )

        if (
            question.get("owner_engine") in PROHIBITED_OWNERS_FOR_AUTOMATION
            and action not in {"shadow_only", "display_only"}
        ):
            fail(errors, f"{label}: prohibited automated owner/policy combination")

        if policy.get("production_approved") is not False:
            fail(errors, f"{label}: design threshold policy must remain unapproved")

        examples = question.get("examples", [])
        kinds = {example.get("kind") for example in examples}
        if not {"example", "counterexample"} <= kinds:
            fail(errors, f"{label}: requires an example and a counterexample")

        input_contract = question.get("input_contract", {})
        allowed = set(input_contract.get("allowed_fields", []))
        forbidden = set(input_contract.get("forbidden_fields", []))
        if allowed & forbidden:
            fail(errors, f"{label}: allowed and forbidden input fields overlap")
        if input_contract.get("raw_documents_allowed") is not False:
            fail(errors, f"{label}: raw documents must remain prohibited")

        qtype = question.get("type")
        if qtype == "choice":
            criteria = question.get("criteria", {})
            required_fallback = question.get("required_fallback_option")
            if required_fallback and required_fallback not in criteria:
                if question.get("criteria_source") is None:
                    fail(
                        errors,
                        f"{label}: required fallback option is absent from choices",
                    )
            if action in {"human_confirmation", "reversible_route"}:
                for key in ("top_probability_min", "margin_min"):
                    if key not in policy:
                        fail(errors, f"{label}: choice action requires {key}")
        elif qtype == "noul" and action not in {"display_only", "shadow_only"}:
            for key in ("low_max", "high_min"):
                if key not in policy:
                    fail(errors, f"{label}: noul action requires {key}")
        elif qtype == "score" and action not in {"display_only", "shadow_only"}:
            fail(errors, f"{label}: score questions must remain display/shadow only")

    for policy_id, policy in policies.items():
        if not SEMVER.match(str(policy.get("version", ""))):
            fail(errors, f"threshold policy {policy_id}: invalid semantic version")
        if policy.get("production_approved") is not False:
            fail(errors, f"threshold policy {policy_id}: must remain unapproved")
        if (
            "low_max" in policy
            and "high_min" in policy
            and policy["low_max"] >= policy["high_min"]
        ):
            fail(errors, f"threshold policy {policy_id}: noul thresholds overlap")

    return questions_by_id, policies


def probabilities_valid(probabilities: dict[str, Any]) -> bool:
    values = list(probabilities.values())
    return bool(values) and math.isclose(sum(values), 1.0, abs_tol=1e-6)


def record_semantic_errors(
    record: dict[str, Any],
    questions: dict[str, dict[str, Any]],
    policies: dict[str, dict[str, Any]],
) -> list[str]:
    errors: list[str] = []
    question = questions.get(record.get("question_id"))
    if not question:
        return ["record references an unknown question"]

    if record.get("question_version") != question.get("version"):
        errors.append("record question version does not match active contract")
    if record.get("owning_engine") != question.get("owner_engine"):
        errors.append("record owning engine does not match question owner")
    if record.get("consent_purpose") != question.get("consent_purpose"):
        errors.append("record consent purpose does not match question purpose")

    policy_id = record.get("threshold_policy_id")
    policy = policies.get(policy_id)
    if not policy:
        errors.append("record references an unknown threshold policy")
    else:
        if policy_id != question.get("threshold_policy"):
            errors.append("record threshold policy differs from question contract")
        if record.get("threshold_policy_version") != policy.get("version"):
            errors.append("record threshold policy version is inconsistent")

    input_contract = question.get("input_contract", {})
    allowed_fields = set(input_contract.get("allowed_fields", []))
    used_fields = set(record.get("input_fields", []))
    if not used_fields <= allowed_fields:
        errors.append("record contains fields outside the question input allowlist")
    if used_fields & set(input_contract.get("forbidden_fields", [])):
        errors.append("record contains a forbidden input field")
    if record.get("minimization_policy_id") != input_contract.get(
        "minimization_policy_id"
    ):
        errors.append("record minimization policy ID is inconsistent")
    if record.get("minimization_policy_version") != input_contract.get(
        "minimization_policy_version"
    ):
        errors.append("record minimization policy version is inconsistent")

    answer = record.get("answer", {})
    if answer.get("type") != question.get("type"):
        errors.append("answer type differs from question type")

    criteria_snapshot = record.get("criteria_snapshot")
    registered_criteria = question.get("criteria")
    if registered_criteria is not None and criteria_snapshot != registered_criteria:
        errors.append("record criteria snapshot differs from question contract")
    if registered_criteria is None and question.get("criteria_source"):
        if not isinstance(criteria_snapshot, dict) or len(criteria_snapshot) < 2:
            errors.append("runtime criteria snapshot must preserve at least two choices")
        required_option = question.get("required_fallback_option")
        if required_option and required_option not in criteria_snapshot:
            errors.append("runtime criteria snapshot omits required fallback option")

    if answer.get("type") in {"choice", "score"}:
        probabilities = answer.get("probabilities", {})
        if not probabilities_valid(probabilities):
            errors.append("answer probabilities must sum to 1")
        if answer.get("type") == "choice":
            if answer.get("choice") not in probabilities:
                errors.append("selected choice is absent from probabilities")
            criteria = criteria_snapshot
            if isinstance(criteria, dict) and set(probabilities) != set(criteria):
                errors.append("choice probability keys differ from criteria snapshot")
        else:
            score_key = str(answer.get("score"))
            if score_key not in probabilities:
                errors.append("selected score is absent from probabilities")
            if isinstance(criteria_snapshot, list):
                expected_keys = {str(index) for index in range(len(criteria_snapshot))}
                if set(probabilities) != expected_keys:
                    errors.append("score probability keys differ from rubric snapshot")

    maximum = question.get("maximum_authority")
    effect = record.get("authority_effect")
    if EFFECT_RANK.get(effect, 99) > AUTHORITY_RANK.get(maximum, -1):
        errors.append("record authority effect exceeds question maximum authority")

    selected_action = record.get("selected_action")
    if maximum == "signal_only" and selected_action not in {
        "display_signal",
        "abstain",
        "no_action",
        "professional_review",
    }:
        errors.append("selected action exceeds signal-only authority")
    if record.get("fallback_reason") is not None and selected_action not in {
        "abstain",
        "no_action",
        "professional_review",
    }:
        errors.append("fallback reason is inconsistent with selected action")
    if effect == "human_confirmed" and record.get("human_override") is None:
        errors.append("human-confirmed effect requires a human action record")

    return errors


def validate_record(
    errors: list[str],
    questions: dict[str, dict[str, Any]],
    policies: dict[str, dict[str, Any]],
) -> None:
    record, schema = validate_with_schema(errors, VALID_RECORD, RECORD_SCHEMA)
    if not errors:
        errors.extend(
            f"{VALID_RECORD.name}: {error}"
            for error in record_semantic_errors(record, questions, policies)
        )

    required = set(schema.get("required", []))
    must_require = {
        "evaluation_id",
        "created_at",
        "actor_type",
        "actor_reference",
        "owning_engine",
        "question_id",
        "question_version",
        "threshold_policy_id",
        "threshold_policy_version",
        "requested_model_alias",
        "returned_model_version",
        "state_hash",
        "request_hash",
        "criteria_hash",
        "criteria_snapshot",
        "external_ai_disclosure_reference",
        "source_references",
        "consent_reference",
        "consent_purpose",
        "minimization_policy_id",
        "minimization_policy_version",
        "input_fields",
        "answer",
        "selected_action",
        "authority_effect",
        "fallback_reason",
        "human_override",
        "correlation_id",
        "causation_id",
    }
    missing = sorted(must_require - required)
    if missing:
        fail(
            errors,
            "evaluation record schema misses required fields: " + ", ".join(missing),
        )

    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    adversarial: list[tuple[str, dict[str, Any]]] = []

    def case(name: str, mutate) -> None:
        value = copy.deepcopy(record)
        mutate(value)
        adversarial.append((name, value))

    case("empty source references", lambda x: x.__setitem__("source_references", []))
    case(
        "choice absent from probabilities",
        lambda x: x.__setitem__(
            "answer",
            {
                "type": "choice",
                "choice": "not-returned",
                "probabilities": {"owner_review": 0.5, "human_triage": 0.5},
                "confidence": 0.5,
            },
        ),
    )
    case(
        "probabilities do not sum to one",
        lambda x: x["answer"].__setitem__(
            "probabilities", {"0": 0.4, "1": 0.4, "2": 0.4, "3": 0.4}
        ),
    )
    case(
        "authority ceiling exceeded",
        lambda x: x.__setitem__("authority_effect", "reversible_route"),
    )
    case(
        "wrong consent purpose",
        lambda x: x.__setitem__("consent_purpose", "unrelated_purpose"),
    )
    case(
        "input allowlist exceeded",
        lambda x: x["input_fields"].append("raw_documents"),
    )
    case(
        "criteria snapshot differs from contract",
        lambda x: x["criteria_snapshot"].__setitem__(
            0, "Mutated rubric meaning"
        ),
    )

    escaped = []
    for name, value in adversarial:
        structural = list(validator.iter_errors(value))
        semantic = record_semantic_errors(value, questions, policies)
        if not structural and not semantic:
            escaped.append(name)
    if escaped:
        fail(
            errors,
            "evaluation adversarial cases were incorrectly accepted: "
            + ", ".join(escaped),
        )


def main() -> int:
    errors: list[str] = []
    try:
        questions, policies = validate_registry(errors)
        validate_record(errors, questions, policies)
    except (OSError, json.JSONDecodeError) as exc:
        errors.append(str(exc))

    if errors:
        print("Jev contract validation failed:")
        for error in errors:
            print(f"- {error}")
        return 1

    print(
        "Jev contract validation passed "
        "(registry + policy + valid record + 7 adversarial cases)"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())