---
change: union-merge-only-the-generated-knowledge-notes-and-cover-the-vault-code-of-the-index-fix
artifact: testing
---

# Testing

tests/vault.rs `only_generated_knowledge_notes_merge_by_union` runs `git check-attr merge` on this repository:
- `knowledge/INDEX.md`, `rules.md`, `timeline.md`, `timeline/*.md`, `archives/*.md`, `modules/*.md` and `intent/*.md` get `union`;
- attachments, snapshots, briefs and `README.md` do not.

It fails first, because `knowledge/**` gave `union` to everything. `fledge lanes run verify` passes on Linux with the audit green.
