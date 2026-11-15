#!/usr/bin/env python3
"""Assemble Chough's changelog fragments into CHANGELOG.md under a release heading.

Chough's change log (LIFE-16) lists every shipped change under the release that first
contained it. A change adds one fragment, `changelog.d/<change-id>.md`, holding its
bullets, instead of editing `CHANGELOG.md`; the release task assembles the fragments
into the release heading and removes the fragments, so two open changes no longer edit
the same `CHANGELOG.md`.

The assembly is deterministic: the same fragments give the same bytes.

Usage:
    python3 tools/changelog.py --root <dir> --release <version> [--date <YYYY-MM-DD>]
    python3 tools/changelog.py --root <dir> --release Unreleased
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

FRAGMENT_DIR = "changelog.d"
HEADING_PREFIX = "## ["


def fragment_paths(root: Path) -> list[Path]:
    directory = root / FRAGMENT_DIR
    if not directory.is_dir():
        return []
    return sorted(
        (path for path in directory.iterdir() if path.is_file() and path.suffix == ".md"),
        key=lambda path: path.name,
    )


def bullet_groups(text: str) -> list[list[str]]:
    """The bullets of a fragment or changelog section, continuations kept, in order."""
    groups: list[list[str]] = []
    current: list[str] | None = None
    for line in text.split("\n"):
        if line.startswith("- "):
            current = [line]
            groups.append(current)
        elif current is not None and line[:1] in (" ", "\t"):
            current.append(line)
    return groups


def assemble(root: Path, release: str, date: str | None) -> int:
    if release == "Unreleased":
        heading = "## [Unreleased]"
    elif date:
        heading = f"## [{release}] - {date}"
    else:
        print(f"changelog: a release heading needs a date, not {release!r}", file=sys.stderr)
        return 2
    fragments = fragment_paths(root)
    if not fragments:
        return 0
    bullets: list[list[str]] = []
    for fragment in fragments:
        bullets.extend(bullet_groups(fragment.read_text(encoding="utf-8")))
    assembled = [line for bullet in bullets for line in bullet]
    changelog = root / "CHANGELOG.md"
    text = changelog.read_text(encoding="utf-8")
    lines_of_text = text.split("\n")
    if lines_of_text and lines_of_text[-1] == "":
        lines_of_text.pop()
    heading_index = None
    for index, line in enumerate(lines_of_text):
        if line == heading:
            heading_index = index
            break
    if heading_index is None:
        print(f"changelog: CHANGELOG.md has no {heading} section", file=sys.stderr)
        return 2
    next_heading = len(lines_of_text)
    for index in range(heading_index + 1, len(lines_of_text)):
        if lines_of_text[index].startswith(HEADING_PREFIX):
            next_heading = index
            break
    body = [""] + assembled + [""] if assembled else [""]
    lines_of_text[heading_index + 1 : next_heading] = body
    changelog.write_text("\n".join(lines_of_text) + "\n", encoding="utf-8")
    for fragment in fragments:
        fragment.unlink()
    print(
        f"changelog: assembled {len(bullets)} bullets from {len(fragments)} fragments "
        f"into {heading}"
    )
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Assemble changelog fragments into CHANGELOG.md under a release heading."
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=Path.cwd(),
        help="project root holding CHANGELOG.md and changelog.d/ (default: the working directory)",
    )
    parser.add_argument(
        "--release",
        required=True,
        help='the release heading to assemble under, e.g. "0.7.0", or "Unreleased"',
    )
    parser.add_argument(
        "--date",
        default=None,
        help="the release date of a versioned heading, YYYY-MM-DD (required then)",
    )
    args = parser.parse_args(argv)
    return assemble(args.root, args.release, args.date)


if __name__ == "__main__":
    sys.exit(main())