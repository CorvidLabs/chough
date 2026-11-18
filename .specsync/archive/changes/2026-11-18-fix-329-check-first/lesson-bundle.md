# Lesson bundle: fix-329-check-first

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 329 check first
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/bugs.rs, src/checkpoint.rs, tests/bugs.rs, docs/cli.md, changelog.d/2026-11-18-fix-329.md, .chough/cases/cli.bug-joins-the-active-session.cases.toml
- **Acceptance**: With a session active that already holds its 200 bug links, chough bug add refuses before writing anything, naming the session and the limit, and no new .chough/bugs record exists afterwards; the session record is byte-identical. With room left, bug add writes the bug and links it as before.

## Evidence

- Verification commit: `44aea719807f00be690d400d35211f89eb231131`
- Base commit: `2e371c8855e4ea8ca965634b59c56c26b0329d06`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

A follow-up to fix-329 (#343), from Leif's P2 review comment on #343: with fix-329, `chough bug add` while a session is active always joins the new bug to that session, but it does so only after `create` has written the bug record. With the active session already holding its 200 bug links, a new `bug add` wrote a 201st `.chough/bugs` record, then failed the join with "too many bug (limit 200)": the session stayed byte-identical and the new bug was left unlinked. That breaks REQ-cli-012's auto-link and leaves a failed command partly applied.

The standing rule Leif gave for 2026-11-18, "Yes, for today", covers a real gap his review finds in an already-approved PR: a small follow-up change inside the approved scope, a tester's glance, then hc t4c resolves the thread. hc t4c added the design point: check first and refuse with nothing written, as in #312, #340 and #332, rather than removing the file afterwards (#306's lesson).

## From the change's design.md

# Design

`bug add` already holds the project lock from its start. Before `create` writes anything, it now reads the active session, as the join does, and, unless no session is active, checks the session with the new bug's link added: the same validation and rendering `save_checkpoint` runs, with nothing written. A session that cannot take it refuses the add, naming the session, the limit and how to go on, with every file unchanged. The join after `create` is unchanged; under the held lock nothing can fill the session in between.

The check uses a placeholder ID of the longest shape `create` can give the bug, so a name suffixed on collision cannot make the real save larger than the one checked.

## From the change's testing.md

# Testing

`tests/bugs.rs::bug_add_refuses_before_writing_when_the_session_is_full`, red on fix-329 before this change: with the active session holding 200 bug links, `bug add` refuses naming the session and the limit, no new `.chough/bugs` record exists, and every file is unchanged. The fix-329 tests of the join still pass.

## Where these lessons go

- `specs/cli/cli.3md`
