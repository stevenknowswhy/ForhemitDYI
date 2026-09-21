from __future__ import annotations

import http.client
import json
import tempfile
import threading
import unittest
import urllib.error
import urllib.request
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any

from app.api import build_server


class Release0ApiTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        root = Path(self.temp.name)
        self.server, self.workflow = build_server(
            root / "workspace.sqlite",
            root / "collaboration.sqlite",
            host="127.0.0.1",
            port=0,
        )
        self.thread = threading.Thread(
            target=self.server.serve_forever, daemon=True
        )
        self.thread.start()
        host, port = self.server.server_address
        self.base = f"http://{host}:{port}"
        self.owner = "owner-001"
        self.business = "business-001"
        self.professional = "professional-001"

    def tearDown(self):
        self.server.shutdown()
        self.thread.join(timeout=5)
        self.server.server_close()
        self.workflow.close()
        self.temp.cleanup()

    def request(
        self,
        method: str,
        path: str,
        *,
        actor: str | None = None,
        key: str | None = None,
        body: dict[str, Any] | None = None,
    ) -> tuple[int, dict[str, Any], dict[str, str]]:
        headers = {}
        data = None
        if actor:
            headers["X-Forhemit-Actor-ID"] = actor
        if key:
            headers["Idempotency-Key"] = key
        if body is not None:
            headers["Content-Type"] = "application/json"
            data = json.dumps(body).encode()
        request = urllib.request.Request(
            f"{self.base}{path}",
            data=data,
            headers=headers,
            method=method,
        )
        try:
            with urllib.request.urlopen(request, timeout=5) as response:
                return (
                    response.status,
                    json.loads(response.read()),
                    dict(response.headers.items()),
                )
        except urllib.error.HTTPError as error:
            try:
                return (
                    error.code,
                    json.loads(error.read()),
                    dict(error.headers.items()),
                )
            finally:
                error.close()

    def request_bytes(
        self, method: str, path: str
    ) -> tuple[int, bytes, dict[str, str]]:
        request = urllib.request.Request(
            f"{self.base}{path}", method=method
        )
        with urllib.request.urlopen(request, timeout=5) as response:
            return (
                response.status,
                response.read(),
                dict(response.headers.items()),
            )

    @staticmethod
    def reference(
        object_type: str, object_id: str, version: int = 1
    ) -> dict[str, Any]:
        return {
            "object_type": object_type,
            "object_id": object_id,
            "version": version,
        }

    @staticmethod
    def facts() -> list[dict[str, Any]]:
        observed = "2026-09-21T16:00:00Z"
        values = {
            "business_name": "Example Manufacturing",
            "entity_type": "corporation",
            "revenue_range": {"min": 5_000_000, "max": 8_000_000},
            "operating_cash_flow_range": {
                "min": 700_000,
                "max": 1_200_000,
            },
            "employee_count": 42,
            "ownership_summary": "Owner controls 100%",
            "management_summary": "Management team exists",
            "debt_range": None,
        }
        result = []
        for index, (field, value) in enumerate(values.items(), 1):
            result.append(
                {
                    "fact_id": f"fact-{index:03d}",
                    "field": field,
                    "value": value,
                    "value_status": (
                        "unknown"
                        if value is None
                        else "range"
                        if isinstance(value, dict)
                        else "known"
                    ),
                    "source_authority": "owner_stated",
                    "sensitivity": "restricted",
                    "provenance": [
                        {
                            "source_id": "owner-entry-001",
                            "source_type": "owner_input",
                            "source_version": "1",
                            "information_status": "stated",
                            "observed_at": observed,
                            "excerpt_hash": None,
                        }
                    ],
                }
            )
        return result

    def destination_body(self) -> dict[str, Any]:
        return {
            "owner_id": self.owner,
            "business_id": self.business,
            "destination_id": "dest-001",
            "transition_horizon": "1_to_3_years",
            "employee_ownership_intent": "explore",
            "owner_involvement": "transition_period",
            "objectives": [
                {
                    "objective_id": "objective-001",
                    "statement": "Explore meaningful employee ownership",
                    "priority": 1,
                }
            ],
            "nonnegotiables": [
                {
                    "nonnegotiable_id": "constraint-001",
                    "statement": "Require qualified professional review",
                    "source_authority": "owner_stated",
                }
            ],
        }

    def test_health_is_public_but_marks_development_auth_mode(self):
        status, payload, headers = self.request("GET", "/health")
        self.assertEqual(status, 200)
        self.assertFalse(payload["data"]["jev_runtime_enabled"])
        self.assertEqual(
            headers["X-Forhemit-Auth-Mode"],
            "development-header-not-authentication",
        )

    def test_non_loopback_bind_is_rejected(self):
        root = Path(self.temp.name)
        with self.assertRaisesRegex(ValueError, "loopback"):
            build_server(
                root / "other-workspace.sqlite",
                root / "other-collaboration.sqlite",
                host="0.0.0.0",
                port=0,
            )

    def test_owner_ui_and_assets_are_same_origin_and_hardened(self):
        status, html, headers = self.request_bytes("GET", "/")
        self.assertEqual(status, 200)
        self.assertIn(b"<title>Forhemit", html)
        self.assertIn(b"Development identity is not authentication", html)
        self.assertIn(b'id="stage-list"', html)
        self.assertEqual(headers["X-Frame-Options"], "DENY")
        self.assertEqual(headers["X-Content-Type-Options"], "nosniff")
        self.assertIn("default-src 'self'", headers["Content-Security-Policy"])

        for path, content_type in (
            ("/app.css", "text/css; charset=utf-8"),
            ("/app.js", "text/javascript; charset=utf-8"),
        ):
            asset_status, content, asset_headers = self.request_bytes(
                "GET", path
            )
            self.assertEqual(asset_status, 200)
            self.assertTrue(content)
            self.assertEqual(asset_headers["Content-Type"], content_type)

    def test_invalid_host_header_is_rejected(self):
        host, port = self.server.server_address
        connection = http.client.HTTPConnection(host, port, timeout=5)
        try:
            connection.request("GET", "/health", headers={"Host": "evil.test"})
            response = connection.getresponse()
            payload = json.loads(response.read())
        finally:
            connection.close()
        self.assertEqual(response.status, 400)
        self.assertEqual(payload["error"]["code"], "invalid_host")

    def test_actor_and_idempotency_headers_are_required(self):
        status, payload, _ = self.request(
            "POST", "/v1/destinations/confirm", body=self.destination_body()
        )
        self.assertEqual(status, 401)
        self.assertEqual(
            payload["error"]["code"], "development_actor_required"
        )

        status, payload, _ = self.request(
            "POST",
            "/v1/destinations/confirm",
            actor=self.owner,
            body=self.destination_body(),
        )
        self.assertEqual(status, 400)
        self.assertEqual(
            payload["error"]["code"], "missing_idempotency_key"
        )

    def test_unknown_or_transport_owned_body_fields_are_rejected(self):
        body = self.destination_body()
        body["actor_id"] = "spoofed-owner"
        status, payload, _ = self.request(
            "POST",
            "/v1/destinations/confirm",
            actor=self.owner,
            key="spoof-attempt",
            body=body,
        )
        self.assertEqual(status, 400)
        self.assertEqual(payload["error"]["code"], "unsupported_fields")
        self.assertIn("actor_id", payload["error"]["message"])

    def test_owner_status_rejects_another_actor(self):
        status, payload, _ = self.request(
            "GET",
            f"/v1/owners/{self.owner}/businesses/{self.business}/status",
            actor="advisor-001",
        )
        self.assertEqual(status, 403)
        self.assertEqual(payload["error"]["code"], "authorization_denied")

    def test_api_idempotency_replays_the_same_result(self):
        first = self.request(
            "POST",
            "/v1/destinations/confirm",
            actor=self.owner,
            key="destination-http-1",
            body=self.destination_body(),
        )
        second = self.request(
            "POST",
            "/v1/destinations/confirm",
            actor=self.owner,
            key="destination-http-1",
            body=self.destination_body(),
        )
        self.assertEqual(first[0], 201)
        self.assertEqual(first[1], second[1])
        self.assertEqual(
            self.workflow.workspace.event_count("DestinationConfirmed"), 1
        )

    def test_idempotency_key_reuse_with_changed_command_conflicts(self):
        first = self.request(
            "POST",
            "/v1/destinations/confirm",
            actor=self.owner,
            key="destination-bound-command",
            body=self.destination_body(),
        )
        changed = self.destination_body()
        changed["owner_involvement"] = "ongoing"
        second = self.request(
            "POST",
            "/v1/destinations/confirm",
            actor=self.owner,
            key="destination-bound-command",
            body=changed,
        )
        self.assertEqual(first[0], 201)
        self.assertEqual(second[0], 409)
        self.assertEqual(second[1]["error"]["code"], "conflict")
        self.assertIn(
            "different command", second[1]["error"]["message"]
        )
        self.assertEqual(
            self.workflow.workspace.event_count("DestinationConfirmed"), 1
        )

    def test_full_manual_http_flow_and_guided_status(self):
        status, initial, _ = self.request(
            "GET",
            f"/v1/owners/{self.owner}/businesses/{self.business}/status",
            actor=self.owner,
        )
        self.assertEqual(status, 200)
        self.assertEqual(
            initial["data"]["next_action"],
            "Confirm the owner's transition destination.",
        )

        self.assertEqual(
            self.request(
                "POST",
                "/v1/destinations/confirm",
                actor=self.owner,
                key="destination-http",
                body=self.destination_body(),
            )[0],
            201,
        )
        snapshot_body = {
            "owner_id": self.owner,
            "business_id": self.business,
            "current_state_id": "state-001",
            "primary_jurisdiction": "US-CA",
            "facts": self.facts(),
        }
        self.assertEqual(
            self.request(
                "POST",
                "/v1/business-snapshots/confirm",
                actor=self.owner,
                key="snapshot-http",
                body=snapshot_body,
            )[0],
            201,
        )

        scenario_body = {
            "owner_id": self.owner,
            "scenario_id": "scenario-001",
            "destination_reference": self.reference(
                "DesiredOutcome", "dest-001"
            ),
            "current_state_reference": self.reference(
                "BusinessCurrentState", "state-001"
            ),
            "assumptions": [
                "Employee ownership remains an exploration goal"
            ],
            "unknowns": ["Independent valuation"],
            "conflicts": [],
            "professional_questions": [
                "What information is needed for feasibility review?"
            ],
        }
        self.assertEqual(
            self.request(
                "POST",
                "/v1/scenarios",
                actor=self.owner,
                key="scenario-http",
                body=scenario_body,
            )[0],
            201,
        )

        expires = (
            datetime.now(timezone.utc) + timedelta(days=30)
        ).isoformat().replace("+00:00", "Z")
        consent_body = {
            "owner_id": self.owner,
            "consent_id": "consent-001",
            "recipient_id": self.professional,
            "purpose": "california_esop_exploration_review",
            "resource_references": [
                self.reference("ScenarioVersion", "scenario-001")
            ],
            "permissions": ["full_view"],
            "expires_at": expires,
        }
        self.assertEqual(
            self.request(
                "POST",
                "/v1/professional-consents",
                actor=self.owner,
                key="consent-http",
                body=consent_body,
            )[0],
            201,
        )

        package_body = {
            "owner_id": self.owner,
            "business_id": self.business,
            "package_id": "package-001",
            "recipient_id": self.professional,
            "professional_role": "ESOP attorney",
            "purpose": "california_esop_exploration_review",
            "destination_reference": self.reference(
                "DesiredOutcome", "dest-001"
            ),
            "current_state_reference": self.reference(
                "BusinessCurrentState", "state-001"
            ),
            "scenario_reference": self.reference(
                "ScenarioVersion", "scenario-001"
            ),
            "consent_reference": self.reference(
                "ConsentRecord", "consent-001"
            ),
            "included_categories": [
                "destination",
                "business_snapshot",
                "exploratory_scenario",
                "professional_questions",
            ],
            "excluded_categories": [
                "raw_documents",
                "external_ai_records",
            ],
        }
        self.assertEqual(
            self.request(
                "POST",
                "/v1/review-packages/authorize",
                actor=self.owner,
                key="package-http",
                body=package_body,
            )[0],
            201,
        )

        delivered = self.request(
            "POST",
            "/v1/dev/workers/deliver-packages",
            actor="local-system",
            key="deliver-http",
            body={},
        )
        self.assertEqual(delivered[1]["data"]["delivered"], 1)

        response = self.request(
            "POST",
            "/v1/review-packages/package-001/versions/1/responses",
            actor=self.professional,
            key="response-http",
            body={
                "professional_role": "ESOP attorney",
                "response_text": (
                    "Qualified valuation and further feasibility work are required."
                ),
                "category": "information_request",
            },
        )
        self.assertEqual(response[0], 201)
        response_id = response[1]["data"]["response_id"]

        imported = self.request(
            "POST",
            "/v1/dev/workers/import-responses",
            actor="local-system",
            key="import-http",
            body={},
        )
        self.assertEqual(imported[1]["data"]["imported"], 1)

        review_status = self.request(
            "GET",
            f"/v1/owners/{self.owner}/businesses/{self.business}/status",
            actor=self.owner,
        )[1]["data"]
        self.assertEqual(
            review_status["stages"][-1]["status"], "in_progress"
        )

        acknowledgment = self.request(
            "POST",
            (
                "/v1/professional-determinations/"
                f"{response_id}/acknowledgments"
            ),
            actor=self.owner,
            key="ack-http",
            body={
                "owner_id": self.owner,
                "statement": (
                    "I acknowledge the feedback without selecting a transaction."
                ),
            },
        )
        self.assertEqual(acknowledgment[0], 201)
        self.assertFalse(
            acknowledgment[1]["data"]["automatic_handoff_created"]
        )

        final_status = self.request(
            "GET",
            f"/v1/owners/{self.owner}/businesses/{self.business}/status",
            actor=self.owner,
        )[1]["data"]
        self.assertTrue(
            all(
                stage["status"] == "complete"
                for stage in final_status["stages"]
            )
        )
        self.assertFalse(
            final_status["automatic_transaction_handoff_created"]
        )
        self.assertIn("stop", final_status["next_action"].lower())


if __name__ == "__main__":
    unittest.main()