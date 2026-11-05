---
change: split-specsync-archive-authentication-across-four-parallel-ci-shards-so-every-archive-is-still-authenticated-with-every
artifact: testing
---

# Testing

`python3 .github/scripts/test-specsync-archive.py` (the `CI gate fixtures` job and
`fledge run ci-archive-fixtures`) gains executor tests for shard partitioning and
argument validation, and real-native fixture tests run with `--shard`. A shard
authenticates only its own archives. Shard 1 still refuses an archive removed during
the PR, and every shard refuses an active change. The new tests fail before the
change. The existing executor and fixture tests pass unchanged, including the 1/1
output "N archive(s) authenticated". The hosted `SpecSync Archive (K/4)` jobs on
this PR must each pass within 10 minutes.
