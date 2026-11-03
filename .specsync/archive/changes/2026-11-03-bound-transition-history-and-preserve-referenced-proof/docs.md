---
change: bound-transition-history-and-preserve-referenced-proof
artifact: docs
---

# Documentation

Reconcile docs/ROADMAP.md with merged PRs #24 and #33 to #36, retaining cold-start Fail/not-run limits. Correct the already-published 0.1.0 changelog wording without claiming future features shipped.

During implementation, update docs/architecture.md storage roles and lifetimes; docs/cli.md history commands, limits, compatibility, cursor merge behavior, retention reasons and partial timing; docs/FRD.md traceability; CHANGELOG.md with implemented behavior; and the checkpoint skill reference with explicit adoption, one-owner maintenance and cursor selection. Register the former canonical checkpoint-reference digest in src/skill.rs and verify old unedited copies update while edits stay preserved. Existing frozen exercise files remain unchanged. Installed host copies may remain known-outdated until their owner explicitly updates them.

Issue bodies #30/#32/#31 receive dated status mappings, preserving the old baseline as history and leaving unmet acceptance criteria open. This is backlog maintenance, not new scope approval or acceptance.
