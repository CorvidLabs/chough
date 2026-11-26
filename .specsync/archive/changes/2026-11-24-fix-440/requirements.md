---
change: fix-440
artifact: requirements
---

# Requirements

Covers FLOW-2 (every stage can go back, and backtracking names what must
be recorded): the go-back hint a refusal gives must itself name the
required `--reason "<why>"`, as every other back or reset hint in Chough
does and as docs/exercises/cold-start.md documents. Observable outcome:
`chough stage advance --to <a stage not ahead>` refuses with a message
that names `--reason` for both `chough stage back` and `chough stage
reset`.