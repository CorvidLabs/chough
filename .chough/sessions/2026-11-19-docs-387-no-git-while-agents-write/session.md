---
kind: session
id: 2026-11-19-docs-387-no-git-while-agents-write
aliases: [docs-387-no-git-while-agents-write]
objective: The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: docs-387-no-git-while-agents-write
pr: ""
criteria: [DRIVE-11, DRIVE-3]
requirements: [REQ-cli-040]
cases: []
design: []
files: []
observations: [2026-11-20-tc-cli-the-docs-say-not-to-rewrite-the-worktree-while-another]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue build
---
# The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)

## Goal

The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of docs-387-no-git-while-agents-write": "Yes I approve" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53), from a Claude Code chat message on 2026-11-19 at about 14:37Z, on docs-387-no-git-while-agents-write at 0892b125: the one sentence and criterion in REQ-cli-040 and the skill digest in src/skill.rs, approved as drafted); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve docs-387-no-git-while-agents-write as 0xLeif
- 0xLeif decided "One send-back of docs-387-no-git-while-agents-write to correct its stale acceptance-criteria answer, run by t4b": "a, all yes.. im back" (Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 00:38Z on 2026-11-20, answering t4c's eight-item round; item 7: yes to one send-back of #387 before any code, run by t4b, correcting only the acceptance-criteria answer to match the delta he approved at 0892b125 (git pull named; a lone agent may run Git between its own commands), with no delta word changed); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve docs-387-no-git-while-agents-write as 0xLeif
- 0xLeif decided "Scope approval of docs-387-no-git-while-agents-write with its corrected acceptance-criteria answer (native approval)": "Yes, record it" (His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, to recording this re-approval. Scope: his 'Yes I approve' of the delta at 0892b125 (relayed by hc t4c through fable-lead, 2026-11-19), delta unchanged. Send-back: his 'a, all yes.. im back' (relayed the same way, about 00:38Z on 2026-11-20, item 7), one send-back by t4b correcting only the acceptance-criteria answer); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve docs-387-no-git-while-agents-write as 0xLeif

## What was verified

- [Ran cargo test --locked --test docs -- --exact the_docs_say_not_to_rewrite_the_worktree_while_another_agent_works for TC-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-agent-works: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-the-docs-say-not-to-rewrite-the-worktree-while-another.md)

## Open threads

## Timeline

- 2026-11-19T14:34:18Z · start · intent → intent · The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387) ^t-961b5f
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:34:18Z · advance · intent → design ·  ^t-d95e39
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:34:18Z · skip · design → plan · Guidance text in the docs and the skill; nothing visual or interactive changes ^t-9fb446
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:37:52Z · gate-opened · plan → plan · Scope approval of docs-387-no-git-while-agents-write ^t-49bda7
  - native_change: docs-387-no-git-while-agents-write
  - revision: 0892b125ba3a7c857efbb4f7c14f02ee290a3f5e
  - holds: plan
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:37:53Z · gate-closed · plan → plan · Scope approval of docs-387-no-git-while-agents-write ^t-2c9611
  - native_change: docs-387-no-git-while-agents-write
  - revision: 0892b125ba3a7c857efbb4f7c14f02ee290a3f5e
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes I approve
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53), from a Claude Code chat message on 2026-11-19 at about 14:37Z, on docs-387-no-git-while-agents-write at 0892b125: the one sentence and criterion in REQ-cli-040 and the skill digest in src/skill.rs, approved as drafted
    - recorded_by: fable-t4b
    - native: specsync change approve docs-387-no-git-while-agents-write
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:40:48Z · gate-opened · plan → plan · One send-back of docs-387-no-git-while-agents-write to correct its stale acceptance-criteria answer, run by t4b ^t-828be7
  - native_change: docs-387-no-git-while-agents-write
  - revision: 90f0c2263c81d737eace1d28583d9f2b9e372a2f
  - holds: plan
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:40:48Z · gate-closed · plan → plan · One send-back of docs-387-no-git-while-agents-write to correct its stale acceptance-criteria answer, run by t4b ^t-dbaead
  - native_change: docs-387-no-git-while-agents-write
  - revision: 90f0c2263c81d737eace1d28583d9f2b9e372a2f
  - relayed_by:
    - decided_by: 0xLeif
    - words: a, all yes.. im back
    - where: Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 00:38Z on 2026-11-20, answering t4c's eight-item round; item 7: yes to one send-back of #387 before any code, run by t4b, correcting only the acceptance-criteria answer to match the delta he approved at 0892b125 (git pull named; a lone agent may run Git between its own commands), with no delta word changed
    - recorded_by: fable-t4b
    - native: specsync change approve docs-387-no-git-while-agents-write
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:43:41Z · gate-opened · plan → plan · Scope approval of docs-387-no-git-while-agents-write with its corrected acceptance-criteria answer (native approval) ^t-6e0dd8
  - native_change: docs-387-no-git-while-agents-write
  - revision: 90f0c2263c81d737eace1d28583d9f2b9e372a2f
  - holds: plan
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:43:41Z · direct-native-write · plan → plan · docs-387-no-git-while-agents-write's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-100594
  - native_change: docs-387-no-git-while-agents-write
  - revision: 90f0c2263c81d737eace1d28583d9f2b9e372a2f
  - direct_write:
    - change: docs-387-no-git-while-agents-write
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:43:41Z · gate-closed · plan → plan · Scope approval of docs-387-no-git-while-agents-write with its corrected acceptance-criteria answer (native approval) ^t-71bf21
  - native_change: docs-387-no-git-while-agents-write
  - revision: 90f0c2263c81d737eace1d28583d9f2b9e372a2f
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, record it
    - where: His direct answer in t4b's Claude Code session (AskUserQuestion), 2026-11-20, to recording this re-approval. Scope: his 'Yes I approve' of the delta at 0892b125 (relayed by hc t4c through fable-lead, 2026-11-19), delta unchanged. Send-back: his 'a, all yes.. im back' (relayed the same way, about 00:38Z on 2026-11-20, item 7), one send-back by t4b correcting only the acceptance-criteria answer
    - recorded_by: fable-t4b
    - native: specsync change approve docs-387-no-git-while-agents-write
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-20T00:46:00Z · advance · plan → build · native:approved ^t-a98dc9
  - native_change: docs-387-no-git-while-agents-write
  - gate: native:approved
  - revision: 347d9344a245aa3df6366cf2a63ff3903c900f56
  - by: fable (claude-opus-5-5, hc t4b)
