---
change: fix-314
artifact: design
---

# Design

One predicate, `evidence::is_os_metadata(name)`, is true for `.DS_Store` and for any name that starts with `._`. Each reader of a `.chough/` folder that refuses a file it can't parse skips such names before it reads:
- the evidence readers in `src/evidence.rs` (entries, receipts, the session-reference scan, migration aliases and the bug folders);
- the case-definition loader in `src/cases.rs`;
- `doctor`'s record bound check in `src/doctor.rs`, so such a file is never `unreadable` or `oversize`;
- `chough migrate`'s 0.5.x readers in `src/migrate.rs` (sessions, archived sessions, inbox entries, history events), `src/evidence_migrate.rs` (observations), `src/bugs.rs` (legacy bugs) and `src/library.rs` (knowledge notes, `files_0_5`), so the plan is unchanged by them.

The migration's two folder clean-ups in `src/migrate.rs` remove a folder only when it holds nothing at all, since macOS's `rmdir` would delete orphaned `._*` files along with it.

`doctor`'s leak scan reads a tracked metadata file's bytes as text with invalid UTF-8 replaced, so a binary AppleDouble file's leak is found; any other file is scanned only when it is valid UTF-8, as before, so no other binary file, such as a screenshot, is read as text. `doctor --fix --redact` rewrites only files that are valid UTF-8, and reports a leak in a binary metadata file as `skipped`, naming it for a person to remove.
