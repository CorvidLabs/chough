---
kind: session
id: 2026-11-13-daily-ci
aliases: [daily-ci]
objective: "#217: CI also runs once a day on main, so the flake window sees a macOS and Linux run every day; the SpecSync Archive jobs take the run's own commit as their base when the event has none (REQ-cli-064)"
intent_scope: LIFE
modules: [cli]
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
pr: ""
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-064]
cases: [TC-cli-ci-runs-daily-with-a-real-archive-base]
design: []
files: [tests/package.rs, .github/workflows/ci.yml, docs/ci.md, CHANGELOG.md]
observations:
  - 2026-11-13-tc-cli-ci-runs-daily-with-a-real-archive-base
  - 2026-11-13-tc-cli-ci-runs-daily-with-a-real-archive-base-d92603
  - 2026-11-13-tc-cli-ci-runs-daily-with-a-real-archive-base-c0b0ac
  - 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
  - 2026-11-13-with-the-base-set-to-the-commit-itself-as-a-scheduled-run-now
  - 2026-11-13-the-full-test-suite-passed-738-of-738-across-48-targets-cargo
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
    holds: accept
    drives: review
    opened: 2026-11-13T06:19:07Z
run_consents: []
next: Review, then Leif's implementation review (relayed by t370); after merge, observe the first scheduled run on GitHub
---
# #217: CI also runs once a day on main, so the flake window sees a macOS and Linux run every day; the SpecSync Archive jobs take the run's own commit as their base when the event has none (REQ-cli-064)

## Goal

#217: CI also runs once a day on main, so the flake window sees a macOS and Linux run every day; the SpecSync Archive jobs take the run's own commit as their base when the event has none (REQ-cli-064)

## Where it stands

- REQ-cli-064: a daily 05:23 UTC schedule and the github.sha archive base fallback; package 27/27 and docs 19/19 pass; the workflow parses with push, pull_request and schedule

## Decisions and why

- A daily scheduled CI run on main, macOS and Linux (Leif, Q3, 2026-11-13T03:30Z, relayed by t370; rejected: counting only real pushes)
- A scheduled event has no pull request base and no previous commit, and check-specsync-archive.py refuses an empty --base, so the SpecSync Archive jobs fall back to github.sha; without it every scheduled run would be red and restart the window
- Requirement ID REQ-cli-064 (t382, records owner, 2026-11-13)
- Leif decided "Approve the intent: CI runs once a day on main on macOS and Linux for the flake window (#217, REQ-cli-064)": "Yes, macOS + Linux (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q3 'Run CI once a day on main so a quiet week still counts?' for #217; relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated
- Leif decided "Approve the scope of 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs": "Yes, macOS + Linux (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q3 'Run CI once a day on main so a quiet week still counts?' for #217; this change's scope is that daily run plus the Archive base fallback a scheduled run needs, REQ-cli-064; relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated; ran specsync change approve 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs as 0xLeif

## What was verified

- [Ran cargo test --locked --test package -- --exact ci_runs_once_a_day_on_main_with_a_real_archive_base for TC-cli-ci-runs-daily-with-a-real-archive-base: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-cli-ci-runs-daily-with-a-real-archive-base.md)
- [Ran cargo test --locked --test package -- --exact ci_runs_once_a_day_on_main_with_a_real_archive_base for TC-cli-ci-runs-daily-with-a-real-archive-base: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-cli-ci-runs-daily-with-a-real-archive-base-d92603.md)
- [Ran cargo test --locked --test package -- --exact ci_runs_once_a_day_on_main_with_a_real_archive_base for TC-cli-ci-runs-daily-with-a-real-archive-base: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-ci-runs-daily-with-a-real-archive-base-c0b0ac.md)
- [The full test suite passed 736 of 736 across 48 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at a2d271c9, the daily CI head on main 38fe24c0 with REQ-cli-064 materialized; rustfmt --check was clean](evidence/2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo.md)
- [With the base set to the commit itself, as a scheduled run now passes, the archive inspector exited 0 on main 38fe24c0: shard 1/4 walked 1 of 1 PR revision and authenticated 45 of 177 archives. This shows a github.sha base is a real base the inspector accepts; the other three shards were not run locally](evidence/2026-11-13-with-the-base-set-to-the-commit-itself-as-a-scheduled-run-now.md)
- [The full test suite passed 738 of 738 across 48 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at 7b9c837f, the daily CI head with main (#242) merged and REQ-cli-064 materialized as cli version 90](evidence/2026-11-13-the-full-test-suite-passed-738-of-738-across-48-targets-cargo.md)

## Open threads

## Timeline

- 2026-11-13T04:57:06Z · start · intent → intent · #217: CI also runs once a day on main, so the flake window sees a macOS and Linux run every day; the SpecSync Archive jobs take the run's own commit as their base when the event has none (REQ-cli-064) ^t-ea0cf2
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t38r)
- 2026-11-13T04:57:06Z · gate-opened · intent → intent · Approve the intent: CI runs once a day on main on macOS and Linux for the flake window (#217, REQ-cli-064) ^t-3f20ef
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - holds: intent
  - drives: none
  - by: claude-code (hc t38r)
- 2026-11-13T04:57:06Z · gate-closed · intent → intent · Approve the intent: CI runs once a day on main on macOS and Linux for the flake window (#217, REQ-cli-064) ^t-6fc1db
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - relayed_by:
    - decided_by: Leif
    - words: Yes, macOS + Linux (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q3 'Run CI once a day on main so a quiet week still counts?' for #217; relayed by claude-code-t370
    - recorded_by: claude-code-t38r
  - by: claude-code (hc t38r)
- 2026-11-13T04:57:06Z · advance · intent → design ·  ^t-a5781c
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t38r)
- 2026-11-13T04:57:06Z · skip · design → plan · No visuals or interactions change: a CI trigger, a base fallback and a test ^t-bfaacc
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t38r)
- 2026-11-13T04:57:54Z · gate-opened · plan → plan · Approve the scope of 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs ^t-de0af7
  - native_change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - holds: plan
  - drives: approve
  - by: claude-code (hc t38r)
- 2026-11-13T04:57:54Z · gate-closed · plan → plan · Approve the scope of 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs ^t-be126c
  - native_change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - relayed_by:
    - decided_by: Leif
    - words: Yes, macOS + Linux (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q3 'Run CI once a day on main so a quiet week still counts?' for #217; this change's scope is that daily run plus the Archive base fallback a scheduled run needs, REQ-cli-064; relayed by claude-code-t370
    - recorded_by: claude-code-t38r
    - native: specsync change approve 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
    - actor: 0xLeif
  - by: claude-code (hc t38r)
- 2026-11-13T04:57:57Z · advance · plan → build · native:approved ^t-773b6e
  - native_change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
  - gate: native:approved
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t38r)
- 2026-11-13T05:48:29Z · advance · build → test · native:approved ^t-6bea3e
  - native_change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
  - gate: native:approved
  - revision: a2d271c9c60126932d4dd03a40ddc5433419eb25
  - by: claude-code (hc t38r)
- 2026-11-13T05:48:30Z · advance · test → accept · native:approved ^t-5ad6d6
  - native_change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
  - gate: native:approved
  - revision: a2d271c9c60126932d4dd03a40ddc5433419eb25
  - by: claude-code (hc t38r)
- 2026-11-13T06:19:07Z · gate-opened · accept → accept · Implementation review of 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs ^t-54cc78
  - native_change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
  - revision: b3c4a010f8a25eae366bf53d48ba7123d0ce681d
  - holds: accept
  - drives: review
  - by: claude-code (hc t38r)
