---
id: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
state: draft
type: feature
base_commit: 4c68cca7275b3ab8f49af94cc925bf547842757a
---

# Rotate the retention ledger: keep the newest 100 entries and move older summarized ones into the vault with their archive (KNOW-23)

## Intent

Rotate the retention ledger: keep the newest 100 entries and move older summarized ones into the vault with their archive (KNOW-23)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- At done the retention ledger keeps its newest 100 entries plus entries without an archive or note; older entries move with their restores into knowledge/attachments/<first archive folder>/pruned.jsonl, shards first, nothing lost or duplicated, reported as applied.ledger_rotation; every ledger reader reads shards then the hot ledger ignoring duplicate lines, so labels, bug views and notes are unchanged; shards union-merge and are not listed as attachments. The new tests fail first and fledge lanes run verify passes on Linux.

## No-spec Rationale

KNOW-23 (Leif: Keep newest N, not 14 days, the rest as proposed, 2026-11-05 18:24 MST): a new requirement for ledger rotation at done and for every ledger reader to read the vault shards.
