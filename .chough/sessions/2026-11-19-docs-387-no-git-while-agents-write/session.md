---
kind: session
id: 2026-11-19-docs-387-no-git-while-agents-write
aliases: [docs-387-no-git-while-agents-write]
objective: The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)
intent_scope: ""
modules: []
stage: plan
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
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Scope approval of docs-387-no-git-while-agents-write\" closed (approved); run `chough stage advance` to move on."
---
# The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)

## Goal

The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of docs-387-no-git-while-agents-write": "Yes I approve" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53), from a Claude Code chat message on 2026-11-19 at about 14:37Z, on docs-387-no-git-while-agents-write at 0892b125: the one sentence and criterion in REQ-cli-040 and the skill digest in src/skill.rs, approved as drafted); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve docs-387-no-git-while-agents-write as 0xLeif

## What was verified

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
