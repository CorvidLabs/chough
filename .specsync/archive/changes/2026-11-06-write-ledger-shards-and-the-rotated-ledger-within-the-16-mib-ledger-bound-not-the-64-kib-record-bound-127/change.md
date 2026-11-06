---
id: write-ledger-shards-and-the-rotated-ledger-within-the-16-mib-ledger-bound-not-the-64-kib-record-bound-127
state: archived
type: bug_fix
base_commit: ec4bf5861d0e13b6ff32731b9e68c0a61944a3d1
---

# Write ledger shards and the rotated ledger within the 16 MiB ledger bound, not the 64 KiB record bound (127)

## Intent

Write ledger shards and the rotated ledger within the 16 MiB ledger bound, not the 64 KiB record bound (127)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A rotation whose shard and kept ledger each exceed 64 KiB writes both within the 16 MiB ledger bound and reports rotated, losing no line; on this repository the rotation leaves 100 entries in .chough/pruned.jsonl. The test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-cli-049: rotation wrote with the 64 KiB managed-record bound and refused on every real ledger; REQ-cli-049 already sets 16 MiB; no requirement text changes.
