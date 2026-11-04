---
id: split-the-cli-spec-move-the-timing-and-reporting-requirements-req-cli-020-req-cli-024-and-req-cli-028-to-a-new
state: implementing
type: refactor
base_commit: 70009d64d4611d91d3b96026919c11b97c90d91f
---

# Split the cli spec: move the timing and reporting requirements REQ-cli-020, REQ-cli-024 and REQ-cli-028 to a new reporting module as REQ-reporting-001 to REQ-reporting-003 with wording unchanged and a documented ID mapping, so the cli requirements companion stays under the 65536-byte record bound

## Intent

Split the cli spec: move the timing and reporting requirements REQ-cli-020, REQ-cli-024 and REQ-cli-028 to a new reporting module as REQ-reporting-001 to REQ-reporting-003 with wording unchanged and a documented ID mapping, so the cli requirements companion stays under the 65536-byte record bound

## Affected Canonical Specs

- `cli`
- `reporting`

## Acceptance Criteria

- specs/reporting is a new module that owns src/timing.rs and tests/timing.rs and holds REQ-reporting-001 to REQ-reporting-003, each with the exact wording of REQ-cli-020, REQ-cli-024 and REQ-cli-028 (their Trace lines unchanged, still naming LIFE-10); the old IDs are removed from specs/cli and become tombstones
- tests/spec_split.rs proves the move lost nothing: every moved requirement body has the SHA-256 it had at main a33c747, every REQ-cli ID from 001 to 034 and 036 is either still in specs/cli or mapped, and a one-time before/after comparison of every cli requirement block is recorded
- The ID mapping is documented in docs/FRD.md and the reporting module context; the cases that link only moved requirements move to .chough/cases/reporting.cases.toml under a documented case ID mapping with their bindings unchanged, and each case that also links a requirement staying in cli keeps its cli ID and those links and gains a TC-reporting twin with the identical binding for the moved requirement, so every requirement keeps every case binding it had
- specs/cli/requirements.md is at most 59000 bytes, the record limit stays 65536 and no requirement, case binding or test is weakened
- fmt, lint, the full tests, every moved and twinned case and fledge lanes run verify pass

## No-spec Rationale

Not applicable
