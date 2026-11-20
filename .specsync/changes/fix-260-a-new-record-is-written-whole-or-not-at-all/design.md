---
change: fix-260-a-new-record-is-written-whole-or-not-at-all
artifact: design
---

# Design

Nothing visual changes.

Options:
- A (recommended): write the new record to a temporary file `.<name>.<pid>.tmp` in its
  folder, write it whole and `sync_all`, then give it its name without replacing
  anything: `renamex_np(RENAME_EXCL)` on macOS, `renameat2(RENAME_NOREPLACE)` on Linux.
  Where the file system does not support that (`EINVAL`, `ENOTSUP`, `ENOSYS`, and on
  other platforms), check that the name is free and rename, which is safe because every
  record writer holds the writer lock (REQ-cli-040). No hard link, so exFAT keeps
  working (#313). The temporary file is removed on every error path. SIGXFSZ is set to
  be ignored at start, so a file-size limit returns `EFBIG` to the write, and the
  clean-up runs, instead of killing the process mid-write. Every reader already skips
  names that are not `*.md`, so a temporary file is never read as a record.
- B: keep create-new at the final name, and remove the file when the write fails. Less
  code, but a reader can meet the half-written file during the write, and a process
  killed mid-write (SIGKILL) still leaves a cut-off record at the name. Not taken.
- Also, in both: an empty record file is named as empty, with "remove it", in place of
  the "starts with a `---` line" message (a message only; the refusal is unchanged).

Left out: a SIGKILL during the write leaves the temporary file (not the record); a
sweep of such files beside bugs and journal posts belongs with #385 (0.6.3). A reader
that lists the readable bugs and names the broken one, rather than stopping (#352's
comment of 2026-11-18), is its own change.
