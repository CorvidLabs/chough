---
change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
artifact: design
---

# Design

## Approach

- **Readers:** a single `ledger_lines(root)` in src/evidence.rs yields each shard's lines, the shards in path order, then the hot ledger's lines. It skips a line identical to one already read and bounds each file at 16 MiB. `read_ledger`, `ledger_entries` and `ledger_records` use it, so labels, bug views, `evidence prune`, the summary and the notes all see moved entries.
- **Rotation:** `rotate_ledger` runs in src/stage.rs after `refresh_at_done`. It pairs each `restored` event with the entry it follows. It keeps the newest 100 entries, plus entries with no archive or no note. It appends the moved lines to their shards, then replaces the hot file. It reports `applied.ledger_rotation`.
- **Attachments:** `attachments_of` in src/attachments.rs skips a top-level `pruned.jsonl`, so notes don't list it as an attachment.
- **Merging:** `.gitattributes` gains `knowledge/attachments/*/pruned.jsonl merge=union`.

## Risks

- A trim larger than 100 entries rotates part of itself straight away. Its entries stay readable in the shard.
- A crash between the two writes leaves duplicates, which readers ignore.

## Constraints consulted

- REQ-cli-041, REQ-cli-043, REQ-cli-046, REQ-cli-047.
- KNOW-13, KNOW-23, DRIVE-12.
- The 16 MiB ledger bound.
