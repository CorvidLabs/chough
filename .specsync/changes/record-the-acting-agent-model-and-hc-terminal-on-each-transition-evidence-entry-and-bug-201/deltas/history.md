# Actor on transition records (issue 201)

## ADDED

### REQUIREMENT REQ-history-016

Actor on transition records. A transition, in a session history event or the legacy log, MAY carry `actor` (REQ-cli-063) with only the keys `agent`, `model` and `terminal`, each present only when known; a transition recorded without a known actor SHALL carry no `actor` and keep its bytes. A `direct-native-write` event (REQ-history-013) SHALL carry the actor of the stage move that recorded it. The actor SHALL hold only the agent name, model and hc terminal id, never a working directory, home path or user name (#234). It is a claim by the recording process, not an authenticated identity, and SHALL never stand in for a human decision, approval or review. Readers that predate this requirement refuse a transition holding `actor`, so the change log SHALL name the first release that reads it. Trace: LIFE-17, LIFE-2, FLOW-16.

Acceptance Criteria
- A stage move recorded with an actor known stores `actor` in its transition with only `agent`, `model` and `terminal`, and a direct-write event recorded by that move carries the same actor.
- A stage move recorded with no actor known stores no `actor` key.
