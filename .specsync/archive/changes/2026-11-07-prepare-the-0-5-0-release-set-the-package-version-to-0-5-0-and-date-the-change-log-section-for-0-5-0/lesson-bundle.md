# Lesson bundle: prepare-the-0-5-0-release-set-the-package-version-to-0-5-0-and-date-the-change-log-section-for-0-5-0

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Prepare the 0.5.0 release: set the package version to 0.5.0 and date the change log section for 0.5.0
- **Kind**: Operations
- **Specs**: cli
- **Paths**: Cargo.toml, Cargo.lock, CHANGELOG.md, tests/changelog.rs
- **Acceptance**: Leif ('Prepare 0.5.0 now', relayed by orc 2026-11-06). The package version is 0.5.0 in Cargo.toml and Cargo.lock. The change log moves every Unreleased entry under '## [0.5.0] - 2026-11-06' and adds a [0.5.0] link to the v0.5.0 release. A new check in tests/changelog.rs fails first at 0.5.0 without that section, and passes once the dated section and the link are in place. cargo package --list --locked shows only the documented files, cargo publish --dry-run --locked succeeds, and fledge lanes run verify and fledge run linux-verify pass. No tag, publish or GitHub release is made here.

## Evidence

- Verification commit: `4c62137054a69051df5dfdbc40715774e2e0adcd`
- Base commit: `c58b4be79e1417662094e4d75c8b416e82123789`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif asked for the 0.5.0 release preparation ("Prepare 0.5.0 now", relayed by the orc on 2026-11-06), per `docs/releasing.md` and `_audits/chough-release-proposal.md`. The cut is main at c58b4be7, after #178. Open #180 (performance) and the parked #69 are not in it.

Why 0.5.0, not 0.4.1: a history event that carries `relayed_by` (#140) cannot be read by 0.4.0, so under Cargo's 0.x rules this is a breaking release. Native Windows is also dropped (#139, FLOW-17), and there are new commands: `stage advance --to`, `session shelve`, `session close`, `checkpoint --close-gate`, `chough doctor` and the fledge Linux tasks.

This follows #116, the 0.4.0 preparation: version, a dated change log section and its link, with no tag, publish or GitHub release.

## From the change's testing.md

# Testing

- **New check:** `the_package_version_has_a_dated_section_and_a_release_link` in tests/changelog.rs. It requires the `CARGO_PKG_VERSION` section to be dated `YYYY-MM-DD`, to list at least one entry, and to have a `[X.Y.Z]: https://github.com/CorvidLabs/chough/releases/tag/vX.Y.Z` link. It passes at 0.4.0 and fails first at 0.5.0, before the change log is dated.
- **Release steps from docs/releasing.md:** `cargo package --list --locked` lists only `src/`, `skills/chough-workflow/`, the Cargo files, `README.md` and `LICENSE`. `cargo publish --dry-run --locked` succeeds. The crates.io owner is checked.
- **Lanes:** `fledge lanes run verify` and `fledge run linux-verify` pass.

## Where these lessons go

- `specs/cli/context.md`
