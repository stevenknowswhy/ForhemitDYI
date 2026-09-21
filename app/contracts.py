"""Runtime validation against the repository's canonical JSON Schema definitions."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker

from .errors import InvariantError


class ContractValidator:
    """Validate emitted domain objects before they enter persistence."""

    def __init__(self, schema_path: str | Path | None = None):
        path = (
            Path(schema_path)
            if schema_path
            else Path(__file__).resolve().parents[1]
            / "contracts"
            / "vertical-slice-contracts.schema.json"
        )
        self.schema = json.loads(path.read_text())
        Draft202012Validator.check_schema(self.schema)
        self._validators: dict[str, Draft202012Validator] = {}

    def validate(self, definition: str, instance: Any) -> None:
        if definition not in self.schema["$defs"]:
            raise InvariantError(f"unknown contract definition: {definition}")
        validator = self._validators.get(definition)
        if validator is None:
            validator = Draft202012Validator(
                {
                    "$schema": self.schema["$schema"],
                    "$ref": f"#/$defs/{definition}",
                    "$defs": self.schema["$defs"],
                },
                format_checker=FormatChecker(),
            )
            self._validators[definition] = validator
        errors = sorted(
            validator.iter_errors(instance),
            key=lambda error: [str(part) for part in error.path],
        )
        if errors:
            details = "; ".join(
                f"{'.'.join(str(part) for part in error.path) or '<root>'}: "
                f"{error.message}"
                for error in errors
            )
            raise InvariantError(
                f"{definition} failed canonical contract validation: {details}"
            )