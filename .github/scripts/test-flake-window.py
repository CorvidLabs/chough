#!/usr/bin/env python3
"""Fixtures for the 1.0 flake window check (REQ-cli-062).

Each test builds CI runs, jobs, `flake` issues and bug records in code and checks the
receipt `flake-window.py` computes. The fetch is exercised through a fake `gh` on
PATH, so nothing here touches the network or commits.
"""

from datetime import datetime, timedelta, timezone
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import tomllib
import unittest
from contextlib import redirect_stdout
from unittest import mock

SCRIPT = Path(__file__).with_name("flake-window.py")
ROOT = Path(__file__).resolve().parents[2]
UTC = timezone.utc
NOW = datetime(2026, 11, 20, 12, 0, tzinfo=UTC)
DAY = timedelta(days=1)
LINUX = "Product / ubuntu-latest"
MACOS = "Product / macos-latest"


def load_script():
    spec = importlib.util.spec_from_file_location("flake_window_under_test", SCRIPT)
    module = importlib.util.module_from_spec(spec)
    with mock.patch.object(sys, "dont_write_bytecode", True):
        spec.loader.exec_module(module)
    return module


def iso(moment):
    return moment.strftime("%Y-%m-%dT%H:%M:%SZ")


def morning(days_ago):
    """06:00 UTC on the day `days_ago` days before NOW."""
    day = (NOW - days_ago * DAY).date()
    return datetime(day.year, day.month, day.day, 6, 0, tzinfo=UTC)


class World:
    """CI runs, jobs, flake issues and bug records for one computation."""

    def __init__(self):
        self.runs = []
        self.jobs = {}
        self.issues = []
        self.bugs = []
        self.next_id = 1000

    def _run(self, branch, event, when, conclusion, attempt, started):
        self.next_id += 1
        run = {
            "id": self.next_id,
            "head_branch": branch,
            "event": event,
            "status": "completed",
            "conclusion": conclusion,
            "run_attempt": attempt,
            "created_at": iso(when),
            "run_started_at": iso(started or when),
            "html_url": f"https://github.com/CorvidLabs/chough/actions/runs/{self.next_id}",
        }
        self.runs.append(run)
        return run

    def main_run(self, when, conclusion="success", linux="success", macos="success",
                 attempt=1, event="push", jobs=None):
        run = self._run("main", event, when, conclusion, attempt, None)
        if jobs is None:
            jobs = [("Archive", "success"), (LINUX, linux), (MACOS, macos)]
        self.jobs[str(run["id"])] = [
            {"name": name, "status": "completed", "conclusion": result} for name, result in jobs
        ]
        return run

    def pr_run(self, when, attempt=1, started=None, conclusion="failure"):
        return self._run("leif/some-change", "pull_request", when, conclusion, attempt, started)

    def green_days(self, days=15, skip=()):
        for days_ago in range(days):
            if days_ago not in skip:
                self.main_run(morning(days_ago))

    def flake_issue(self, number, created, closed=None):
        self.issues.append({
            "number": number,
            "title": f"flake {number}",
            "state": "closed" if closed else "open",
            "created_at": iso(created),
            "closed_at": iso(closed) if closed else None,
            "html_url": f"https://github.com/CorvidLabs/chough/issues/{number}",
        })

    def bug(self, bug_id, created, status="verified", text=""):
        self.bugs.append({"id": bug_id, "status": status, "created": iso(created), "text": text})

    def inputs(self):
        return {"runs": self.runs, "jobs": self.jobs, "issues": self.issues}


class WindowCase(unittest.TestCase):
    def setUp(self):
        self.script = load_script()
        self.world = World()

    def compute(self):
        return self.script.compute(self.world.inputs(), self.world.bugs, NOW)

    def assert_running(self, receipt, kind, at):
        self.assertEqual(receipt["verdict"], "running", receipt)
        self.assertEqual(receipt["latest_restart"]["kind"], kind, receipt)
        self.assertEqual(receipt["latest_restart"]["at"], iso(at), receipt)
        self.assertEqual(receipt["clean_from"], iso(at + 14 * DAY), receipt)


class CleanWindowTests(WindowCase):
    def test_fourteen_green_days_without_reruns_or_flakes_are_clean(self):
        self.world.green_days(15)
        receipt = self.compute()
        self.assertEqual(receipt["verdict"], "clean", receipt)
        self.assertEqual(receipt["blocked_by"], [])
        self.assertIsNone(receipt["clean_from"])
        self.assertEqual(receipt["green_main_runs"], {"linux": 15, "macos": 15})
        self.assertEqual(receipt["schema"], "chough-flake-window/1")
        self.assertEqual(receipt["required_days"], 14)

    def test_a_restart_older_than_fourteen_days_reads_clean(self):
        self.world.main_run(morning(20), conclusion="failure", macos="failure")
        self.world.pr_run(morning(25), attempt=2, started=morning(18))
        self.world.green_days(15)
        receipt = self.compute()
        self.assertEqual(receipt["verdict"], "clean", receipt)
        self.assertEqual(receipt["latest_restart"]["kind"], "rerun")
        self.assertEqual(receipt["latest_restart"]["at"], iso(morning(18)))

    def test_a_closed_flake_with_its_record_and_a_green_run_since_reads_clean(self):
        self.world.flake_issue(187, created=morning(30), closed=morning(16))
        self.world.bug("2026-10-21-kill-zero", created=morning(30),
                       text="criteria: [DRIVE-19]\nTracked by the flake window; GitHub #187")
        self.world.green_days(15)
        receipt = self.compute()
        self.assertEqual(receipt["verdict"], "clean", receipt)
        self.assertEqual([flake["number"] for flake in receipt["flakes"]["closed"]], [187])


class RestartTests(WindowCase):
    def test_a_red_main_run_restarts_the_window(self):
        self.world.green_days(15)
        red = morning(3) + timedelta(hours=2)
        self.world.main_run(red, conclusion="failure", linux="failure")
        self.assert_running(self.compute(), "red-main-run", red)

    def test_a_main_run_with_a_skipped_job_restarts_the_window(self):
        self.world.green_days(15)
        skipped = morning(2) + timedelta(hours=1)
        self.world.main_run(skipped, macos="skipped")
        self.assert_running(self.compute(), "red-main-run", skipped)

    def test_a_rerun_on_a_pull_request_branch_restarts_the_window(self):
        self.world.green_days(15)
        rerun = morning(4) + timedelta(hours=3)
        self.world.pr_run(morning(10), attempt=2, started=rerun, conclusion="success")
        self.assert_running(self.compute(), "rerun", rerun)

    def test_a_newly_filed_flake_issue_restarts_the_window(self):
        self.world.green_days(15)
        filed = morning(5) + timedelta(hours=1)
        self.world.flake_issue(301, created=filed, closed=morning(4) + timedelta(hours=1))
        self.world.bug("2026-11-15-new-flake", created=morning(5),
                       text="Tracked by the flake window (DRIVE-19); GitHub #301")
        self.assert_running(self.compute(), "flake-filed", filed)

    def test_a_new_bug_record_naming_drive_19_restarts_the_window(self):
        self.world.green_days(15)
        created = morning(6) + timedelta(hours=4)
        self.world.bug("2026-11-14-local-flake", created=created, text="criteria: [DRIVE-19]")
        self.assert_running(self.compute(), "flake-filed", created)

    def test_a_day_with_no_main_run_restarts_the_window_at_its_end(self):
        self.world.green_days(15, skip=(3,))
        end_of_gap = morning(3).replace(hour=0) + DAY
        receipt = self.compute()
        self.assert_running(receipt, "no-green-run", end_of_gap)
        self.assertIn("ubuntu-latest", receipt["latest_restart"]["detail"])
        self.assertIn("macos-latest", receipt["latest_restart"]["detail"])

    def test_a_day_without_a_green_macos_job_restarts_the_window(self):
        self.world.green_days(15, skip=(2,))
        self.world.main_run(morning(2), jobs=[("Archive", "success"), (LINUX, "success")])
        end_of_gap = morning(2).replace(hour=0) + DAY
        receipt = self.compute()
        self.assert_running(receipt, "no-green-run", end_of_gap)
        self.assertIn("macos-latest", receipt["latest_restart"]["detail"])
        self.assertNotIn("ubuntu-latest", receipt["latest_restart"]["detail"])

    def test_two_runs_that_each_pass_only_one_product_job_do_not_cover_their_day(self):
        # t384's probe on #252: a day counts only when one run passed both jobs.
        self.world.green_days(15, skip=(3,))
        self.world.main_run(morning(3), conclusion="failure", macos="failure")
        self.world.main_run(morning(3) + timedelta(hours=2), conclusion="failure", linux="failure")
        end_of_gap = morning(3).replace(hour=0) + DAY
        receipt = self.compute()
        self.assert_running(receipt, "no-green-run", end_of_gap)


class BlockedTests(WindowCase):
    def setUp(self):
        super().setUp()
        self.world.green_days(15)

    def assert_blocked(self, fragment):
        receipt = self.compute()
        self.assertEqual(receipt["verdict"], "blocked", receipt)
        self.assertTrue(any(fragment in reason for reason in receipt["blocked_by"]), receipt)
        self.assertIsNone(receipt["clean_from"])
        return receipt

    def test_an_open_flake_issue_blocks_the_window(self):
        self.world.flake_issue(132, created=morning(40))
        self.world.bug("2026-10-11-fd-inheritance", created=morning(40),
                       text="DRIVE-19; GitHub #132")
        receipt = self.assert_blocked("#132 is open")
        self.assertEqual([flake["number"] for flake in receipt["flakes"]["open"]], [132])

    def test_an_open_bug_record_naming_drive_19_blocks_the_window(self):
        self.world.bug("2026-10-01-open-flake", created=morning(40), status="open",
                       text="criteria: [DRIVE-19]")
        self.assert_blocked("2026-10-01-open-flake")

    def test_a_deferred_bug_record_naming_drive_19_blocks_the_window(self):
        self.world.bug("2026-10-01-deferred-flake", created=morning(40), status="deferred",
                       text="criteria: [DRIVE-19]")
        self.assert_blocked("2026-10-01-deferred-flake")

    def test_a_closed_flake_with_no_green_main_run_since_blocks_the_window(self):
        self.world.flake_issue(240, created=morning(40), closed=NOW - timedelta(hours=2))
        self.world.bug("2026-10-11-closed-today", created=morning(40), text="DRIVE-19 GitHub #240")
        self.assert_blocked("#240 has had no green main run since it closed")

    def test_a_flake_issue_without_a_bug_record_blocks_the_window(self):
        self.world.flake_issue(250, created=morning(40), closed=morning(30))
        self.world.bug("2026-10-11-loose-mention", created=morning(40),
                       text="DRIVE-19; mentioned as #250 only, not as its GitHub issue")
        self.assert_blocked("#250 has no bug record naming GitHub #250")


FAKE_GH = r'''#!/usr/bin/env python3
"""A stand-in for `gh api`: serves runs, jobs and issues from FAKE_GH_SPEC."""
import json, os, sys
spec = json.load(open(os.environ["FAKE_GH_SPEC"]))
args = sys.argv[1:]
if spec.get("fail") or args[:1] != ["api"]:
    sys.stderr.write("HTTP 502: Bad Gateway\n")
    sys.exit(1)
path = next(arg for arg in args[1:] if not arg.startswith("-") and arg != "GET")
params = dict(args[i + 1].split("=", 1) for i, arg in enumerate(args) if arg == "-f")
with open(os.environ["FAKE_GH_LOG"], "a") as log:
    log.write(json.dumps({"path": path, "params": params}) + "\n")
page, per_page = int(params.get("page", "1")), int(params.get("per_page", "30"))
def window(items):
    return items[(page - 1) * per_page:page * per_page]
if path.endswith("/actions/workflows/ci.yml/runs"):
    low, high = params["created"].split("..")
    runs = [run for run in spec["runs"] if low <= run["created_at"] <= high]
    total = len(runs) + spec.get("lie_total", 0)
    print(json.dumps({"total_count": total, "workflow_runs": window(runs)}))
elif "/actions/runs/" in path and path.endswith("/jobs"):
    jobs = spec["jobs"].get(path.split("/")[-2], [])
    print(json.dumps({"total_count": len(jobs), "jobs": window(jobs)}))
elif path.endswith("/issues"):
    print(json.dumps(window(spec["issues"])))
else:
    sys.stderr.write("HTTP 404: Not Found\n")
    sys.exit(1)
'''


def tree_digest(root):
    digest = hashlib.sha256()
    for path in sorted(root.rglob("*")):
        digest.update(str(path.relative_to(root)).encode())
        if path.is_file():
            digest.update(path.read_bytes())
    return digest.hexdigest()


class FailClosedTests(WindowCase):
    def setUp(self):
        super().setUp()
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name) / "project"
        bugs = self.root / ".chough" / "bugs"
        bugs.mkdir(parents=True)
        (bugs / "2026-10-11-fd-inheritance.md").write_text(
            "---\nkind: bug\nid: 2026-10-11-fd-inheritance\nstatus: verified\n"
            "criteria: [DRIVE-19]\ncreated: 2026-10-11T04:00:00Z\n---\n"
            "# fd inheritance\n\n## Log\n\n- note: Tracked by the flake window; GitHub #132\n"
        )
        self.world.flake_issue(132, created=morning(40), closed=morning(30))

    def tearDown(self):
        self.temp.cleanup()

    def run_main(self, *args, env=None):
        stdout, stderr = io.StringIO(), io.StringIO()
        with redirect_stdout(stdout), mock.patch.object(sys, "stderr", stderr), \
                mock.patch.dict(os.environ, env or {}):
            code = self.script.main(["--root", str(self.root), "--now", iso(NOW), *args])
        return code, stdout.getvalue(), stderr.getvalue()

    def saved_input(self):
        path = Path(self.temp.name) / "input.json"
        path.write_text(json.dumps({"schema": "chough-flake-window-input/1", **self.world.inputs()}))
        return path

    def fake_gh(self, **spec):
        bin_dir = Path(self.temp.name) / "bin"
        bin_dir.mkdir(exist_ok=True)
        gh = bin_dir / "gh"
        with open(gh, "w") as handle:
            handle.write(FAKE_GH)
        gh.chmod(0o755)
        spec_path = Path(self.temp.name) / "spec.json"
        spec_path.write_text(json.dumps({**self.world.inputs(), **spec}))
        log = Path(self.temp.name) / "gh.log"
        return {
            "PATH": f"{bin_dir}{os.pathsep}{os.environ.get('PATH', '')}",
            "FAKE_GH_SPEC": str(spec_path),
            "FAKE_GH_LOG": str(log),
        }, log

    def test_a_recent_main_run_without_its_jobs_exits_2_with_no_verdict(self):
        self.world.green_days(15)
        del self.world.jobs[str(self.world.runs[0]["id"])]
        code, stdout, stderr = self.run_main("--input", str(self.saved_input()))
        self.assertEqual(code, 2, stderr)
        self.assertNotIn('"verdict"', stdout)
        self.assertIn("jobs", stderr)

    def test_a_failed_github_read_exits_2_with_no_verdict(self):
        env, _ = self.fake_gh(fail=True)
        code, stdout, stderr = self.run_main(env=env)
        self.assertEqual(code, 2, stderr)
        self.assertNotIn('"verdict"', stdout)
        self.assertIn("502", stderr)

    def test_a_day_with_more_runs_than_were_returned_exits_2(self):
        self.world.green_days(15)
        env, _ = self.fake_gh(lie_total=1)
        code, stdout, stderr = self.run_main(env=env)
        self.assertEqual(code, 2, stderr)
        self.assertNotIn('"verdict"', stdout)

    def test_the_fetch_reads_every_page_of_every_day_and_the_flake_issues(self):
        for hour in range(0, 24, 2):
            self.world.pr_run(morning(1).replace(hour=hour), conclusion="success")
        self.world.green_days(15)
        env, log = self.fake_gh()
        with mock.patch.object(self.script, "PER_PAGE", 5):
            code, stdout, stderr = self.run_main(env=env)
        self.assertEqual(code, 0, stderr)
        receipt = json.loads(stdout)
        self.assertEqual(receipt["verdict"], "clean", receipt)
        self.assertEqual(receipt["inputs"]["runs"], len(self.world.runs))
        calls = [json.loads(line) for line in log.read_text().splitlines()]
        days = {call["params"]["created"] for call in calls if "created" in call["params"]}
        self.assertEqual(len(days), 45)
        busy_day = morning(1).strftime("%Y-%m-%d")
        pages = sorted(int(call["params"]["page"]) for call in calls
                       if call["params"].get("created", "").startswith(busy_day))
        self.assertEqual(pages, [1, 2, 3])
        self.assertTrue(any(call["path"].endswith("/issues") and call["params"]["labels"] == "flake"
                            for call in calls))

    def test_require_clean_exits_1_unless_the_window_is_clean(self):
        self.world.green_days(15, skip=(1,))
        code, stdout, _ = self.run_main("--input", str(self.saved_input()), "--require-clean")
        self.assertEqual(code, 1)
        self.assertEqual(json.loads(stdout)["verdict"], "running")
        self.world = World()
        self.world.flake_issue(132, created=morning(40), closed=morning(30))
        self.world.green_days(15)
        code, stdout, _ = self.run_main("--input", str(self.saved_input()), "--require-clean")
        self.assertEqual(code, 0, stdout)
        self.assertEqual(json.loads(stdout)["verdict"], "clean")

    def test_the_check_leaves_the_project_tree_unchanged(self):
        self.world.green_days(15)
        before = tree_digest(self.root)
        code, stdout, _ = self.run_main("--input", str(self.saved_input()))
        self.assertEqual(code, 0)
        self.assertEqual(json.loads(stdout)["inputs"]["bug_records"], 1)
        self.assertEqual(tree_digest(self.root), before)

    def test_an_unreadable_bug_record_exits_2_with_no_verdict(self):
        self.world.green_days(15)
        (self.root / ".chough" / "bugs" / "2026-11-01-not-utf-8.md").write_bytes(
            b"---\nkind: bug\nid: x\nstatus: open\n---\n\xff\xfe not UTF-8\n")
        code, stdout, stderr = self.run_main("--input", str(self.saved_input()))
        self.assertEqual(code, 2, stderr)
        self.assertNotIn('"verdict"', stdout)
        self.assertIn("2026-11-01-not-utf-8.md", stderr)

    def test_the_receipt_names_a_digest_of_the_bug_records_it_read(self):
        self.world.green_days(15)
        code, stdout, _ = self.run_main("--input", str(self.saved_input()))
        self.assertEqual(code, 0)
        first = json.loads(stdout)["inputs"]["bug_records_sha256"]
        self.assertRegex(first, r"^[0-9a-f]{64}$")
        record = self.root / ".chough" / "bugs" / "2026-10-11-fd-inheritance.md"
        record.write_text(record.read_text() + "- note: another line\n")
        code, stdout, _ = self.run_main("--input", str(self.saved_input()))
        self.assertNotEqual(json.loads(stdout)["inputs"]["bug_records_sha256"], first)


class WiringTests(unittest.TestCase):
    def test_fledge_runs_the_check_and_its_fixtures(self):
        tasks = tomllib.loads((ROOT / "fledge.toml").read_text())["tasks"]
        self.assertEqual(tasks.get("flake-window"), "python3 .github/scripts/flake-window.py")
        self.assertEqual(tasks.get("ci-flake-window-fixtures"),
                         "python3 .github/scripts/test-flake-window.py")

    def test_the_ci_gate_fixtures_job_runs_the_fixtures(self):
        lines = (ROOT / ".github" / "workflows" / "ci.yml").read_text().splitlines()
        start = lines.index("  ci-fixtures:")
        end = next((index for index in range(start + 1, len(lines))
                    if re.fullmatch(r"  [A-Za-z0-9_-]+:\s*", lines[index])), len(lines))
        job = "\n".join(lines[start:end])
        self.assertIn("name: CI gate fixtures", job)
        self.assertIn("run: python3 .github/scripts/test-flake-window.py", job)

    def test_bug_records_are_read_with_their_status_creation_time_and_text(self):
        script = load_script()
        with tempfile.TemporaryDirectory() as temp:
            bugs = Path(temp) / ".chough" / "bugs"
            bugs.mkdir(parents=True)
            (bugs / "2026-11-07-kill-zero.md").write_text(
                "---\nkind: bug\nid: 2026-11-07-kill-zero\nstatus: verified\n"
                "criteria: [DRIVE-19]\ncreated: 2026-11-07T10:00:00Z\n---\n# Kill zero\n"
            )
            (bugs / "README.md").write_text("Not a record.\n")
            records = script.read_bugs(Path(temp))
        self.assertEqual(len(records), 1)
        self.assertEqual(records[0]["id"], "2026-11-07-kill-zero")
        self.assertEqual(records[0]["status"], "verified")
        self.assertEqual(records[0]["created"], "2026-11-07T10:00:00Z")
        self.assertIn("DRIVE-19", records[0]["text"])


if __name__ == "__main__":
    unittest.main()
