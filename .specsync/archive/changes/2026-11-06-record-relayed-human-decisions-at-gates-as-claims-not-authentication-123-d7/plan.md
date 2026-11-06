---
change: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7
artifact: plan
---

# Plan

## Approach

Add one structured relay to the existing gate-closed record, plus one handoff decision.
Then narrow the wording wherever Chough describes approvals, so it no longer reads
as more than a recorded claim.

## Out of scope

- Showing SpecSync's native `relayed_by` in Chough output. That waits for a SpecSync
  release that has it.
- Relay fields on `pref set` and `consent grant`, which already take `--source`.
- Any authentication or signing (spec-sync#633: attest).
- The timing report, which keeps counting gate-closed records as before.

## Steps

1. Commit the failing tests: tests/relay.rs, plus the refusal and waiting-text
   expectations in tests/stages.rs.
2. Implement the transition field and validation, then `--close-gate`.
3. Update the wording and docs. Add the KNOWN_PREVIOUS digests of the changed skill
   files from main, and update the installed copy.
4. Add the cases in `.chough/cases/cli.relayed-decisions.cases.toml`, and run them.
5. Run `fledge lanes run verify` locally and on Linux.

## Risks

- Older binaries refuse events carrying `relayed_by` (same as `backfill`). This is
  noted in the CHANGELOG, and this repository's agents use the repo build.
- Sibling branches may also add REQ-cli-050. If so, renumber after main's at merge
  (AGENTS.md).

## Constraints consulted

- AGENTS.md: never create approval or review records from an agent label.
- LIFE-15 / D-020: Chough's size bounds apply only to its own records.
- REQ-history-005: the pattern for an optional field.
- FLOW-3: add FLOW-16; FLOW-10 is not rewritten.
