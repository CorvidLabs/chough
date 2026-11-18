---
kind: session
id: 2026-11-18-fix-314
aliases: [fix-314]
objective: "Readers in .chough and the knowledge directory, chough migrate's 0.5.x readers included, skip the macOS metadata files .DS_Store and ._*, and the leak scan reads a tracked one's bytes (REQ-cli-068, REQ-records-015, #314)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-314
pr: ""
criteria: [DRIVE-14]
requirements: [REQ-cli-068, REQ-records-015]
cases:
  - TC-cli-macos-metadata-files-stop-no-reader
  - TC-cli-a-leak-in-a-binary-metadata-file-is-found
  - TC-records-migrate-skips-macos-metadata
design: []
files: []
observations:
  - 2026-11-18-tc-cli-macos-metadata-files-stop-no-reader
  - 2026-11-18-tc-cli-a-leak-in-a-binary-metadata-file-is-found
  - 2026-11-18-tc-records-migrate-skips-macos-metadata
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-314
    holds: accept
    drives: review
    opened: 2026-11-18T17:54:38Z
run_consents: []
next: Continue accept
---
# Readers in .chough and the knowledge directory, chough migrate's 0.5.x readers included, skip the macOS metadata files .DS_Store and ._*, and the leak scan reads a tracked one's bytes (REQ-cli-068, REQ-records-015, #314)

## Goal

Readers in .chough and the knowledge directory, chough migrate's 0.5.x readers included, skip the macOS metadata files .DS_Store and ._*, and the leak scan reads a tracked one's bytes (REQ-cli-068, REQ-records-015, #314)

## Where it stands

- Carried the source fix from the previous recreation onto main cef86e2; its tests were red before their fixes (the readers on main, the binary leak scan, migrate's 0.5.x readers, and on the recheck the knowledge notes and the emptied-folder removal); the tester's full p05-ds tree gives the clean digest, applies and keeps all 28 metadata files

## Decisions and why

- fix-314 recreated once more from main on Leif's word ('Recreate once more'), after the tester's recheck of #321 found metadata beside 0.5.x knowledge notes listed for removal by src/library.rs and macOS's rmdir deleting orphaned ._ files with emptied folders. It declares src/library.rs, adds the library.rs skip and the empty-folder rule (820af00 on the previous build), and replaces #321's branch by force-push as before.
- 0xLeif decided "Scope approval of fix-314, recreated once more with src/library.rs": "Recreate once more" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, in answer to whether to add src/library.rs to #321 by recreating fix-314 once more, reopening it himself, or with a migrate.rs workaround; it follows his earlier 'Skip them', 'Recreate on my word', 'Force-push onto #321' and 'Approve' for the wider scope (REQ-records-015 beside REQ-cli-068, src/migrate.rs, src/evidence_migrate.rs and src/bugs.rs declared)); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-314 as 0xLeif

## What was verified

- [Ran cargo test --locked --test special_files -- --exact macos_metadata_files_in_chough_stop_no_reader for TC-cli-macos-metadata-files-stop-no-reader: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-macos-metadata-files-stop-no-reader.md)
- [Ran cargo test --locked --test special_files -- --exact a_leak_in_a_binary_metadata_file_is_found_and_left_unrewritten for TC-cli-a-leak-in-a-binary-metadata-file-is-found: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-leak-in-a-binary-metadata-file-is-found.md)
- [Ran cargo test --locked --test migrate -- --exact migrate_skips_macos_metadata_beside_0_5_records for TC-records-migrate-skips-macos-metadata: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-migrate-skips-macos-metadata.md)

## Open threads

## Timeline

- 2026-11-18T17:53:11Z · start · intent → intent · Readers in .chough and the knowledge directory, chough migrate's 0.5.x readers included, skip the macOS metadata files .DS_Store and ._*, and the leak scan reads a tracked one's bytes (REQ-cli-068, REQ-records-015, #314) ^t-9c9f24
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T17:53:11Z · advance · intent → design ·  ^t-3d1d1d
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T17:53:11Z · skip · design → plan · Record readers skip macOS metadata files; nothing visual or interactive changes ^t-3a47cb
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T17:54:11Z · gate-opened · plan → plan · Scope approval of fix-314, recreated once more with src/library.rs ^t-1cfd21
  - native_change: fix-314
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T17:54:11Z · gate-closed · plan → plan · Scope approval of fix-314, recreated once more with src/library.rs ^t-e2fb2e
  - native_change: fix-314
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - relayed_by:
    - decided_by: 0xLeif
    - words: Recreate once more
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, in answer to whether to add src/library.rs to #321 by recreating fix-314 once more, reopening it himself, or with a migrate.rs workaround; it follows his earlier 'Skip them', 'Recreate on my word', 'Force-push onto #321' and 'Approve' for the wider scope (REQ-records-015 beside REQ-cli-068, src/migrate.rs, src/evidence_migrate.rs and src/bugs.rs declared)
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-314
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T17:54:21Z · advance · plan → build · native:approved ^t-54965f
  - native_change: fix-314
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T17:54:22Z · advance · build → test · native:approved ^t-5b0d05
  - native_change: fix-314
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T17:54:38Z · advance · test → accept · native:verifying ^t-c99f89
  - native_change: fix-314
  - gate: native:verifying
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T17:54:38Z · gate-opened · accept → accept · Implementation review of fix-314 ^t-004331
  - native_change: fix-314
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
