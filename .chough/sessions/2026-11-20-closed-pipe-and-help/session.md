---
kind: session
id: 2026-11-20-closed-pipe-and-help
aliases: [closed-pipe-and-help]
objective: "A closed standard output ends a command quietly with its own exit code, and help works after global options, with -V documented as the top-level version flag (#383's closed pipe, #399)"
intent_scope: ""
modules: [cli]
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: ""
pr: ""
criteria: [LIFE-4]
requirements: [REQ-cli-021, REQ-cli-019]
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Read scoped intent and native status, then choose the next action within the authorized scope.
---
# A closed standard output ends a command quietly with its own exit code, and help works after global options, with -V documented as the top-level version flag (#383's closed pipe, #399)

## Goal

A closed standard output ends a command quietly with its own exit code, and help works after global options, with -V documented as the top-level version flag (#383's closed pipe, #399)

## Where it stands

## Decisions and why

## What was verified

## Open threads

## Timeline

- 2026-11-20T15:10:25Z · start · intent → intent · A closed standard output ends a command quietly with its own exit code, and help works after global options, with -V documented as the top-level version flag (#383's closed pipe, #399) ^t-a1be46
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:10:25Z · advance · intent → design ·  ^t-31a106
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:10:25Z · skip · design → plan · Nothing visual changes: quiet exits on a closed pipe, help parsing and a docs sentence. ^t-386d1c
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
