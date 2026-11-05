---
change: keep-vault-attachment-paths-within-windows-limits-and-keep-file-modes-when-the-trim-moves-or-restores-files
artifact: requirements
---

# Requirements

MODIFIED REQ-cli-046 (see `deltas/cli.md`):
- an archive ID over 24 characters gets a stable attachments folder of at most 25 characters;
- a key item whose attachment path would pass 220 characters stays, protected with that reason;
- moved and restored files keep their permissions (KNOW-10).

Everything else in REQ-cli-046 is unchanged, including the agent-assisted observations among key items, which KNOW-19 now names too.
