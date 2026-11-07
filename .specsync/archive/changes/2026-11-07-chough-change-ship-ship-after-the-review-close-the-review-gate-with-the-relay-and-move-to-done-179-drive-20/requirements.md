---
change: chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20
artifact: requirements
---

# Requirements

ADDED REQ-cli-059 (see `deltas/cli.md`): `chough change ship <id>` with the review gate's relay.
- Before running anything: checks the session, the stage, the open gates and the relay; each refusal names its recovery.
- Runs `specsync change ship <id>` unbounded and passes its output through.
- After success: closes the gate as a relayed claim and moves the session to done.
- After a refused follow-up step: names each remaining command.
- A stop signal stops SpecSync (FLOW-14).

REQ-cli-050 and REQ-cli-052 are unchanged; this change runs their commands. Trace: DRIVE-20, FLOW-16 (as amended), FLOW-14.
