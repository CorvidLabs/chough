# Lesson bundle: split-the-cli-spec-move-the-timing-and-reporting-requirements-req-cli-020-req-cli-024-and-req-cli-028-to-a-new

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Split the cli spec: move the timing and reporting requirements REQ-cli-020, REQ-cli-024 and REQ-cli-028 to a new reporting module as REQ-reporting-001 to REQ-reporting-003 with wording unchanged and a documented ID mapping, so the cli requirements companion stays under the 65536-byte record bound
- **Kind**: Refactor
- **Specs**: cli, reporting
- **Paths**: specs/cli/, specs/reporting/, .specsync/registry.toml, .chough/cases/, docs/FRD.md, docs/ROADMAP.md, docs/cli.md, src/records.rs, src/skill.rs, src/timing.rs, tests/, CHANGELOG.md
- **Acceptance**: specs/reporting is a new module that owns src/timing.rs and tests/timing.rs and holds REQ-reporting-001 to REQ-reporting-003, each with the exact wording of REQ-cli-020, REQ-cli-024 and REQ-cli-028 (their Trace lines unchanged, still naming LIFE-10); the old IDs are removed from specs/cli and become tombstones
- **Acceptance**: tests/spec_split.rs proves the move lost nothing: every moved requirement body has the SHA-256 it had at main a33c747, every REQ-cli ID from 001 to 034 and 036 is either still in specs/cli or mapped, and a one-time before/after comparison of every cli requirement block is recorded
- **Acceptance**: The ID mapping is documented in docs/FRD.md and the reporting module context; the cases that link only moved requirements move to .chough/cases/reporting.cases.toml under a documented case ID mapping with their bindings unchanged, and each case that also links a requirement staying in cli keeps its cli ID and those links and gains a TC-reporting twin with the identical binding for the moved requirement, so every requirement keeps every case binding it had
- **Acceptance**: specs/cli/requirements.md is at most 59000 bytes, the record limit stays 65536 and no requirement, case binding or test is weakened
- **Acceptance**: fmt, lint, the full tests, every moved and twinned case and fledge lanes run verify pass

## Evidence

- Verification commit: `78f2cfed95d94b54338e987b3728075923ea8d4c`
- Base commit: `70009d64d4611d91d3b96026919c11b97c90d91f`
- Verified by: `specsync check --spec cli --spec history --spec reporting`

## From the change's context.md

# Context

PR #71 (refuse an unknown native change, issue 63) cannot land after merging main
a33c747 (#79): its cli requirements companion grows to 65,698 bytes, over the
65,536-byte bound that tests/spec_split.rs (`every_requirements_companion_fits_the_record_bound`)
enforces. Main alone is 64,152 bytes. The next PRs in the train add more to cli
(#72 about 248 bytes, #78 about 988, #77 about 1,658), so the companion would reach
about 68,600 bytes before #69.

The parent agent chose, on the evening of 2026-11-03 (MST) under Leif's (0xLeif)
standing Chough authority (2026-11-03 7:49 AM MST) and his t820u and t822u asks to
finish and merge the Chough PRs, to split again rather than raise the bound, as the
history split did (#59, 4daf861): keep every requirement's wording, document the ID
mapping, and weaken nothing. Leif's own words on 2026-11-03 at 10:05 AM MT favour
this: "we should break stuff apart and decouple it if possible but if things need
to be coupled, that's fine."

The timing and reporting group (REQ-cli-020, REQ-cli-024, REQ-cli-028, 7,604 bytes)
was chosen because all three trace LIFE-10, they have natural owned files
(src/timing.rs, tests/timing.rs), and no open train PR before #69 modifies any of
them (#72 touches 009, #71 and #77 touch 007, #78 touches 021). #69 modifies
REQ-cli-024 and REQ-cli-028 and adds REQ-cli-035, so its growth lands in the new
module once its delta is pointed at the new IDs.

Unlike the history group, REQ-cli-020 and REQ-cli-024 share six cases with
requirements that stay in cli (TC-cli-0080, 0082, 0088, 0089, 0100, 0104). Chough
validates a case's requirement links only against its own module's companion, so
such a case cannot link both. Each keeps its cli ID and its links to the cli
requirements and gains a TC-reporting twin with the identical binding for the moved
requirement; no requirement loses a case binding.

Already ruled out: raising the limit (Leif's no-weakening rule and the parent's
choice); shortening requirement text (weakens requirements); keeping `REQ-cli-` IDs
in another module (SpecSync refuses it); the cases and bugs group (REQ-cli-004 to
017, about 16,300 bytes and 54 cases, collides with #72's REQ-cli-009 delta).

## From the change's design.md

# Design

No visual or interaction design. Module boundary only:

| Module | Owns files | Requirements |
|---|---|---|
| cli | every current file except the two below and history's | REQ-cli-001 to 017, 019, 021 to 023, 029 to 034, 036 |
| history | unchanged | REQ-history-001 to 005 |
| reporting | src/timing.rs, tests/timing.rs | REQ-reporting-001 to 003 |

Requirement mapping: REQ-cli-020 -> REQ-reporting-001, REQ-cli-024 ->
REQ-reporting-002, REQ-cli-028 -> REQ-reporting-003.

Case mapping: `TC-cli-<rest>` -> `TC-reporting-<rest>`, and `TC-cli-report-<rest>`
-> `TC-reporting-<rest>` (the repeated `report` is dropped). Moved: TC-cli-0081,
0083 to 0087, report-case-read-budget, report-read-invalidation and
report-trace-parity. Twinned (the cli case keeps its other links, the twin carries
the moved requirement): TC-cli-0080, 0082, 0088, 0089, 0100 and 0104.

## From the change's testing.md

# Testing

- tests/spec_split.rs: each moved body's SHA-256 equals its digest at main a33c747;
  every old ID is gone from specs/cli and its new ID is in specs/reporting; every
  REQ-cli ID from 001 to 034 and 036 is in cli or mapped; every companion, reporting
  included, stays under the record bound.
- A one-time before/after comparison of every cli requirement block (old file at
  70009d6 against the three companions) is recorded as an observation artifact.
- Every moved and twinned case (`chough cases run TC-reporting-...`) and the six cli
  cases that lost a moved link run on local-linux-x86_64.
- chough next, status and bug list read the checkpoint and bug records that cite old
  requirement and case IDs without error.
- fmt, lint, the full tests and `fledge lanes run verify`.

## Where these lessons go

- `specs/cli/context.md`
- `specs/reporting/context.md`
