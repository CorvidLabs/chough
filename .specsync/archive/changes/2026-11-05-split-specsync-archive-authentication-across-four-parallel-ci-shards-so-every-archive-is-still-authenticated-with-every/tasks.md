---
change: split-specsync-archive-authentication-across-four-parallel-ci-shards-so-every-archive-is-still-authenticated-with-every
artifact: tasks
---

# Tasks

- [x] Add failing tests: shards partition the sorted archives exactly once; `--shard` accepts only K/N with 1 <= K <= N; a shard authenticates only its own archives; shard 1 refuses an omitted archive; every shard refuses an active change.
- [x] Add `--shard K/N` (default 1/1, output unchanged) to .github/scripts/check-specsync-archive.py.
- [x] Run the SpecSync Archive job as four shards in .github/workflows/ci.yml, each keeping the 10-minute limit.
- [x] Update docs/ci.md and CHANGELOG.md.
- [x] Run the fixtures and `fledge lanes run verify` on Linux, and observe the hosted shards on the PR.
