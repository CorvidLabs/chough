# Acting agent identity (issue 201)

## ADDED

### REQUIREMENT REQ-cli-063

Acting agent identity. Each stage transition, evidence entry and bug the helper records SHALL carry an optional `actor` of `agent`, `model` and `terminal`, each present only when known: `CHOUGH_ACTOR`, `CHOUGH_ACTOR_MODEL` and `CHOUGH_HC_TERMINAL` SHALL take precedence; otherwise the agent SHALL be `claude-code` when `CLAUDECODE` is set and the terminal SHALL be `HC_TERMINAL_ID`. Each value SHALL be trimmed, single-line and at most 128 bytes. With nothing known the record SHALL carry no `actor`, so its bytes are as before, and no command SHALL fail because the identity is unknown. The actor SHALL NOT change a bug's ID. It is a claim by the recording process, not an authenticated identity, and SHALL never stand in for a human decision, approval or review. `chough next` (text and JSON) and `chough status` SHALL name who made the latest recorded transition as `last_actor`; `next` SHALL name who recorded each observation it lists as `recorded_by`, and `status` SHALL show each observation record with its `actor`. Chough builds before this requirement refuse a record holding `actor`, so the change log SHALL name the first release that contains it as the oldest Chough that reads it. Trace: LIFE-17, LIFE-2, FLOW-16.

Acceptance Criteria
- With the three `CHOUGH_*` variables set, a stage transition, an evidence add and a bug add each record `actor` with the agent, model and terminal; with `CLAUDECODE` and `HC_TERMINAL_ID` set, they record agent `claude-code` and that terminal.
- With none of the variables set, every command succeeds, no record carries `actor`, and `next` reports `last_actor` as null.
- `next` and `status` report the latest transition's actor as `last_actor`, and `next`'s text form shows it as `Last move by:`.
