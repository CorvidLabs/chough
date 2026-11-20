---
change: fix-260-a-new-record-is-written-whole-or-not-at-all
artifact: testing
---

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
