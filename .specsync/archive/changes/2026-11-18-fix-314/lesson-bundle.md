# Lesson bundle: fix-314

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 314
- **Kind**: BugFix
- **Specs**: cli, records
- **Paths**: src/evidence.rs, src/cases.rs, src/doctor.rs, src/bugs.rs, src/library.rs, src/migrate.rs, src/evidence_migrate.rs, tests/special_files.rs, tests/migrate.rs, .chough/cases/cli.macos-metadata.cases.toml, .chough/cases/records.migrate-macos-metadata.cases.toml, changelog.d/2026-11-18-fix-314.md
- **Acceptance**: With an AppleDouble ._<name>.md file in .chough/bugs/, in a session's evidence/ and in .chough/archive/sessions/, chough next and chough status succeed as they do without it; a .DS_Store or ._<name>.cases.toml file in .chough/cases/ leaves chough cases check passing; chough doctor reports no unreadable finding for a committed ._ file; a 0.5.x project with ._ and .DS_Store files beside its bugs, sessions, archived sessions, inbox entries, evidence, history events and knowledge notes previews with the same plan digest as without them, applies, and still holds every one of them after; a committed binary AppleDouble file holding an email gives a leak finding from chough doctor --leaks, and chough doctor --fix --redact leaves it byte-identical; a malformed record with an ordinary name is still refused, naming the file.

## Evidence

- Verification commit: `6a54e36a56765353a847110f977e3891571c95ae`
- Base commit: `cef86e2622a2a908856b0c2cf11b29d3c1d3e472`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

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

## From the change's design.md

# Design

One predicate, `evidence::is_os_metadata(name)`, is true for `.DS_Store` and for any name that starts with `._`. Each reader of a `.chough/` folder that refuses a file it can't parse skips such names before it reads:
- the evidence readers in `src/evidence.rs` (entries, receipts, the session-reference scan, migration aliases and the bug folders);
- the case-definition loader in `src/cases.rs`;
- `doctor`'s record bound check in `src/doctor.rs`, so such a file is never `unreadable` or `oversize`;
- `chough migrate`'s 0.5.x readers in `src/migrate.rs` (sessions, archived sessions, inbox entries, history events), `src/evidence_migrate.rs` (observations), `src/bugs.rs` (legacy bugs) and `src/library.rs` (knowledge notes, `files_0_5`), so the plan is unchanged by them.

The migration's two folder clean-ups in `src/migrate.rs` remove a folder only when it holds nothing at all, since macOS's `rmdir` would delete orphaned `._*` files along with it.

`doctor`'s leak scan reads a tracked metadata file's bytes as text with invalid UTF-8 replaced, so a binary AppleDouble file's leak is found; any other file is scanned only when it is valid UTF-8, as before, so no other binary file, such as a screenshot, is read as text. `doctor --fix --redact` rewrites only files that are valid UTF-8, and reports a leak in a binary metadata file as `skipped`, naming it for a person to remove.

## From the change's testing.md

# Testing

- `cargo test --locked --test special_files macos_metadata_files_in_chough_stop_no_reader` puts AppleDouble `._<name>.md` and `.DS_Store` files into `.chough/bugs/`, a session's `evidence/` and `.chough/archive/sessions/` and requires `next`, `status` and `bug list` to succeed. It requires `cases check` to pass with both kinds in `.chough/cases/`. It requires `doctor` to report only a committed malformed ordinary record as `unreadable`, not a committed `._` file. It fails on main.
- `cargo test --locked --test special_files a_leak_in_a_binary_metadata_file_is_found_and_left_unrewritten` commits a binary AppleDouble file holding an email. `doctor --leaks` must report it, and `doctor --fix --redact` must leave it byte-identical with the finding `skipped`. It fails on main.
- `cargo test --locked --test migrate migrate_skips_macos_metadata_beside_0_5_records` takes the plan digest of a 0.5.x fixture, adds AppleDouble and `.DS_Store` files beside its bugs, sessions, archived sessions, inbox entries, evidence, history events and knowledge notes, and requires the same digest from the preview, a successful apply, and every one of those files still there after it. It fails on main. Its knowledge and kept-after parts also failed on the first recreation, before the `library.rs` skip and the empty-folder rule.

Case `TC-cli-macos-metadata-files-stop-no-reader` binds the first, `TC-cli-a-leak-in-a-binary-metadata-file-is-found` the second, and `TC-records-migrate-skips-macos-metadata` the third.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/records/context.md`
