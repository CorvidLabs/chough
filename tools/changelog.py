#!/usr/bin/env python3
"""Assemble Chough's changelog fragments into CHANGELOG.md under a release heading.

Chough's change log (LIFE-16) lists every shipped change under the release that first
contained it. A change adds one fragment, `changelog.d/<YYYY-MM-DD>-<change-id>.md`, holding
its bullets, instead of editing `CHANGELOG.md`; the release task assembles the fragments
into the release heading and removes the fragments, so two open changes no longer edit
the same `CHANGELOG.md`.

The assembly is deterministic: the same fragments give the same bytes.

Usage:
    python3 tools/changelog.py --root <dir> --release <version> [--date <YYYY-MM-DD>]
    python3 tools/changelog.py --root <dir> --release Unreleased
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

FRAGMENT_DIR = "changelog.d"
HEADING_PREFIX = "## ["


def fragment_sort_key(path: Path) -> tuple[str, int, str]:
    """Sort key: release date, then numeric leading issue number, then the rest of the name."""
    name = path.name
    stem = name[: -len(".md")] if name.endswith(".md") else name
    date_match = re.match(r"^(\d{4}-\d{2}-\d{2})", stem)
    date = date_match.group(1) if date_match else ""
    rest = stem[date_match.end() :] if date_match else stem
    issue_match = re.match(r"^[^0-9]*(\d+)", rest)
    issue = int(issue_match.group(1)) if issue_match else 0
    return (date, issue, rest)


def fragment_paths(root: Path) -> list[Path]:
    directory = root / FRAGMENT_DIR
    if not directory.is_dir():
        return []
    return sorted(
        (path for path in directory.iterdir() if path.is_file() and path.suffix == ".md"),
        key=fragment_sort_key,
    )


def bullet_groups(text: str) -> list[list[str]]:
    """The bullets of a fragment or changelog section, continuations kept, in order."""
    if text.startswith("\ufeff"):
        text = text[1:]
    groups: list[list[str]] = []
    current: list[str] | None = None
    pending_blank: str | None = None
    for line in text.split("\n"):
        if not line.strip():
            if current is not None and pending_blank is None:
                pending_blank = ""
            continue
        if line.startswith("- "):
            pending_blank = None
            current = [line]
            groups.append(current)
        elif current is not None and line[:1] in (" ", "\t"):
            if pending_blank is not None:
                current.append(pending_blank)
            pending_blank = None
            current.append(line)
        else:
            current = None
            pending_blank = None
    return groups


def fragment_error(path: Path, text: str) -> str | None:
    """Why a fragment cannot be assembled, or None when its bullets are fine."""
    if text.startswith("\ufeff"):
        text = text[1:]
    current = False
    bullet_seen = False
    first_content: int | None = None
    for number, line in enumerate(text.split("\n"), start=1):
        if not line.strip():
            continue
        if first_content is None:
            first_content = number
        if line.startswith("- "):
            current = True
            bullet_seen = True
            continue
        if current and line[:1] in (" ", "\t"):
            continue
        return (
            f"changelog: changelog.d/{path.name} line {number}: not a changelog bullet: "
            f"{line.strip()[:60]!r}"
        )
    if not bullet_seen:
        return (
            f"changelog: changelog.d/{path.name} line "
            f"{first_content if first_content is not None else 1}: no changelog bullet found"
        )
    return None


def heading_index(lines: list[str], heading: str) -> int | None:
    for index, line in enumerate(lines):
        if line == heading:
            return index
    return None


def next_heading(lines: list[str], index: int) -> int:
    for following in range(index + 1, len(lines)):
        if lines[following].startswith(HEADING_PREFIX):
            return following
    return len(lines)


def release_heading(lines: list[str], version: str) -> str | None:
    """The first existing heading for `version`, whatever its date or spacing follows."""
    prefix = f"## [{version}]"
    for line in lines:
        if line.startswith(prefix):
            return line
    return None


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
    parsed: list[tuple[Path, list[list[str]]]] = []
    for fragment in fragments:
        text = fragment.read_text(encoding="utf-8")
        problem = fragment_error(fragment, text)
        if problem is not None:
            print(problem, file=sys.stderr)
            return 2
        parsed.append((fragment, bullet_groups(text)))
    bullets: list[list[str]] = []
    for _fragment, groups in reversed(parsed):
        bullets.extend(groups)
    assembled = [line for bullet in bullets for line in bullet]
    changelog = root / "CHANGELOG.md"
    text = changelog.read_text(encoding="utf-8")
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if release == "Unreleased":
        index = heading_index(lines, heading)
        if index is None:
            print(f"changelog: CHANGELOG.md has no {heading} section", file=sys.stderr)
            return 2
        end = next_heading(lines, index)
        body = list(lines[index + 1 : end])
        while body and body[-1].strip() == "":
            body.pop()
        if not body or body[-1].strip() == "":
            body.append("")
        body.extend(assembled)
        if not body or body[-1].strip() != "":
            body.append("")
        lines[index + 1 : end] = body
    else:
        existing = release_heading(lines, release)
        if existing is not None:
            print(f"changelog: CHANGELOG.md already has {existing}", file=sys.stderr)
            return 2
        unopened = heading_index(lines, "## [Unreleased]")
        if unopened is None:
            print(
                "changelog: CHANGELOG.md has no ## [Unreleased] section to release after",
                file=sys.stderr,
            )
            return 2
        insert_at = next_heading(lines, unopened)
        block = [heading, ""] + assembled
        if insert_at < len(lines):
            block.append("")
        lines[insert_at:insert_at] = block
    changelog.write_text("\n".join(lines) + "\n", encoding="utf-8")
    for fragment, _groups in parsed:
        fragment.unlink()
    print(
        f"changelog: assembled {len(bullets)} bullets from {len(parsed)} fragments "
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