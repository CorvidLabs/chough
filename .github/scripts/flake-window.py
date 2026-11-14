#!/usr/bin/env python3
"""Compute Chough's 1.0 flake window from CI history (REQ-cli-062, LIFE-21 bar 2).

Read-only. It reads the CI workflow's runs, their jobs and the issues labelled
`flake` through `gh api`, and the bug records in `.chough/bugs/`. It writes nothing
and prints one JSON receipt with a verdict:

- clean: no restart in the last 14 days and nothing blocks;
- running: a restart fell in the last 14 days (the receipt names it and when the
  window can first be clean);
- blocked: a flake is open, a closed flake has had no green main run since, or a
  flake issue has no bug record naming it as `GitHub #<n>`.

Restarts:
- a completed main run with any job that did not succeed;
- a rerun (run attempt above 1) on any branch;
- a filed flake (a `flake` issue, or a bug record naming DRIVE-19);
- a full UTC day with no main run whose ubuntu and macOS Product jobs succeeded.

Exit 0 with a receipt; 1 with `--require-clean` unless clean; 2 when GitHub's data
cannot be read in full, with no verdict.
"""

import argparse
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timedelta, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

UTC = timezone.utc
DAY = timedelta(days=1)
REQUIRED_DAYS = 14
# A run can be rerun for 30 days after it was created, so a rerun inside the window
# may belong to a run created up to 30 days before it.
LOOKBACK_DAYS = 45
PER_PAGE = 100
MAX_RESULTS = 1000  # GitHub returns at most this many runs for a filtered query.
WORKFLOW = "ci.yml"
PRODUCT_JOBS = {"linux": "Product / ubuntu-latest", "macos": "Product / macos-latest"}
MAIN_EVENTS = {"push", "schedule", "workflow_dispatch"}
CLOSED_BUG_STATUSES = {"verified", "duplicate", "wont-fix", "fixed-upstream"}  # src/bugs.rs
FLAKE_CRITERION = "DRIVE-19"
GH_TIMEOUT_SECONDS = 60
GH_WORKERS = 8  # concurrent `gh api` calls; one run reads about 200 pages
RUN_FIELDS = ("id", "head_branch", "head_sha", "event", "status", "conclusion", "run_attempt",
              "created_at", "run_started_at", "html_url")
SHOWN_RESTARTS = 20


class Unavailable(Exception):
    """GitHub's data could not be read in full; no verdict may be given."""


def parse_time(value):
    if not isinstance(value, str) or not value:
        raise Unavailable(f"missing or malformed time: {value!r}")
    try:
        moment = datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError as error:
        raise Unavailable(f"malformed time {value!r}: {error}") from error
    if moment.tzinfo is None:
        moment = moment.replace(tzinfo=UTC)
    return moment.astimezone(UTC)


def iso(moment):
    return moment.astimezone(UTC).strftime("%Y-%m-%dT%H:%M:%SZ")


def start_of_day(moment):
    return datetime(moment.year, moment.month, moment.day, tzinfo=UTC)


# MARK: - Reading


def gh_json(path, params):
    """Run `gh api -X GET <path>` with query parameters and decode its JSON."""
    argv = ["gh", "api", "-X", "GET", path]
    for key, value in params.items():
        argv += ["-f", f"{key}={value}"]
    try:
        result = subprocess.run(argv, capture_output=True, text=True, timeout=GH_TIMEOUT_SECONDS)
    except FileNotFoundError as error:
        raise Unavailable("gh is not installed or not on PATH") from error
    except subprocess.TimeoutExpired as error:
        raise Unavailable(f"gh api {path} timed out after {GH_TIMEOUT_SECONDS} s") from error
    if result.returncode != 0:
        detail = (result.stderr or result.stdout).strip().splitlines()
        raise Unavailable(f"gh api {path} failed (exit {result.returncode}): {' '.join(detail[-3:])}")
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as error:
        raise Unavailable(f"gh api {path} returned invalid JSON: {error}") from error


def fetch_day_runs(repo, day):
    created = f"{day:%Y-%m-%d}T00:00:00Z..{day:%Y-%m-%d}T23:59:59Z"
    path = f"repos/{repo}/actions/workflows/{WORKFLOW}/runs"
    runs, page, total = [], 1, None
    while True:
        body = gh_json(path, {"created": created, "per_page": PER_PAGE, "page": page})
        if not isinstance(body, dict) or not isinstance(body.get("workflow_runs"), list):
            raise Unavailable(f"runs for {day:%Y-%m-%d}: unexpected response shape")
        total = body.get("total_count")
        if not isinstance(total, int):
            raise Unavailable(f"runs for {day:%Y-%m-%d}: no total_count")
        if total > MAX_RESULTS:
            raise Unavailable(f"runs for {day:%Y-%m-%d}: {total} runs exceed GitHub's {MAX_RESULTS}-result limit")
        runs += body["workflow_runs"]
        if len(runs) >= total or not body["workflow_runs"]:
            break
        page += 1
    if len(runs) != total:
        raise Unavailable(f"runs for {day:%Y-%m-%d}: read {len(runs)} of {total}")
    return runs


def fetch_jobs(repo, run_id):
    path = f"repos/{repo}/actions/runs/{run_id}/jobs"
    jobs, page = [], 1
    while True:
        body = gh_json(path, {"filter": "latest", "per_page": PER_PAGE, "page": page})
        if not isinstance(body, dict) or not isinstance(body.get("jobs"), list):
            raise Unavailable(f"jobs of run {run_id}: unexpected response shape")
        total = body.get("total_count")
        if not isinstance(total, int):
            raise Unavailable(f"jobs of run {run_id}: no total_count")
        jobs += body["jobs"]
        if len(jobs) >= total or not body["jobs"]:
            break
        page += 1
    if len(jobs) != total:
        raise Unavailable(f"jobs of run {run_id}: read {len(jobs)} of {total}")
    return [{"name": job.get("name"), "status": job.get("status"), "conclusion": job.get("conclusion")}
            for job in jobs]


def fetch_flake_issues(repo):
    issues, page = [], 1
    while True:
        body = gh_json(f"repos/{repo}/issues",
                       {"labels": "flake", "state": "all", "per_page": PER_PAGE, "page": page})
        if not isinstance(body, list):
            raise Unavailable("flake issues: unexpected response shape")
        issues += [issue for issue in body if "pull_request" not in issue]
        if len(body) < PER_PAGE:
            return [{key: issue.get(key) for key in
                     ("number", "title", "state", "created_at", "closed_at", "html_url")}
                    for issue in issues]
        page += 1


def parallel(function, items):
    """Map in GH_WORKERS threads, keeping order; the first failure is raised."""
    with ThreadPoolExecutor(max_workers=GH_WORKERS) as pool:
        return list(pool.map(function, items))


def is_main(run):
    return run.get("head_branch") == "main" and run.get("event") in MAIN_EVENTS


def green(run, jobs, os_key=None):
    """Whether a completed main run passed every job (or, with os_key, that OS's job)."""
    if run.get("status") != "completed" or jobs is None:
        return False
    results = {job.get("name"): job.get("conclusion") for job in jobs}
    if os_key is not None:
        return results.get(PRODUCT_JOBS[os_key]) == "success"
    return (run.get("conclusion") == "success"
            and all(result == "success" for result in results.values())
            and all(results.get(name) == "success" for name in PRODUCT_JOBS.values()))


def fetch(repo, now, lookback_days=LOOKBACK_DAYS, days=REQUIRED_DAYS):
    """Read every input the computation needs from GitHub."""
    today = start_of_day(now)
    days_read = [today - offset * DAY for offset in range(lookback_days - 1, -1, -1)]
    runs = [{key: run.get(key) for key in RUN_FIELDS}
            for day_runs in parallel(lambda day: fetch_day_runs(repo, day), days_read)
            for run in day_runs]
    issues = fetch_flake_issues(repo)
    main = sorted((run for run in runs if is_main(run)), key=lambda run: run["created_at"])
    floor = today - days * DAY
    recent = [run for run in main
              if run.get("status") == "completed" and parse_time(run["created_at"]) >= floor]
    jobs = {str(run["id"]): run_jobs
            for run, run_jobs in zip(recent, parallel(lambda run: fetch_jobs(repo, run["id"]), recent))}
    # A flake closed before the floor needs one green main run after it closed.
    for issue in issues:
        if issue.get("state") != "closed" or not issue.get("closed_at"):
            continue
        closed = parse_time(issue["closed_at"])
        for run in main:
            if parse_time(run["created_at"]) <= closed or run.get("status") != "completed":
                continue
            key = str(run["id"])
            if key not in jobs:
                jobs[key] = fetch_jobs(repo, run["id"])
            if green(run, jobs[key]):
                break
    return {
        "schema": "chough-flake-window-input/1",
        "repository": repo,
        "fetched_at": iso(now),
        "lookback_days": lookback_days,
        "runs": runs,
        "jobs": jobs,
        "issues": issues,
    }


FRONT_MATTER = re.compile(r"\A---\n(.*?)\n---\n", re.DOTALL)


def read_bugs(root):
    """Every `.chough/bugs/*.md` record: id, status, creation time and full text."""
    records = []
    folder = Path(root) / ".chough" / "bugs"
    if not folder.is_dir():
        return records
    for path in sorted(folder.glob("*.md")):
        try:
            text = path.read_text(encoding="utf-8").replace("\r\n", "\n")
        except (OSError, UnicodeDecodeError) as error:
            raise Unavailable(f"cannot read bug record {path}: {error}") from error
        match = FRONT_MATTER.match(text)
        if not match:
            continue
        fields = {}
        for line in match.group(1).splitlines():
            key, sep, value = line.partition(":")
            if sep and not line.startswith((" ", "-")):
                fields[key.strip()] = value.strip().strip('"')
        if fields.get("kind") != "bug":
            continue
        records.append({
            "id": fields.get("id") or path.stem,
            "status": fields.get("status", ""),
            "created": fields.get("created", ""),
            "text": text,
        })
    return records


# MARK: - Computing


def names_flake_criterion(bug):
    return re.search(rf"\b{FLAKE_CRITERION}\b", bug.get("text", "")) is not None


def names_issue(bug, number):
    text = bug.get("text", "")
    return (re.search(rf"\bGitHub #{number}\b", text) is not None
            or re.search(rf"github\.com/[^/\s]+/[^/\s]+/issues/{number}\b", text) is not None)


def compute(inputs, bugs, now, days=REQUIRED_DAYS):
    """Turn runs, jobs, flake issues and bug records into the receipt (no I/O)."""
    runs = inputs.get("runs")
    jobs = inputs.get("jobs")
    issues = inputs.get("issues")
    if not isinstance(runs, list) or not isinstance(jobs, dict) or not isinstance(issues, list):
        raise Unavailable("input needs runs, jobs and issues")
    today = start_of_day(now)
    floor = today - days * DAY
    since = now - days * DAY
    main = [run for run in runs if is_main(run)]
    for run in main:
        if run.get("status") == "completed" and parse_time(run["created_at"]) >= floor \
                and str(run.get("id")) not in jobs:
            raise Unavailable(f"main run {run.get('id')} has no jobs in the input")

    restarts = []

    def restart(kind, at, detail):
        restarts.append({"kind": kind, "at": iso(at), "detail": detail})

    for run in runs:
        attempt = run.get("run_attempt")
        if isinstance(attempt, int) and attempt > 1:
            started = parse_time(run.get("run_started_at") or run.get("created_at"))
            restart("rerun", started,
                    f"run {run.get('id')} on {run.get('head_branch')} reached attempt {attempt}")
    for run in main:
        if run.get("status") != "completed":
            continue
        run_jobs = jobs.get(str(run.get("id")))
        failed = [job.get("name") for job in run_jobs or [] if job.get("conclusion") != "success"]
        if run.get("conclusion") != "success" or failed:
            what = ", ".join(f"{name}" for name in failed) or f"conclusion {run.get('conclusion')}"
            restart("red-main-run", parse_time(run["created_at"]),
                    f"main run {run.get('id')} did not succeed: {what}")
    for issue in issues:
        restart("flake-filed", parse_time(issue.get("created_at")), f"flake issue #{issue.get('number')} filed")
    flake_bugs = [bug for bug in bugs if names_flake_criterion(bug)]
    for bug in flake_bugs:
        if bug.get("created"):
            restart("flake-filed", parse_time(bug["created"]),
                    f"bug record {bug.get('id')} names {FLAKE_CRITERION}")

    # A day counts only when one main run passed both Product jobs (REQ-cli-062);
    # two runs that each pass one job do not cover it.
    covered, seen = set(), {}
    for run in main:
        if run.get("status") != "completed":
            continue
        day = start_of_day(parse_time(run["created_at"]))
        passed = {key for key in PRODUCT_JOBS if green(run, jobs.get(str(run.get("id"))), key)}
        seen.setdefault(day, set()).update(passed)
        if passed == set(PRODUCT_JOBS):
            covered.add(day)
    for offset in range(1, days + 1):
        day = today - offset * DAY
        if day in covered:
            continue
        missing = [PRODUCT_JOBS[key].split(" / ")[1] for key in PRODUCT_JOBS if key not in seen.get(day, set())]
        if missing:
            detail = f"no green main run on {day:%Y-%m-%d} for {' and '.join(missing)}"
        else:
            detail = (f"no main run on {day:%Y-%m-%d} passed both Product jobs;"
                      " each passed only in a different run")
        restart("no-green-run", day + DAY, detail)

    blocked = []
    open_flakes, closed_flakes = [], []
    for issue in sorted(issues, key=lambda issue: issue.get("number") or 0):
        number = issue.get("number")
        entry = {"number": number, "title": issue.get("title"), "url": issue.get("html_url")}
        if issue.get("state") == "open":
            open_flakes.append(entry)
            blocked.append(f"flake issue #{number} is open")
        else:
            closed_flakes.append(entry)
            closed = parse_time(issue.get("closed_at"))
            if not any(parse_time(run["created_at"]) > closed and green(run, jobs.get(str(run.get("id"))))
                       for run in main):
                blocked.append(f"flake issue #{number} has had no green main run since it closed")
        if not any(names_issue(bug, number) for bug in bugs):
            blocked.append(f"flake issue #{number} has no bug record naming GitHub #{number}")
    for bug in flake_bugs:
        if bug.get("status") not in CLOSED_BUG_STATUSES:
            blocked.append(f"bug record {bug.get('id')} names {FLAKE_CRITERION} and is {bug.get('status') or 'unknown'}")

    restarts.sort(key=lambda item: item["at"])
    latest = restarts[-1] if restarts else None
    recent = [item for item in restarts if parse_time(item["at"]) > since]
    if blocked:
        verdict, clean_from = "blocked", None
    elif recent:
        verdict, clean_from = "running", iso(parse_time(latest["at"]) + days * DAY)
    else:
        verdict, clean_from = "clean", None
    green_runs = {
        key: sum(1 for run in main if parse_time(run["created_at"]) >= floor
                 and green(run, jobs.get(str(run.get("id"))), key))
        for key in PRODUCT_JOBS
    }
    days_clean = None
    if latest is not None:
        days_clean = round((now - parse_time(latest["at"])) / DAY, 2)
    canonical = json.dumps({"runs": runs, "jobs": jobs, "issues": issues}, sort_keys=True,
                           separators=(",", ":")).encode()
    # --input recomputes from saved GitHub data but reads .chough/bugs/ again, so the
    # receipt names both digests: it reproduces only at the same bug records.
    bug_canonical = json.dumps(sorted(bugs, key=lambda bug: str(bug.get("id"))), sort_keys=True,
                               separators=(",", ":")).encode()
    return {
        "schema": "chough-flake-window/1",
        "repository": inputs.get("repository"),
        "computed_at": iso(now),
        "required_days": days,
        "verdict": verdict,
        "latest_restart": latest,
        "clean_from": clean_from,
        "days_since_latest_restart": days_clean,
        "blocked_by": blocked,
        "restarts_in_window": len(recent),
        "recent_restarts": recent[-SHOWN_RESTARTS:],
        "green_main_runs": green_runs,
        "flakes": {"open": open_flakes, "closed": closed_flakes},
        "inputs": {
            "runs": len(runs),
            "main_runs": len(main),
            "jobs_read": len(jobs),
            "flake_issues": len(issues),
            "bug_records": len(bugs),
            "flake_bug_records": len(flake_bugs),
            "lookback_days": inputs.get("lookback_days"),
            "sha256": hashlib.sha256(canonical).hexdigest(),
            "bug_records_sha256": hashlib.sha256(bug_canonical).hexdigest(),
        },
    }


# MARK: - Command line


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--repo", default=os.environ.get("GITHUB_REPOSITORY", "CorvidLabs/chough"))
    parser.add_argument("--root", default=".", help="the project whose .chough/bugs/ is read")
    parser.add_argument("--now", help="compute as of this UTC time (default: now)")
    parser.add_argument("--days", type=int, default=REQUIRED_DAYS)
    parser.add_argument("--lookback-days", type=int, default=LOOKBACK_DAYS)
    source = parser.add_mutually_exclusive_group()
    source.add_argument("--input", help="recompute from saved input instead of reading GitHub")
    source.add_argument("--save-input", help="also write the input read from GitHub to this file")
    parser.add_argument("--require-clean", action="store_true", help="exit 1 unless the verdict is clean")
    args = parser.parse_args(argv)
    try:
        now = parse_time(args.now) if args.now else datetime.now(UTC)
        if args.input:
            try:
                inputs = json.loads(Path(args.input).read_text(encoding="utf-8"))
            except (OSError, json.JSONDecodeError) as error:
                raise Unavailable(f"cannot read {args.input}: {error}") from error
            if not isinstance(inputs, dict):
                raise Unavailable(f"{args.input} is not a JSON object")
        else:
            inputs = fetch(args.repo, now, args.lookback_days, args.days)
        receipt = compute(inputs, read_bugs(args.root), now, args.days)
    except Unavailable as error:
        print(f"flake-window: unavailable: {error}; no verdict", file=sys.stderr)
        return 2
    if args.save_input:
        Path(args.save_input).write_text(json.dumps(inputs, indent=1) + "\n", encoding="utf-8")
    print(json.dumps(receipt, indent=2))
    if args.require_clean and receipt["verdict"] != "clean":
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
