---
id: prepare-the-0-2-0-release-set-the-package-version-to-0-2-0-and-date-the-change-log-section-for-0-2-0
state: archived
type: operations
base_commit: 8298e5ab773aa7f6fb4c8685a6a29689951fd92b
---

# Prepare the 0.2.0 release: set the package version to 0.2.0 and date the change log section for 0.2.0

## Intent

Prepare the 0.2.0 release: set the package version to 0.2.0 and date the change log section for 0.2.0

## Affected Canonical Specs

- None

## Acceptance Criteria

- Source: Leif (0xLeif), 2026-11-03 4:52 PM MST: "Tag v0.2.0 after #66 lands". #66 merged as 8298e5a.
- Cargo.toml declares version = "0.2.0", Cargo.lock records chough 0.2.0, and `chough --version` built from this commit prints `chough 0.2.0`.
- CHANGELOG.md: `## [0.2.0] - 2026-11-03` holds the former Unreleased entries, moved unchanged, plus entries for the merged changes since v0.1.0 that never touched the change log: #29 (`chough checkpoint --merge` and the `chough-session` Git merge-driver setup), #33 (slug IDs for new cases, observations, validation runs and snapshots) and #26 (`BUG-<slug>-<24 hex>` bug IDs), each written from its pull request and the current docs (found missing by review 5761319339). A new empty `## [Unreleased]` heading sits above it, and a `[0.2.0]` link points at https://github.com/CorvidLabs/chough/releases/tag/v0.2.0. The 0.1.0 section is unchanged.
- No behavior, requirement, skill bundle or HI criterion changes; `cargo package --list --locked` lists the same file set as for 0.1.0.
- Nothing is released by this change itself: the tag, any crates.io publish and the GitHub release are separate steps after merge (docs/releasing.md). The full suite on Ubuntu, macOS and Windows, the MSRV job, the Fledge verify lane, HI and SpecSync checks pass in CI.

## No-spec Rationale

A version bump and change log dating change no requirement; behavior is unchanged
