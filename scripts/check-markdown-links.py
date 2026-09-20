#!/usr/bin/env python3
"""Check local Markdown targets and heading anchors without network access."""

from __future__ import annotations

import re
import sys
import urllib.parse
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LINK = re.compile(r"(?<!!)\[[^\]]*\]\(([^)]+)\)")
HEADING = re.compile(r"^#{1,6}\s+(.+?)\s*#*\s*$")


def slug(text: str) -> str:
    text = text.strip().lower()
    text = re.sub(r"[^\w\s-]", "", text, flags=re.UNICODE)
    return re.sub(r"[\s-]+", "-", text).strip("-")


def main() -> int:
    errors: list[str] = []
    markdown_files = sorted(ROOT.rglob("*.md"))

    for source in markdown_files:
        text = source.read_text(encoding="utf-8", errors="replace")
        for match in LINK.finditer(text):
            raw = match.group(1).strip().split()[0].strip("<>")
            if re.match(r"^(https?:|mailto:)", raw):
                continue

            path_part, separator, fragment = raw.partition("#")
            decoded_path = urllib.parse.unquote(path_part)
            target = (
                (source.parent / decoded_path).resolve()
                if decoded_path
                else source.resolve()
            )

            try:
                target.relative_to(ROOT)
            except ValueError:
                errors.append(f"{source.relative_to(ROOT)}: target escapes repository: {raw}")
                continue

            if decoded_path and not target.exists():
                errors.append(f"{source.relative_to(ROOT)}: missing target: {raw}")
                continue

            if separator and target.exists() and target.suffix.lower() == ".md":
                anchors = {
                    slug(heading.group(1))
                    for line in target.read_text(
                        encoding="utf-8", errors="replace"
                    ).splitlines()
                    if (heading := HEADING.match(line))
                }
                expected = urllib.parse.unquote(fragment).lower()
                if expected not in anchors:
                    errors.append(
                        f"{source.relative_to(ROOT)}: missing anchor {raw}"
                    )

    if errors:
        print("Markdown link validation failed:")
        for error in errors:
            print(f"- {error}")
        return 1

    print(f"Markdown link validation passed ({len(markdown_files)} files)")
    return 0


if __name__ == "__main__":
    sys.exit(main())