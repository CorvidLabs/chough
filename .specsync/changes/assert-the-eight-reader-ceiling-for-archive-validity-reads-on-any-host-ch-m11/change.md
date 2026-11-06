---
id: assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11
state: approved
type: bug_fix
base_commit: 73a7da5653312e907621c0165d0b5f903c22ff53
---

# Assert the eight-reader ceiling for archive validity reads on any host (CH-M11)

## Intent

Assert the eight-reader ceiling for archive validity reads on any host (CH-M11)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- REQ-cli-033 gains a criterion: with twelve affected archives no more than 8 validity reads run at once, and the reader count is at most 8 whatever parallelism the host reports. The reader count is a pure function with no behavior change. A unit test holds the ceiling for a host reporting 128 cores, and a 12-cold-archive integration test observes at most 8 overlapping reads. Both fail with the ceiling raised to 128 (the audit's CH-M11 mutant) and pass restored; the existing overlap test still passes. fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
