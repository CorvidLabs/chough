---
change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
artifact: context
---

# Context

Issue #105 (P2): on the final #99 head `6a9bfad6`, against trusted base `8b7f8bf6` (361 PR revisions, 86 archives), hosted job `SpecSync Archive (1/4)` was cancelled at its 10-minute limit. #103 split archive authentication across four shards, but shard 1 alone still walks every PR revision.

## Measurement before the change

- Inspector from main `a688800e`, SpecSync 6.2.0 built from source.
- Docker `chough-linux:3`, Linux aarch64, `--cpuset-cpus=0-3`, cold clone of the candidate.

| Phase | Shard 1/4 | Shard 2/4 |
|---|---|---|
| History walk: 362 historical `change list` reads | 234.8 s (about 0.65 s each) | none |
| Checkouts | 4.3 s | none |
| Archive scans | 3.6 s | none |
| Authenticating 22 archives (wall) | 22.5 s | 4.5 s |
| Audit | 0.4 s | 0.2 s |
| **Total** | **268.6 s** | **4.9 s** |

The walk is about 87% of shard 1. With SpecSync 6.2.0, authentication is no longer the bulk. The hosted runner is at least 2.2 times slower: it was cancelled at 600 s.

## Measurement after the change

Same conditions, inspector with `shard_revisions`; logs in `.chough/artifacts/archive-history-shards-105/`.

| Shard | Revisions walked | Walk | Archives authenticated | Authentication wall | Total |
|---|---|---|---|---|---|
| 1/4 | 91 | 64.2 s | 22 | 4.6 s | 72.7 s |
| 2/4 | 91 | 63.4 s | 22 | 6.1 s | 73.5 s |
| 3/4 | 90 | 71.2 s | 21 | 6.1 s | 81.9 s |
| 4/4 | 90 | 78.6 s | 21 | 5.8 s | 88.8 s |

Together: 362 of 362 revisions (361 plus the base), and 86 of 86 archives. The slowest shard is 88.8 s, against 268.6 s before. At the hosted slowdown implied by the cancellation (at least 2.2 times), that is about 3.3 minutes against a 10-minute limit. Hosted proof on this long candidate needs a hosted run on it; this PR's own hosted shards cover only its own short history.

Skipping revisions with an identical `.specsync` tree was considered. Only 103 of the 361 repeat a tree (258 are distinct), so it saves little. It would also rely on native `list` reading nothing outside that tree. It was not chosen.

Human intent: existing DRIVE-10. Leif's agreed order, via the orc, 2026-11-05: #105 after #114, then the reduction work.

Out of scope:
- SpecSync's own per-read cost;
- the 10-minute limit, which stays;
- branch protection.
