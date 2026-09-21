"""Loopback-only development HTTP API for the Release 0 workflow.

The actor header is an explicit development seam, not authentication. This
server must not be exposed beyond loopback until real authentication,
authorization middleware, TLS, rate limits, and production hardening exist.
"""

from __future__ import annotations

import argparse
import json
import re
from http import HTTPStatus
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path
from typing import Any
from urllib.parse import urlparse

from .errors import (
    AuthorizationError,
    ConflictError,
    ForhemitError,
    InvariantError,
    NotFoundError,
)
from .workflow import Release0Workflow

MAX_BODY_BYTES = 1_048_576
ACTOR_HEADER = "X-Forhemit-Actor-ID"
IDEMPOTENCY_HEADER = "Idempotency-Key"
DEV_SYSTEM_ACTOR = "local-system"
STATIC_DIR = Path(__file__).with_name("static")
STATIC_ROUTES = {
    "/": ("index.html", "text/html; charset=utf-8"),
    "/index.html": ("index.html", "text/html; charset=utf-8"),
    "/app.css": ("app.css", "text/css; charset=utf-8"),
    "/app.js": ("app.js", "text/javascript; charset=utf-8"),
    "/favicon.svg": ("favicon.svg", "image/svg+xml"),
}


class ApiProblem(Exception):
    def __init__(self, status: int, code: str, message: str):
        super().__init__(message)
        self.status = status
        self.code = code
        self.message = message


class Release0Api:
    """Transport adapter around the deterministic application service."""

    def __init__(self, workflow: Release0Workflow):
        self.workflow = workflow

    @staticmethod
    def _required(body: dict[str, Any], *fields: str) -> None:
        missing = [field for field in fields if field not in body]
        if missing:
            raise ApiProblem(
                HTTPStatus.BAD_REQUEST,
                "missing_fields",
                f"required fields are missing: {', '.join(missing)}",
            )

    @staticmethod
    def _fields(
        body: dict[str, Any],
        *,
        required: set[str],
        optional: set[str] | None = None,
    ) -> None:
        Release0Api._required(body, *sorted(required))
        allowed = required | (optional or set())
        unsupported = sorted(set(body) - allowed)
        if unsupported:
            raise ApiProblem(
                HTTPStatus.BAD_REQUEST,
                "unsupported_fields",
                f"unsupported fields: {', '.join(unsupported)}",
            )

    @staticmethod
    def _command_key(headers: Any) -> str:
        value = headers.get(IDEMPOTENCY_HEADER)
        if not value:
            raise ApiProblem(
                HTTPStatus.BAD_REQUEST,
                "missing_idempotency_key",
                f"{IDEMPOTENCY_HEADER} is required for commands",
            )
        return value

    @staticmethod
    def _actor(headers: Any) -> str:
        actor = headers.get(ACTOR_HEADER)
        if not actor:
            raise ApiProblem(
                HTTPStatus.UNAUTHORIZED,
                "development_actor_required",
                (
                    f"{ACTOR_HEADER} is required. It is a development identity "
                    "seam and is not production authentication."
                ),
            )
        return actor

    def dispatch(
        self,
        method: str,
        path: str,
        headers: Any,
        body: dict[str, Any] | None,
    ) -> tuple[int, dict[str, Any]]:
        if method == "GET" and path == "/health":
            return HTTPStatus.OK, {
                "status": "ok",
                "service": "forhemit-release0",
                "auth_mode": "development_header_not_authentication",
                "jev_runtime_enabled": False,
            }

        actor = self._actor(headers)

        status_match = re.fullmatch(
            r"/v1/owners/([A-Za-z0-9][A-Za-z0-9._-]{0,127})"
            r"/businesses/([A-Za-z0-9][A-Za-z0-9._-]{0,127})/status",
            path,
        )
        if method == "GET" and status_match:
            owner_id, business_id = status_match.groups()
            return HTTPStatus.OK, self.workflow.get_owner_workflow_status(
                actor_id=actor,
                owner_id=owner_id,
                business_id=business_id,
            )

        if method != "POST":
            raise ApiProblem(
                HTTPStatus.NOT_FOUND, "route_not_found", "route not found"
            )
        if body is None:
            raise ApiProblem(
                HTTPStatus.BAD_REQUEST, "json_body_required", "JSON body required"
            )

        key = self._command_key(headers)

        if path == "/v1/destinations/confirm":
            self._fields(
                body,
                required={
                    "owner_id",
                    "business_id",
                    "destination_id",
                    "transition_horizon",
                    "employee_ownership_intent",
                    "owner_involvement",
                    "objectives",
                    "nonnegotiables",
                },
                optional={"preferences", "unknowns", "contradictions"},
            )
            result = self.workflow.confirm_destination(
                actor_id=actor,
                idempotency_key=key,
                **body,
            )
            return HTTPStatus.CREATED, result

        if path == "/v1/business-snapshots/confirm":
            self._fields(
                body,
                required={
                    "owner_id",
                    "business_id",
                    "current_state_id",
                    "primary_jurisdiction",
                    "facts",
                },
            )
            result = self.workflow.confirm_business_snapshot(
                actor_id=actor,
                idempotency_key=key,
                **body,
            )
            return HTTPStatus.CREATED, result

        if path == "/v1/scenarios":
            self._fields(
                body,
                required={
                    "owner_id",
                    "scenario_id",
                    "destination_reference",
                    "current_state_reference",
                    "assumptions",
                    "unknowns",
                    "conflicts",
                    "professional_questions",
                },
            )
            result = self.workflow.create_scenario(
                actor_id=actor,
                idempotency_key=key,
                **body,
            )
            return HTTPStatus.CREATED, result

        if path == "/v1/professional-consents":
            self._fields(
                body,
                required={
                    "owner_id",
                    "consent_id",
                    "recipient_id",
                    "purpose",
                    "resource_references",
                    "permissions",
                    "expires_at",
                },
            )
            result = self.workflow.grant_professional_consent(
                actor_id=actor,
                idempotency_key=key,
                **body,
            )
            return HTTPStatus.CREATED, result

        revoke_match = re.fullmatch(
            r"/v1/professional-consents/([^/]+)/revoke", path
        )
        if revoke_match:
            self._fields(body, required={"owner_id"})
            result = self.workflow.revoke_professional_consent(
                actor_id=actor,
                owner_id=body["owner_id"],
                consent_id=revoke_match.group(1),
                idempotency_key=key,
            )
            return HTTPStatus.OK, result

        if path == "/v1/review-packages/authorize":
            self._fields(
                body,
                required={
                    "owner_id",
                    "business_id",
                    "package_id",
                    "recipient_id",
                    "professional_role",
                    "purpose",
                    "destination_reference",
                    "current_state_reference",
                    "scenario_reference",
                    "consent_reference",
                    "included_categories",
                    "excluded_categories",
                },
            )
            result = self.workflow.authorize_review_package(
                actor_id=actor,
                idempotency_key=key,
                **body,
            )
            return HTTPStatus.CREATED, result

        response_match = re.fullmatch(
            r"/v1/review-packages/([^/]+)/versions/(\d+)/responses", path
        )
        if response_match:
            self._fields(
                body,
                required={
                    "professional_role",
                    "response_text",
                    "category",
                },
            )
            result = self.workflow.submit_professional_response(
                actor_id=actor,
                package_id=response_match.group(1),
                package_version=int(response_match.group(2)),
                idempotency_key=key,
                **body,
            )
            return HTTPStatus.CREATED, result

        acknowledgment_match = re.fullmatch(
            r"/v1/professional-determinations/([^/]+)/acknowledgments", path
        )
        if acknowledgment_match:
            self._fields(
                body, required={"owner_id", "statement"}
            )
            result = self.workflow.acknowledge_professional_response(
                actor_id=actor,
                owner_id=body["owner_id"],
                determination_id=acknowledgment_match.group(1),
                statement=body["statement"],
                idempotency_key=key,
            )
            return HTTPStatus.CREATED, result

        if path == "/v1/dev/workers/deliver-packages":
            self._fields(body, required=set())
            self._require_dev_system(actor)
            return HTTPStatus.OK, {
                "delivered": self.workflow.deliver_packages()
            }

        if path == "/v1/dev/workers/import-responses":
            self._fields(body, required=set())
            self._require_dev_system(actor)
            return HTTPStatus.OK, {
                "imported": self.workflow.import_professional_responses()
            }

        raise ApiProblem(
            HTTPStatus.NOT_FOUND, "route_not_found", "route not found"
        )

    @staticmethod
    def _require_dev_system(actor: str) -> None:
        if actor != DEV_SYSTEM_ACTOR:
            raise AuthorizationError(
                "development worker endpoints require local-system"
            )


class Release0RequestHandler(BaseHTTPRequestHandler):
    server_version = "ForhemitRelease0/0.2"
    # Close each development response so the serialized server cannot be held by
    # one browser keep-alive connection while CSS, JavaScript, or API requests wait.
    protocol_version = "HTTP/1.0"

    def do_GET(self) -> None:  # noqa: N802
        try:
            self._validate_host()
            path = urlparse(self.path).path
            static = STATIC_ROUTES.get(path)
            if static:
                filename, content_type = static
                self._send_bytes(
                    HTTPStatus.OK,
                    (STATIC_DIR / filename).read_bytes(),
                    content_type,
                )
                return
            self._handle()
        except ApiProblem as error:
            self._send_problem(error.status, error.code, error.message)

    def do_POST(self) -> None:  # noqa: N802
        self._handle()

    def _handle(self) -> None:
        try:
            self._validate_host()
            path = urlparse(self.path).path
            body = self._read_json() if self.command == "POST" else None
            status, payload = self.server.api.dispatch(  # type: ignore[attr-defined]
                self.command, path, self.headers, body
            )
            self._send_json(status, {"data": payload})
        except ApiProblem as error:
            self._send_problem(error.status, error.code, error.message)
        except AuthorizationError as error:
            self._send_problem(
                HTTPStatus.FORBIDDEN, "authorization_denied", str(error)
            )
        except NotFoundError as error:
            self._send_problem(HTTPStatus.NOT_FOUND, "not_found", str(error))
        except ConflictError as error:
            self._send_problem(HTTPStatus.CONFLICT, "conflict", str(error))
        except InvariantError as error:
            self._send_problem(
                HTTPStatus.UNPROCESSABLE_ENTITY,
                "invariant_failed",
                str(error),
            )
        except ForhemitError as error:
            self._send_problem(
                HTTPStatus.BAD_REQUEST, "domain_error", str(error)
            )
        except json.JSONDecodeError:
            self._send_problem(
                HTTPStatus.BAD_REQUEST, "invalid_json", "request body is not valid JSON"
            )
        except Exception:
            self._send_problem(
                HTTPStatus.INTERNAL_SERVER_ERROR,
                "internal_error",
                "unexpected server error",
            )

    def _read_json(self) -> dict[str, Any]:
        content_type = self.headers.get("Content-Type", "")
        if not content_type.lower().startswith("application/json"):
            raise ApiProblem(
                HTTPStatus.UNSUPPORTED_MEDIA_TYPE,
                "json_content_type_required",
                "Content-Type must be application/json",
            )
        raw_length = self.headers.get("Content-Length")
        if raw_length is None:
            raise ApiProblem(
                HTTPStatus.LENGTH_REQUIRED,
                "content_length_required",
                "Content-Length is required",
            )
        try:
            length = int(raw_length)
        except ValueError as error:
            raise ApiProblem(
                HTTPStatus.BAD_REQUEST,
                "invalid_content_length",
                "Content-Length must be an integer",
            ) from error
        if length < 0 or length > MAX_BODY_BYTES:
            raise ApiProblem(
                HTTPStatus.REQUEST_ENTITY_TOO_LARGE,
                "request_too_large",
                f"request body exceeds {MAX_BODY_BYTES} bytes",
            )
        value = json.loads(self.rfile.read(length))
        if not isinstance(value, dict):
            raise ApiProblem(
                HTTPStatus.BAD_REQUEST,
                "json_object_required",
                "request body must be a JSON object",
            )
        return value

    def _validate_host(self) -> None:
        host = self.headers.get("Host", "")
        hostname = host.rsplit(":", 1)[0].lower()
        if hostname not in {"127.0.0.1", "localhost"}:
            raise ApiProblem(
                HTTPStatus.BAD_REQUEST,
                "invalid_host",
                "development server accepts only loopback Host headers",
            )

    def _send_problem(self, status: int, code: str, message: str) -> None:
        self._send_json(
            status,
            {
                "error": {
                    "code": code,
                    "message": message,
                }
            },
        )

    def _send_json(self, status: int, payload: dict[str, Any]) -> None:
        encoded = json.dumps(
            payload, separators=(",", ":"), ensure_ascii=False
        ).encode()
        self._send_bytes(
            status, encoded, "application/json; charset=utf-8"
        )

    def _send_bytes(
        self, status: int, content: bytes, content_type: str
    ) -> None:
        self.send_response(int(status))
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(content)))
        self.send_header(
            "X-Forhemit-Auth-Mode",
            "development-header-not-authentication",
        )
        self.send_header("Cache-Control", "no-store")
        self.send_header("X-Content-Type-Options", "nosniff")
        self.send_header("Referrer-Policy", "no-referrer")
        self.send_header("X-Frame-Options", "DENY")
        self.send_header(
            "Content-Security-Policy",
            "default-src 'self'; script-src 'self'; style-src 'self'; "
            "connect-src 'self'; img-src 'self'; font-src 'self'; "
            "base-uri 'none'; form-action 'self'; frame-ancestors 'none'; "
            "object-src 'none'",
        )
        self.end_headers()
        self.wfile.write(content)

    def log_message(self, format: str, *args: Any) -> None:
        return


class Release0HttpServer(HTTPServer):
    """Single-process development server constrained to loopback.

    A single request loop deliberately serializes access to the two SQLite
    connections. Production deployment needs a different connection model.
    """

    allow_reuse_address = True

    def __init__(
        self,
        server_address: tuple[str, int],
        api: Release0Api,
    ):
        host, _ = server_address
        if host not in {"127.0.0.1", "localhost"}:
            raise ValueError("Release 0 development API may bind only to loopback")
        super().__init__(server_address, Release0RequestHandler)
        self.api = api


def build_server(
    workspace_path: str | Path,
    collaboration_path: str | Path,
    host: str = "127.0.0.1",
    port: int = 8765,
) -> tuple[Release0HttpServer, Release0Workflow]:
    if host not in {"127.0.0.1", "localhost"}:
        raise ValueError("Release 0 development API may bind only to loopback")
    workflow = Release0Workflow(workspace_path, collaboration_path)
    try:
        server = Release0HttpServer((host, port), Release0Api(workflow))
    except Exception:
        workflow.close()
        raise
    return server, workflow


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Run the loopback-only Forhemit Release 0 development API"
    )
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=8765)
    parser.add_argument(
        "--workspace-db", default=".forhemit/workspace.sqlite"
    )
    parser.add_argument(
        "--collaboration-db", default=".forhemit/collaboration.sqlite"
    )
    args = parser.parse_args()

    for value in (args.workspace_db, args.collaboration_db):
        Path(value).parent.mkdir(parents=True, exist_ok=True)

    server, workflow = build_server(
        args.workspace_db,
        args.collaboration_db,
        args.host,
        args.port,
    )
    try:
        print(f"Forhemit Release 0 listening on http://{args.host}:{args.port}")
        print(
            "Development only: X-Forhemit-Actor-ID is not authentication."
        )
        server.serve_forever()
    finally:
        server.server_close()
        workflow.close()


if __name__ == "__main__":
    main()