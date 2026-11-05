---
change: reduce-old-observation-proof-drop-closed-cycle-checkpoint-references-at-done-trim-superseded-runs-of-cold-changes
artifact: research
---

# Research

These figures were measured on Chough's own repository at main 8fbbbe7 plus #107.

- `.chough/` is 31 MB: 850 evidence records (3.3 MB) and 1,485 artifact files (27 MB).
- `chough library archive` (preview) found 0 eligible and 502 protected observation
  sets. The protections were:

  | Reason | Sets |
  |---|---|
  | newest 3 runs of a case and environment | 298 |
  | checkpoint reference | 245 |
  | agent-assisted or manual (non-routine) | 71 |
  | last failure | 20 |
  | bugs | about 50 |

- The checkpoint holds 359 observation references and never drops one.
- 348 of the 850 records are listed by no archive manifest; only 26 of 90 archives list
  any `.chough/` path. The summary skipped every unlisted record.
- Each native `specsync change status <archived-id>` read costs 26 to 35 s
  (CorvidLabs/spec-sync#831). The validity check reads the affected archives one at a
  time, before and after removal.
