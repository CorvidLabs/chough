---
id: keep-trimming-when-an-earlier-removal-releases-a-shared-artifact-to-another-eligible-set-and-count-stub-status-calls
state: implementing
type: bug_fix
base_commit: f40ac53b8de66ebfc07d51294c8f6c13e22f8ad3
---

# Keep trimming when an earlier removal releases a shared artifact to another eligible set, and count stub status calls under a lock now that validity reads run in parallel (108 closeout)

## Intent

Keep trimming when an earlier removal releases a shared artifact to another eligible set, and count stub status calls under a lock now that validity reads run in parallel (108 closeout)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Two eligible observation sets sharing one artifact are both trimmed in one apply, their own artifacts removed and the shared one kept; the per-file re-check still refuses when a planned file becomes shared with a protected set or the set becomes protected; the knowledge fixture stub counts status calls under a lock so hook-timed safety tests are exact under parallel validity reads. The new test fails first, the trim on this repository then completes, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-cli-033: only planned files are removed and each is still the set alone; the test stub counts calls safely under parallel reads; no requirement text changes.
