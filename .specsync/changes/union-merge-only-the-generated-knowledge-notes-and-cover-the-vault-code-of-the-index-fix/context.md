---
change: union-merge-only-the-generated-knowledge-notes-and-cover-the-vault-code-of-the-index-fix
artifact: context
---

# Context

corvid-agent's review of #112 at 71a3efd3 found two blockers:
1. `.gitattributes` gave `knowledge/**` the union driver. That reaches attachments, snapshots, briefs and the README, which `library index --check` never compares. Two branches editing one attachment JSON would merge cleanly into invalid JSON while the check still reported the vault current.
2. The index-bound fix changed `src/vault.rs` (`RECENT`, `timeline_notes`, the module-note cap), which was outside its declared paths, so the audit refuses that path.

Both fall under Leif's decisions for #112: "Yes, record and merge", and "Widen #112 with the fix" (09:21 MST).
