---
change: fix-349
artifact: testing
---

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
