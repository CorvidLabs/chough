# Lesson bundle: prepare-the-0-2-0-release-set-the-package-version-to-0-2-0-and-date-the-change-log-section-for-0-2-0

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Prepare the 0.2.0 release: set the package version to 0.2.0 and date the change log section for 0.2.0
- **Kind**: Operations
- **Paths**: Cargo.toml, Cargo.lock, CHANGELOG.md
- **Acceptance**: Source: Leif (0xLeif), 2026-11-03 4:52 PM MST: "Tag v0.2.0 after #66 lands". #66 merged as 8298e5a.
- Cargo.toml declares version = "0.2.0", Cargo.lock records chough 0.2.0, and `chough --version` built from this commit prints `chough 0.2.0`.
- CHANGELOG.md: `## [0.2.0] - 2026-11-03` holds the former Unreleased entries, moved unchanged, plus entries for the merged changes since v0.1.0 that never touched the change log: #29 (`chough checkpoint --merge` and the `chough-session` Git merge-driver setup), #33 (slug IDs for new cases, observations, validation runs and snapshots) and #26 (`BUG-<slug>-<24 hex>` bug IDs), each written from its pull request and the current docs (found missing by review 5761319339). A new empty `## [Unreleased]` heading sits above it, and a `[0.2.0]` link points at https://github.com/CorvidLabs/chough/releases/tag/v0.2.0. The 0.1.0 section is unchanged.
- No behavior, requirement, skill bundle or HI criterion changes; `cargo package --list --locked` lists the same file set as for 0.1.0.
- Nothing is released by this change itself: the tag, any crates.io publish and the GitHub release are separate steps after merge (docs/releasing.md). The full suite on Ubuntu, macOS and Windows, the MSRV job, the Fledge verify lane, HI and SpecSync checks pass in CI.

## Evidence

- Verification commit: `c3fd10d7837eb888bcf2cddadab441fee4d06dd0`
- Base commit: `8298e5ab773aa7f6fb4c8685a6a29689951fd92b`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

Leif (0xLeif) asked on 2026-11-03 at 4:52 PM MST: "Tag v0.2.0 after #66 lands". PR #66
merged as 8298e5a. Chough 0.1.0 was tagged at c03d419 after a release preparation PR
(#22) and a change log dating PR (#27), then published to crates.io and as a GitHub
release (docs/ROADMAP.md, "0.1.0 release" row). Main still declares version 0.1.0 and
its change log holds everything since 0.1.0 under Unreleased, so a tag of v0.2.0 on main
as it stands would build a binary that reports 0.1.0.

This change only sets the version and dates the change log. The tag, any crates.io
publish and the GitHub release are separate steps after merge; each is its own
decision (docs/releasing.md, skills/chough-workflow/references/release.md).

## From the change's testing.md

# Testing

- `cargo build --locked` then `./target/debug/chough --version` prints `chough 0.2.0`.
- `fledge lanes run verify` passes (format, lint, the full test suite, HI and SpecSync
  checks), so no test depends on the old version string.
- `cargo package --list --locked` lists only `src/`, `skills/chough-workflow/`,
  `Cargo.toml`, `Cargo.lock`, `README.md` and `LICENSE`, plus cargo's generated files.
- Hosted CI on the pull request: Product on Ubuntu, macOS and Windows, MSRV 1.88.0,
  Fledge/HI/SpecSync and the archive gate.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
