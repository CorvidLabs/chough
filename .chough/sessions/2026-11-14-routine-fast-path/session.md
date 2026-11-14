---
kind: session
id: 2026-11-14-routine-fast-path
aliases: [routine-fast-path]
objective: Give agents, local models included, a short fast path through the workflow skill for routine changes (LIFE-22)
intent_scope: LIFE
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
pr: ""
criteria: [LIFE-22]
requirements: []
cases: []
design: []
files: []
observations: [2026-11-14-tc-cli-skill-routine-fast-path]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
---
# Give agents, local models included, a short fast path through the workflow skill for routine changes (LIFE-22)

## Goal

Give agents, local models included, a short fast path through the workflow skill for routine changes (LIFE-22)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required": "0xLeif ran: specsync change approve workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required --actor 0xLeif" (0xLeif's own terminal in the Claude Code desktop app, 2026-11-14; the native approval is already recorded); recorded by claude-code; a relayed claim, not authenticated; ran specsync change approve workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required as 0xLeif
- 0xLeif decided "Implementation review of workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required": "i approve" (Claude Code desktop chat, 2026-11-14, answering that PR #285 was ready for review); recorded by claude-code; a relayed claim, not authenticated; ran specsync change review workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test package -- --exact the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change for TC-cli-skill-routine-fast-path: passed (exit 0, tests run 1).](evidence/2026-11-14-tc-cli-skill-routine-fast-path.md)

## Open threads

## Timeline

- 2026-11-14T15:48:56Z · start · intent → intent · Give agents, local models included, a short fast path through the workflow skill for routine changes (LIFE-22) ^t-74ee97
  - revision: ddc56e96ff77cb0313d7591bee61d5a4a2950f0e
  - by: claude-code
- 2026-11-14T15:49:09Z · advance · intent → design · Intent is LIFE-22 (local agents and any Markdown-reading agent should work), already captured in hi/life.md; no new wording needed. ^t-0e26aa
  - revision: ddc56e96ff77cb0313d7591bee61d5a4a2950f0e
  - by: claude-code
- 2026-11-14T15:49:14Z · skip · design → plan · Skill text and its digests only; no visual or interaction change. ^t-1e87d4
  - revision: ddc56e96ff77cb0313d7591bee61d5a4a2950f0e
  - by: claude-code
- 2026-11-14T15:54:00Z · gate-opened · plan → plan · Scope approval of workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required ^t-cb2531
  - revision: ddc56e96ff77cb0313d7591bee61d5a4a2950f0e
  - holds: plan
  - drives: approve
  - by: claude-code
- 2026-11-14T15:59:24Z · direct-native-write · plan → plan · workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state draft → approved ^t-a39c7e
  - native_change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
  - revision: 1ba9f702814a7d879cb9786c34bf5ce19d88e476
  - direct_write:
    - change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: draft
    - after_state: approved
  - by: claude-code
- 2026-11-14T15:59:24Z · gate-closed · plan → plan · Scope approval of workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required ^t-b90958
  - native_change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
  - revision: 1ba9f702814a7d879cb9786c34bf5ce19d88e476
  - relayed_by:
    - decided_by: 0xLeif
    - words: 0xLeif ran: specsync change approve workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required --actor 0xLeif
    - where: 0xLeif's own terminal in the Claude Code desktop app, 2026-11-14; the native approval is already recorded
    - recorded_by: claude-code
    - native: specsync change approve workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
    - actor: 0xLeif
  - by: claude-code
- 2026-11-14T15:59:41Z · advance · plan → build · Scope approved by 0xLeif; the implementation was drafted with the definition and is already in 1ba9f70. ^t-317528
  - native_change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
  - gate: native:approved
  - revision: 1ba9f702814a7d879cb9786c34bf5ce19d88e476
  - by: claude-code
- 2026-11-14T15:59:47Z · advance · build → test · Build is done in 1ba9f70; fmt, test and lint passed. ^t-a5a6f4
  - native_change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
  - gate: native:approved
  - revision: 94af1a8a87582e6ba23acb16bcf400fc7f2a06d7
  - by: claude-code
- 2026-11-14T16:03:14Z · advance · test → accept · Native check passed (b8f30a7), case TC-cli-skill-routine-fast-path passed, verify lane green. ^t-70369b
  - native_change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
  - gate: native:verifying
  - revision: e597cd3485ee2235f3faf00778acee7e9bd4a8ac
  - by: claude-code
- 2026-11-14T16:03:14Z · gate-opened · accept → accept · Implementation review of workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required ^t-4c5039
  - native_change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
  - revision: e597cd3485ee2235f3faf00778acee7e9bd4a8ac
  - holds: accept
  - drives: review
  - by: claude-code
- 2026-11-14T16:18:52Z · gate-closed · accept → accept · Implementation review of workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required ^t-8ba034
  - native_change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
  - revision: b4e8353959ad39566950dc2c14af75d5acf79166
  - relayed_by:
    - decided_by: 0xLeif
    - words: i approve
    - where: Claude Code desktop chat, 2026-11-14, answering that PR #285 was ready for review
    - recorded_by: claude-code
    - native: specsync change review workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
    - actor: 0xLeif
  - by: claude-code
- 2026-11-14T16:18:53Z · advance · accept → done · native:archived ^t-25fcaa
  - native_change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
  - gate: native:archived
  - revision: b4e8353959ad39566950dc2c14af75d5acf79166
  - by: claude-code
