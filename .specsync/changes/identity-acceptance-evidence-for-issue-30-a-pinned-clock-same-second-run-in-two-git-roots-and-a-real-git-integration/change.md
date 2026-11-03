---
id: identity-acceptance-evidence-for-issue-30-a-pinned-clock-same-second-run-in-two-git-roots-and-a-real-git-integration
state: implementing
type: feature
base_commit: a788869b32b8442cbc8d7be2bf19c19a922954a8
---

# Identity acceptance evidence for issue 30: a pinned-clock same-second run in two Git roots and a real Git integration fixture for generated records

## Intent

Identity acceptance evidence for issue 30: a pinned-clock same-second run in two Git roots and a real Git integration fixture for generated records

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A deterministic test pins the clock (a cfg(test)-only hook, no runtime switch) and runs the same bound case in two independent Git roots on separate branches at the same second; after merging, the generated observation IDs and artifact paths are distinct, both records keep the pinned time, both artifacts keep their recorded bytes and fingerprints, and both observations read as current
- A real Git fixture adds generated records independently on two branches in two roots (cases run observations and artifacts, evidence add with an artifact, bug add); the merge has no conflicts, every record from both sides is byte-identical to its branch version under a distinct ID, and cases check and status validate the merged tree with every observation current
- No product behavior changes; fledge lanes run verify passes; hosted CI is reported only as observed

## No-spec Rationale

Not applicable
