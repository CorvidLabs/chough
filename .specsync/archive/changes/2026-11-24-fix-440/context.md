---
change: fix-440
artifact: context
---

# Context

Found by the 0.6.2 documentation audit and filed as chough#440. When
`stage advance --to` names a stage that is not ahead, its refusal says
"(use `chough stage back` or `chough stage reset` to go back)"
(src/stage.rs, the `ensure!` in the advance-to handler). Both of those
commands require `--reason`, and every other back or reset hint in Chough
gives `--reason "<why>"` (src/stage.rs around lines 1117, 1120, 1901 and
3013). docs/exercises/cold-start.md says Chough's guidance gives every
reset with `--reason`, so this one hint is off-contract.

Constraints: refusal wording is behavior other tests match on; the fix is
one message string plus its test. No contract text changes (rationale
recorded at change new).