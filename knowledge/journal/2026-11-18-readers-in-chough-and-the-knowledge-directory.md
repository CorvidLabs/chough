---
id: 2026-11-18-readers-in-chough-and-the-knowledge-directory
title: Readers in .chough and the knowledge directory
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T18:12:54Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-314.md
  - .specsync/archive/changes/2026-11-18-fix-314/
record: .chough/archive/sessions/2026-11-18-fix-314.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# Readers in .chough and the knowledge directory

Readers in .chough and the knowledge directory, chough migrate's 0.5.x readers included, skip the macOS metadata files .DS_Store and ._*, and the leak scan reads a tracked one's bytes (REQ-cli-068, REQ-records-015, #314)

Why: fix-314 recreated once more from main on Leif's word ('Recreate once more'), after the tester's recheck of #321 found metadata beside 0.5.x knowledge notes listed for removal by src/library.rs and macOS's rmdir deleting orphaned ._ files with emptied folders. It declares src/library.rs, adds the library.rs skip and the empty-folder rule (820af00 on the previous build), and replaces #321's branch by force-push as before.

Verified: Ran cargo test --locked --test migrate -- --exact migrate_skips_macos_metadata_beside_0_5_records for TC-records-migrate-skips-macos-metadata: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-314.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-314/)
