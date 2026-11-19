---
id: 2026-11-18-evidence-add-cases-run-and-cases-record-re-read-session-md
title: evidence add, cases run and cases record re-read session.md under…
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T17:03:45Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-317.md
  - .specsync/archive/changes/2026-11-18-fix-317/
record: .chough/archive/sessions/2026-11-18-fix-317.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# evidence add, cases run and cases record re-read session.md under…

evidence add, cases run and cases record re-read session.md under the writer lock, so parallel writers lose no update (REQ-cli-040, #317)

Why: No requirement text changes: REQ-cli-040 already requires these writers to re-read under the writer lock. The lock is taken in the shared evidence save after any case has run, so cases run still runs its case without holding it.

Verified: Ran cargo test --locked --test evidence -- --exact bug_add_holds_the_writer_lock_and_keeps_another_writers_update for TC-cli-bug-add-holds-the-writer-lock: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-317.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-317/)
