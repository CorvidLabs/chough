---
id: assert-positive-track-selection-and-every-delivery-observation-kind-and-that-a-dry-run-never-stands-in-for-one-152-ma
state: implementing
type: bug_fix
base_commit: 64aa77e01d2763395d8af7566e62743ede37cc32
---

# Assert positive track selection and every delivery-observation kind, and that a dry run never stands in for one (152, MA-01, MA-02)

## Intent

Assert positive track selection and every delivery-observation kind, and that a dry run never stands in for one (152, MA-01, MA-02)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- REQ-cli-014 gains a criterion: publication, installation, deployment and live-health observations are each accepted and reported with their own kind and kept as key-item proof (protected by kind, moved into the vault when cold), while a dry run's kind is never a protection and a superseded dry run is trimmed with no vault copy. Tests assert positive copy-localization selection (REQ-cli-010), every delivery kind recorded and reported as itself with a dry run as a dry run (REQ-cli-014, REQ-cli-017), and the retention distinction for each kind. Each new test fails under a recorded targeted mutation that the existing tests let pass; no runtime change. fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
