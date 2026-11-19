---
change: fix-314
artifact: context
---

# Context

GitHub #314, found by the macOS swarm test on 2026-11-18 and present since 0.6.0. A Finder `.DS_Store` or an AppleDouble `._*` file in `.chough/`, which macOS writes beside every file on exFAT, made Chough's readers fail:
- `chough next` and `chough status` exited 1 with one in `.chough/bugs/`, a session's `evidence/` or `.chough/archive/sessions/`.
- `chough cases check` failed with one in `.chough/cases/`.
- `chough doctor` reported a committed one as `unreadable`.
- `chough migrate` made no plan with one beside 0.5.x records ("is not a 0.5.x bug record", "cannot be read"), and listed one beside 0.5.x knowledge notes for removal.
- `chough doctor --leaks` skipped a tracked binary AppleDouble file, because it is not UTF-8, though macOS writes extended attributes such as a download URL into it.

Leif decided "Skip them" for the readers and the requirement sentence that says so (2026-11-18, relayed by hc t4c). The tester blocked the first version of PR #321 on migrate's 0.5.x readers and binary files in the leak scan. Leif chose "Recreate on my word" and approved the wider scope ("Approve"): `src/migrate.rs`, `src/evidence_migrate.rs` and `src/bugs.rs` declared, REQ-records-015 beside REQ-cli-068, and `doctor --fix --redact` reporting a leak in a binary file and leaving it unrewritten. That version was force-pushed onto #321 ("Force-push onto #321").

The tester's recheck then found two more things:
- Metadata beside 0.5.x knowledge notes was listed for removal by `files_0_5` in `src/library.rs`, which changed the digest and made the apply ask for those files to be committed.
- macOS's `rmdir` deleted orphaned `._*` files with a folder the migration emptied.

Leif chose "Recreate once more": this change is fix-314 recreated again from main with `src/library.rs` declared. It includes the empty-folder rule in `src/migrate.rs`.

The rule is REQ-cli-068 for the readers and doctor, beside the code that owns them, and REQ-records-015 for migration, since `src/migrate.rs` belongs to `records` and SpecSync needs a semantic delta for every declared module. REQ-cli-067 is left to #310 and #311, which both add it. `records::publish_new` is not touched.
