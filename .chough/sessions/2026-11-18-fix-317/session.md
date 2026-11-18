---
kind: session
id: 2026-11-18-fix-317
aliases: [fix-317]
objective: "evidence add, cases run and cases record re-read session.md under the writer lock, so parallel writers lose no update (REQ-cli-040, #317)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-317
pr: ""
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
cases: [TC-cli-evidence-writers-reread-under-the-lock, TC-cli-bug-add-holds-the-writer-lock]
design: []
files: []
observations:
  - 2026-11-18-tc-cli-evidence-writers-reread-under-the-lock
  - 2026-11-18-tc-cli-bug-add-holds-the-writer-lock
  - 2026-11-18-tc-cli-evidence-writers-reread-under-the-lock-310eeb
  - 2026-11-18-tc-cli-bug-add-holds-the-writer-lock-cd5e75
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-317
    holds: accept
    drives: review
    opened: 2026-11-18T16:27:01Z
run_consents: []
next: Continue accept
---
# evidence add, cases run and cases record re-read session.md under the writer lock, so parallel writers lose no update (REQ-cli-040, #317)

## Goal

evidence add, cases run and cases record re-read session.md under the writer lock, so parallel writers lose no update (REQ-cli-040, #317)

## Where it stands

- Failing tests first: evidence_add_rereads_the_session_under_the_writer_lock and parallel_decisions_and_evidence_adds_all_land failed on main (lost updates); the evidence save now reads the session under the writer lock and both pass. bug add already takes the lock at its start, so its test is a guard that passes on main, and src/bugs.rs is unchanged

## Decisions and why

- No requirement text changes: REQ-cli-040 already requires these writers to re-read under the writer lock. The lock is taken in the shared evidence save after any case has run, so cases run still runs its case without holding it.
- 0xLeif decided "Scope approval of fix-317": "File it, fixer's A list" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g), 2026-11-18: his choice for #317, given with his scope approval to take the lock before reading, with a failing parallel test first); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-317 as 0xLeif
- Leif chose 'Recreate the draft' (relayed by hc t4c, 2026-11-18): the first fix-317 draft, approved earlier in this session, was never committed and did not declare src/bugs.rs, so it was removed and recreated with src/bugs.rs declared and the same rationale. bug add and bug update --observation are now in scope under REQ-cli-040, as a commit of their own. The earlier scope-approval entry in this Timeline belongs to the removed draft.
- 0xLeif decided "Scope approval of fix-317, recreated with bug add and bug update": "Yes, include them" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g, later 'fixer'), 2026-11-18: his answer to including bug add/update --observation (bugs.rs join_active_session, REQ-cli-040) in #317, added to his earlier 'File it, fixer's A list' approval of #317, with his choice 'Recreate the draft'); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-317 as 0xLeif

## What was verified

- [Ran cargo test --locked --test evidence -- --exact evidence_add_rereads_the_session_under_the_writer_lock for TC-cli-evidence-writers-reread-under-the-lock: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-evidence-writers-reread-under-the-lock.md)
- [Ran cargo test --locked --test evidence -- --exact bug_add_holds_the_writer_lock_and_keeps_another_writers_update for TC-cli-bug-add-holds-the-writer-lock: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-bug-add-holds-the-writer-lock.md)
- [Ran cargo test --locked --test evidence -- --exact evidence_add_rereads_the_session_under_the_writer_lock for TC-cli-evidence-writers-reread-under-the-lock: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-evidence-writers-reread-under-the-lock-310eeb.md)
- [Ran cargo test --locked --test evidence -- --exact bug_add_holds_the_writer_lock_and_keeps_another_writers_update for TC-cli-bug-add-holds-the-writer-lock: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-bug-add-holds-the-writer-lock-cd5e75.md)

## Open threads

## Timeline

- 2026-11-18T15:49:46Z · start · intent → intent · evidence add, cases run and cases record re-read session.md under the writer lock, so parallel writers lose no update (REQ-cli-040, #317) ^t-bea69a
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:49:46Z · advance · intent → design ·  ^t-364275
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:49:46Z · skip · design → plan · Writer-lock ordering in the evidence save; nothing visual or interactive changes ^t-b73879
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:50:41Z · gate-opened · plan → plan · Scope approval of fix-317 ^t-28497f
  - native_change: fix-317
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:50:41Z · gate-closed · plan → plan · Scope approval of fix-317 ^t-02ede9
  - native_change: fix-317
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - relayed_by:
    - decided_by: 0xLeif
    - words: File it, fixer's A list
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g), 2026-11-18: his choice for #317, given with his scope approval to take the lock before reading, with a failing parallel test first
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-317
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:24:45Z · gate-opened · plan → plan · Scope approval of fix-317, recreated with bug add and bug update ^t-a9b87e
  - native_change: fix-317
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T16:24:45Z · gate-closed · plan → plan · Scope approval of fix-317, recreated with bug add and bug update ^t-35e06e
  - native_change: fix-317
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, include them
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g, later 'fixer'), 2026-11-18: his answer to including bug add/update --observation (bugs.rs join_active_session, REQ-cli-040) in #317, added to his earlier 'File it, fixer's A list' approval of #317, with his choice 'Recreate the draft'
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-317
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T16:24:57Z · advance · plan → build · native:approved ^t-131dc5
  - native_change: fix-317
  - gate: native:approved
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T16:26:51Z · advance · build → test · native:approved ^t-fe5a74
  - native_change: fix-317
  - gate: native:approved
  - revision: caa810e4ef02f64753c2d8ddb1bd0b61a9e2c25d
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T16:27:00Z · advance · test → accept · native:approved ^t-fb6883
  - native_change: fix-317
  - gate: native:approved
  - revision: caa810e4ef02f64753c2d8ddb1bd0b61a9e2c25d
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T16:27:01Z · gate-opened · accept → accept · Implementation review of fix-317 ^t-4447e6
  - native_change: fix-317
  - revision: caa810e4ef02f64753c2d8ddb1bd0b61a9e2c25d
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
