---
id: split-specsync-archive-authentication-across-four-parallel-ci-shards-so-every-archive-is-still-authenticated-with-every
state: archived
type: bug_fix
base_commit: 166a8f52caa772fa6bb33361e195772f0005e2f4
---

# Split SpecSync Archive authentication across four parallel CI shards so every archive is still authenticated with every check unchanged within the unchanged 10-minute job limit

## Intent

Split SpecSync Archive authentication across four parallel CI shards so every archive is still authenticated with every check unchanged within the unchanged 10-minute job limit

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With --shard K/N each shard authenticates the sorted archives at positions K, K+N and so on, so N shards read every archive exactly once; shard 1 also walks every PR revision and refuses an omitted archive; every shard refuses an active change, an unhealthy audit, a wrong SpecSync version and an invalid result; --shard defaults to 1/1 with output unchanged; ci.yml runs four shards, each with the unchanged 10-minute limit; the new tests fail before the change and the existing fixtures pass; the hosted shards pass on this PR.

## No-spec Rationale

CI-only change: the hosted Archive inspection is split into shards; Chough product behavior and requirements are unchanged.
