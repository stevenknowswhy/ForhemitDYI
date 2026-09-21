# /// script
# requires-python = ">=3.12"
# dependencies = ["playwright==1.55.0", "jsonschema==4.25.1"]
# ///
"""Run the complete owner UI path in a real Chromium browser.

Usage:
    uv run tests/browser_owner_ui_smoke.py

The script uses an installed Chrome/Chromium executable, starts the API on a
temporary loopback port with temporary SQLite databases, and leaves no customer
data behind.
"""

from __future__ import annotations

import os
import shutil
import socket
import subprocess
import sys
import tempfile
import time
import urllib.request
from pathlib import Path

from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parents[1]


def available_port() -> int:
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        return int(sock.getsockname()[1])


def browser_executable() -> str | None:
    override = os.environ.get("FORHEMIT_BROWSER_EXECUTABLE")
    if override:
        return override
    for candidate in (
        "google-chrome",
        "chromium",
        "chromium-browser",
        "chrome",
    ):
        found = shutil.which(candidate)
        if found:
            return found
    return None


def wait_for_health(base: str, process: subprocess.Popen[bytes]) -> None:
    deadline = time.monotonic() + 15
    while time.monotonic() < deadline:
        if process.poll() is not None:
            detail = (
                process.stderr.read().decode(errors="replace")
                if process.stderr
                else ""
            )
            raise RuntimeError(
                f"development server exited with {process.returncode}: {detail}"
            )
        try:
            with urllib.request.urlopen(f"{base}/health", timeout=1) as response:
                if response.status == 200:
                    return
        except OSError:
            time.sleep(0.1)
    raise RuntimeError("development server did not become healthy")


def run_browser(base: str, screenshot: Path) -> None:
    executable = browser_executable()
    with sync_playwright() as playwright:
        launch = {"headless": True}
        if executable:
            launch["executable_path"] = executable
        browser = playwright.chromium.launch(**launch)
        page = browser.new_page(viewport={"width": 1440, "height": 1000})
        errors: list[str] = []
        page.on(
            "console",
            lambda message: errors.append(
                f"console {message.type}: {message.text}"
            )
            if message.type == "error"
            else None,
        )
        page.on("pageerror", lambda error: errors.append(f"pageerror: {error}"))

        page.goto(base, wait_until="networkidle")
        page.locator("#next-action strong").wait_for()
        assert "Confirm the owner's transition destination" in page.locator(
            "#next-action strong"
        ).inner_text()
        assert page.locator(".safety-banner").is_visible()

        page.locator("#destination-form button[type=submit]").click()
        page.locator("#notice").filter(
            has_text="Owner intent confirmed"
        ).wait_for()

        snapshot = page.locator("#snapshot-form")
        for name, value in {
            "business_name": "Synthetic Example Manufacturing",
            "entity_type": "corporation",
            "employee_count": "42",
            "revenue_min": "5000000",
            "revenue_max": "8000000",
            "cashflow_min": "700000",
            "cashflow_max": "1200000",
            "ownership_summary": "Synthetic owner controls 100%",
            "management_summary": "Synthetic management team is in place",
        }.items():
            snapshot.locator(f"[name={name}]").fill(value)
        snapshot.locator("button[type=submit]").click()
        page.locator("#notice").filter(
            has_text="Business snapshot confirmed"
        ).wait_for()

        page.locator("#scenario-form button[type=submit]").click()
        page.locator("#notice").filter(
            has_text="Exploratory scenario created"
        ).wait_for()

        package = page.locator("#package-form")
        package.locator("[name=consent_confirmed]").check()
        package.locator("button[type=submit]").click()
        page.locator("#notice").filter(
            has_text="Consent and disclosure recorded"
        ).wait_for()

        page.locator(".dev-tools summary").click()
        page.locator("#professional-form button[type=submit]").click()
        page.locator("#notice").filter(
            has_text="Attributed professional feedback imported"
        ).wait_for()
        assert page.locator("#determination-card").is_visible()

        page.locator(
            "#acknowledgment-form [name=acknowledge]"
        ).check()
        page.locator(
            "#acknowledgment-form button[type=submit]"
        ).click()
        page.locator("#notice").filter(
            has_text="No agreement or transaction handoff"
        ).wait_for()
        assert page.locator("#completion-card").is_visible()
        assert "stop after owner acknowledgment" in page.locator(
            "#next-action strong"
        ).inner_text().lower()
        assert all(
            "complete" in (item.get_attribute("class") or "")
            for item in page.locator("#stage-list li").all()
        )

        screenshot.write_bytes(page.screenshot(full_page=True))
        browser.close()
        if errors:
            raise AssertionError("\n".join(errors))


def main() -> None:
    port = available_port()
    base = f"http://127.0.0.1:{port}"
    screenshot = Path(
        os.environ.get(
            "FORHEMIT_UI_SCREENSHOT",
            str(Path(tempfile.gettempdir()) / "forhemit-owner-ui.png"),
        )
    )
    with tempfile.TemporaryDirectory() as directory:
        temp = Path(directory)
        process = subprocess.Popen(
            [
                sys.executable,
                "-m",
                "app.api",
                "--port",
                str(port),
                "--workspace-db",
                str(temp / "workspace.sqlite"),
                "--collaboration-db",
                str(temp / "collaboration.sqlite"),
            ],
            cwd=ROOT,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
        )
        try:
            wait_for_health(base, process)
            run_browser(base, screenshot)
        finally:
            process.terminate()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait(timeout=5)
    print(
        "Owner UI browser smoke test passed: five stages, no console errors, "
        f"screenshot={screenshot}"
    )


if __name__ == "__main__":
    main()