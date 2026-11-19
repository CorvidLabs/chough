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
native_change: ""
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
next: Read scoped intent and native status, then choose the next action within the authorized scope.
---
# The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)

## Goal

The docs and the workflow skill tell agents not to run git merge, rebase, checkout or stash in a worktree while Chough writers run there, since Git can drop writes that already exited 0 (#387)

## Where it stands

## Decisions and why

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
