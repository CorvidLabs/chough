---
change: advance-through-several-stages-on-one-native-read-with-stage-advance-to-and-read-native-status-once-on-the-move-into
artifact: requirements
---

# Requirements

This requirement is ADDED (see `deltas/cli.md`):
- **REQ-cli-052:**
  - `chough stage advance --to <stage>` moves through each main stage up to the target on one shared `specsync change status` read.
  - Every step is checked before anything is recorded, and a refused step refuses the whole batch with nothing recorded.
  - On a move into done, the trim of REQ-cli-041 takes the change's acceptance manifest from the gate's response, so the move makes one native read when the trim needs no other change's status.
  - Traces DRIVE-9 and DRIVE-2.

No existing requirement is modified:
- REQ-cli-008 already records "each advance" as a transition, so a batch records one per step.
- REQ-cli-041 and REQ-cli-033 name no separate read of the manifest.
