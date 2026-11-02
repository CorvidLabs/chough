---
id: integrate-the-approved-case-ordering-and-native-authority-workflow-changes
state: implementing
type: refactor
base_commit: 6fde285473f3fa3b6efd45bc0582b3826ec8775d
---

# Integrate the approved case ordering and native authority workflow changes

## Intent

Integrate the approved case ordering and native authority workflow changes

## Affected Canonical Specs

- None

## Acceptance Criteria

- Merge main8fb0ea7 into PR44 without rebase or force, preserve case-ordering and acting-agent/human-source guidance across canonical and both host bundles, retain all existing predecessor identities/tests and both approved requirement effects with monotonic canonical version, select the complete PR44 checkpoint current-cycle group while preserving additive collections and every transition occurrence, leave existing archives/cold-start/observations/artifacts byte-identical, and pass all required Fledge steps, relevant bound cases and complete Rust1.95/1.88 tests before normal push. Native review/ship remains with the root closing session.

## No-spec Rationale

Reconcile independently approved and archived PR41 ordering and PR44 authority changes on one merge result. Preserve both existing requirement meanings, native records and all proof; canonical version advancement and combined files/metadata introduce no new requirement, helper behavior or lifecycle state.
