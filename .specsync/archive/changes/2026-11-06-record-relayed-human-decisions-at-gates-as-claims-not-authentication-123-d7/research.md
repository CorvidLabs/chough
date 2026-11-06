---
change: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7
artifact: research
---

# Research

Measured at main ec830cfb.

- Gates are free text in `awaiting_human`. `--clear-awaiting-human` clears them all
  and writes one `gate-closed` transition per gate, with no field for who decided.
  Decisions are free-text `--decision` strings.
- `Transition` already carries one optional field, `backfill` (REQ-history-005),
  omitted when absent so older records keep their bytes. `relayed_by` follows that
  pattern. `Transition` denies unknown fields, so a binary without this change
  refuses an event that carries `relayed_by`, as with `backfill`.
- The timing report reads transitions as JSON values and ignores extra keys.
- `ensure_text` bounds checkpoint text at 4096 bytes. The decision line is one
  checkpoint decision, so it gets the same bound.
- `stage show` and `context` print recent transitions as stored, so they show
  `relayed_by` with no extra code.
