---
change: parse-native-status-beyond-the-display-bound-and-apply-checkpoint-clears-first
artifact: requirements
---

# Requirements

No requirement text changes. The fix restores behavior the canonical contract
already requires:

- REQ-cli-008: leaving accept SHALL require the native change to be archived, and
  unavailable, unknown or undecodable native state SHALL refuse. A valid archived
  response must therefore be read in full; only a genuinely failed, unavailable,
  undecodable or oversized response may count as unknown.
- REQ-cli-007: the checkpoint SHALL hold awaiting-human gates and blockers, so an
  entry given on the command line must not be dropped by a clear in the same call.
- Invariant 5 (bounded input) is kept: native output is parsed within a 4 MiB bound,
  the same as SpecSync's own artifact limit, and displayed within the existing
  16 KiB bound.

Traceability: FLOW-10, FLOW-13, LIFE-7; BUG-0004, BUG-0005.
