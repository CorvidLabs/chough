# Lesson bundle: fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix #249: a relayed decision reaches SpecSync's approval and review records, each value joined to its flag
- **Kind**: BugFix
- **Specs**: cli, records
- **Paths**: src/humans.rs, src/checkpoint.rs, src/change.rs, tests/change.rs, tests/humans.rs, tests/relay.rs, docs/cli.md, CHANGELOG.md, specs/cli/requirements.md, specs/records/requirements.md, specs/records/records.spec.md, .chough/cases/cli.ship-relay.cases.toml
- **Acceptance**: Closing a gate that drives approval or review with a human's relay, including chough change ship's review, runs SpecSync with the relay's words, place and recording agent as --words, --where and --recorded-by, each value joined to its flag so text starting with - is never read as a flag

## Evidence

- Verification commit: `f9b016771f741f4d11f812c8feadeadd9b3b5dfa`
- Base commit: `d3138bed5495be77e66d35502a0891b156ffed8e`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

Issue #249: when a human's relayed decision drives a native step, Chough passed SpecSync no relay, so `review.json` and approvals carried no words, place or recording agent. PR #261 fixed this by passing `--words`, `--where` and `--recorded-by` (SpecSync 6.3.0). corvid-agent's review of #261 then found that relay text starting with `-`, which Chough's own checkpoint and gate flags accept, failed in SpecSync's argument parser.

#261's change was finalized on its branch before main moved (#284 changed REQ-cli-059). SpecSync refuses to redefine an applied change, and #261's archive never reached main, so this change replaces it from current main: #261's code and tests, the `--flag=value` fix, and deltas written against today's REQ-cli-059 and REQ-records-008. #261 will be closed in favor of this PR.

## From the change's design.md

# Design

`Step::arguments` (`src/humans.rs`) builds the native approve or review call and appends the relay. Every value is emitted joined to its flag (`--words=<value>`), so clap in SpecSync never reads a hyphen-leading value as a flag. `checkpoint` and `change ship` pass the relay through. No visual or interaction change.

## From the change's testing.md

# Testing

- `tests/change.rs`: `ship_passes_the_relay_to_the_native_review`, `ship_passes_a_relay_that_starts_with_a_hyphen_as_one_argument_per_field`.
- `tests/humans.rs`: scope, review and gate-record calls carry the relay; `a_relay_that_starts_with_a_hyphen_reaches_the_native_review_joined_to_its_flag`.
- The hyphen tests fail on the previous `Step::arguments` and pass with this change.
- Cases in `.chough/cases/cli.ship-relay.cases.toml`, including `TC-cli-ship-relay-hyphen-values-join-their-flags`.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/records/context.md`
