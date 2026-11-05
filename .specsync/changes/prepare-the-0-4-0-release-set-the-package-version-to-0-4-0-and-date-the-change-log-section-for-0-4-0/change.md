---
id: prepare-the-0-4-0-release-set-the-package-version-to-0-4-0-and-date-the-change-log-section-for-0-4-0
state: implementing
type: operations
base_commit: 1eacdfd17eefe093f797d5fc8fe2dd0e09248bd6
---

# Prepare the 0.4.0 release: set the package version to 0.4.0 and date the change log section for 0.4.0

## Intent

Prepare the 0.4.0 release: set the package version to 0.4.0 and date the change log section for 0.4.0

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The package version is 0.4.0 in Cargo.toml and Cargo.lock, the change log has a dated 0.4.0 section including #110 and a v0.4.0 link, cargo package --list shows only the documented files, cargo publish --dry-run succeeds, and fledge lanes run verify passes on Linux. No tag, publish or GitHub release is made here.

## No-spec Rationale

Release preparation only: package version and change log; no contract changes.
