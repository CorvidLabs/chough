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
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Approve the scope of the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
    holds: plan
    drives: approve
    opened: 2026-11-15T18:24:55Z
run_consents: []
next: Read scoped intent and native status, then choose the next action within the authorized scope.
---
# Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml

## Goal

Fix #289: rebuild the knowledge index at release, not in each PR, so open PRs stop conflicting on knowledge/index.md and feed.xml

## Where it stands

## Decisions and why

## What was verified

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
