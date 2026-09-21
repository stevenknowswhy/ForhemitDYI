"""SQLite persistence for the private workspace and collaboration boundary."""

from __future__ import annotations

import contextlib
import hashlib
import json
import sqlite3
from pathlib import Path
from typing import Any, Iterator

from .errors import ConflictError, NotFoundError


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


class WorkspaceStore:
    """Private owner-workspace store.

    Aggregate versions, audit events, disclosures, idempotency records, and
    outbound messages are committed in the same SQLite transaction.
    """

    def __init__(self, path: str | Path):
        self.path = str(path)
        self.connection = sqlite3.connect(self.path)
        self.connection.row_factory = sqlite3.Row
        self.connection.execute("PRAGMA foreign_keys = ON")
        self.connection.execute("PRAGMA journal_mode = WAL")
        self._migrate()

    def close(self) -> None:
        self.connection.close()

    def _migrate(self) -> None:
        self.connection.executescript(
            """
            CREATE TABLE IF NOT EXISTS aggregate_versions (
                object_type TEXT NOT NULL,
                object_id TEXT NOT NULL,
                version INTEGER NOT NULL CHECK (version > 0),
                data_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY (object_type, object_id, version)
            );

            CREATE TABLE IF NOT EXISTS current_versions (
                object_type TEXT NOT NULL,
                object_id TEXT NOT NULL,
                version INTEGER NOT NULL,
                PRIMARY KEY (object_type, object_id)
            );

            CREATE TABLE IF NOT EXISTS dependencies (
                dependent_type TEXT NOT NULL,
                dependent_id TEXT NOT NULL,
                dependent_version INTEGER NOT NULL,
                source_type TEXT NOT NULL,
                source_id TEXT NOT NULL,
                source_version INTEGER NOT NULL,
                PRIMARY KEY (
                    dependent_type, dependent_id, dependent_version,
                    source_type, source_id, source_version
                )
            );

            CREATE TABLE IF NOT EXISTS invalidations (
                object_type TEXT NOT NULL,
                object_id TEXT NOT NULL,
                version INTEGER NOT NULL,
                reason TEXT NOT NULL,
                invalidated_at TEXT NOT NULL,
                source_type TEXT NOT NULL,
                source_id TEXT NOT NULL,
                source_version INTEGER NOT NULL,
                PRIMARY KEY (object_type, object_id, version, reason)
            );

            CREATE TABLE IF NOT EXISTS audit_events (
                sequence INTEGER PRIMARY KEY AUTOINCREMENT,
                event_id TEXT NOT NULL UNIQUE,
                event_type TEXT NOT NULL,
                occurred_at TEXT NOT NULL,
                actor_type TEXT NOT NULL,
                actor_reference TEXT NOT NULL,
                source_module TEXT NOT NULL,
                subject_type TEXT NOT NULL,
                subject_id TEXT NOT NULL,
                subject_version INTEGER NOT NULL,
                correlation_id TEXT NOT NULL,
                causation_id TEXT NOT NULL,
                idempotency_key TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                previous_hash TEXT,
                event_hash TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS idempotency_results (
                scope TEXT NOT NULL,
                idempotency_key TEXT NOT NULL,
                result_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY (scope, idempotency_key)
            );

            CREATE TABLE IF NOT EXISTS outbox (
                message_id TEXT PRIMARY KEY,
                topic TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                status TEXT NOT NULL CHECK (status IN ('pending', 'delivered', 'cancelled')),
                created_at TEXT NOT NULL,
                delivered_at TEXT
            );

            CREATE TABLE IF NOT EXISTS acknowledgments (
                acknowledgment_id TEXT PRIMARY KEY,
                owner_id TEXT NOT NULL,
                determination_id TEXT NOT NULL,
                acknowledged_at TEXT NOT NULL,
                statement TEXT NOT NULL,
                UNIQUE(owner_id, determination_id)
            );
            """
        )
        self.connection.commit()

    @contextlib.contextmanager
    def transaction(self) -> Iterator[sqlite3.Connection]:
        conn = self.connection
        conn.execute("BEGIN IMMEDIATE")
        try:
            yield conn
        except Exception:
            conn.rollback()
            raise
        else:
            conn.commit()

    def get_idempotent(
        self, conn: sqlite3.Connection, scope: str, key: str
    ) -> dict[str, Any] | None:
        row = conn.execute(
            """
            SELECT result_json FROM idempotency_results
            WHERE scope = ? AND idempotency_key = ?
            """,
            (scope, key),
        ).fetchone()
        return json.loads(row["result_json"]) if row else None

    def save_idempotent(
        self,
        conn: sqlite3.Connection,
        scope: str,
        key: str,
        result: dict[str, Any],
        created_at: str,
    ) -> None:
        conn.execute(
            """
            INSERT INTO idempotency_results
                (scope, idempotency_key, result_json, created_at)
            VALUES (?, ?, ?, ?)
            """,
            (scope, key, canonical_json(result), created_at),
        )

    def next_version(
        self, conn: sqlite3.Connection, object_type: str, object_id: str
    ) -> int:
        row = conn.execute(
            """
            SELECT version FROM current_versions
            WHERE object_type = ? AND object_id = ?
            """,
            (object_type, object_id),
        ).fetchone()
        return int(row["version"]) + 1 if row else 1

    def store_version(
        self,
        conn: sqlite3.Connection,
        object_type: str,
        object_id: str,
        version: int,
        data: dict[str, Any],
        created_at: str,
        dependencies: list[dict[str, Any]] | None = None,
    ) -> None:
        expected = self.next_version(conn, object_type, object_id)
        if version != expected:
            raise ConflictError(
                f"{object_type} {object_id} expected version {expected}, got {version}"
            )
        conn.execute(
            """
            INSERT INTO aggregate_versions
                (object_type, object_id, version, data_json, created_at)
            VALUES (?, ?, ?, ?, ?)
            """,
            (object_type, object_id, version, canonical_json(data), created_at),
        )
        conn.execute(
            """
            INSERT INTO current_versions (object_type, object_id, version)
            VALUES (?, ?, ?)
            ON CONFLICT(object_type, object_id)
            DO UPDATE SET version = excluded.version
            """,
            (object_type, object_id, version),
        )
        for source in dependencies or []:
            conn.execute(
                """
                INSERT INTO dependencies (
                    dependent_type, dependent_id, dependent_version,
                    source_type, source_id, source_version
                ) VALUES (?, ?, ?, ?, ?, ?)
                """,
                (
                    object_type,
                    object_id,
                    version,
                    source["object_type"],
                    source["object_id"],
                    source["version"],
                ),
            )

    def invalidate_older_dependents(
        self,
        conn: sqlite3.Connection,
        source_type: str,
        source_id: str,
        new_version: int,
        invalidated_at: str,
    ) -> int:
        rows = conn.execute(
            """
            SELECT dependent_type, dependent_id, dependent_version, source_version
            FROM dependencies
            WHERE source_type = ? AND source_id = ? AND source_version < ?
            """,
            (source_type, source_id, new_version),
        ).fetchall()
        count = 0
        for row in rows:
            reason = (
                f"{source_type} {source_id} advanced from "
                f"version {row['source_version']} to {new_version}"
            )
            cursor = conn.execute(
                """
                INSERT OR IGNORE INTO invalidations (
                    object_type, object_id, version, reason, invalidated_at,
                    source_type, source_id, source_version
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                """,
                (
                    row["dependent_type"],
                    row["dependent_id"],
                    row["dependent_version"],
                    reason,
                    invalidated_at,
                    source_type,
                    source_id,
                    new_version,
                ),
            )
            count += cursor.rowcount
        return count

    def get_version(
        self,
        object_type: str,
        object_id: str,
        version: int | None = None,
        conn: sqlite3.Connection | None = None,
    ) -> dict[str, Any]:
        db = conn or self.connection
        if version is None:
            row = db.execute(
                """
                SELECT a.data_json
                FROM aggregate_versions a
                JOIN current_versions c
                  ON c.object_type = a.object_type
                 AND c.object_id = a.object_id
                 AND c.version = a.version
                WHERE a.object_type = ? AND a.object_id = ?
                """,
                (object_type, object_id),
            ).fetchone()
        else:
            row = db.execute(
                """
                SELECT data_json FROM aggregate_versions
                WHERE object_type = ? AND object_id = ? AND version = ?
                """,
                (object_type, object_id, version),
            ).fetchone()
        if not row:
            suffix = f" version {version}" if version is not None else ""
            raise NotFoundError(f"{object_type} {object_id}{suffix} not found")
        return json.loads(row["data_json"])

    def is_invalidated(
        self,
        object_type: str,
        object_id: str,
        version: int,
        conn: sqlite3.Connection | None = None,
    ) -> bool:
        db = conn or self.connection
        row = db.execute(
            """
            SELECT 1 FROM invalidations
            WHERE object_type = ? AND object_id = ? AND version = ?
            LIMIT 1
            """,
            (object_type, object_id, version),
        ).fetchone()
        return row is not None

    def append_event(
        self, conn: sqlite3.Connection, event: dict[str, Any]
    ) -> dict[str, Any]:
        previous = conn.execute(
            "SELECT event_hash FROM audit_events ORDER BY sequence DESC LIMIT 1"
        ).fetchone()
        previous_hash = previous["event_hash"] if previous else None
        material = {
            **event,
            "previous_hash": previous_hash,
        }
        event_hash = hashlib.sha256(canonical_json(material).encode()).hexdigest()
        subject = event["subject_reference"]
        conn.execute(
            """
            INSERT INTO audit_events (
                event_id, event_type, occurred_at, actor_type, actor_reference,
                source_module, subject_type, subject_id, subject_version,
                correlation_id, causation_id, idempotency_key, payload_json,
                previous_hash, event_hash
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                event["event_id"],
                event["event_type"],
                event["occurred_at"],
                event["actor_type"],
                event["actor_reference"],
                event["source_module"],
                subject["object_type"],
                subject["object_id"],
                subject["version"],
                event["correlation_id"],
                event["causation_id"],
                event["idempotency_key"],
                canonical_json(event.get("payload", {})),
                previous_hash,
                event_hash,
            ),
        )
        return {**event, "previous_hash": previous_hash, "event_hash": event_hash}

    def enqueue(
        self,
        conn: sqlite3.Connection,
        message_id: str,
        topic: str,
        payload: dict[str, Any],
        created_at: str,
    ) -> None:
        conn.execute(
            """
            INSERT INTO outbox
                (message_id, topic, payload_json, status, created_at)
            VALUES (?, ?, ?, 'pending', ?)
            """,
            (message_id, topic, canonical_json(payload), created_at),
        )

    def pending_messages(self) -> list[dict[str, Any]]:
        rows = self.connection.execute(
            """
            SELECT message_id, topic, payload_json, created_at
            FROM outbox WHERE status = 'pending' ORDER BY created_at, message_id
            """
        ).fetchall()
        return [
            {
                "message_id": row["message_id"],
                "topic": row["topic"],
                "payload": json.loads(row["payload_json"]),
                "created_at": row["created_at"],
            }
            for row in rows
        ]

    def mark_delivered(self, message_id: str, delivered_at: str) -> None:
        with self.transaction() as conn:
            conn.execute(
                """
                UPDATE outbox SET status = 'delivered', delivered_at = ?
                WHERE message_id = ?
                """,
                (delivered_at, message_id),
            )

    def cancel_message(self, message_id: str) -> None:
        with self.transaction() as conn:
            conn.execute(
                """
                UPDATE outbox SET status = 'cancelled'
                WHERE message_id = ? AND status = 'pending'
                """,
                (message_id,),
            )

    def event_count(self, event_type: str | None = None) -> int:
        if event_type:
            row = self.connection.execute(
                "SELECT COUNT(1) AS n FROM audit_events WHERE event_type = ?",
                (event_type,),
            ).fetchone()
        else:
            row = self.connection.execute(
                "SELECT COUNT(1) AS n FROM audit_events"
            ).fetchone()
        return int(row["n"])


class CollaborationStore:
    """Purpose-limited professional collaboration persistence boundary."""

    def __init__(self, path: str | Path):
        self.path = str(path)
        self.connection = sqlite3.connect(self.path)
        self.connection.row_factory = sqlite3.Row
        self.connection.execute("PRAGMA foreign_keys = ON")
        self.connection.execute("PRAGMA journal_mode = WAL")
        self._migrate()

    def close(self) -> None:
        self.connection.close()

    def _migrate(self) -> None:
        self.connection.executescript(
            """
            CREATE TABLE IF NOT EXISTS inbox (
                message_id TEXT PRIMARY KEY,
                received_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS disclosed_packages (
                package_id TEXT NOT NULL,
                version INTEGER NOT NULL,
                recipient_id TEXT NOT NULL,
                purpose TEXT NOT NULL,
                package_json TEXT NOT NULL,
                disclosure_json TEXT NOT NULL,
                received_at TEXT NOT NULL,
                PRIMARY KEY (package_id, version)
            );

            CREATE TABLE IF NOT EXISTS professional_responses (
                response_id TEXT PRIMARY KEY,
                package_id TEXT NOT NULL,
                package_version INTEGER NOT NULL,
                professional_id TEXT NOT NULL,
                professional_role TEXT NOT NULL,
                response_text TEXT NOT NULL,
                category TEXT NOT NULL,
                submitted_at TEXT NOT NULL,
                owner_acknowledged_at TEXT
            );

            CREATE TABLE IF NOT EXISTS idempotency_results (
                scope TEXT NOT NULL,
                idempotency_key TEXT NOT NULL,
                result_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY (scope, idempotency_key)
            );

            CREATE TABLE IF NOT EXISTS outbox (
                message_id TEXT PRIMARY KEY,
                topic TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                status TEXT NOT NULL CHECK (status IN ('pending', 'delivered')),
                created_at TEXT NOT NULL,
                delivered_at TEXT
            );
            """
        )
        self.connection.commit()

    @contextlib.contextmanager
    def transaction(self) -> Iterator[sqlite3.Connection]:
        conn = self.connection
        conn.execute("BEGIN IMMEDIATE")
        try:
            yield conn
        except Exception:
            conn.rollback()
            raise
        else:
            conn.commit()

    def receive_package(
        self, message_id: str, payload: dict[str, Any], received_at: str
    ) -> bool:
        with self.transaction() as conn:
            seen = conn.execute(
                "SELECT 1 FROM inbox WHERE message_id = ?", (message_id,)
            ).fetchone()
            if seen:
                return False
            package = payload["package"]
            disclosure = payload["disclosure"]
            conn.execute(
                """
                INSERT INTO disclosed_packages (
                    package_id, version, recipient_id, purpose,
                    package_json, disclosure_json, received_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
                """,
                (
                    package["package_id"],
                    package["version"],
                    package["recipient_id"],
                    package["purpose"],
                    canonical_json(package),
                    canonical_json(disclosure),
                    received_at,
                ),
            )
            conn.execute(
                "INSERT INTO inbox (message_id, received_at) VALUES (?, ?)",
                (message_id, received_at),
            )
            return True

    def get_package(self, package_id: str, version: int) -> dict[str, Any]:
        row = self.connection.execute(
            """
            SELECT package_json FROM disclosed_packages
            WHERE package_id = ? AND version = ?
            """,
            (package_id, version),
        ).fetchone()
        if not row:
            raise NotFoundError(
                f"disclosed package {package_id} version {version} not found"
            )
        return json.loads(row["package_json"])

    def submit_response(
        self,
        response: dict[str, Any],
        message_id: str,
        created_at: str,
    ) -> dict[str, Any]:
        scope = "professional_response.submit"
        key = response["idempotency_key"]
        with self.transaction() as conn:
            cached = conn.execute(
                """
                SELECT result_json FROM idempotency_results
                WHERE scope = ? AND idempotency_key = ?
                """,
                (scope, key),
            ).fetchone()
            if cached:
                return json.loads(cached["result_json"])
            conn.execute(
                """
                INSERT INTO professional_responses (
                    response_id, package_id, package_version, professional_id,
                    professional_role, response_text, category, submitted_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                """,
                (
                    response["response_id"],
                    response["package_id"],
                    response["package_version"],
                    response["professional_id"],
                    response["professional_role"],
                    response["response_text"],
                    response["category"],
                    response["submitted_at"],
                ),
            )
            conn.execute(
                """
                INSERT INTO outbox
                    (message_id, topic, payload_json, status, created_at)
                VALUES (?, 'professional.response.submitted', ?, 'pending', ?)
                """,
                (message_id, canonical_json(response), created_at),
            )
            conn.execute(
                """
                INSERT INTO idempotency_results
                    (scope, idempotency_key, result_json, created_at)
                VALUES (?, ?, ?, ?)
                """,
                (scope, key, canonical_json(response), created_at),
            )
            return response

    def pending_messages(self) -> list[dict[str, Any]]:
        rows = self.connection.execute(
            """
            SELECT message_id, topic, payload_json, created_at
            FROM outbox WHERE status = 'pending' ORDER BY created_at, message_id
            """
        ).fetchall()
        return [
            {
                "message_id": row["message_id"],
                "topic": row["topic"],
                "payload": json.loads(row["payload_json"]),
                "created_at": row["created_at"],
            }
            for row in rows
        ]

    def mark_delivered(self, message_id: str, delivered_at: str) -> None:
        with self.transaction() as conn:
            conn.execute(
                """
                UPDATE outbox SET status = 'delivered', delivered_at = ?
                WHERE message_id = ?
                """,
                (delivered_at, message_id),
            )

    def disclosed_package_count(self) -> int:
        row = self.connection.execute(
            "SELECT COUNT(1) AS n FROM disclosed_packages"
        ).fetchone()
        return int(row["n"])