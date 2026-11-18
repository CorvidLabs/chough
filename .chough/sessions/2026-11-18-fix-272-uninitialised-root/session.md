---
kind: session
id: 2026-11-18-fix-272-uninitialised-root
aliases: [fix-272-uninitialised-root]
objective: "journal add holds the writer lock in a project with no .chough directory too, so parallel posts all reach the feed there (REQ-cli-040, review of #302)"
intent_scope: ""
modules: []
stage: accept
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
observations: [2026-11-18-tc-cli-journal-add-holds-the-writer-lock-with-no-chough]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-272-uninitialised-root
    holds: accept
    drives: review
    opened: 2026-11-18T02:09:01Z
run_consents: []
next: Wait for Leif's implementation review of fix-272-uninitialised-root; read the PR's reviews first; do not ship
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
- 0xLeif decided "Scope approval of fix-272-uninitialised-root": "approve all three, record as 0xLeif" (Claude Code session with Fable, 2026-11-17, in reply to Fable's report of the gates on #308, #305 and #306); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-272-uninitialised-root as 0xLeif

## What was verified

- [Ran cargo test --locked --test journal -- --exact journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory for TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-journal-add-holds-the-writer-lock-with-no-chough.md)

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
- 2026-11-18T02:05:50Z · gate-closed · plan → plan · Scope approval of fix-272-uninitialised-root ^t-a9f12d
  - native_change: fix-272-uninitialised-root
  - revision: ea5224b381a4b9ce0e51c61cda419fb848ab46d2
  - relayed_by:
    - decided_by: 0xLeif
    - words: approve all three, record as 0xLeif
    - where: Claude Code session with Fable, 2026-11-17, in reply to Fable's report of the gates on #308, #305 and #306
    - recorded_by: fable-t4b
    - native: specsync change approve fix-272-uninitialised-root
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T02:08:59Z · advance · plan → build · native:approved ^t-4a8e15
  - native_change: fix-272-uninitialised-root
  - gate: native:approved
  - revision: ea5224b381a4b9ce0e51c61cda419fb848ab46d2
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T02:08:59Z · advance · build → test · native:approved ^t-0a988c
  - native_change: fix-272-uninitialised-root
  - gate: native:approved
  - revision: ea5224b381a4b9ce0e51c61cda419fb848ab46d2
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T02:08:59Z · advance · test → accept · native:approved ^t-bde05f
  - native_change: fix-272-uninitialised-root
  - gate: native:approved
  - revision: ea5224b381a4b9ce0e51c61cda419fb848ab46d2
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T02:09:01Z · gate-opened · accept → accept · Implementation review of fix-272-uninitialised-root ^t-9f8db0
  - native_change: fix-272-uninitialised-root
  - revision: ea5224b381a4b9ce0e51c61cda419fb848ab46d2
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
