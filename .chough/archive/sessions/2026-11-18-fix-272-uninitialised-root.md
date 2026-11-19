---
kind: session
id: 2026-11-18-fix-272-uninitialised-root
aliases: [fix-272-uninitialised-root]
objective: "journal add holds the writer lock in a project with no .chough directory too, so parallel posts all reach the feed there (REQ-cli-040, review of #302)"
stage: done
stage_status: archived
iteration: 1
native_change: fix-272-uninitialised-root
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
cases: [TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory]
files: [src/journal.rs, tests/journal.rs]
observations:
  - 2026-11-18-tc-cli-journal-add-holds-the-writer-lock-with-no-chough
  - 2026-11-18-tc-cli-journal-add-holds-the-writer-lock-with-no-chough-2cdcfe
title: journal add holds the writer lock with no .chough directory
summary:
  - "In a project with no .chough directory, journal add took no lock, so parallel posts could still miss the feed after #302. It now takes the checkpoint lock directly, as REQ-cli-040 requires. Two tests failed first; the verify lane passed on macOS after main (5408c44) was merged in. Linux was not run locally."
closed: 2026-11-18
outcome: done
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
- 0xLeif decided "Implementation review of fix-272-uninitialised-root": "Pass (Recommended)" (Claude Code session with Fable, 2026-11-17, answer to the interview question 'Do you pass the implementation review of fix-272-uninitialised-root?' after the agent review passed 552d236); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change review fix-272-uninitialised-root as 0xLeif with verdict pass

## What was verified

- 1 earlier run of the cases below is kept in this story's receipts, not listed here.
- Ran cargo test --locked --test journal -- --exact journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory for TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory: passed (exit 0, tests run 1). - 02fc65ea289e ^2026-11-18-tc-cli-journal-add-holds-the-writer-lock-with-no-chough-2cdcfe

## Bugs

None.

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
- 2026-11-18T05:59:22Z · gate-closed · accept → accept · Implementation review of fix-272-uninitialised-root ^t-af7335
  - native_change: fix-272-uninitialised-root
  - revision: 46c9080ba4055008f83b39ca022bad46ff9a2b65
  - relayed_by:
    - decided_by: 0xLeif
    - words: Pass (Recommended)
    - where: Claude Code session with Fable, 2026-11-17, answer to the interview question 'Do you pass the implementation review of fix-272-uninitialised-root?' after the agent review passed 552d236
    - recorded_by: fable-t4b
    - native: specsync change review fix-272-uninitialised-root
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T05:59:23Z · advance · accept → done · native:archived ^t-bf12aa
  - native_change: fix-272-uninitialised-root
  - gate: native:archived
  - revision: 46c9080ba4055008f83b39ca022bad46ff9a2b65
  - by: fable (claude-fable-5-2, hc t4b)

Closed 2026-11-18: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-18-tc-cli-journal-add-holds-the-writer-lock-with-no-chough
    claim: "Ran cargo test --locked --test journal -- --exact journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory for TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory
    revision: ea5224b381a4b9ce0e51c61cda419fb848ab46d2
    environment: local-macos-arm64
    content: 1333c626b2a2b43886fb834482f27a2b7b636e7a76b89f250aab4ae71ddd56bd
    fingerprint: sha256:792644d7e2ef59bd93975d2d885b8f3bf166496f7ae483275199118179c57067
    case: TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory
    case_digest: 28f1b67751997c8ab2317cd79e35944b6200f2e7c4ea66e276b90685dc273abd
    observed: 2026-11-18T02:09:01Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
  - id: 2026-11-18-tc-cli-journal-add-holds-the-writer-lock-with-no-chough-2cdcfe
    claim: "Ran cargo test --locked --test journal -- --exact journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory for TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test journal -- --exact journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory
    revision: e157bdd269e733a5b42eb453598c629bc4565065
    environment: local-macos-arm64
    content: fee319a43a0114520f76b43153ba65fe2ff77388873fc8d06a9cc045d7745f8c
    fingerprint: sha256:02fc65ea289e12ec0c736e2732a62fd049d814595f0dfd1685c7bdbe9500daca
    case: TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory
    case_digest: 28f1b67751997c8ab2317cd79e35944b6200f2e7c4ea66e276b90685dc273abd
    observed: 2026-11-18T05:58:17Z
    duration_seconds: 1
    actor: fable (claude-fable-5-2, hc t4b)
```
