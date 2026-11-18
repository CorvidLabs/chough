---
id: 2026-11-18-the-migration-neither-descends-into-nor-removes-a-folder
title: The migration neither descends into nor removes a ._* folder
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T18:25:06Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-314-folders.md
  - .specsync/archive/changes/2026-11-18-fix-314-folders/
record: .chough/archive/sessions/2026-11-18-fix-314-folders.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# The migration neither descends into nor removes a ._* folder

The migration neither descends into nor removes a ._* folder (REQ-records-015, #321 review P3)

Why: 0xLeif decided "Scope approval of fix-314-folders": "Fix it in #321 first" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his choice for his [P3] review thread on #321 ('Exclude metadata directories from recursive cleanup ... Skip is_os_metadata entries before recursing/removing and cover a ._* directory alongside the existing metadata-file regression'), approving a small follow-up change on #321's branch); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-314-folders as 0xLeif

Verified: Ran cargo test --locked --test migrate -- --exact migrate_skips_macos_metadata_beside_0_5_records for TC-records-migrate-skips-macos-metadata: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-314-folders.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-314-folders/)
