# Lesson bundle: fix-262

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 262
- **Kind**: BugFix
- **Specs**: cli, history
- **Paths**: src/records.rs, src/evidence.rs, src/sessions.rs, tests/special_files.rs, .chough/cases/history.non-regular-files.cases.toml, changelog.d/2026-11-18-fix-262.md
- **Acceptance**: With a FIFO named like a bug record in .chough/bugs/, chough status, chough next and chough bug list each exit 0 without blocking. With a FIFO in place of an open session's session.md, chough session list reports that session as not readable with a reason and the recovery chough session shelve <id>, another session's commands still work, and chough session shelve <id> moves the folder without blocking.

## Evidence

- Verification commit: `5418a53e82a53aee9f14f37435e6fdb6a41271e6`
- Base commit: `8abb5bb89b5244e72327e1ea6abfc7e66634ebe1`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

GitHub #262: a FIFO named like a bug record in `.chough/bugs/` makes `chough status` and `chough next` block for ever, while `chough bug list` answers. A FIFO in place of an open session's `session.md` makes that session vanish from `chough session list`, with no reason and no recovery named. Both reproduce on main at 8abb5bb.

REQ-history-006 already says a session that cannot be read stops no other command and that `session list` reports it with the reason and the recovery `chough session shelve <id>`. The session part of this change makes the code do that for a record that is not a regular file. For the bugs folder no requirement names non-regular files. `chough bug list` already skips them, and the readers `status` and `next` use did not, so this change makes those readers skip them the same way. Whether skipping is the right answer there, rather than refusing with the path named, is put to Leif at the scope gate.

Out of scope: `chough doctor` does not report such a file. A new doctor finding is new behaviour and needs a sentence in REQ-history-012 first.

## From the change's design.md

# Design

Opening a FIFO for reading blocks until a writer appears, so the fix is to never open one.

1. `read_bounded_record` in `src/records.rs` is the one reader every Chough record goes through. It now checks that the path is a regular file before it opens it and refuses otherwise, naming the path. This alone ends every hang.
2. The three folder readers in `src/evidence.rs` that `status` and `next` use (receipts of bugs and stories, the observation index, the receipt views) skip an entry that is not a regular file. This matches the bug readers in `src/bugs.rs`, so `status`, `next` and `bug list` agree that such an entry is not a record.
3. `open_ids` and `active` in `src/sessions.rs` asked whether `session.md` is a regular file, so a folder with a FIFO there was not a session at all. They now ask whether it exists. The folder is then a session whose record cannot be read, and the existing path in `session list` reports it with the reason and `chough session shelve <id>`.
4. `shelve` read `session.md` to describe it. It now reads it only when it is a regular file, and the shelved note lists a non-regular entry as `not a regular file, not read` instead of as a symbolic link.

Rejected: opening with `O_NONBLOCK`. It needs platform-specific code and still leaves a device or directory to handle; a metadata check covers all of them.

Risk: a session folder whose `session.md` is a directory or a device is now listed as not readable where it was ignored before. That is what REQ-history-006 asks for.

## From the change's testing.md

# Testing

`cargo test --locked --test special_files` (Unix only) runs each command with a 20 second deadline, so a blocked reader fails the test instead of hanging the suite. `a_fifo_in_the_bugs_folder_blocks_no_reader` puts a FIFO in `.chough/bugs/` and checks that `status`, `next`, `next --json` and `bug list` exit 0. `a_session_whose_record_is_a_fifo_is_listed_with_the_reason_and_recovery` replaces an open session's `session.md` with a FIFO and checks that `session list` reports it as not readable with a reason and `chough session shelve <id>`, that the other session's `next` and `status` work, and that the shelve moves the folder and its note names the entry as not a regular file. Both fail on main: the first by the deadline, the second because the session is not listed.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
