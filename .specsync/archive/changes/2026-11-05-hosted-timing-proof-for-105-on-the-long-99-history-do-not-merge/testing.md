---
change: hosted-timing-proof-for-105-on-the-long-99-history-do-not-merge
artifact: testing
---

# Testing

- The three files are byte-identical to #118's head `e18e0941`, where the archive fixtures pass (25 tests, OK, on Linux) and every hosted check passed. Run `python3 .github/scripts/test-specsync-archive.py` here too; the hosted `CI gate fixtures` job also runs it.
- The measurement is the hosted `SpecSync Archive (K/4)` jobs on this branch: each job's duration, and each shard's walked and authenticated counts.
- The other jobs run #99's old product code. They are outside this proof.
