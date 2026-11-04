---
id: remove-the-65536-byte-cap-on-spec-requirements-files-from-tests-spec-split-rs-spec-files-have-no-size-cap
state: archived
type: bug_fix
base_commit: 270ca0cbadea844d6239c4b48da8c29bdb439713
---

# Remove the 65536-byte cap on spec requirements files from tests/spec_split.rs (spec files have no size cap)

## Intent

Remove the 65536-byte cap on spec requirements files from tests/spec_split.rs (spec files have no size cap)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- tests/spec_split.rs no longer fails when a spec requirements file is over 65536 bytes; its wording and mapping tests still pass; the two split cases expect 4 passing tests; fledge lanes run verify passes on Linux with specs/cli/requirements.md over 65536 bytes.

## No-spec Rationale

Test-only change: Chough bounds cover only its own .chough records (LIFE-15, REQ-cli-042); spec files have no size cap. No requirement text changes.
