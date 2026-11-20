# Lesson bundle: doctor-fix-safety

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Doctor fix safety
- **Kind**: BugFix
- **Specs**: history, cli, records
- **Paths**: src/doctor.rs, src/sessions.rs, src/main.rs, src/records.rs, tests/doctor.rs, tests/sessions.rs, tests/next.rs, tests/records.rs, tests/git_merge.rs, tests/leaks.rs, tests/history.rs, docs/cli.md, changelog.d/2026-11-19-doctor-fix-safety.md, .chough/cases/history.doctor-fix-safety.cases.toml, tests/reduce.rs, src/evidence.rs, .chough/cases/records.doctor-fix-safety.cases.toml, .chough/cases/cli.doctor-fix-safety.cases.toml
- **Acceptance**: doctor --fix never deletes an evidence entry its archived story does not hold (by ID and receipt) or an unfolded inbox entry, and names each kept entry with the way out by hand (#381); macOS metadata never blocks the archive finish (#377); next names committing the session folder, story and journal post, then doctor --fix, while an archive is unfinished (#371); a front-matter conflict is reported as conflict_markers and unreadable names resolving it, and a Timeline entry that only redaction changed is not reported as rewritten while a hand edit still is (#374); the spec walk follows no symlink, is bounded at 4,096 entries and 32 folders, and says when it stops at the bound (#402).

## Evidence

- Verification commit: `8a586283d6787f2b128ec9760d8c8cef31f26ed2`
- Base commit: `6936f88f47a2c69f9febf15822678f1d804ff027`
- Verified by: `specsync check --spec cli --spec history --spec records`

## From the change's context.md

# Context

Batch D1 of the 0.6.2 bug round: `chough doctor --fix` must neither destroy work nor hang. From the 0.6.2 candidate swarm and the agent reviewer:
- #381 (data loss, 62-F7): a branch adds an evidence entry to a session that main archives; after the merge keeps main's archive, `doctor --fix` finishes the archive and deletes the entry, which no receipt holds. An unfolded inbox entry a branch adds is lost the same way.
- #402 (hang): `root_code`'s spec walk follows symlinks, so a link out of the project counts outside code, and two folder links back to the project loop forever. Leif decided "fix in 0.6.2", with the finding as the scope.
- #377: a `.DS_Store` or `._session.md` in an archived folder is uncommitted, so the archive finish skips the folder forever.
- #371: after a move into done, nothing says to commit and run `chough doctor --fix`.
- #374: a front-matter conflict is reported only as `unreadable`, with shelve advice; and after `doctor --fix --redact` redacts a Timeline entry, `timeline_rewritten` advises restoring the leak.

File boundaries, from fable-lead: doctor.rs (the archive finish, the spec walk, the conflict and timeline advice), the archive finish in sessions.rs and next's line in main.rs. t4b keeps the stale-pointer check, session_md.rs, stage.rs, change.rs and the inbox, shelve and session-use code.

## From the change's design.md

# Design

- `src/sessions.rs`: `remove_archived_folder` reads the story's receipts and keeps an evidence entry, with its log and screenshot, unless a receipt has its ID and equals the receipt the entry gives now (`evidence::receipts_of` for that entry), and keeps every entry file under `agent/inbox/`; it removes macOS metadata in the folder whether committed or not. It returns what it kept and why.
- `src/doctor.rs`: the `unfinished_archive` finding names each kept entry and the way out by hand; `timeline_rewritten` skips an entry equal to HEAD's entry redacted again; the conflict scan covers the front matter, and `unreadable` names resolving a front-matter conflict instead of shelving; `root_code` walks with `symlink_metadata`, follows no link, and stops at 4,096 entries or 32 folders deep, saying so in the advice.
- `src/main.rs`: `next`'s no-open-session view names committing the session folder, story and journal post, then `chough doctor --fix`, while an archive is unfinished.

## From the change's testing.md

# Testing

Failing tests first, on main:
- #381: a real merge in which main archives a session and a branch adds an evidence entry and an inbox entry to it; `doctor --fix` keeps both, the finding names them and the way out, and the entry still resolves. An entry whose receipt no longer matches is kept too.
- #377: a `.DS_Store` and a `._session.md` in an archived folder; `doctor --fix` removes the folder.
- #371: `next` after done, after the commit, and after `doctor --fix`.
- #374: a front-matter conflict gives `conflict_markers` and an `unreadable` finding naming the conflict; after `doctor --fix --redact` on a Timeline leak no `timeline_rewritten`, while a hand edit writing `<redacted:token>` over ordinary words is still reported.
- #402: a spec file link out of the project, a folder link out of it and a link cycle; `doctor` and `init` finish within a time bound and give the same advice; a `specs/` past the bound gets the note.

## Where these lessons go

- `specs/history/context.md`
- `specs/cli/cli.3md`
- `specs/records/context.md`
