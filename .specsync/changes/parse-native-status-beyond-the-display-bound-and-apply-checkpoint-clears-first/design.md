---
change: parse-native-status-beyond-the-display-bound-and-apply-checkpoint-clears-first
artifact: design
---

# Design

## Native output

`records.rs` gains a parsing path next to the display path. One process run yields
both:

- display: the existing bounded `Section` (16 KiB per stream, truncation marker);
- parsed: the full stdout decoded as JSON when the process succeeded and stdout is
  at most 4 MiB. Larger output is `oversized`; non-JSON output is `invalid-output`.

`stage::native_state` reads `summary.state` from the parsed value. Failed,
unavailable, invalid or oversized responses stay `NativeState::Unknown`, so the
gates still refuse.

`main::native_status` sets the section state from the full parse rather than the
displayed text. The report gains `native_summary` with `state` and `next_action`
when present. When the displayed content is cut, the section says it was
truncated for display only and was parsed in full.

## Checkpoint flags

`checkpoint_command` applies `--clear-blockers` and `--clear-awaiting-human` before
adding entries, so `--clear-awaiting-human --awaiting-human X` leaves `[X]`.

## Not changed

No new commands, record schemas or dependencies. Context assembly keeps its 16 KiB
section bound. Module entry points keep `pub(super)`.
