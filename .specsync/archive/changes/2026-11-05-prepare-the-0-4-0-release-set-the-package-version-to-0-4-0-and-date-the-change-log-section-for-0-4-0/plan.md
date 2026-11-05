---
change: prepare-the-0-4-0-release-set-the-package-version-to-0-4-0-and-date-the-change-log-section-for-0-4-0
artifact: plan
---

# Plan

## Approach

- Set `version = "0.4.0"` in Cargo.toml and refresh Cargo.lock.
- Move the Unreleased entries under `## [0.4.0] - 2026-11-05`, adding the missing entry for #110 (merged in #115).
- Add the `[0.4.0]` release link.

## Out of scope

Tagging `v0.4.0`, `cargo publish`, the GitHub release and the ROADMAP row. Each needs Leif's approval after this merges.

## Steps

1. Edit the version and the change log.
2. `cargo build --locked` to refresh the lock file; `cargo package --list --locked`; `cargo publish --dry-run --locked`.
3. Verify, review, finalize, merge.

## Risks

None to behavior: no source changes. The package contents must stay limited to the files docs/releasing.md lists.

## Constraints consulted

docs/releasing.md and the skill's release reference.
