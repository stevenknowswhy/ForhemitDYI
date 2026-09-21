#!/usr/bin/env python3
import base64
import json
import os
import sqlite3
import uuid
from http.server import ThreadingHTTPServer, SimpleHTTPRequestHandler
from pathlib import Path
from urllib.parse import unquote, urlparse

ROOT = Path(__file__).resolve().parent
STATIC = ROOT / "static"
DB = Path(os.environ.get("DATABASE_PATH", str(ROOT / "navigator.db")))
PORT = int(os.environ.get("PORT", "4173"))
MAX_DOCUMENT_BYTES = int(os.environ.get("MAX_DOCUMENT_BYTES", str(25 * 1024 * 1024)))

DEFAULT_STATE = {
    "currentStep": 0,
    "completedSteps": [],
    "destination": {
        "primaryGoal": "100% employee ownership",
        "priorities": ["Preserve jobs", "Meaningful employee ownership"],
        "nonnegotiables": ["100% employee ownership"],
        "avoid": [],
        "timing": "1–3 years",
        "ownerRole": "Temporary transition advisor",
        "ownershipEssential": True
    },
    "business": {
        "companyName": "",
        "industry": "",
        "revenue": 12000000,
        "ebitda": 1800000,
        "purchaseMultiple": 5.0,
        "employees": 80,
        "enterpriseValue": 9000000,
        "existingDebt": 1000000,
        "transactionCosts": 450000,
        "ownerDependency": "Team could take over",
        "managementReadiness": "Some",
        "employeeInterest": "Not yet discussed",
        "customerConcentration": "Moderate"
    },
    "preferences": {
        "cashPreference": "Balanced cash now + future income",
        "cashAtClosingPct": 40,
        "minimumCash": 3600000,
        "sellerNoteRate": 7,
        "sellerNoteTerm": 7,
        "maxTransitionMonths": 24,
        "riskComfort": "Balanced"
    },
    "documentLocker": {
        "checklist": {},
        "sharingConfirmed": False
    },
    "selectedScenario": None
}


def connect():
    con = sqlite3.connect(DB)
    con.row_factory = sqlite3.Row
    con.execute("""
        CREATE TABLE IF NOT EXISTS journey_state (
            visitor_id TEXT PRIMARY KEY,
            state_json TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
    """)
    con.execute("""
        CREATE TABLE IF NOT EXISTS locker_documents (
            id TEXT PRIMARY KEY,
            visitor_id TEXT NOT NULL,
            filename TEXT NOT NULL,
            content_type TEXT NOT NULL,
            size_bytes INTEGER NOT NULL,
            category TEXT NOT NULL,
            sensitivity TEXT NOT NULL,
            recipients_json TEXT NOT NULL,
            data BLOB NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
    """)
    con.execute("""
        CREATE INDEX IF NOT EXISTS locker_documents_visitor
        ON locker_documents(visitor_id, created_at)
    """)
    return con


def visitor_id(handler):
    # Keep the established per-visitor key so existing journey progress remains available.
    return (
        handler.headers.get("X-PromptQL-User-Id")
        or handler.headers.get("X-PromptQL-Visitor-Id")
        or handler.headers.get("X-Forwarded-User")
        or "default"
    )[:200]


def safe_state():
    return json.loads(json.dumps(DEFAULT_STATE))


def document_json(row):
    return {
        "id": row["id"],
        "filename": row["filename"],
        "contentType": row["content_type"],
        "sizeBytes": row["size_bytes"],
        "category": row["category"],
        "sensitivity": row["sensitivity"],
        "recipients": json.loads(row["recipients_json"]),
        "createdAt": row["created_at"],
    }


class Handler(SimpleHTTPRequestHandler):
    def log_message(self, fmt, *args):
        print(f"[navigator] {self.address_string()} {fmt % args}")

    def send_json(self, obj, status=200):
        payload = json.dumps(obj).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(payload)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(payload)

    def read_json(self):
        length = int(self.headers.get("Content-Length", "0"))
        if length > MAX_DOCUMENT_BYTES * 1.5:
            raise ValueError("Request is too large")
        return json.loads(self.rfile.read(length))

    def do_GET(self):
        path = unquote(urlparse(self.path).path)
        vid = visitor_id(self)

        if path == "/readyz":
            self.send_response(204)
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            return

        if path == "/api/state":
            with connect() as con:
                row = con.execute(
                    "SELECT state_json FROM journey_state WHERE visitor_id = ?", (vid,)
                ).fetchone()
            self.send_json(json.loads(row[0]) if row else safe_state())
            return

        if path == "/api/reset":
            with connect() as con:
                con.execute("DELETE FROM journey_state WHERE visitor_id = ?", (vid,))
                con.execute("DELETE FROM locker_documents WHERE visitor_id = ?", (vid,))
            self.send_json(safe_state())
            return

        if path == "/api/documents":
            with connect() as con:
                rows = con.execute(
                    """SELECT id, filename, content_type, size_bytes, category,
                              sensitivity, recipients_json, created_at
                       FROM locker_documents
                       WHERE visitor_id = ?
                       ORDER BY created_at DESC""",
                    (vid,),
                ).fetchall()
            self.send_json([document_json(row) for row in rows])
            return

        if path.startswith("/api/documents/") and path.endswith("/download"):
            document_id = path.split("/")[3]
            with connect() as con:
                row = con.execute(
                    """SELECT filename, content_type, size_bytes, data
                       FROM locker_documents WHERE id = ? AND visitor_id = ?""",
                    (document_id, vid),
                ).fetchone()
            if not row:
                return self.send_json({"error": "Document not found"}, 404)
            filename = row["filename"].replace('"', "").replace("\r", "").replace("\n", "")
            self.send_response(200)
            self.send_header("Content-Type", row["content_type"])
            self.send_header("Content-Length", str(row["size_bytes"]))
            self.send_header("Content-Disposition", f'attachment; filename="{filename}"')
            self.send_header("Cache-Control", "private, no-store")
            self.send_header("X-Content-Type-Options", "nosniff")
            self.end_headers()
            self.wfile.write(row["data"])
            return

        if path == "/":
            self.path = "/index.html"
        elif not (STATIC / path.lstrip("/")).exists():
            self.path = "/index.html"
        return super().do_GET()

    def do_POST(self):
        path = unquote(urlparse(self.path).path)
        vid = visitor_id(self)
        try:
            body = self.read_json()

            if path == "/api/state":
                if not isinstance(body, dict):
                    raise ValueError("State must be an object")
                with connect() as con:
                    con.execute(
                        """INSERT INTO journey_state(visitor_id, state_json, updated_at)
                           VALUES(?, ?, CURRENT_TIMESTAMP)
                           ON CONFLICT(visitor_id) DO UPDATE SET
                             state_json=excluded.state_json,
                             updated_at=CURRENT_TIMESTAMP""",
                        (vid, json.dumps(body)),
                    )
                return self.send_json({"ok": True})

            if path == "/api/documents":
                filename = Path(str(body.get("filename", "document"))).name[:240]
                content_type = str(body.get("contentType") or "application/octet-stream")[:200]
                category = str(body.get("category") or "Other")[:100]
                sensitivity = str(body.get("sensitivity") or "Confidential")[:100]
                recipients = body.get("recipients") or []
                if not isinstance(recipients, list):
                    raise ValueError("Recipients must be a list")
                encoded = body.get("data")
                if not encoded:
                    raise ValueError("Choose a file to upload")
                try:
                    data = base64.b64decode(encoded, validate=True)
                except Exception as exc:
                    raise ValueError("The uploaded file could not be decoded") from exc
                if not data:
                    raise ValueError("The uploaded file is empty")
                if len(data) > MAX_DOCUMENT_BYTES:
                    raise ValueError("Each document must be 25 MB or smaller")
                document_id = str(uuid.uuid4())
                with connect() as con:
                    con.execute(
                        """INSERT INTO locker_documents(
                               id, visitor_id, filename, content_type, size_bytes,
                               category, sensitivity, recipients_json, data
                           ) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?)""",
                        (
                            document_id, vid, filename, content_type, len(data),
                            category, sensitivity, json.dumps(recipients[:20]), data
                        ),
                    )
                    row = con.execute(
                        """SELECT id, filename, content_type, size_bytes, category,
                                  sensitivity, recipients_json, created_at
                           FROM locker_documents WHERE id = ?""",
                        (document_id,),
                    ).fetchone()
                return self.send_json(document_json(row), 201)

            if path.startswith("/api/documents/"):
                document_id = path.split("/")[3]
                category = str(body.get("category") or "Other")[:100]
                sensitivity = str(body.get("sensitivity") or "Confidential")[:100]
                recipients = body.get("recipients") or []
                if not isinstance(recipients, list):
                    raise ValueError("Recipients must be a list")
                with connect() as con:
                    result = con.execute(
                        """UPDATE locker_documents
                           SET category = ?, sensitivity = ?, recipients_json = ?
                           WHERE id = ? AND visitor_id = ?""",
                        (category, sensitivity, json.dumps(recipients[:20]), document_id, vid),
                    )
                if not result.rowcount:
                    return self.send_json({"error": "Document not found"}, 404)
                return self.send_json({"ok": True})

            return self.send_json({"error": "not found"}, 404)
        except Exception as exc:
            self.send_json({"error": str(exc)}, 400)

    def do_DELETE(self):
        path = unquote(urlparse(self.path).path)
        if not path.startswith("/api/documents/"):
            return self.send_json({"error": "not found"}, 404)
        document_id = path.split("/")[3]
        with connect() as con:
            result = con.execute(
                "DELETE FROM locker_documents WHERE id = ? AND visitor_id = ?",
                (document_id, visitor_id(self)),
            )
        if not result.rowcount:
            return self.send_json({"error": "Document not found"}, 404)
        self.send_json({"ok": True})

    def translate_path(self, path):
        clean = Path(unquote(urlparse(path).path)).name
        return str(STATIC / clean)


if __name__ == "__main__":
    STATIC.mkdir(parents=True, exist_ok=True)
    connect().close()
    server = ThreadingHTTPServer(("0.0.0.0", PORT), Handler)
    print(f"Private IPO Navigator listening on :{PORT}")
    server.serve_forever()