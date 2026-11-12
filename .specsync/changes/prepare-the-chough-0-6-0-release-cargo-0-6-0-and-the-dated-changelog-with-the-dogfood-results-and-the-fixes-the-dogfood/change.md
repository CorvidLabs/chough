---
id: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
state: approved
type: operations
base_commit: 166d1ea1d37971e4c95ee8ba02249a1a46b2a0d2
---

# Prepare the Chough 0.6.0 release: Cargo 0.6.0 and the dated CHANGELOG with the dogfood results, and the fixes the dogfood found

## Intent

Prepare the Chough 0.6.0 release: Cargo 0.6.0 and the dated CHANGELOG with the dogfood results, and the fixes the dogfood found

## Affected Canonical Specs

- `cli`
- `history`
- `records`

## Acceptance Criteria

- Cargo.toml and Cargo.lock name version 0.6.0, and chough --version prints chough 0.6.0
- CHANGELOG.md has a dated [0.6.0] section whose upgrade notes give the dogfood results, and tests/changelog.rs passes
- After an in-place migration, the local pointer that names a 0.5.x key resolves to its session through the alias, so next and status read it (D1, red then green)
- No test helper is named session_id, so CodeQL rust/cleartext-logging has nothing to flag in the tests
- Every live KNOW criterion has an active case linking it with a passing receipt recorded through Chough, so the KNOW gap list of #216 is empty
- The #148 bug record is verified through Chough: a red run against the code before #157, then a later green run
- Each of the four flaky-test bugs is settled through Chough: fixed and verified, or left open with its diagnostics and named to Leif
- The closeout leaves chough doctor clean and every earlier archived story byte-identical

## No-spec Rationale

Release preparation and dogfood fixes within the 0.6.0 contract: D1 makes the pointer resolve as REQ-records-009 already requires ('every record, alias and timeline entry is readable by next, status...'), and the rest is the version, the CHANGELOG, test-only renames, case links and evidence
