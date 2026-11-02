---
id: update-compact-next-fixture-for-slug-observations
state: approved
type: refactor
base_commit: 6800b2dadebec358ab1a6fdb18b5d78136eb6631
---

# Update compact-next fixture for slug observations

## Intent

Update compact-next fixture for slug observations

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The compact-next large-record fixture creates its current observation through automatic allocation and references the returned ID; all existing 16384-byte bounds, one-native-read counts, totals and historical-label assertions remain unchanged; tests/next.rs and the full suite pass with the approved identity behavior.

## No-spec Rationale

The existing compact-next output-bound test must create and reference a new-form observation under the approved identity policy. This changes only test fixture construction and preserves every output-bound assertion.
