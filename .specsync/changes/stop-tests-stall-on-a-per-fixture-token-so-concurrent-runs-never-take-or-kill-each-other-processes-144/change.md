---
id: stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs-never-take-or-kill-each-other-processes-144
state: implementing
type: bug_fix
base_commit: c675c72e023fbdf4a80f762b8ef3f70330b4a83c
---

# Stop tests stall on a per-fixture token so concurrent runs never take or kill each other processes (144)

## Intent

Stop tests stall on a per-fixture token so concurrent runs never take or kill each other processes (144)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Every stop test stalls on a unique per-fixture token and matches and reaps only it, so a concurrent run of the same test (simulated by sleep 641/642/643 decoys) is neither taken for a survivor nor killed; every existing stop assertion holds; the decoy test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Issue #144: stop tests matched their stalled child machine-wide by sleep 6xx; test isolation only, no requirement text changes.
