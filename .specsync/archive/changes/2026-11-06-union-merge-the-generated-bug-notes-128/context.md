---
change: union-merge-the-generated-bug-notes-128
artifact: context
---

# Context

Issue #128. KNOW-22 (#121) added generated bug notes at `knowledge/bugs/<id>.md`, but `.gitattributes` has no `merge=union` line for them. Every other generated knowledge note union-merges and is rebuilt by `library index --check` (REQ-cli-047, DRIVE-12). So two branches that each change a bug note can still conflict.

This was found while scoping KNOW-23 (#127). Leif chose "Yes, small separate fix" on 2026-11-05, via the orc.
