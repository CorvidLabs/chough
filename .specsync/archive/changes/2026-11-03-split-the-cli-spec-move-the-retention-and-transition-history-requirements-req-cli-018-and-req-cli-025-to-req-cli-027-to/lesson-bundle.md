# Lesson bundle: split-the-cli-spec-move-the-retention-and-transition-history-requirements-req-cli-018-and-req-cli-025-to-req-cli-027-to

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Split the cli spec: move the retention and transition-history requirements REQ-cli-018 and REQ-cli-025 to REQ-cli-027 to a new history module as REQ-history-001 to REQ-history-004 with wording unchanged and a documented ID mapping, so the cli requirements companion has headroom under the 65536-byte record bound
- **Kind**: Refactor
- **Specs**: cli, history
- **Paths**: specs/cli/, specs/history/, .specsync/registry.toml, .chough/cases/, docs/FRD.md, docs/architecture.md, src/evidence.rs, tests/, CHANGELOG.md
- **Acceptance**: specs/history is a new module that owns src/history.rs, tests/history.rs and tests/evidence.rs and holds REQ-history-001 to REQ-history-004, each with the exact wording of REQ-cli-018, REQ-cli-025, REQ-cli-026 and REQ-cli-027; the old IDs are removed from specs/cli and become tombstones
- **Acceptance**: A test proves the move lost nothing: every moved requirement body has the SHA-256 it had at main 7bd1b62, every REQ-cli ID from 001 to 032 is either still in specs/cli or mapped, and a one-time before/after comparison of all 32 blocks is recorded
- **Acceptance**: The ID mapping is documented in docs/FRD.md and the history module context, and the cases that link the moved requirements move to .chough/cases/history.cases.toml under a documented case ID mapping with their bindings unchanged
- **Acceptance**: specs/cli/requirements.md is at most 59000 bytes, the record limit stays 65536 and no requirement, case binding or test is weakened
- **Acceptance**: fmt, lint, the full tests, every moved case and fledge lanes run verify pass

## Evidence

- Verification commit: `878c409732e761574897a3c46e24846d9bf80aa7`
- Base commit: `7bd1b62dc8ac563c4100302088c89113977ffe7f`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

PR #55 (knowledge directory) cannot verify after merging main: its cli requirements
companion grows to 66,244 bytes, over Chough's 65,536-byte structured record bound
(`read_record`, RECORD_LIMIT in src/records.rs). Main already sits at 65,051 bytes,
so only 485 bytes remain for any future cli requirement change.

The parent agent chose, at 2026-11-03 9:46 AM MT, to split the cli spec module
rather than raise the limit, keeping every requirement's wording, documenting an ID
mapping if SpecSync needs one, and weakening nothing.

SpecSync 6.1.2 needs one: a requirement in a module delta must be named
`REQ-<module>-<number>`, and a removed requirement ID is a permanent tombstone. So
the moved requirements get new `REQ-history-` IDs. A source file may be owned by one
spec only, and every spec must own at least one file.

The retention and transition-history group (REQ-cli-018, REQ-cli-025, REQ-cli-026,
REQ-cli-027) was chosen because no case links one of them together with a
requirement that stays in cli, and it has natural owned files: src/history.rs,
tests/history.rs and tests/evidence.rs. Other candidate groups (timing, bugs,
stage gates) share cases with requirements that stay, which would mean dropping or
duplicating case links.

Already ruled out: raising the limit (parent decision); shortening requirement text
(would weaken requirements); keeping `REQ-cli-` IDs in another module (SpecSync
refuses it).

## From the change's design.md

# Design

No visual or interaction design. Module boundary only:

| Module | Owns files | Requirements |
|---|---|---|
| cli | every current file except the three below | REQ-cli-001 to 017, 019 to 024, 028 to 032 |
| history | src/history.rs, tests/history.rs, tests/evidence.rs | REQ-history-001 to 004 |

Requirement mapping: REQ-cli-018 -> REQ-history-001, REQ-cli-025 -> REQ-history-002,
REQ-cli-026 -> REQ-history-003, REQ-cli-027 -> REQ-history-004.

Case mapping: `TC-cli-<rest>` -> `TC-history-<rest>`, and
`TC-cli-history-<rest>` -> `TC-history-<rest>`.

## From the change's testing.md

# Testing

- tests/spec_split.rs: each moved body's SHA-256 equals its digest at main 7bd1b62;
  every old ID is gone from specs/cli and its new ID is in specs/history; every
  REQ-cli ID from 001 to 032 is in cli or mapped; the cli companion stays under the
  record bound with headroom.
- A one-time before/after comparison of all 32 requirement blocks (old file at
  7bd1b62 against the two new companions) is recorded as an observation artifact.
- Every moved case (`chough cases run TC-history-...`) and the remaining cli cases
  touched by the module change run on local-linux-x86_64.
- chough next, status and bug list read the checkpoint and bug records that cite old
  case IDs without error.
- fmt, lint, the full tests and `fledge lanes run verify`.

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
