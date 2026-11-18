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

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-006 | the test above and the case in `.chough/cases/history.story-line-shows-limitations.cases.toml` |
