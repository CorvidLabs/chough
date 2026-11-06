---
change: union-merge-the-generated-bug-notes-128
artifact: testing
---

# Testing

- tests/vault.rs `only_generated_knowledge_notes_merge_by_union` checks that a bug note path is `union`. It fails first, with `unspecified`. Attachments, snapshots, briefs and the README stay non-union.
- `fledge lanes run verify` passes on Linux.
