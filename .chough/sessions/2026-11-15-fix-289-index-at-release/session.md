---
kind: session
id: 2026-11-15-fix-289-index-at-release
aliases: [fix-289-index-at-release]
objective: "Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml"
intent_scope: KNOW
modules: [cli]
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
pr: ""
criteria: [KNOW-4]
requirements: []
cases: []
design: []
files: []
observations: [2026-11-15-tc-cli-documented-merge-drivers-match]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Approve the scope of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289\" closed (approved); run `chough stage advance` to move on."
---
# Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml

## Goal

Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289": "yes I approve" (Claude Code chat (hc t28), 2026-11-15, on the #289 scope summary; list refreshed only for Flint's two announced tests (7321283) and a re-run of TC-cli-documented-merge-drivers-match); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 as 0xLeif

## What was verified

- [Ran cargo test --locked --test docs -- --exact cli_reference_declares_the_merge_drivers_this_repository_declares for TC-cli-documented-merge-drivers-match: passed (exit 0, tests run 1).](evidence/2026-11-15-tc-cli-documented-merge-drivers-match.md)

## Open threads

## Timeline

- 2026-11-15T18:23:57Z · start · intent → intent · Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml ^t-133098
  - revision: 4d3c1a5cfddc9e6710be2dad1c39162816adacf0
  - by: claude-code (hc t28)
- 2026-11-15T18:23:57Z · skip · intent → design · Intent is #289 and Leif's choice in chat of design B ("B"): the index is rebuilt and checked only at release; KNOW-4 is unchanged ^t-7aba5e
  - revision: 4d3c1a5cfddc9e6710be2dad1c39162816adacf0
  - by: claude-code (hc t28)
- 2026-11-15T18:23:57Z · skip · design → plan · No visual design: a lane, a release step and documentation ^t-325e4d
  - revision: 4d3c1a5cfddc9e6710be2dad1c39162816adacf0
  - by: claude-code (hc t28)
- 2026-11-15T18:24:54Z · gate-opened · plan → plan · Approve the scope of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 ^t-9f612b
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: c08afc2ad5ce36675fe01bf5640a5322e17f31ba
  - holds: plan
  - drives: approve
  - by: claude-code (hc t28)
- 2026-11-15T20:18:20Z · direct-native-write · plan → plan · the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-3477ca
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: 7321283c6a3194154feab7d635fd2aea08b23efd
  - direct_write:
    - change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: claude-code (hc t28)
- 2026-11-15T20:18:20Z · gate-closed · plan → plan · Approve the scope of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289 ^t-26e2ca
  - native_change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
  - revision: 7321283c6a3194154feab7d635fd2aea08b23efd
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes I approve
    - where: Claude Code chat (hc t28), 2026-11-15, on the #289 scope summary; list refreshed only for Flint's two announced tests (7321283) and a re-run of TC-cli-documented-merge-drivers-match
    - recorded_by: claude-code (hc t28)
    - summary_digest: ac908fdb93a42f40b3fb98e9807339189dbcfa45b53b6c760fcaf8fdf04771b8
    - list: faa979714b50
    - native: specsync change approve the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
    - actor: 0xLeif
  - by: claude-code (hc t28)
