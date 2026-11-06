---
change: advance-through-several-stages-on-one-native-read-with-stage-advance-to-and-read-native-status-once-on-the-move-into
artifact: context
---

# Context

## Intent and decisions

DRIVE-9 and DRIVE-2 are in `hi/drive.md`, quoted from Leif's t811u brief and confirmed by him on 2026-11-03.
- DRIVE-9: "Several `stage advance` steps in a row share one native status read."
- DRIVE-2: each command makes "at most one native status read".

Leif's decisions, relayed by the orc (t18):
- 2026-11-05 21:51 MST, "Split" for PR #69:
  - a small new PR from main carries `stage advance --to` (DRIVE-9), as one SpecSync change with failing-first tests under per-change sessions;
  - the archive cache goes to kimi-perf32's #32 change;
  - #69 closes only after both replacements are open.
- 2026-11-05 21:52 MST, Q2 "Yes": the librarian at done reuses the gate's native read, so advance into done is one read (DRIVE-2).

This change takes REQ-cli-052. REQ-cli-050 (ch-d7) and REQ-cli-051 (KNOW-29) are claimed, and #32 took REQ-cli-053.

## What led here

PR #69 (corvid-agent, base 8298e5a) combined two things: a persistent cache of SpecSync's authenticated-archive verdict, and `stage advance --to`. It went stale and conflicting. On main 73a7da56 with SpecSync 6.2.0, measured on fresh scratch clones of Chough (122 archives) and Raven (148 archives), with Raven runs interleaved A/B:
- Test to done on Raven:
  - main took two commands, 55 to 66 s and three native reads;
  - #69's batch with its cache emptied took one command, 33 s and two reads.
- Main's move into done reads twice:
  - once for the gate;
  - once again for the same change, when the trim reads the latest archive's acceptance manifest.
- Load average was 42 to 117 on 20 cores, so the absolute times are inflated. The read counts are exact.

## Prior art and what is ruled out

- #69's `advance_to` and its two batch tests are the starting point. They are re-asserted here under per-change sessions and event history.
- Out of this change:
  - any cache that outlives a command (#32 owns the next/status cache);
  - any change to `next`, `status` or `timing`;
  - a cached gate for stage moves.
  Every native read stays live; a read is shared only within one command.
- Main is affected by chough#141: KNOW-29's roll-up hid the legacy history anchor, so new sessions cannot write events. Implementation waits for #141 to merge, then merges main.
