---
id: prepare-the-0-5-0-release-set-the-package-version-to-0-5-0-and-date-the-change-log-section-for-0-5-0
state: approved
type: operations
base_commit: c58b4be79e1417662094e4d75c8b416e82123789
---

# Prepare the 0.5.0 release: set the package version to 0.5.0 and date the change log section for 0.5.0

## Intent

Prepare the 0.5.0 release: set the package version to 0.5.0 and date the change log section for 0.5.0

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Leif ('Prepare 0.5.0 now', relayed by orc 2026-11-06). The package version is 0.5.0 in Cargo.toml and Cargo.lock. The change log moves every Unreleased entry under '## [0.5.0] - 2026-11-06' and adds a [0.5.0] link to the v0.5.0 release. A new check in tests/changelog.rs fails first at 0.5.0 without that section, and passes once the dated section and the link are in place. cargo package --list --locked shows only the documented files, cargo publish --dry-run --locked succeeds, and fledge lanes run verify and fledge run linux-verify pass. No tag, publish or GitHub release is made here.

## No-spec Rationale

Release preparation only: package version, change log and a release check on them; no contract changes.
