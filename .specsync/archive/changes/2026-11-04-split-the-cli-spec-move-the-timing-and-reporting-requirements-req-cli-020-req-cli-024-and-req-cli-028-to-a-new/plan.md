---
change: split-the-cli-spec-move-the-timing-and-reporting-requirements-req-cli-020-req-cli-024-and-req-cli-028-to-a-new
artifact: plan
---

# Plan

## Approach

Create the `reporting` module and move three requirements into it byte for byte
under new IDs, with the cases that link them, through one native change whose
deltas remove the old IDs from cli and add the new IDs to reporting. Same shape as
the history split (#59).

## Out of scope

- Raising RECORD_LIMIT or any other bound.
- Editing any requirement wording, case binding, test assertion or HI criterion.
- Rewriting archived changes, evidence, bug records or decisions that cite old IDs;
  they stay as historical records and the mapping resolves them.
- PR #69's delta: it modifies REQ-cli-024 and REQ-cli-028 and adds REQ-cli-035, and
  must be pointed at REQ-reporting-002, REQ-reporting-003 and a new reporting ID on
  its own branch after this lands.

## Steps

1. Record the pre-split SHA-256 of every cli requirement block at main a33c747 and
   at this branch's merge 70009d6.
2. Add specs/reporting (spec, requirements preamble, context, testing, tasks) owning
   src/timing.rs and tests/timing.rs, removed from cli's files; register it.
3. Deltas: cli REMOVED REQ-cli-020, 024 and 028; reporting ADDED REQ-reporting-001
   to 003 with the identical bodies.
4. Move the nine cases that link only moved requirements to
   .chough/cases/reporting.cases.toml, and add a TC-reporting twin for each of the six
   co-linked cases, every other field unchanged; drop only the moved requirement from
   the six cli cases.
5. Document the requirement and case ID mappings in docs/FRD.md and
   specs/reporting/; update the IDs cited in live docs, cli testing and context notes
   and code comments.
6. Extend tests/spec_split.rs with the reporting mapping and digests and cover the
   new companion in the record-bound test.
7. Materialize, run every moved and twinned case, verify, and record a before/after
   comparison of every cli block.

## Risks

- Chough records that cite old case IDs (checkpoint references, bug links) could
  read differently; checked by running next, status and bug list after the move.
- PRs #72, #78, #70 and #77 merge main again after this; only #77 touches
  tests/spec_split.rs (one digest line in the history table, not changed here).

## Constraints consulted

AGENTS.md (preserve files and records, never invent results); SpecSync's ID and
tombstone rules; Chough's same-module case link rule; Leif's standing no-weakening
rule and the parent's no-limit-raise instruction; the history split (#59) as the
precedent.
