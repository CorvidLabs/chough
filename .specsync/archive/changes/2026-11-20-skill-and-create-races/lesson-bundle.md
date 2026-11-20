# Lesson bundle: skill-and-create-races

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Skill and create races
- **Kind**: Feature
- **Specs**: cli, history
- **Paths**: src/skill.rs, src/records.rs, src/doctor.rs, src/main.rs, tests/cli.rs, tests/doctor.rs, tests/parallel.rs, docs/cli.md, changelog.d/2026-11-19-skill-and-create-races.md, .chough/cases/cli.skill-and-create-races.cases.toml, .chough/cases/history.skill-and-create-races.cases.toml, tests/skill_releases.rs
- **Acceptance**: A CRLF skill copy counts as the bundle file it equals and is refreshed with CRLF line breaks, and a file skill update cannot write is refused naming it, never assertion text (#373). Parallel skill update and init runs each succeed, or refuse only as a run on its own would (#386). Plain skill status and update skip a host folder that cannot be inspected, report it relative to the project and update the rest, exiting 0, while --host for it refuses (#403). chough doctor reports an untracked record under .chough/ that its reader refuses as unreadable, with the file and the reason (#408).

## Evidence

- Verification commit: `3b5890ae3edd848815a9271832beaaa2d846512a`
- Base commit: `9d260310bca8553a0203e5ef90bc8372b4e15aae`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issues #373 (lead-F5, F6), #386 (s3-F2, F7), #403 (from the check of #362) and #408 (t4b, on main), with fable-lead's addition of uncommitted unreadable records beside #408 (the testers' triage notes).

Measured on main (9d26031), after #362:
- #373: an unreadable skill file now reads as `edited` (#362), so the assertion text no longer comes from it; it still comes from the race (#386). A missing file in a read-only folder gives a bare `Permission denied (os error 13)` naming no file. A CRLF copy reads `edited` and is never updated. With a symlinked host folder, doctor reports `skill_uninspected` (#362) and next says nothing.
- #386: eight parallel `skill update` runs over missing files fail 0 to 7 of 8 with `File exists (os error 17)` or `Condition failed: write_new(...)`; six parallel `init --objective` runs fail 1 to 5 of 6 with a bare `File exists`; the end state is usable. The cause is `write_new_bytes`, which checks for the file and then opens it create-new, with no path in the error.
- #403: plain `skill status` and `skill update` exit 1 with `symlink is not an allowed source: <absolute path>/.codex/skills`; `--host openai` works.
- #408: doctor walks tracked files only, so an uncommitted session failing validation, an uncommitted 0-byte evidence entry and an uncommitted cut-off bug get no finding while `next` and `status` stop on them; committed ones are reported. A FIFO in an evidence folder stops `status` and `next` ("reading <file>") while doctor is silent: Git lists no FIFO as untracked, so the walk reads the folders, not Git's list.

Based on main: no requirement here is modified by #407, D1, L2 or #311. #408's sentence is a new requirement, not D1's REQ-history-012, so it does not wait on D1's send-back.

## From the change's design.md

# Design

- `src/records.rs` (cli): `write_new_bytes` treats a create-new that finds the file already there, between its check and its open, as "it exists" (`Ok(false)`), as the earlier check does, and names the file in every other error, relative to the project.
- `src/skill.rs` (cli): `file_state` compares a CRLF copy with the bundle once each CRLF is read as LF, and a refresh of such a copy writes CRLF line breaks; `report` keeps a host whose folder cannot be inspected as `uninspected`, with its relative directory and why, when no `--host` named it, and refuses naming it when one did; a missing file another run writes first is read again, counted as added when it holds the bundle's bytes and preserved as edited otherwise, so no `ensure!` text is ever printed.
- `src/main.rs` (cli): `init` reports a file another run wrote first as preserved, as it does a person's.
- `src/doctor.rs` (cli): the `unreadable` check also walks the folders the readers read (a session's folder, `evidence/`, the inbox, `bugs/`, the archived stories) with `symlink_metadata`, following no link, since Git lists no FIFO or socket; an entry Git does not track and does not ignore is read through `records::open_regular`, which opens without waiting and refuses a non-regular file on its handle, and a FIFO, socket or folder is reported by its type without being opened; the readers and the recovery are the committed records'.

## From the change's testing.md

# Testing

Failing tests first, each red on main first:
- `tests/cli.rs`: a CRLF copy of each bundle file reads as its LF state and is refreshed with CRLF; with `.codex/skills` a symlink to `../.agents/skills`, plain status and update exit 0, report codex `uninspected` relative and update `.agents`, while `--host codex` refuses; a missing file in a read-only folder is refused naming it; no output holds an absolute path.
- `tests/parallel.rs` (new): eight `skill update` runs at once over missing files and six `init --objective` runs at once in a fresh repository, repeated, with every run exiting as REQ-cli-001 and REQ-cli-016 say and no `Condition failed` or `File exists` in any output.
- `tests/doctor.rs`: an uncommitted session with an unknown stage, an uncommitted 0-byte evidence entry and an uncommitted cut-off bug each give `unreadable` with the file and reason; a FIFO, a socket and a folder at an evidence entry's name each give `unreadable` naming it, with doctor finishing within 10 seconds; a file under `.chough/local/` gives none.

One case file per module: `.chough/cases/cli.skill-and-create-races.cases.toml` (REQ-cli-016, REQ-cli-001) and `.chough/cases/history.skill-and-create-races.cases.toml` (REQ-history-016).

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
