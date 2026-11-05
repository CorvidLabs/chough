---
change: move-old-proof-into-the-knowledge-vault-trim-loose-artifacts-of-cold-changes-and-move-key-items-into-knowledge
artifact: requirements
---

# Requirements

ADDED REQ-cli-046 (see `deltas/cli.md`):
- loose artifacts of cold changes are summarized and trimmed (KNOW-18);
- key items move into `knowledge/attachments/<archive>/` and are linked from the note
  (KNOW-19).

The safeguards are unchanged: ledger first, the KNOW-10 restore check, no removal while
a change is open, and every byte in Git.
