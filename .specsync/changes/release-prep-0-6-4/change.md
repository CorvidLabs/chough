---
id: release-prep-0-6-4
state: verifying
type: feature
base_commit: 2577da8e23a07a308511a21d992c83419eac74e2
---

# Release-prep-0-6-4

## Intent

release-prep-0-6-4

## Affected Canonical Specs

- None

## Acceptance Criteria

- Cargo.toml and Cargo.lock say 0.6.4; CHANGELOG.md has a 0.6.4 section dated 2026-11-26 with every merged fragment from changelog.d, which is then empty, and its release link; fledge run release-check passes; cargo build --locked and cargo test --locked pass. Nothing is tagged or published.

## No-spec Rationale

A patch release changes no requirement text: the crate version becomes 0.6.4, the changelog fragments of merged changes are assembled into CHANGELOG.md, and the generated knowledge index is rebuilt.
