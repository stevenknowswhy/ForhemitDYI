#!/usr/bin/env python3
import base64
import json
import os
import socket
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
IDENTITY = {"X-Forwarded-User": "smoke-test-owner"}


def free_port():
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


def request(base, path, method="GET", body=None):
    payload = None if body is None else json.dumps(body).encode()
    headers = {**IDENTITY}
    if payload is not None:
        headers["Content-Type"] = "application/json"
    req = urllib.request.Request(
        base + path, data=payload, headers=headers, method=method
    )
    with urllib.request.urlopen(req, timeout=5) as response:
        data = response.read()
        return response.status, (
            json.loads(data) if data and "json" in response.headers.get_content_type() else data
        )


def main():
    port = free_port()
    with tempfile.TemporaryDirectory() as tmp:
        env = {
            **os.environ,
            "PORT": str(port),
            "DATABASE_PATH": str(Path(tmp) / "navigator.db"),
        }
        proc = subprocess.Popen(
            [sys.executable, "server.py"],
            cwd=ROOT,
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
        )
        base = f"http://127.0.0.1:{port}"
        try:
            for _ in range(50):
                try:
                    status, _ = request(base, "/readyz")
                    if status == 204:
                        break
                except (OSError, urllib.error.URLError):
                    time.sleep(0.1)
            else:
                raise AssertionError("Server did not become ready")

            status, state = request(base, "/api/state")
            assert status == 200
            assert state["destination"]["primaryGoal"] == "100% employee ownership"
            assert state["preferences"]["cashAtClosingPct"] == 40

            state["business"]["companyName"] = "Smoke Test Company"
            status, result = request(base, "/api/state", "POST", state)
            assert status == 200 and result["ok"]
            _, persisted = request(base, "/api/state")
            assert persisted["business"]["companyName"] == "Smoke Test Company"

            contents = b"smoke-test-document"
            status, document = request(
                base,
                "/api/documents",
                "POST",
                {
                    "filename": "financials.txt",
                    "contentType": "text/plain",
                    "category": "Financial",
                    "sensitivity": "Confidential",
                    "recipients": ["Valuation advisor"],
                    "data": base64.b64encode(contents).decode(),
                },
            )
            assert status == 201
            assert document["sizeBytes"] == len(contents)

            _, documents = request(base, "/api/documents")
            assert len(documents) == 1
            assert documents[0]["recipients"] == ["Valuation advisor"]

            status, downloaded = request(
                base, f"/api/documents/{document['id']}/download"
            )
            assert status == 200 and downloaded == contents

            status, result = request(
                base, f"/api/documents/{document['id']}", "DELETE"
            )
            assert status == 200 and result["ok"]
            _, documents = request(base, "/api/documents")
            assert documents == []

            status, index = request(base, "/")
            assert status == 200 and b"Private IPO Navigator" in index
            print("Private IPO Navigator smoke test passed")
        finally:
            proc.terminate()
            proc.wait(timeout=5)


if __name__ == "__main__":
    main()
