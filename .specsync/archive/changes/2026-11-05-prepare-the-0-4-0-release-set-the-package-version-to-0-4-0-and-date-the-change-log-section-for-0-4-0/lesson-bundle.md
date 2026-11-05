# Lesson bundle: prepare-the-0-4-0-release-set-the-package-version-to-0-4-0-and-date-the-change-log-section-for-0-4-0

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Prepare the 0.4.0 release: set the package version to 0.4.0 and date the change log section for 0.4.0
- **Kind**: Operations
- **Specs**: cli
- **Paths**: Cargo.toml, Cargo.lock, CHANGELOG.md
- **Acceptance**: The package version is 0.4.0 in Cargo.toml and Cargo.lock, the change log has a dated 0.4.0 section including #110 and a v0.4.0 link, cargo package --list shows only the documented files, cargo publish --dry-run succeeds, and fledge lanes run verify passes on Linux. No tag, publish or GitHub release is made here.

## Evidence

- Verification commit: `a789da26fa3e2675a05a6962739dc8a8c16cf517`
- Base commit: `1eacdfd17eefe093f797d5fc8fe2dd0e09248bd6`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif decided (2026-11-05 09:43 MST, through the orc) that Chough 0.4.0 is tagged and released after #112, #114 and #110 are merged. All three are now on main: #112 as 6b0c0ea0, #114 as 99c02a57, and #115, which fixes #110, as 1eacdfd1. This change prepares the release as 0.2.0 and 0.3.0 were: it sets the package version and dates the change log section. Tagging, crates.io publication and the GitHub release are separate human decisions that come after this merges (docs/releasing.md).

## From the change's testing.md

# Testing

- `chough --version` reports 0.4.0.
- `cargo package --list --locked` lists only `src/`, `skills/chough-workflow/`, `Cargo.toml`, `Cargo.lock`, `README.md`, `LICENSE` and the files cargo generates.
- `cargo publish --dry-run --locked` succeeds.
- `fledge lanes run verify` passes on Linux.

These are recorded as observations.

## Where these lessons go

- `specs/cli/context.md`
