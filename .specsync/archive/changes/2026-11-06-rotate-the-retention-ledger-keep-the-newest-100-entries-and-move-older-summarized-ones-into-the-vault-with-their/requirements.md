---
change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
artifact: requirements
---

# Requirements

ADDED REQ-cli-049 (see `deltas/cli.md`). At done the retention ledger keeps its newest 100 entries. Older entries that their archive notes summarize move into the vault with their first archive. Every ledger reader reads the shards as well (KNOW-23).
