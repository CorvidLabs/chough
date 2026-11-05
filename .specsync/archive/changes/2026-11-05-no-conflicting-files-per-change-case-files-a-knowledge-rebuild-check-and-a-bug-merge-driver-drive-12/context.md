---
change: no-conflicting-files-per-change-case-files-a-knowledge-rebuild-check-and-a-bug-merge-driver-drive-12
artifact: context
---

# Context

DRIVE-12, widened on 2026-11-05: parallel pull requests that each record Chough work merge with no hand-resolved conflict in any file Chough writes. #111 adopted per-change sessions and event history here, and gave the retention ledger and CHANGELOG.md Git's union driver. The 89-merge replay found these still conflicting:
- **Case definitions:** `.chough/cases/cli.cases.toml` conflicted in 31 merges. Every change appends to one file per module.
- **Generated knowledge:** `knowledge/` conflicted in 16 merges. The index, module, intent and archive notes are rebuilt by every move into done.
- **Bug records:** two branches noting or verifying one bug both edit the same JSON file.

Evidence and history events are immutable files with unique names, and finished sessions are archived by identical renames, so they don't conflict.
