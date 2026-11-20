# Lesson bundle: fix-260-a-new-record-is-written-whole-or-not-at-all

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 260: a new record is written whole or not at all
- **Kind**: BugFix
- **Specs**: records, cli
- **Paths**: src/records.rs, src/main.rs, src/markdown.rs, tests/records.rs, docs/cli.md, specs/records/requirements.md, specs/records/records.spec.md, specs/cli/requirements.md, specs/cli/cli.spec.md, changelog.d/, .chough/cases/records.whole-or-nothing.cases.toml, src/checkpoint.rs
- **Acceptance**: A new record whose write fails or is cut off, by a full disk or a file-size limit, leaves no file at its name and no temporary file, the command exits non-zero naming the record, and bug list, status, next and library index --check work as before; an empty record file is named as empty; on exFAT, bug add and journal add still write whole records.

## Evidence

- Verification commit: `34fd2a92d1ad9a81db575af01bbd7430d9acd8e7`
- Base commit: `056cbb8029f6535ff74f866a0523893916f682db`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

#260, reopened for 0.6.2 when #306's hard-link writer was reverted (#313, exFAT), and
#352's "cut-off record after a failed write" entry. Every new record goes through
`records::write_new_bytes`: create-new at the final name, then one write, no sync and
no clean-up. A write that fails or is cut off leaves a partial file at the record's name.

Measured on main 056cbb80 (t4b, 2026-11-20, macOS):
- `bug add` with large fields under `ulimit -f 4`: killed by SIGXFSZ (exit 153), leaves
  a 2,048-byte cut-off bug at `.chough/bugs/<date>-truncated-bug.md`, and `chough bug
  list` lists it as a whole bug (exit 0).
- A 0-byte bug (what a full disk leaves, made by hand here): `status` and `next` exit 1
  with "a record starts with a `---` line before its front matter", which does not fit
  an empty file; `chough doctor` reports nothing.
- A 0-byte journal post: `library index --check` fails.
- `ulimit -f 0` dies at the lock file before any record exists, so the 0-byte case here
  comes from ENOSPC, as the issue's tmpfs run shows.

REQ-cli-023 already says "Creation SHALL use atomic create-new", and REQ-records-001
"by create-new for a new record". The code claims the name atomically but not its
content; the fix brings it in line. No refusal is added and no sentence changes: one
criterion is added to each requirement.

## From the change's design.md

# Design

Nothing visual changes.

Options:
- A (recommended): write the new record whole to a temporary file in `.chough/local/`
  (Git-ignored, inside the project), `sync_all` it, then give it its name in the target
  folder without replacing anything: `renamex_np(RENAME_EXCL)` on macOS,
  `renameat2(RENAME_NOREPLACE)` on Linux. `EEXIST` there means the name is taken and is
  answered as create-new answers it today, so `identity::create_named`'s suffix retry
  works; today a lost race gives an error instead. The temporary file is removed on every
  error path. No hard link, so exFAT keeps working (#313).
- B: create-new at the final name and remove the file when the write fails. A reader can
  meet the half-written file, and a SIGKILL mid-write still leaves a cut-off record at
  the name. Not taken, except as A's fallback below.

Where the no-replace rename is not supported, detected by its error (`EINVAL`, `ENOTSUP`,
`ENOSYS`, `EXDEV` when `.chough/local/` is on another file system, and on platforms
without either call): fall back to create-new at the final name, write, `sync_all`, and
remove the file on any error. Not a free-name check and rename: six create-new sites run
without the writer lock on some path (`init` for `.chough/README.md`, `.chough/local/.gitignore`,
the skill files and the knowledge index; `skill update`; `validate stop`; `validate run`'s
evidence entry), and four more in a project with no `.chough/` yet (`session new`,
`init --objective`, `bug add`, the `.chough/sessions/.gitignore`), so a check and a
rename could replace another writer's file there. The fallback never replaces, and is
whole-or-nothing except for a SIGKILL mid-write. tester2 runs the Docker Desktop mount
and an exFAT volume, where the fallback is expected.

A file-size limit: SIGXFSZ is set to be ignored only around the write of the record and
restored right after, so the write gets `EFBIG` and the clean-up runs. Process-wide it
would be inherited by every process Chough starts (git, specsync, case commands,
validation runs), whose behaviour under a size limit would change; scoped to the write,
no child ever inherits it.

A temporary file a SIGKILL leaves: it is in `.chough/local/`, so no record folder holds
it. Measured on main with a `.half.md.4242.tmp` planted in `.chough/bugs/`,
`knowledge/journal/`, a session folder, its `evidence/` and its `agent/inbox/`: `bug
list`, `status`, `next`, `session list`, `inbox list`, `library index --check` and
`doctor` all pass and none takes it for a record; but `session close` then leaves the
session folder, and `doctor --fix` skips the unfinished archive as "not committed at
HEAD; commit it first". So a temporary file in a record folder would block the archive
finish; in `.chough/local/` it cannot. A sweep of `.chough/local/` temporaries is #385's.

An empty record file is named as empty, with "remove it", in place of the "starts with
a `---` line" message (a message only; the refusal is unchanged).

Order: D2 (built, waiting for a review slot) changes `write_new_bytes` too, so this
builds after D2 merges.

## From the change's testing.md

# Testing

Each failing on main first, in `tests/records.rs` (Unix, with `ulimit -f` in the
child): `bug add` and `journal add` cut off by a file-size limit exit non-zero naming
the record, leave no file at the new name and no temporary file in `.chough/local/`, and
`bug list`, `status`, `next` and `library index --check` then pass; an empty bug file
makes the readers name it as empty. A guard that passes on main too: a case whose
command writes past a file-size limit still dies of SIGXFSZ under `chough cases run`, as
today. The existing `tests/no_hard_links.rs` (exFAT) must still pass.

Then every verify step, the case on macOS, tester2 on Linux (`renameat2`), on a Docker
Desktop mount and on an exFAT volume (both where the fallback is expected), and the
tester's run on the real repositories, which must show: on a full volume (a small disk
image on macOS, a tmpfs on Linux) and under `ulimit -f`, `bug add`, `journal add`,
`session new`, `evidence add` and `inbox add` leave no file at the record's name and no
temporary file, and every reader works after; and on the real repositories every
command that creates a record makes the same file as main, byte for byte.

## Where these lessons go

- `specs/records/context.md`
- `specs/cli/cli.3md`
