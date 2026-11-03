---
change: split-the-cli-spec-move-the-retention-and-transition-history-requirements-req-cli-018-and-req-cli-025-to-req-cli-027-to
artifact: plan
---

# Plan

## Approach

Create the `history` module and move four requirements into it byte-for-byte under
new IDs, with the cases that link them, through one native change whose deltas
remove the old IDs from cli and add the new IDs to history.

## Out of scope

- Raising RECORD_LIMIT or any other bound.
- Editing any requirement wording, case binding, test assertion or HI criterion.
- Rewriting archived changes, evidence, bug records or decisions that cite old IDs;
  they stay as historical records and the mapping resolves them.
- Further splits of the cli module.

## Steps

1. Record the pre-split SHA-256 of every cli requirement block at main 7bd1b62.
2. Add specs/history (spec, requirements preamble, context, testing, tasks) owning
   src/history.rs, tests/history.rs and tests/evidence.rs, removed from cli's files.
3. Deltas: cli REMOVED REQ-cli-018 and REQ-cli-025 to 027; history ADDED
   REQ-history-001 to 004 with the identical bodies.
4. Move the 22 cases that link only those requirements to
   .chough/cases/history.cases.toml, renaming `TC-cli-` to `TC-history-` (dropping a
   repeated `history-`), with every other field unchanged.
5. Document the requirement and case ID mappings in docs/FRD.md and
   specs/history/context.md; update the IDs cited in docs, cli testing notes and two
   comments.
6. Add tests/spec_split.rs pinning the mapping and the moved bodies' digests.
7. Materialize, run every moved case, verify, and record a before/after comparison
   of all 32 blocks.

## Risks

- Chough records that cite old case IDs (checkpoint references, bug links) could
  read differently; checked by running next, status and bug list after the move.
- PR #55 must merge main again; its deltas touch only REQ-cli-015 and REQ-cli-033.

## Constraints consulted

AGENTS.md (preserve files and records, never invent results); SpecSync's ID and
tombstone rules; the parent's no-weakening and no-limit-raise instruction.
