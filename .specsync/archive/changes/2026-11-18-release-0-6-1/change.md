---
id: release-0-6-1
state: archived
type: operations
base_commit: 975af07cc89912f9ea02d27c81fe87f046685eaa
---

# Release 0 6 1

## Intent

release 0 6 1

## Affected Canonical Specs

- None

## Acceptance Criteria

- Cargo.toml and Cargo.lock say 0.6.1; CHANGELOG.md has a 0.6.1 section holding every fragment of a change merged to main and changelog.d holds none of them; fledge run release-check passes; cargo publish --dry-run --locked succeeds; the two changelog tests that read live fragments pass with changelog.d empty, from fixture copies and the released section. Nothing is tagged or published.

## No-spec Rationale

A patch release changes no requirement text: the crate version becomes 0.6.1, the changelog fragments of merged changes are assembled into CHANGELOG.md, and the generated knowledge index is rebuilt.
