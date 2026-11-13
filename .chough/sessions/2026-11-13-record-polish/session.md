---
kind: session
id: 2026-11-13-record-polish
aliases: [record-polish]
objective: "0.6.1: record polish: a bug's Actual line stands apart, migrated front matter names the new entries, and a fresh project's records README and the README's init paragraph are trimmed"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
pr: ""
criteria: []
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue build
---
# 0.6.1: record polish: a bug's Actual line stands apart, migrated front matter names the new entries, and a fresh project's records README and the README's init paragraph are trimmed

## Goal

0.6.1: record polish: a bug's Actual line stands apart, migrated front matter names the new entries, and a fresh project's records README and the README's init paragraph are trimmed

## Where it stands

## Decisions and why

- Leif decided "Approve the intent: record polish for bugs, migrated front matter and the READMEs": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-13T02:28:24Z · start · intent → intent · 0.6.1: record polish: a bug's Actual line stands apart, migrated front matter names the new entries, and a fresh project's records README and the README's init paragraph are trimmed ^t-514089
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:28:24Z · gate-opened · intent → intent · Approve the intent: record polish for bugs, migrated front matter and the READMEs ^t-1079b1
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T02:28:24Z · gate-closed · intent → intent · Approve the intent: record polish for bugs, migrated front matter and the READMEs ^t-c78a0a
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T02:28:24Z · advance · intent → design · Leif approved 0.6.1's PRs (02:03Z, relayed by t370) ^t-2e82a7
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:28:24Z · skip · design → plan · No visuals or interactions change ^t-6f7bf4
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:28:52Z · gate-opened · plan → plan · Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s ^t-f19d29
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T02:28:52Z · gate-closed · plan → plan · Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s ^t-0f2953
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:28:54Z · advance · plan → build · Leif approved the scope (02:03Z, relayed by t370) ^t-2dac3e
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - gate: native:approved
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
