#!/usr/bin/env python3
"""Exercise Archive with real native evidence in disposable Git repositories."""

import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import threading
import unittest
from contextlib import redirect_stderr
from unittest import mock

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


class ArchiveExecutorTests(unittest.TestCase):
    def setUp(self):
        spec = importlib.util.spec_from_file_location("archive_gate_under_test", GATE)
        self.gate = importlib.util.module_from_spec(spec)
        with mock.patch.object(sys, "dont_write_bytecode", True):
            spec.loader.exec_module(self.gate)

    @staticmethod
    def response(change_id):
        return {
            "change": {"id": change_id, "state": "archived"},
            "summary": {
                "id": change_id, "state": "archived",
                "terminal_evidence": {"validity": "authenticated-history"},
            },
        }

    @staticmethod
    def start(call):
        returned = threading.Event()
        result = {}

        def invoke():
            try:
                result["value"] = call()
            except Exception as error:
                result["error"] = error
            finally:
                returned.set()

        thread = threading.Thread(target=invoke, daemon=True)
        thread.start()
        return thread, returned, result

    def test_archive_workers_follow_the_cpu_count_between_two_and_eight(self):
        for cpus, workers in [(1, 2), (3, 3), (4, 4), (64, 8), (None, 2)]:
            with mock.patch.object(self.gate.os, "cpu_count", return_value=cpus):
                self.assertEqual(self.gate.archive_workers(), workers, cpus)

    def test_one_worker_per_cpu_overlaps_and_reads_every_id_once(self):
        ids = {f"change-{number}" for number in range(6)}
        release, all_started, extra_started = (threading.Event() for _ in range(3))
        lock = threading.Lock()
        calls = []
        active = peak = 0

        def read(binary, root, action, change_id):
            nonlocal active, peak
            self.assertEqual((binary, root, action), ("specsync", Path("."), "status"))
            with lock:
                calls.append(change_id)
                active += 1
                peak = max(peak, active)
                if len(calls) == 3:
                    all_started.set()
                if len(calls) > 3:
                    extra_started.set()
            try:
                self.assertTrue(release.wait(5), "worker was not released")
                return self.response(change_id), 0
            finally:
                with lock:
                    active -= 1

        with mock.patch.object(self.gate, "native", side_effect=read), mock.patch.object(
            self.gate.os, "cpu_count", return_value=3
        ):
            thread, _, result = self.start(
                lambda: self.gate.authenticate_archives("specsync", Path("."), ids, ids)
            )
            try:
                self.assertTrue(all_started.wait(5), "three native reads never overlapped")
                self.assertFalse(extra_started.wait(0.1), "more than three native reads started")
            finally:
                release.set()
                thread.join(5)
        self.assertFalse(thread.is_alive())
        self.assertNotIn("error", result)
        self.assertEqual(sorted(calls), sorted(ids))
        self.assertEqual(peak, 3)

    def test_shards_partition_the_sorted_archives_exactly_once(self):
        ids = {f"change-{number:02}" for number in range(11)}
        for total in range(1, 6):
            seen = []
            for index in range(1, total + 1):
                part = self.gate.shard_ids(ids, (index, total))
                self.assertEqual(part, sorted(ids)[index - 1::total], (index, total))
                seen += part
            self.assertEqual(sorted(seen), sorted(ids), total)
            self.assertEqual(len(seen), len(set(seen)), total)

    def test_shards_partition_the_pr_revisions_exactly_once_in_order(self):
        # #105: the history walk is split like authentication, so no single shard
        # inventories every revision. Repeated revisions are walked once.
        revisions = [f"rev-{number:02}" for number in range(10)]
        for total in range(1, 6):
            seen = []
            for index in range(1, total + 1):
                part = self.gate.shard_revisions(revisions + revisions[:3], (index, total))
                self.assertEqual(part, revisions[index - 1::total], (index, total))
                seen += part
            self.assertEqual(sorted(seen), revisions, total)
            self.assertEqual(len(seen), len(set(seen)), total)

    def test_shard_must_be_k_of_n_within_the_bound(self):
        for value, shard in [("1/1", (1, 1)), ("3/4", (3, 4)), ("8/8", (8, 8))]:
            self.assertEqual(self.gate.parse_shard(value), shard)
        for value in ["", "0/4", "5/4", "1/0", "a/b", "1/4/2", " 1/4", "-1/4", "1/9", "01/4"]:
            with self.subTest(value=value), self.assertRaises(self.gate.GateError):
                self.gate.parse_shard(value)

    def test_a_shard_authenticates_only_its_own_archives(self):
        ids = {f"change-{number}" for number in range(7)}
        calls = []

        def read(binary, root, action, change_id):
            calls.append(change_id)
            return self.response(change_id), 0

        with mock.patch.object(self.gate, "native", side_effect=read):
            self.gate.authenticate_archives("specsync", Path("."), ids, ids, (2, 3))
        self.assertEqual(sorted(calls), sorted(ids)[1::3])
        with mock.patch.object(self.gate, "native", side_effect=read), self.assertRaises(
            self.gate.GateError
        ):
            self.gate.authenticate_archives("specsync", Path("."), ids | {"omitted"}, ids, (3, 3))

    def test_errors_are_reported_in_id_order_not_completion_order(self):
        release, beta_failed = threading.Event(), threading.Event()
        calls = []

        def read(binary, root, action, change_id):
            calls.append(change_id)
            if change_id == "alpha":
                self.assertTrue(release.wait(5))
                raise self.gate.GateError("alpha failed")
            if change_id == "beta":
                beta_failed.set()
                raise self.gate.GateError("beta failed first")
            return self.response(change_id), 0

        ids = {"gamma", "beta", "alpha"}
        with mock.patch.object(self.gate, "native", side_effect=read):
            thread, returned, result = self.start(
                lambda: self.gate.authenticate_archives("specsync", Path("."), ids, ids)
            )
            try:
                self.assertTrue(beta_failed.wait(5))
                self.assertFalse(returned.wait(0.1))
            finally:
                release.set()
                thread.join(5)
        self.assertFalse(thread.is_alive())
        self.assertIsInstance(result.get("error"), self.gate.GateError)
        self.assertEqual(str(result["error"]), "alpha failed")
        self.assertEqual(sorted(calls), sorted(ids))

    def test_invalid_archive_results_block(self):
        invalid = [None, [], {}, {"change": [], "summary": {}},
                   {"change": {}, "summary": []}]
        for section, key, value in [
            ("change", "id", "wrong"), ("summary", "id", "wrong"),
            ("change", "state", "accepted"), ("summary", "state", "accepted"),
            ("summary", "terminal_evidence", {}),
            ("summary", "terminal_evidence", {"validity": "exact"}),
            ("summary", "terminal_evidence", {"validity": "successor-covered"}),
        ]:
            result = self.response("alpha")
            result[section][key] = value
            invalid.append(result)
        for result in invalid:
            with self.subTest(result=result):
                with mock.patch.object(self.gate, "native", return_value=(result, 0)) as read:
                    with self.assertRaises(self.gate.GateError):
                        self.gate.authenticate_archives("specsync", Path("."), {"alpha"}, {"alpha"})
                    read.assert_called_once_with("specsync", Path("."), "status", "alpha")

    def test_native_errors_timeouts_and_output_bound_block(self):
        valid = json.dumps(self.response("alpha"))
        oversized = valid + " " * (8 * 1024 * 1024 + 1 - len(valid))
        results = [
            subprocess.CompletedProcess([], 0, "{}{}", ""),
            subprocess.CompletedProcess([], 7, valid, "native refused"),
            subprocess.TimeoutExpired("specsync", 90),
            OSError("native unavailable"),
            subprocess.CompletedProcess([], 0, oversized, ""),
        ]
        for number, result in enumerate(results):
            with self.subTest(result=number):
                kwargs = {"side_effect": result} if isinstance(result, Exception) else {"return_value": result}
                with mock.patch.object(self.gate.subprocess, "run", **kwargs) as run:
                    with self.assertRaises(self.gate.GateError) as error:
                        self.gate.authenticate_archives("specsync", Path("."), {"alpha"}, {"alpha"})
                    if number == 4:
                        self.assertEqual(str(error.exception), "Inspection output exceeds 8388608 bytes")
                    self.assertEqual(run.call_count, 1)
                    self.assertEqual(run.call_args.kwargs["timeout"], 90)

    def test_failure_drains_running_and_queued_reads_before_final_fingerprint(self):
        ids = {"alpha", "beta", "gamma", "omega"}
        release, beta_started, alpha_failed = (threading.Event() for _ in range(3))
        completed = []
        fingerprints = []

        def read(binary, root, action, change_id):
            try:
                if change_id == "alpha":
                    self.assertTrue(beta_started.wait(5))
                    alpha_failed.set()
                    raise self.gate.GateError("alpha refused")
                if change_id == "beta":
                    beta_started.set()
                    self.assertTrue(release.wait(5))
                return self.response(change_id), 0
            finally:
                completed.append(change_id)

        def fingerprint(root):
            if fingerprints:
                self.assertEqual(sorted(completed), sorted(ids), "fingerprinted before reads drained")
            fingerprints.append(root)
            return "clean", "unchanged"

        with (
            mock.patch.object(self.gate, "native", side_effect=read),
            mock.patch.object(self.gate, "inspect", side_effect=lambda root, base, binary:
                              self.gate.authenticate_archives(binary, root, ids, ids)),
            mock.patch.object(self.gate, "fingerprint", side_effect=fingerprint),
            mock.patch.object(self.gate, "safe_native_tree"),
            mock.patch.object(self.gate.shutil, "which", return_value="specsync"),
            mock.patch.object(self.gate, "execute", return_value=subprocess.CompletedProcess(
                [], 0, self.gate.VERSION + "\n", "")),
            mock.patch.object(sys, "argv", [str(GATE), "--base", "fixture-base"]),
            redirect_stderr(io.StringIO()),
        ):
            thread, returned, result = self.start(self.gate.main)
            try:
                self.assertTrue(alpha_failed.wait(5))
                self.assertFalse(returned.wait(0.1), "returned while a native read was running")
                self.assertEqual(len(fingerprints), 1)
            finally:
                release.set()
                thread.join(5)
        self.assertFalse(thread.is_alive())
        self.assertNotIn("error", result)
        self.assertEqual(result.get("value"), 1)
        self.assertEqual(sorted(completed), sorted(ids))
        self.assertEqual(len(fingerprints), 2)

    def test_missing_archive_blocks_before_native_reads(self):
        with mock.patch.object(self.gate, "native") as read:
            with self.assertRaisesRegex(self.gate.GateError, "omitted instead of archived"):
                self.gate.authenticate_archives("specsync", Path("."), {"alpha", "missing"}, {"alpha"})
            read.assert_not_called()


class ArchiveFixtures(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        if command(Path.cwd(), "specsync", "--version") != "specsync 6.3.0":
            raise AssertionError("Fixtures require real published SpecSync 6.3.0")
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

    def check(self, root, succeeds, *, base=None, env=None, contains=None, shard=None):
        before = native_hash(root)
        status = command(root, "git", "status", "--porcelain=v1", "--untracked-files=all")
        result = subprocess.run(
            [sys.executable, str(GATE), "--root", str(root), "--base", base or self.base]
            + (["--shard", shard] if shard else []),
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
        # An archive authenticates its historical inputs. Later legitimate
        # commits must not re-run old behavioral evidence against today's tree.
        root = self.fixture(self.archived)
        (root / "README.md").write_text("Later legitimate fixture revision after native archive.\n")
        (root / "unrelated.txt").write_text("Unrelated later fixture content.\n")
        commit(root, "legitimate later fixture content")
        self.check(root, True, contains="1 archive(s) authenticated")

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

    def test_shards_split_authentication_and_the_history_walk(self):
        total = len(command(self.archived, "git", "rev-list", f"{self.base}..HEAD").splitlines()) + 1
        first, second = (total + 1) // 2, total // 2
        self.check(self.archived, True, shard="1/2", contains=(
            f"Shard 1/2: walked {first} of {total} PR revision(s); 1 of 1 archive(s) authenticated"))
        self.check(self.archived, True, shard="2/2", contains=(
            f"Shard 2/2: walked {second} of {total} PR revision(s); 0 of 1 archive(s) authenticated"))
        for shard in ["1/2", "2/2"]:
            with self.subTest(shard=shard):
                self.check(self.draft, False, shard=shard, contains="unfinished")
        root = self.fixture(self.archived)
        shutil.rmtree(root / ".specsync/archive")
        commit(root, "synthetic archive incorrectly removed")
        for shard in ["1/2", "2/2"]:
            with self.subTest(shard=shard):
                self.check(root, False, shard=shard, contains="omitted instead of archived")
        self.check(self.clean, False, shard="3/2", contains="--shard")

    def test_the_shard_that_walks_a_revision_refuses_a_change_omitted_from_it(self):
        # #105: revisions are base, introduced, removed. Shard 2/2 walks only the
        # revision where the draft exists, so it alone must find the omission.
        root = self.fixture(self.draft)
        commit(root, "synthetic draft scope introduced")
        shutil.rmtree(root / ".specsync/changes")
        commit(root, "synthetic scope incorrectly removed")
        self.check(root, False, contains="omitted instead of archived")
        self.check(root, False, shard="2/2", contains="omitted instead of archived")
        self.check(root, True, shard="1/2", contains="Shard 1/2: walked 2 of 3 PR revision(s)")

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
                    version = "specsync 0.0.0" if output == "wrong-version" else "specsync 6.3.0"
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
