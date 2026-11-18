---
kind: session
id: 2026-11-18-fix-272-uninitialised-root
aliases: [fix-272-uninitialised-root]
objective: "journal add holds the writer lock in a project with no .chough directory too, so parallel posts all reach the feed there (REQ-cli-040, review of #302)"
intent_scope: ""
modules: []
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: fix-272-uninitialised-root
pr: ""
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
cases: [TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory]
design: []
files: [src/journal.rs, tests/journal.rs]
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Scope approval of fix-272-uninitialised-root
    holds: plan
    drives: approve
    opened: 2026-11-18T01:57:28Z
run_consents: []
next: Wait for Leif's scope decision on fix-272-uninitialised-root
---
# journal add holds the writer lock in a project with no .chough directory too, so parallel posts all reach the feed there (REQ-cli-040, review of #302)

## Goal

journal add holds the writer lock in a project with no .chough directory too, so parallel posts all reach the feed there (REQ-cli-040, review of #302)

## Where it stands

- Failing tests first on main at 7aed03f: no lock file after journal add in a project with no .chough directory, and a storm round lost posts
- journal add calls the lock directly; all 23 journal tests pass

## Decisions and why

- Leif, 2026-11-17, answering Fable's question on the agent review of #302: 'Fix forward: lock'. journal add takes the writer lock in a project with no .chough directory too; refusing such a project and reverting 8083df5 were the rejected options.
- No requirement text changes (--no-spec-change): REQ-cli-040 names journal add with no exception for an uninitialised project.

## What was verified

## Open threads

## Timeline

- 2026-11-18T01:54:22Z · start · intent → intent · journal add holds the writer lock in a project with no .chough directory too, so parallel posts all reach the feed there (REQ-cli-040, review of #302) ^t-1d8db6
  - revision: 7aed03f6993eef37826de783c84ccb4d9ab52648
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T01:54:22Z · advance · intent → design ·  ^t-ca053d
  - revision: 7aed03f6993eef37826de783c84ccb4d9ab52648
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T01:54:22Z · skip · design → plan · CLI locking change; nothing visual or interactive changes ^t-c1b0bb
  - revision: 7aed03f6993eef37826de783c84ccb4d9ab52648
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T01:57:28Z · gate-opened · plan → plan · Scope approval of fix-272-uninitialised-root ^t-41c559
  - native_change: fix-272-uninitialised-root
  - revision: 7aed03f6993eef37826de783c84ccb4d9ab52648
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
