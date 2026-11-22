---
id: release-0-6-3
state: draft
type: operations
base_commit: 61479feb1e88a57bc45716ff4e42ce84fb2b463b
---

# Release 0.6.3

## Intent

Release 0.6.3

## Affected Canonical Specs

- None

## Acceptance Criteria

- Cargo.toml and Cargo.lock say 0.6.3; CHANGELOG.md has a 0.6.3 section dated 2026-11-22 with every merged fragment from changelog.d, which is then empty, and its release link; fledge run release-check passes; cargo package --list --locked lists only the package files; cargo publish --dry-run --locked passes. Nothing is tagged or published.

## No-spec Rationale

A patch release changes no requirement text: the crate version becomes 0.6.3, the changelog fragments of merged changes are assembled into CHANGELOG.md, and the generated knowledge index is rebuilt.
