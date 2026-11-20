---
change: fix-260-a-new-record-is-written-whole-or-not-at-all
artifact: design
---

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
