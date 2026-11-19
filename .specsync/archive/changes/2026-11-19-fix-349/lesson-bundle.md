# Lesson bundle: fix-349

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 349
- **Kind**: BugFix
- **Specs**: history, cli
- **Paths**: src/sessions.rs, src/evidence.rs, src/cases.rs, tests/evidence.rs, tests/cases.rs, docs/cli.md, specs/history/requirements.md, specs/history/history.spec.md, specs/cli/requirements.md, specs/cli/cli.spec.md, .chough/cases/history.story-line-shows-limitations.cases.toml, changelog.d/2026-11-18-fix-349.md
- **Acceptance**: An archived story's What was verified line of an entry that has limitations ends with '; limits: ' and those limitations before its ^<evidence id>, and the line of an entry without any holds no such text. The receipts are unchanged.

## Evidence

- Verification commit: `345f866c700c94830473ec8975f0ea62e8ec6254`
- Base commit: `235c3bfb063e835f67c4aae12f873ceac4074be7`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

GitHub #349, part 1, a follow-up to #342 (#286). Since #342 an archived story's
receipts keep each evidence entry's limitations, in the `## Receipts` block.
The human-readable What was verified line still showed only the claim, the
command and the fingerprint digits, so a reader had to read the receipts to see
what a claim did not cover.

Leif's decision in #349: "yes. The archived story's human-readable What was
verified line shows an entry's limitations next to its claim". On the scope of
this change, relayed by his planning session (hc t4c) on 2026-11-18: "Part 1
now; 2 and 3 later". Part 2 (limitations as a list in receipts) has a design
note on the issue; part 3 (a bound) waits for his number.

The line prints the receipt's `limitations` value as it is stored.

Part 3 was added to this change before it merged. The swarm found (62-F5) that
limitations are now written twice in a story, in the receipt and on the line, so 32
entries of 5 limitations of 3,500 characters pushed an archive past 1 MiB and left
the session stuck at done. Leif set the bound, relayed by his planning session:
"512 bytes × 8 per entry". It applies to what a person gives with `--limitation`;
the few limitations `cases run` writes itself are not refused, so a failed run is
always recorded.

## From the change's design.md

# Design

Nothing visual changes. A What was verified line of an archived story reads, for
example:

```text
- Probe reports healthy - `sh -c echo healthy: yes` 1a2b3c4d5e6f; limits: inherited environment variables were not recorded ^2026-11-18-tc-cli-0001
```

## From the change's testing.md

# Testing

`tests/evidence.rs::a_story_receipt_carries_each_field_its_entry_knows` is extended:
the story's What was verified line of an entry with two limitations ends
`; limits: <first>; <second> ^<id>`, and the line of an entry without any holds
no `limits:`. It fails on main, where the line is `<claim> ^<id>`. The whole test
suite passes with the change, so no other test pins a line that gains limits.

`tests/evidence.rs::a_verified_line_stays_one_line_whatever_its_claim_or_limits_hold`
(added after the tester's finding on #358): a summary and a limitation with line
breaks, one holding `## Timeline`, are folded into the one line, so the story gains no
extra item and no second Timeline heading. It fails without the fold; the summary case
fails on main too.

`tests/evidence.rs::limitations_are_bounded_at_512_bytes_and_8_per_entry` (part 3):
8 limitations of 512 bytes are written; a 513-byte one and a ninth are each refused
as `refused:limitations` with no entry written and the session unchanged.
`tests/cases.rs` refuses a 513-byte `--limitation` on `cases record` the same way.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-017 | the bound tests above |
| REQ-history-006 | the test above and the case in `.chough/cases/history.story-line-shows-limitations.cases.toml` |

## Where these lessons go

- `specs/history/context.md`
- `specs/cli/cli.3md`
