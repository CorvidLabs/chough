# Lesson bundle: fix-314-folders

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 314 folders
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/migrate.rs, tests/migrate.rs, changelog.d/2026-11-18-fix-314-folders.md
- **Acceptance**: A 0.5.x project with a ._* folder holding only an empty folder under .chough/history/ previews with the same plan digest as without it, and after the apply that ._* folder and the folder inside it are both still there; the migration neither descends into nor removes a folder whose name starts with ._.

## Evidence

- Verification commit: `38f22e1b7c7da51325d3afa78415ad88832b641e`
- Base commit: `b2a227e3aa07fed5bf465dca8c70a8c35c4476ca`
- Verified by: `specsync check --spec records`

## From the change's context.md

# Context

A follow-up to fix-314 on PR #321, from Leif's review thread [P3] "Exclude metadata directories from recursive cleanup". REQ-records-015 says the migration neither reads, lists nor removes a file or folder whose name starts with `._`. But `prune_empty_folders` in `src/migrate.rs` still descends into such a folder and removes it when it is empty. In a 0.5.x fixture, `.chough/history/._metadata-folder/empty-child` left the preview digest unchanged and was not listed, yet the apply deleted both folders. No file bytes were lost, but the ignored metadata subtree changed outside the preview.

Leif chose "Fix it in #321 first" (2026-11-18, relayed by hc t4c): a small follow-up change on #321's branch, approved by his P3 comment and that answer. fix-314 itself is already archived on the branch. No requirement text changes.

## From the change's testing.md

# Testing

`cargo test --locked --test migrate migrate_skips_macos_metadata_beside_0_5_records` now also adds `.chough/history/._metadata-folder/empty-child`, an empty folder inside a `._*` folder. It requires the same plan digest and both folders still there after the apply. It fails before this change. Case `TC-records-migrate-skips-macos-metadata` binds it.

## Where these lessons go

- `specs/records/context.md`
