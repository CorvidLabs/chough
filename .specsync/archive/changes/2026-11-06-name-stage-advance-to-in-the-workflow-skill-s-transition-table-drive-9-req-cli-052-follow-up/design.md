---
change: name-stage-advance-to-in-the-workflow-skill-s-transition-table-drive-9-req-cli-052-follow-up
artifact: design
---

# Design

- **The row:** one new row in the "Transitions shared by every stage" table, after "Next stage":
  - Intent: "Several stages at once".
  - Command: `chough stage advance --to <stage>`.
  - Notes: every step is checked first, on one native read; a refused step records nothing; into done it reads native status once.
- **Both copies:** the same bytes go in `skills/chough-workflow/references/stages.md` and in this repository's installed copy, `.agents/skills/chough-workflow/references/stages.md`.
- **`src/skill.rs`:** `KNOWN_PREVIOUS` gains `references/stages.md` at 658ce798…, the copy shipped from 0.1.0 (#22) to main 01f1d023. Installed copies of it then report `known-outdated` and update. An edited copy still reports `edited` and is never overwritten.
- **Test:** `tests/package.rs` gets one test.
  - It writes main 01f1d023's `stages.md` into an installed copy and asserts its digest.
  - `skill status` reports it `known-outdated`.
  - `skill update` makes it `current`, and the result names `chough stage advance --to <stage>`.

## Risks

- **Byte drift between copies:** none expected. Both copies get the same bytes, and the existing package tests compare them.
- **Prose rules:** the no-em-or-en-dash check still applies.

## Constraints consulted

- REQ-cli-052 and DRIVE-9.
- The skill-copy rules in `src/skill.rs` (#54, 0.4.0).
- tests/prose.rs.
