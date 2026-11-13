---
kind: session
id: 2026-11-13-flake-window
aliases: [flake-window]
objective: "#217: fledge run flake-window computes Chough's two-week clean window from CI history, flake issues and bug records (REQ-cli-062)"
intent_scope: LIFE
modules: [cli]
stage: build
stage_status: active
iteration: 1
callers: []
native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
pr: ""
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
cases:
  - TC-cli-flake-window-clean
  - TC-cli-flake-window-restarts
  - TC-cli-flake-window-blocked
  - TC-cli-flake-window-fails-closed
  - TC-cli-flake-window-wiring
design: []
files: [.github/scripts/test-flake-window.py]
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Record the red fixture run, then add .github/scripts/flake-window.py
---
# #217: fledge run flake-window computes Chough's two-week clean window from CI history, flake issues and bug records (REQ-cli-062)

## Goal

#217: fledge run flake-window computes Chough's two-week clean window from CI history, flake issues and bug records (REQ-cli-062)

## Where it stands

## Decisions and why

- The check is a repo script plus a fledge task, recorded as evidence at release; Chough itself gains no command and no GitHub dependency (Leif, Q1, 2026-11-13T03:30Z, relayed by t370; rejected: a chough command, a daily GitHub job posting to an issue)
- Any red CI run on main, any rerun on any branch, or a newly filed flake restarts the window (Leif, Q2, 2026-11-13T03:30Z, relayed by t370; rejected: only failures traced to a filed flake)
- Requirement ID REQ-cli-062 (t382, records owner, 2026-11-13); a flake bug record names its issue as 'GitHub #<n>' in a bug update note, since bug update has no criterion flag
- Leif decided "Approve the intent: fledge run flake-window computes the two-week clean window from CI history (#217, bar 2)": "Repo script + fledge task (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q1 'Where should the two-week check live?' for #217 (same answer set: Q2 'What restarts the window?': 'Any red, rerun or new flake (Recommended)'); relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated
- Leif decided "Approve the scope of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read": "Any red, rerun or new flake (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q2 'What restarts the window?' for #217 (same answer set: Q1 'Where should the two-week check live?': 'Repo script + fledge task (Recommended)'); this change's scope is that design: a read-only repo script and fledge task, REQ-cli-062; relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated; ran specsync change approve 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-13T03:35:00Z · start · intent → intent · #217: fledge run flake-window computes Chough's two-week clean window from CI history, flake issues and bug records (REQ-cli-062) ^t-fe3698
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (hc t38r)
- 2026-11-13T03:35:23Z · gate-opened · intent → intent · Approve the intent: fledge run flake-window computes the two-week clean window from CI history (#217, bar 2) ^t-7625ac
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - holds: intent
  - drives: none
  - by: claude-code (hc t38r)
- 2026-11-13T03:35:42Z · gate-closed · intent → intent · Approve the intent: fledge run flake-window computes the two-week clean window from CI history (#217, bar 2) ^t-d5a7b1
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - relayed_by:
    - decided_by: Leif
    - words: Repo script + fledge task (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q1 'Where should the two-week check live?' for #217 (same answer set: Q2 'What restarts the window?': 'Any red, rerun or new flake (Recommended)'); relayed by claude-code-t370
    - recorded_by: claude-code-t38r
  - by: claude-code (hc t38r)
- 2026-11-13T03:35:42Z · advance · intent → design ·  ^t-77a4e6
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (hc t38r)
- 2026-11-13T03:35:42Z · skip · design → plan · No visuals or interactions change: a read-only repo script and docs ^t-086976
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (hc t38r)
- 2026-11-13T03:38:31Z · gate-opened · plan → plan · Approve the scope of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read ^t-e9482b
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - holds: plan
  - drives: approve
  - by: claude-code (hc t38r)
- 2026-11-13T03:38:31Z · gate-closed · plan → plan · Approve the scope of 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read ^t-5bb0de
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - relayed_by:
    - decided_by: Leif
    - words: Any red, rerun or new flake (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q2 'What restarts the window?' for #217 (same answer set: Q1 'Where should the two-week check live?': 'Repo script + fledge task (Recommended)'); this change's scope is that design: a read-only repo script and fledge task, REQ-cli-062; relayed by claude-code-t370
    - recorded_by: claude-code-t38r
    - native: specsync change approve 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
    - actor: 0xLeif
  - by: claude-code (hc t38r)
- 2026-11-13T03:38:38Z · advance · plan → build · native:approved ^t-800fe3
  - native_change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
  - gate: native:approved
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (hc t38r)
