## ADDED

### REQUIREMENT REQ-cli-030

Open human gates hold forward moves. While the checkpoint's `awaiting_human` list is not empty, `chough stage advance` and `chough stage skip` SHALL be refused with `refused:awaiting-human` from every main stage and from release, before any native read; the refusal SHALL name at most 3 open gates, in checkpoint order, with the count of the rest, and SHALL say to record the human's decision and clear the gate with `chough checkpoint --clear-awaiting-human`, re-passing gates still waiting with `--awaiting-human`. No flag or reason SHALL bypass it. Iterate, back, reset, pause, resume and side-stage enter and return SHALL NOT be refused by it. Once the list is empty the same move SHALL be applied and gated as before. While a gate is open, the waiting next action in `chough next` and `chough status` SHALL also say that advance and skip are refused until it is cleared. Trace: FLOW-3, FLOW-5, FLOW-13, FLOW-2.

Acceptance Criteria
- With one open gate, advance and skip are refused with `refused:awaiting-human` at intent, design, plan with SDD disabled, and release; the stage and the transitions log are unchanged and the message names the gate and the clearing command.
- With 5 open gates the message names the first 3 and says 2 more.
- After `chough checkpoint --clear-awaiting-human` the same advance is applied with its usual gate label; back, iterate, pause and resume, and side-stage enter and return, still work with a gate open.
- `chough next` and `chough status` give the same waiting action, which says advance and skip are refused until the gate is cleared.
