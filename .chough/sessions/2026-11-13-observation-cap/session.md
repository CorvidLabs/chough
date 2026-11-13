---
kind: session
id: 2026-11-13-observation-cap
aliases: [observation-cap]
objective: "0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
pr: ""
criteria: []
requirements: []
cases: [TC-cli-observation-cap-refuses-before-writing]
design: []
files: []
observations:
  - 2026-11-13-tc-cli-observation-cap-refuses-before-writing
  - 2026-11-13-tc-cli-observation-cap-refuses-before-writing-578f54
  - 2026-11-13-tc-cli-observation-cap-refuses-before-writing-ffac2d
  - 2026-11-13-the-full-test-suite-passed-731-of-731-across-48-targets-cargo
bugs: [2026-11-13-at-the-400-observation-cap-cases-run-writes-its-evidence-entry]
blockers: []
waiting_on_human:
  - gate: Implementation review of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
    holds: accept
    drives: review
    opened: 2026-11-13T03:00:00Z
run_consents: []
next: Continue accept
title: "0.6.1: at the observation cap, writes refuse before writing"
summary:
  - At REQ-cli-032's 400-observation cap, every write that adds an observation refuses before running or writing anything, naming chough session shelve <id>; next and the leave-test gate read a case's latest run only from the observations the checkpoint references.
  - "Verified: the case red then green, the bug verified, and the full suite 731 of 731. Next: Leif's review and merge."
---
# 0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve

## Goal

0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve

## Where it stands

- PR 2: at the 400 cap every observation write refuses before writing, naming shelve; next and the leave-test gate read only referenced runs (REQ-cli-032, REQ-cli-021); red at c7bf6347, green at 3df0102c, bug verified

## Decisions and why

- Leif decided "Approve the intent: at the observation cap, writes refuse before writing and name the recovery": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Observation cap: cases run and evidence add refuse at 400 before writing anything, and name chough session shelve; next stops counting unlinked entries as a case's latest run'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Observation cap: cases run and evidence add refuse at 400 before writing anything, and name chough session shelve; next stops counting unlinked entries as a case's latest run'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough as 0xLeif
- Leif decided "Approve the scope of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough with its corrected file list": "Yes, start (Recommended) / Yes, fix and keep (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Observation cap: cases run and evidence add refuse at 400 before writing anything, and name chough session shelve; next stops counting unlinked entries as a case's latest run'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough as 0xLeif

## What was verified

- [Ran cargo test --locked --test next -- --exact at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs for TC-cli-observation-cap-refuses-before-writing: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-cli-observation-cap-refuses-before-writing.md)
- [Ran cargo test --locked --test next -- --exact at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs for TC-cli-observation-cap-refuses-before-writing: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-observation-cap-refuses-before-writing-578f54.md)
- [Ran cargo test --locked --test next -- --exact at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs for TC-cli-observation-cap-refuses-before-writing: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-observation-cap-refuses-before-writing-ffac2d.md)
- [The full test suite passed 731 of 731 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 31f85795, PR 2 with main merged in; clippy --all-targets -D warnings and rustfmt --check were clean](evidence/2026-11-13-the-full-test-suite-passed-731-of-731-across-48-targets-cargo.md)

## Open threads

## Timeline

- 2026-11-13T02:08:33Z · start · intent → intent · 0.6.1: at the 400-observation cap, every write that adds an observation refuses before writing anything and names chough session shelve ^t-e5d776
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:08:33Z · gate-opened · intent → intent · Approve the intent: at the observation cap, writes refuse before writing and name the recovery ^t-e293d7
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T02:08:33Z · gate-closed · intent → intent · Approve the intent: at the observation cap, writes refuse before writing and name the recovery ^t-6752fa
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Observation cap: cases run and evidence add refuse at 400 before writing anything, and name chough session shelve; next stops counting unlinked entries as a case's latest run'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T02:08:33Z · advance · intent → design · Leif approved 0.6.1's PRs (02:03Z, relayed by t370) ^t-fc8207
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:08:33Z · skip · design → plan · No visuals or interactions change ^t-64f1ed
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:09:08Z · gate-opened · plan → plan · Approve the scope of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough ^t-9b7c49
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T02:09:08Z · gate-closed · plan → plan · Approve the scope of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough ^t-f963d4
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Observation cap: cases run and evidence add refuse at 400 before writing anything, and name chough session shelve; next stops counting unlinked entries as a case's latest run'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:09:10Z · advance · plan → build · Leif approved the scope (02:03Z, relayed by t370) ^t-7ddb33
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - gate: native:approved
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:43:27Z · advance · build → test · The fix is green ^t-99314b
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - gate: native:approved
  - revision: 31f85795b0f2fb2c89a8ffc4bc0dee8728ca1178
  - by: claude-code (hc t382)
- 2026-11-13T02:43:27Z · advance · test → accept · The fix is green ^t-e17582
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - gate: native:approved
  - revision: 31f85795b0f2fb2c89a8ffc4bc0dee8728ca1178
  - by: claude-code (hc t382)
- 2026-11-13T02:59:08Z · direct-native-write · accept → accept · 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-778ef6
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - revision: 31f85795b0f2fb2c89a8ffc4bc0dee8728ca1178
  - direct_write:
    - change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: claude-code (hc t382)
- 2026-11-13T02:59:08Z · back · accept → plan · Leif chose to fix the file list and keep his approvals (02:53Z, relayed by t370) ^t-6d39e3
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - revision: 31f85795b0f2fb2c89a8ffc4bc0dee8728ca1178
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, accept: native verification and the human implementation review, checkpoint references: 1 case(s) and 2 observation(s) to re-check with `chough status`
  - by: claude-code (hc t382)
- 2026-11-13T02:59:09Z · gate-opened · plan → plan · Approve the scope of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough with its corrected file list ^t-337edb
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - revision: 31f85795b0f2fb2c89a8ffc4bc0dee8728ca1178
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T02:59:10Z · gate-closed · plan → plan · Approve the scope of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough with its corrected file list ^t-2eda29
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - revision: 31f85795b0f2fb2c89a8ffc4bc0dee8728ca1178
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended) / Yes, fix and keep (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Observation cap: cases run and evidence add refuse at 400 before writing anything, and name chough session shelve; next stops counting unlinked entries as a case's latest run'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:59:11Z · advance · plan → build · Leif kept his approval with the corrected file list (02:53Z, relayed by t370) ^t-8f60c4
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - gate: native:approved
  - revision: 31f85795b0f2fb2c89a8ffc4bc0dee8728ca1178
  - by: claude-code (hc t382)
- 2026-11-13T02:59:11Z · advance · build → test · Leif kept his approval with the corrected file list (02:53Z, relayed by t370) ^t-29fdca
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - gate: native:approved
  - revision: 31f85795b0f2fb2c89a8ffc4bc0dee8728ca1178
  - by: claude-code (hc t382)
- 2026-11-13T02:59:11Z · advance · test → accept · Leif kept his approval with the corrected file list (02:53Z, relayed by t370) ^t-ef7053
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - gate: native:approved
  - revision: 31f85795b0f2fb2c89a8ffc4bc0dee8728ca1178
  - by: claude-code (hc t382)
- 2026-11-13T03:00:00Z · gate-opened · accept → accept · Implementation review of 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough ^t-eb3355
  - native_change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
  - revision: d247ac6b17a0a48cfa00ece86730ed758d212389
  - holds: accept
  - drives: review
  - by: claude-code (hc t382)
