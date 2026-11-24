---
change: fix-440
artifact: plan
---

# Plan

## Approach

Change the refusal message in the `stage advance --to` handler
(src/stage.rs, "is not ahead of") to name `--reason "<why>"` for both
suggested commands, matching the other back/reset hints, for example:
"use `chough stage back --reason \"<why>\"` or `chough stage reset
--reason \"<why>\"` to go back". Update or add the test that asserts on
this refusal text.

## Out of scope

Other refusal messages, the back/reset commands themselves, and any
contract (specs/) text.

## Steps

1. Fix the message string in src/stage.rs.
2. Update the test that covers the not-ahead refusal.
3. Add changelog fragment changelog.d/2026-11-23-fix-440.md.
4. cargo test the touched tests, then the change check lane.

## Risks

A test matching the old wording fails until updated; both edits land in
one commit.

## Constraints consulted

docs/exercises/cold-start.md (guidance gives every reset with --reason);
the four existing hints with `--reason "<why>"` in src/stage.rs.