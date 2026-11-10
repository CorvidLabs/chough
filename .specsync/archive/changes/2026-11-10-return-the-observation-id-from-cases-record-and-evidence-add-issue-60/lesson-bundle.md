# Lesson bundle: return-the-observation-id-from-cases-record-and-evidence-add-issue-60

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Return the observation ID from cases record and evidence add (issue 60)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/cases.rs, src/evidence.rs, tests/cases.rs, .chough/cases/cli.observation-ids-60.cases.toml, docs/cli.md, CHANGELOG.md, knowledge/
- **Acceptance**: On base 5ff54dd7, new focused tests fail because cases record omits case and observation and evidence add omits observation. After the fix, cases record returns case equal to the requested case ID and observation equal to both the recorded file stem and stored record id; evidence add returns observation with the same equality. Existing keys, record bytes apart from ordinary ID allocation, exit codes and stored schemas remain unchanged; docs/cli.md names the returned keys, the focused cases and full Fledge verify lane pass locally.

## Evidence

- Verification commit: `980c3bec791d3b2b7b906dab90e4a5f2417c40fa`
- Base commit: `5ff54dd726367321b4e7ede3f931d38657b9b81e`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #60 reports that automation cannot directly consume the observation ID it
just created: `cases run` returns `case` and `observation`, while `cases record`
and `evidence add` return only the evidence path in `recorded`. Kimi reproduced
this on main, and the issue's bounded fix is to expose the IDs already present in
memory and in the stored record.

The earlier issue comment describes a prepared local branch, but no such branch,
pull request or native approval exists in the current repository. This change is
therefore planned from main `5ff54dd7`; it inherits no approval or verification.

Existing HI criteria FLOW-7 and LIFE-4 trace this small CLI correction. The
canonical amendment modifies REQ-cli-011 for `cases record` and REQ-cli-017 for
`evidence add`; REQ-cli-023's allocation behavior stays unchanged. No new HI or
requirement ID is needed. The related bug-output and stage-refusal envelope
suggestions in issue #60 are explicitly out of scope; so is adding unreferenced
observations to `next`.

Leif said "yes to all" to the original scope at 91345d26, but native approval
was refused because its no-spec flag conflicted with the declared public
contract change. No approval was recorded. This draft-only amendment preserves
the accepted behavior and acceptance criteria while supplying the required
canonical delta for a fresh decision.

Constraints: failing tests first; one theme per PR; preserve existing JSON keys,
exit codes, record files and schemas; use a change-specific cases file; strict
audit before every push; archive before waiting for hosted CI; no release work.

## From the change's design.md

# Design

## Approach

Amend REQ-cli-011 and REQ-cli-017 through one `cli` delta, using the existing
IDs and preserving every existing sentence and acceptance criterion while adding
the two additive response contracts. Do not change REQ-cli-023 or allocate a new
HI or requirement ID.

In `record_manual`, add the existing `id` as `case` and the allocated or explicit
`observation` as `observation` to the JSON value it already returns. In
`evidence::add`, add the stored record's existing `id` as `observation` to its
JSON output. Do not add a shared envelope type or alter any writer, ID allocator,
record schema, exit path or unrelated command.

Add two integration regressions in `tests/cases.rs`: one for manual case
recording and one for direct evidence addition. Each parses the returned JSON,
checks the additive keys, and proves the observation ID equals both the
`recorded` path's file stem and the stored record's `id`. The manual case also
checks `case` and preserves assertions for the existing keys and outcome.

Bind each regression in `.chough/cases/cli.observation-output.cases.toml` with
criteria FLOW-7 and LIFE-4 and the existing requirement links. Update the CLI
reference and add one Unreleased changelog bullet.

## Out of scope

Bug add/update/show output normalization, structured stage refusals, `next`
listing unreferenced observations, stored-record changes, ID allocation changes,
and any release or deployment step.

## Size and risk

Small: two additive response fields at two existing return sites plus focused
regressions and documentation. The main uncertainty is reusing the existing
fixtures without weakening their current assertions. Recorded history has 53 bug
fix times with a 10m47s median and a 45s to 37h30m25s range, and too few human
gate samples; these elapsed wall-clock figures are context, not an estimate or
deadline for this change. A consumer that rejects unknown JSON keys could notice
the additive fields; that is the explicitly requested public-contract correction,
not a reason to change existing keys.

Only k-ch-docs owns this lane and its listed files. No parallel implementation
or file handoff is planned.

## From the change's testing.md

# Testing

Add `TC-cli-manual-record-returns-observation-id` and
`TC-cli-evidence-add-returns-observation-id` in a change-specific cases file.
Both bind focused `cargo-test` regressions in `tests/cases.rs` and run on local
macOS arm64 and the hosted Linux and macOS environments. The manual-record case
links REQ-cli-011, and the direct-evidence case links REQ-cli-017; both trace
FLOW-7 and LIFE-4 and may use their approved delta links before materialization.

The manual-record regression invokes `cases record` and asserts `case`, the new
`observation`, the pre-existing `recorded`, `outcome` and `provenance` fields.
The evidence regression invokes `evidence add` and asserts its new `observation`
beside the existing fields. In both, the returned observation must equal the
recorded path stem and the stored record's `id`; parsing an ID from a path alone
is not sufficient.

After scope approval, add the tests and definitions before changing either
response. Run both against the unfixed implementation, retain the failed output,
and link it to one deduplicated local bug. After implementation, rerun the same
cases and verify the bug only from a strictly later pass. Run the complete Rust
suite through Fledge, the full verify lane, `specsync change audit --strict`, and
pre-push. Record what actually ran on the final content; do not claim hosted CI,
human review, archive, merge or deployment before it is observed.

Existing suites cover the unchanged stored schemas, ID allocation, legacy reads,
exit codes and failure outcomes. No visual, localization, release or live-health
check applies.

## Where these lessons go

- `specs/cli/cli.3md`
