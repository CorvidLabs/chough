#!/usr/bin/env python3
"""Read native lifecycle evidence; never approve, verify, or finalize a change."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib

VERSION = "specsync 6.1.1"
MAX_OUTPUT = 8 * 1024 * 1024


class GateError(Exception):
    pass


def execute(argv, root, *, env=None, checked=True):
    try:
        result = subprocess.run(
            argv, cwd=root, env=env, capture_output=True, text=True, timeout=90
        )
    except (OSError, subprocess.TimeoutExpired, UnicodeError) as error:
        raise GateError(f"Inspection command unavailable: {argv[0]}: {error}") from error
    if len(result.stdout) + len(result.stderr) > MAX_OUTPUT:
        raise GateError(f"Inspection output exceeds {MAX_OUTPUT} bytes")
    if checked and result.returncode:
        detail = (result.stderr or result.stdout).strip()[-4096:]
        raise GateError(f"{' '.join(argv[:4])} failed ({result.returncode}): {detail}")
    return result


def git(root, *args):
    return execute(["git", *args], root).stdout.strip()


def native(binary, root, *args, historical=False):
    # Historical snapshots are not the PR head. Preserve full Git history but
    # remove the current Actions comparison ref from these inventory-only reads.
    env = os.environ.copy()
    if historical:
        for key in list(env):
            if key.startswith("GITHUB_") or key == "CI":
                env.pop(key)
    result = execute(
        [binary, "--root", str(root), "change", *args, "--json"],
        root,
        env=env,
        checked=False,
    )
    try:
        value = json.loads(result.stdout)
    except (ValueError, UnicodeError) as error:
        raise GateError(f"Native {' '.join(args)} did not return one JSON document") from error
    if result.returncode and not historical:
        raise GateError(
            f"Native {' '.join(args)} failed ({result.returncode}): "
            f"{(result.stderr or result.stdout).strip()[-4096:]}"
        )
    return value, result.returncode


def identifier(value):
    if (
        not isinstance(value, str)
        or not value
        or len(value.encode()) > 255
        or value in {".", ".."}
        or any(ord(char) < 32 or char in "/\\" for char in value)
    ):
        raise GateError("Native inventory has a missing or invalid change ID")
    return value


def directory_entries(path):
    if not path.exists() and not path.is_symlink():
        return []
    if path.is_symlink() or not path.is_dir():
        raise GateError(f"Lifecycle inventory is not a real directory: {path}")
    entries = sorted(path.iterdir())
    for entry in entries:
        if entry.is_symlink() or not entry.is_dir():
            raise GateError(f"Unexpected lifecycle inventory entry: {entry}")
    return entries


def archives(root):
    ids = set()
    for directory in directory_entries(root / ".specsync/archive/changes"):
        for entry in directory.rglob("*"):
            if entry.is_symlink():
                raise GateError(f"Symlink in archive package: {entry}")
        state = directory / "state.json"
        try:
            value = json.loads(state.read_text(encoding="utf-8"))
            change_id = identifier(value.get("id") if isinstance(value, dict) else None)
        except (OSError, ValueError) as error:
            raise GateError(f"Unreadable archive identity: {state}: {error}") from error
        if change_id in ids:
            raise GateError(f"Duplicate archived identity: {change_id}")
        ids.add(change_id)
    return ids


def safe_native_tree(root):
    directory = root / ".specsync"
    if directory.is_symlink() or not directory.is_dir():
        raise GateError("Required native directory is missing or unsafe")
    for path in directory.rglob("*"):
        if path.is_symlink():
            raise GateError(f"Symlink in native lifecycle inputs: {path}")


def roster(binary, root, *, historical=False):
    # Native enumeration deliberately ignores non-directory entries. Refuse
    # those here so a symlink/file cannot silently hide a damaged workspace.
    safe_native_tree(root)
    directory_entries(root / ".specsync/changes")
    value, code = native(binary, root, "list", historical=historical)
    if isinstance(value, list) and code == 0:
        rows = value
    elif historical and isinstance(value, dict) and code != 0:
        # A damaged intermediate commit can be repaired and finalized later.
        # Carry every native unreadable ID forward as a closure obligation.
        if not isinstance(value.get("changes"), list) or not isinstance(
            value.get("unreadable"), list
        ):
            raise GateError("Incomplete historical native roster")
        rows = value["changes"] + value["unreadable"]
        if not rows:
            raise GateError("Native historical roster failed without an inventory")
    else:
        raise GateError("Unknown native roster shape; cannot establish zero active changes")
    if not all(isinstance(row, dict) for row in rows):
        raise GateError("Invalid native roster row")
    ids = [identifier(row.get("id")) for row in rows]
    if len(ids) != len(set(ids)):
        raise GateError("Duplicate native roster identity")
    return set(ids)


def fingerprint(root):
    digest = hashlib.sha256()
    directory = root / ".specsync"
    if directory.exists():
        for path in sorted(directory.rglob("*")):
            digest.update(str(path.relative_to(root)).encode())
            if path.is_symlink():
                digest.update(os.readlink(path).encode())
            elif path.is_file():
                digest.update(path.read_bytes())
    return (git(root, "status", "--porcelain=v1", "--untracked-files=all"), digest.hexdigest())


def inspect(root, base, binary):
    if git(root, "rev-parse", "--show-toplevel") != str(root):
        raise GateError("Run Archive at the repository root")
    if git(root, "rev-parse", "--is-shallow-repository") != "false":
        raise GateError("Archive requires full Git history, including the trusted PR base")
    if not base or set(base) == {"0"}:
        raise GateError("A real trusted base revision is required")
    trusted_base = git(root, "rev-parse", "--verify", f"{base}^{{commit}}")
    base = git(root, "merge-base", trusted_base, "HEAD")
    for name in [".specsync/config.toml", ".specsync/sdd.json"]:
        path = root / name
        if not path.is_file() or path.is_symlink():
            raise GateError(f"Required native input missing or unsafe: {name}")
    # Chough commits the v2 policy explicitly. Reject unknown/missing schema
    # rather than relying on native deserialization defaults for omitted fields.
    policy = json.loads((root / ".specsync/sdd.json").read_text(encoding="utf-8"))
    if (
        not isinstance(policy, dict)
        or type(policy.get("version")) is not int
        or policy["version"] != 2
        or not isinstance(policy.get("enabled"), bool)
    ):
        raise GateError("Chough requires an explicit native v2 policy and boolean enabled field")
    config = tomllib.loads((root / ".specsync/config.toml").read_text(encoding="utf-8"))
    if not isinstance(config.get("specs_dir"), str) or not config["specs_dir"].strip():
        raise GateError("Required native specs_dir configuration is missing")
    report, _ = native(binary, root, "audit")
    report = report.get("report") if isinstance(report, dict) else None
    if (
        not isinstance(report, dict)
        or not isinstance(report.get("enabled"), bool)
        or type(report.get("checked_changes")) is not int
        or report["checked_changes"] < 0
        or not isinstance(report.get("errors"), list)
        or not isinstance(report.get("warnings"), list)
        or not all(isinstance(value, str) for value in report["warnings"])
        or report["errors"]
    ):
        raise GateError("Native lifecycle audit is unhealthy or has an unknown report shape")
    for warning in report["warnings"]:
        print(f"Native audit warning: {warning}")
    active = roster(binary, root)
    if active:
        raise GateError(
            "This pull request is unfinished. Active native changes: "
            + ", ".join(sorted(active))
            + ". Follow `specsync change status <ID>`; obtain real review and finalize before merge."
        )

    current_archives = archives(root)
    obligations = set(current_archives)
    revisions = [base] + git(root, "rev-list", "--reverse", f"{base}..HEAD").splitlines()
    # One temporary clone retains object/ancestry evidence while leaving the
    # checked source and its lifecycle bytes untouched. Nothing is executed from
    # historical product source; only the pinned native readers inspect it.
    with tempfile.TemporaryDirectory(prefix="chough-archive-inventory-") as temporary:
        snapshot = Path(temporary) / "history"
        git(root, "clone", "--quiet", "--shared", "--no-checkout", str(root), str(snapshot))
        for revision in dict.fromkeys(revisions):
            git(snapshot, "checkout", "--quiet", "--detach", "--force", revision)
            obligations.update(roster(binary, snapshot, historical=True))
            obligations.update(archives(snapshot))

    for change_id in sorted(obligations):
        if change_id not in current_archives:
            raise GateError(f"Native change {change_id} was omitted instead of archived")
        value, _ = native(binary, root, "status", change_id)
        change = value.get("change") if isinstance(value, dict) else None
        summary = value.get("summary") if isinstance(value, dict) else None
        if (
            not isinstance(change, dict)
            or not isinstance(summary, dict)
            or change.get("id") != change_id
            or summary.get("id") != change_id
            or change.get("state") != "archived"
            or summary.get("state") != "archived"
            or summary.get("terminal_evidence", {}).get("validity") != "authenticated-history"
        ):
            raise GateError(f"Native archive {change_id} does not authenticate: {summary}")
    print(f"All native changes are archived; {len(obligations)} archive(s) authenticated by SpecSync.")
    print("Other checks and genuine human review remain required before merge.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument("--base", required=True, help="Trusted PR base SHA or previous main SHA")
    args = parser.parse_args()
    root = args.root.resolve()
    try:
        binary = shutil.which("specsync")
        if not binary:
            raise GateError("Required published SpecSync binary is unavailable")
        if execute([binary, "--version"], root).stdout.strip() != VERSION:
            raise GateError(f"Archive requires exactly {VERSION}")
        safe_native_tree(root)
        before = fingerprint(root)
        try:
            inspect(root, args.base, binary)
        finally:
            if fingerprint(root) != before:
                raise GateError("Native inspection changed the source or lifecycle bytes")
    except (GateError, OSError, ValueError, AttributeError) as error:
        print(f"SpecSync Archive: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
