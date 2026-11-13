# Lesson bundle: fix-249-chough-passes-a-relayed-decision-s-words-place-and-recording-agent-to-specsync-s-review-and-approve

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix #249: chough passes a relayed decision's words, place and recording agent to SpecSync's review and approve
- **Kind**: BugFix
- **Specs**: records, cli
- **Paths**: src/humans.rs, src/checkpoint.rs, src/change.rs, tests/change.rs, tests/humans.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: Closing a gate that drives approval with a human's relay runs specsync change approve with the configured actor, the note, and the relay's words, place and recording agent as --words, --where and --recorded-by (red then green)
- **Acceptance**: Closing a review gate, through chough checkpoint --close-gate or chough change ship, runs specsync change review --reviewer with the verdict and the relay as --words, --where and --recorded-by, so SpecSync's review record carries the relayed decision (red then green)

## Evidence

- Verification commit: `6821f9a6106eb57815d09af59184167b3e27b30b`
- Base commit: `338bbae296e3a7f9b00f57018b46ab8f27b932f8`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

t384's reviews of #242 and #244 found that SpecSync's `review.json` for a review Chough
ran names the reviewer (0xLeif) and the verdict, but not the relay: Leif's words, where
he said them, and the agent that recorded them. `src/humans.rs` passes only
`--reviewer` and `--verdict` to `specsync change review`, and `--actor` and `--note`
to `specsync change approve`, where the relay lives only in the note's text, as
REQ-records-008 words it. SpecSync 6.3.0, which Chough installs, accepts `--words`,
`--where` and `--recorded-by` on both commands. t370 reported it; it is recorded as
`.chough/bugs/2026-11-13-chough-change-ship-s-native-review-drops-the-relay-review-json.md`
and filed as GitHub #249.

On 2026-11-13T03:44Z Leif chose "Reviews and approvals (Recommended)" (relayed by
t370): both records carry his words, where he said them and who relayed them; t38b
builds it, and an issue gets filed. The note on approvals stays as it is.

## From the change's design.md

# Design

`Step::arguments` in `src/humans.rs` takes the relay and appends `--words`, `--where`
and `--recorded-by` to both `specsync change approve` (after `--actor` and `--note`)
and `specsync change review` (after `--reviewer` and `--verdict`). The callers in
`src/checkpoint.rs` (gate closes, `chough gate record`) and `src/change.rs` (`chough
change ship`) pass the relay they already checked. The approval note keeps its text.
No new flag on any Chough command. REQ-records-008 names the new arguments, and
REQ-cli-059's ship criterion says the review gets the relay. `docs/cli.md` and the
CHANGELOG's Unreleased section describe it.

## From the change's testing.md

# Testing

With a stub `specsync` that records its arguments: closing a scope gate that drives
approval runs `specsync change approve` with `--words`, `--where` and `--recorded-by`
equal to the relay's values; closing a review gate, and `chough change ship` with the
review gate's relay, run `specsync change review` with the same three. Red at the test
commit, green at the fix. The full local suite at the final head.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-008 | `tests/humans.rs`: approve and review closes pass the relay as `--words`, `--where` and `--recorded-by` |
| REQ-cli-059 | `tests/change.rs`: the ship's review carries the relay |

## Where these lessons go

- `specs/records/context.md`
- `specs/cli/cli.3md`
