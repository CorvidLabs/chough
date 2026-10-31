#!/usr/bin/env python3
"""Exercise Archive with real native evidence in disposable Git repositories."""

import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

GATE = Path(__file__).with_name("check-specsync-archive.py")
ENV = {
    key: value
    for key, value in os.environ.items()
    if not key.startswith("GITHUB_") and key not in {"CI", "SPECSYNC_VERIFY_DEPTH"}
}


def command(root, *args):
    result = subprocess.run(args, cwd=root, env=ENV, capture_output=True, text=True, timeout=90)
    if result.returncode:
        raise AssertionError(f"{args}: {result.stdout}\n{result.stderr}")
    return result.stdout.strip()


def commit(root, message):
    command(root, "git", "add", "--all")
    command(root, "git", "commit", "--quiet", "-m", message)


def native(root, *args):
    return json.loads(command(root, "specsync", "change", *args, "--json"))


def native_hash(root):
    digest = hashlib.sha256()
    for path in sorted((root / ".specsync").rglob("*")):
        digest.update(str(path.relative_to(root)).encode())
        if path.is_symlink():
            digest.update(os.readlink(path).encode())
        elif path.is_file():
            digest.update(path.read_bytes())
    return digest.hexdigest()


class ArchiveFixtures(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        if command(Path.cwd(), "specsync", "--version") != "specsync 6.1.1":
            raise AssertionError("Fixtures require real published SpecSync 6.1.1")
        cls.temporary = tempfile.TemporaryDirectory(prefix="chough-ci-archive-fixtures-")
        cls.directory = Path(cls.temporary.name)
        cls.clean = cls.directory / "clean"
        cls.clean.mkdir()
        command(cls.clean, "git", "init", "--quiet", "-b", "main")
        cls.configure(cls.clean)
        (cls.clean / ".specsync").mkdir()
        (cls.clean / "specs").mkdir()
        (cls.clean / "README.md").write_text("Synthetic fixture only; never human approval.\n")
        (cls.clean / ".specsync/config.toml").write_text('specs_dir = "specs"\nsource_dirs = []\n')
        (cls.clean / ".specsync/sdd.json").write_text(
            json.dumps({"version": 2, "enabled": True, "require_change_for_meaningful_files": False,
                        "verification_commands": []})
        )
        commit(cls.clean, "synthetic fixture base")
        cls.base = command(cls.clean, "git", "rev-parse", "HEAD")
        cls.archived = cls.copy(cls.clean, "archived")
        value = native(
            cls.archived, "new", "Synthetic fixture workflow", "--kind", "documentation",
            "--path", "README.md", "--no-spec-change", "--rationale",
            "Synthetic fixture prose only; no product contract changes or real human identity."
        )
        cls.change_id = value["change"]["id"]
        for question, answer in [
            ("acceptance_criteria", "The synthetic fixture prose is readable without product changes."),
            ("public_contract", "no"), ("architecture_risk", "no"),
        ]:
            native(cls.archived, "answer", cls.change_id, question, answer)
        folder = cls.archived / ".specsync/changes" / cls.change_id
        for artifact in json.loads((folder / "state.json").read_text())["selected_artifacts"]:
            (folder / f"{artifact}.md").write_text(
                f"# Synthetic fixture {artifact}\n\n## Approach\n\n"
                "Keep a synthetic prose edit isolated from Chough and inspect it with native readers.\n\n"
                "## Out of scope\n\nReal product behavior, identities and human approvals.\n\n"
                "## Steps\n\n1. Update synthetic prose.\n2. Verify native fixture consistency.\n\n"
                "## Risks\n\nFixture actors are unauthenticated synthetic claims.\n\n"
                "## Constraints consulted\n\nNo project lessons exist in this isolated fixture.\n"
            )
        cls.draft = cls.copy(cls.archived, "draft")
        # These mutations construct TEST DATA in a disposable repo. They are
        # never invoked by the inspection gate or against real Chough records.
        native(cls.archived, "approve", cls.change_id, "--actor", "synthetic-fixture-only")
        (cls.archived / "README.md").write_text("Synthetic fixture update, never human approval.\n")
        commit(cls.archived, "synthetic fixture implementation")
        # Writer commands may emit commit progress as well as JSON. The gate
        # itself uses only the strict reader protocol, never these writers.
        command(cls.archived, "specsync", "change", "check", cls.change_id, "--commit")
        command(cls.archived, "specsync", "change", "review", cls.change_id,
                "--reviewer", "synthetic-fixture-only")
        cls.verifying = cls.copy(cls.archived, "verifying")
        command(cls.archived, "specsync", "change", "finalize", cls.change_id)
        status = native(cls.archived, "status", cls.change_id)
        if status["summary"]["terminal_evidence"]["validity"] != "authenticated-history":
            raise AssertionError("Positive archive fixture did not authenticate natively")

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    @staticmethod
    def configure(root):
        command(root, "git", "config", "user.email", "fixture@example.invalid")
        command(root, "git", "config", "user.name", "Synthetic CI Fixture")
        command(root, "git", "config", "commit.gpgsign", "false")
        command(root, "git", "config", "core.hooksPath", "/dev/null")

    @classmethod
    def copy(cls, source, name):
        root = cls.directory / name
        shutil.copytree(source, root)
        cls.configure(root)
        return root

    def fixture(self, source):
        return self.copy(source, f"case-{self._testMethodName}-{len(list(self.directory.iterdir()))}")

    def check(self, root, succeeds, *, base=None, env=None, contains=None):
        before = native_hash(root)
        status = command(root, "git", "status", "--porcelain=v1", "--untracked-files=all")
        result = subprocess.run(
            [sys.executable, str(GATE), "--root", str(root), "--base", base or self.base],
            env=ENV if env is None else env, capture_output=True, text=True, timeout=180,
        )
        self.assertEqual(result.returncode == 0, succeeds, result.stdout + result.stderr)
        if contains:
            self.assertIn(contains, result.stdout + result.stderr)
        self.assertEqual(native_hash(root), before, "gate changed native fixture bytes")
        self.assertEqual(command(root, "git", "status", "--porcelain=v1", "--untracked-files=all"), status)

    def test_no_active_and_authenticated_archive_pass(self):
        self.check(self.clean, True)
        self.check(self.archived, True, contains="1 archive(s) authenticated")
        # Main may advance after a branch forks. Its later-only archives will
        # merge in normally; absence on the older branch is not scope deletion.
        root = self.fixture(self.clean)
        command(root, "git", "fetch", "--quiet", "--no-write-fetch-head", str(self.archived), "HEAD")
        self.check(root, True, base=command(self.archived, "git", "rev-parse", "HEAD"))

    def test_every_active_stage_and_scope_only_draft_fails(self):
        for state in ["draft", "approved", "implementing", "verifying", "accepted", "archived"]:
            with self.subTest(state=state):
                root = self.fixture(self.draft)
                path = root / ".specsync/changes" / self.change_id / "state.json"
                value = json.loads(path.read_text())
                value["state"] = state
                path.write_text(json.dumps(value))
                self.check(root, False)

    def test_disabled_policy_cannot_hide_a_draft(self):
        root = self.fixture(self.clean)
        path = root / ".specsync/sdd.json"
        value = json.loads(path.read_text())
        value["enabled"] = False
        path.write_text(json.dumps(value))
        # Make disabled policy the trusted fixture baseline too, so rejection
        # demonstrates active inventory, not a policy-change audit failure.
        commit(root, "synthetic disabled policy baseline")
        base = command(root, "git", "rev-parse", "HEAD")
        native(root, "new", "Disabled policy scope", "--kind", "documentation", "--path", "README.md",
               "--no-spec-change", "--rationale", "Synthetic draft scope with no implementation.")
        self.check(root, False, base=base, contains="Active native changes")

    def test_malformed_missing_and_wrong_type_policy_fail(self):
        for content in [None, "{", '{"enabled":"yes"}', "[]", '{"enabled":false}',
                        '{"version":999,"enabled":false}', '{"version":2}']:
            with self.subTest(content=content):
                root = self.fixture(self.clean)
                path = root / ".specsync/sdd.json"
                if content is None:
                    path.unlink()
                else:
                    path.write_text(content)
                self.check(root, False)
        root = self.fixture(self.clean)
        (root / ".specsync/config.toml").write_text("not [valid TOML")
        self.check(root, False)

    def test_malformed_missing_and_incomplete_package_fail(self):
        for mode in ["missing", "json", "unknown-state", "missing-artifact"]:
            with self.subTest(mode=mode):
                root = self.fixture(self.draft)
                folder = root / ".specsync/changes" / self.change_id
                if mode == "missing":
                    (folder / "state.json").unlink()
                elif mode == "json":
                    (folder / "state.json").write_text("{")
                elif mode == "unknown-state":
                    value = json.loads((folder / "state.json").read_text())
                    value["state"] = "finished"
                    (folder / "state.json").write_text(json.dumps(value))
                else:
                    artifact = json.loads((folder / "state.json").read_text())["selected_artifacts"][0]
                    (folder / f"{artifact}.md").unlink()
                self.check(root, False)

    def test_archived_label_without_native_terminal_evidence_fails(self):
        root = self.fixture(self.draft)
        source = root / ".specsync/changes" / self.change_id
        target = root / ".specsync/archive/changes" / self.change_id
        target.parent.mkdir(parents=True)
        source.rename(target)
        value = json.loads((target / "state.json").read_text())
        value["state"] = "archived"
        (target / "state.json").write_text(json.dumps(value))
        self.check(root, False)

    def test_corrupt_archive_history_and_missing_evidence_fail(self):
        for name in ["state.json", "verification.json", "review.json", "approvals.json"]:
            with self.subTest(name=name):
                root = self.fixture(self.archived)
                folder = next((root / ".specsync/archive/changes").iterdir())
                (folder / name).unlink()
                self.check(root, False)
        root = self.fixture(self.archived)
        folder = next((root / ".specsync/archive/changes").iterdir())
        (folder / "context.md").write_text("tampered synthetic historical context\n")
        self.check(root, False, contains="does not authenticate")

    def test_archived_sibling_cannot_hide_draft(self):
        root = self.fixture(self.archived)
        native(root, "new", "Another isolated scope", "--kind", "documentation", "--path", "README.md",
               "--no-spec-change", "--rationale", "Synthetic fixture scope only; no implementation.")
        self.check(root, False)

    def test_real_current_verification_becomes_stale(self):
        root = self.fixture(self.verifying)
        (root / "README.md").write_text("Synthetic input changed after verification and review.\n")
        self.check(root, False)

    def test_removed_new_draft_and_removed_archive_cannot_disappear(self):
        root = self.fixture(self.draft)
        commit(root, "synthetic draft scope introduced")
        shutil.rmtree(root / ".specsync/changes")
        commit(root, "synthetic scope incorrectly removed")
        self.check(root, False, contains="omitted instead of archived")
        root = self.fixture(self.archived)
        shutil.rmtree(root / ".specsync/archive")
        commit(root, "synthetic archive incorrectly removed")
        self.check(root, False, contains="omitted instead of archived")

    def test_bad_base_shallow_and_unsafe_inventory_fail(self):
        self.check(self.clean, False, base="0" * 40)
        self.check(self.clean, False, base="f" * 40)
        root = self.fixture(self.clean)
        (root / ".git/shallow").write_text(command(root, "git", "rev-parse", "HEAD") + "\n")
        self.check(root, False, contains="full Git history")
        for mode in ["file", "symlink"]:
            root = self.fixture(self.clean)
            folder = root / ".specsync/changes"
            folder.mkdir()
            if mode == "file":
                (folder / "damaged").write_text("not a native workspace\n")
            else:
                (folder / "damaged").symlink_to(root / "README.md")
            self.check(root, False, contains="Unexpected lifecycle inventory" if mode == "file" else "Symlink")

    def test_missing_wrong_version_and_broken_native_protocol_fail(self):
        for output in [None, "wrong-version", "native-error", "", "{}{}", "[]", '{"report":null}']:
            with self.subTest(output=output):
                root = self.fixture(self.clean)
                binary_dir = root / "fake-negative-tools"
                binary_dir.mkdir()
                (binary_dir / "git").symlink_to(shutil.which("git"))
                if output is not None:
                    script = binary_dir / "specsync"
                    version = "specsync 0.0.0" if output == "wrong-version" else "specsync 6.1.1"
                    script.write_text(
                        "#!/bin/sh\n"
                        f"if [ \"$1\" = --version ]; then printf '%s\\n' '{version}'; exit 0; fi\n"
                        f"printf '%s\\n' '{output}'\n"
                        + ("exit 7\n" if output == "native-error" else "")
                    )
                    script.chmod(0o755)
                env = {**ENV, "PATH": str(binary_dir)}
                self.check(root, False, env=env)


if __name__ == "__main__":
    unittest.main(verbosity=2)
