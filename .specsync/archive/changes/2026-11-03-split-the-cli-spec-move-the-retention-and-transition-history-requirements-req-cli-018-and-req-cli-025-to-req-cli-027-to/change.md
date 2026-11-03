---
id: split-the-cli-spec-move-the-retention-and-transition-history-requirements-req-cli-018-and-req-cli-025-to-req-cli-027-to
state: archived
type: refactor
base_commit: 7bd1b62dc8ac563c4100302088c89113977ffe7f
---

# Split the cli spec: move the retention and transition-history requirements REQ-cli-018 and REQ-cli-025 to REQ-cli-027 to a new history module as REQ-history-001 to REQ-history-004 with wording unchanged and a documented ID mapping, so the cli requirements companion has headroom under the 65536-byte record bound

## Intent

Split the cli spec: move the retention and transition-history requirements REQ-cli-018 and REQ-cli-025 to REQ-cli-027 to a new history module as REQ-history-001 to REQ-history-004 with wording unchanged and a documented ID mapping, so the cli requirements companion has headroom under the 65536-byte record bound

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- specs/history is a new module that owns src/history.rs, tests/history.rs and tests/evidence.rs and holds REQ-history-001 to REQ-history-004, each with the exact wording of REQ-cli-018, REQ-cli-025, REQ-cli-026 and REQ-cli-027; the old IDs are removed from specs/cli and become tombstones
- A test proves the move lost nothing: every moved requirement body has the SHA-256 it had at main 7bd1b62, every REQ-cli ID from 001 to 032 is either still in specs/cli or mapped, and a one-time before/after comparison of all 32 blocks is recorded
- The ID mapping is documented in docs/FRD.md and the history module context, and the cases that link the moved requirements move to .chough/cases/history.cases.toml under a documented case ID mapping with their bindings unchanged
- specs/cli/requirements.md is at most 59000 bytes, the record limit stays 65536 and no requirement, case binding or test is weakened
- fmt, lint, the full tests, every moved case and fledge lanes run verify pass

## No-spec Rationale

Not applicable
