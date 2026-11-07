---
change: prepare-the-0-5-0-release-set-the-package-version-to-0-5-0-and-date-the-change-log-section-for-0-5-0
artifact: plan
---

# Plan

## Approach

Set the package version to 0.5.0, then date the change log: every `[Unreleased]` entry moves under `## [0.5.0] - 2026-11-06`, with a `[0.5.0]` link. A new release check in tests/changelog.rs holds both together.

## Out of scope

- Tagging, `cargo publish`, the GitHub release, the clean install and the ROADMAP row. Leif does those after this merges, from the list sent with the review.
- Any code or contract change, and the open #180 and #69.

## Steps

1. Add `the_package_version_has_a_dated_section_and_a_release_link` to tests/changelog.rs. It passes at 0.4.0. Bump Cargo.toml and Cargo.lock to 0.5.0, and it fails ("no dated [0.5.0] section").
2. Get scope approval through the orc.
3. Move the Unreleased entries under the dated [0.5.0] section and add the link. Keep an empty `## [Unreleased]` heading.
4. Run `cargo package --list --locked` and `cargo publish --dry-run --locked`, and check the crates.io owner. Run `fledge lanes run verify` and `fledge run linux-verify`, then the native check last. Open the pull request.

## Risks

- The release check is generic: every later version bump needs its dated section first, which is the intent.
- The tests in tests/changelog.rs that pin where entries first shipped are unchanged.

## Constraints consulted

docs/releasing.md, `_audits/chough-release-proposal.md`, #116, LIFE-16.
