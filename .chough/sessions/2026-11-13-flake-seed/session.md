---
kind: session
id: 2026-11-13-flake-seed
aliases: [flake-seed]
objective: "#217: seed the flake tracker: create the GitHub flake label, label the known flake issues, and note DRIVE-19 and each GitHub issue in the flake bug records"
intent_scope: DRIVE
modules: [cli]
stage: build
stage_status: active
iteration: 1
callers: []
native_change: 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each
pr: ""
criteria: [DRIVE-19, LIFE-21]
requirements: []
cases: []
design: []
files: [docs/test-plan-1.0.md]
observations:
  - 2026-11-13-before-the-notes-the-flake-window-is-blocked-132-169-187-and
  - 2026-11-13-after-the-notes-132-169-and-187-each-match-their-bug-record
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Accept: change check, full suite, review"
---
# #217: seed the flake tracker: create the GitHub flake label, label the known flake issues, and note DRIVE-19 and each GitHub issue in the flake bug records

## Goal

#217: seed the flake tracker: create the GitHub flake label, label the known flake issues, and note DRIVE-19 and each GitHub issue in the flake bug records

## Where it stands

- The flake label exists and is on #187, #132, #169 and #247; six flake records carry DRIVE-19 notes; the check went from blocked on all four issues to blocked only on #247

## Decisions and why

- The tracker is the GitHub flake label plus one bug record per flake, as docs/test-plan-1.0.md bar 2 already says; this change adds no contract text
- Notes, not criteria: bug update has no criterion flag and --related takes bug IDs, so each flake record gets a Log note naming DRIVE-19 and its issue as 'GitHub #<n>' (t382, records owner, 2026-11-13)
- Labelled: #187, #132 and #169 (closed flakes with records) and #247 (open, t37f's fix). Three local-only flakes (HUP survivor, timing budget, checkpoint lock timeout) get a note that no issue was filed; no issue is filed after the fact
- Leif decided "Approve the intent: seed the flake tracker with the flake label, the known flake issues and DRIVE-19 notes on their bug records (#217)": "Repo script + fledge task (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q1 'Where should the two-week check live?' for #217, whose design's first PR is this seed: the flake label and one bug record per flake (same answer set: Q2 'Any red, rerun or new flake (Recommended)'); relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated
- Leif decided "Approve the scope of 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each": "Repo script + fledge task (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q1 'Where should the two-week check live?' for #217; this change's scope is that design's seed step: the flake label on #187, #132, #169 and #247, DRIVE-19 notes on their bug records, and one docs line, with no contract text; relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated; ran specsync change approve 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each as 0xLeif

## What was verified

- [Before the notes, the flake window is blocked: #132, #169, #187 and #247 are labelled flake but no bug record names them as GitHub #<n>, and #247 is open (4 flake issues, 205 bug records, 3 naming DRIVE-19; input sha256 a4634e4dea37a09b)](evidence/2026-11-13-before-the-notes-the-flake-window-is-blocked-132-169-187-and.md)
- [After the notes, #132, #169 and #187 each match their bug record and 6 records name DRIVE-19. The only blockers left are #247: open, and its record (on t37f's fix branch leif/fix-247-example-race) is not on main yet. Same GitHub input as the red run (sha256 a4634e4dea37a09b), so only the records changed](evidence/2026-11-13-after-the-notes-132-169-and-187-each-match-their-bug-record.md)

## Open threads

## Timeline

- 2026-11-13T04:30:21Z · start · intent → intent · #217: seed the flake tracker: create the GitHub flake label, label the known flake issues, and note DRIVE-19 and each GitHub issue in the flake bug records ^t-eab40f
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t38r)
- 2026-11-13T04:30:40Z · gate-opened · intent → intent · Approve the intent: seed the flake tracker with the flake label, the known flake issues and DRIVE-19 notes on their bug records (#217) ^t-0c4575
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - holds: intent
  - drives: none
  - by: claude-code (hc t38r)
- 2026-11-13T04:30:40Z · gate-closed · intent → intent · Approve the intent: seed the flake tracker with the flake label, the known flake issues and DRIVE-19 notes on their bug records (#217) ^t-70545c
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - relayed_by:
    - decided_by: Leif
    - words: Repo script + fledge task (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q1 'Where should the two-week check live?' for #217, whose design's first PR is this seed: the flake label and one bug record per flake (same answer set: Q2 'Any red, rerun or new flake (Recommended)'); relayed by claude-code-t370
    - recorded_by: claude-code-t38r
  - by: claude-code (hc t38r)
- 2026-11-13T04:30:40Z · advance · intent → design ·  ^t-65780d
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t38r)
- 2026-11-13T04:30:40Z · skip · design → plan · No visuals or interactions change: a GitHub label, bug record notes and one docs line ^t-370c25
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t38r)
- 2026-11-13T04:31:40Z · gate-opened · plan → plan · Approve the scope of 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each ^t-6a7913
  - native_change: 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - holds: plan
  - drives: approve
  - by: claude-code (hc t38r)
- 2026-11-13T04:31:41Z · gate-closed · plan → plan · Approve the scope of 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each ^t-8be09d
  - native_change: 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - relayed_by:
    - decided_by: Leif
    - words: Repo script + fledge task (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q1 'Where should the two-week check live?' for #217; this change's scope is that design's seed step: the flake label on #187, #132, #169 and #247, DRIVE-19 notes on their bug records, and one docs line, with no contract text; relayed by claude-code-t370
    - recorded_by: claude-code-t38r
    - native: specsync change approve 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each
    - actor: 0xLeif
  - by: claude-code (hc t38r)
- 2026-11-13T04:31:44Z · advance · plan → build · native:approved ^t-07a47d
  - native_change: 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each
  - gate: native:approved
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t38r)
