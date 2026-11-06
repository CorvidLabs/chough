## ADDED

### REQUIREMENT REQ-cli-050

Relayed decision at a human gate. `chough checkpoint --close-gate <gate>` SHALL close the one open awaiting-human gate whose text is exactly `<gate>`, and SHALL require `--decided-by`, `--words`, `--where` and `--recorded-by`: who is said to have decided, their words verbatim, where and when they said it including any relay, and the agent recording it. It SHALL keep every other open gate open, append one `gate-closed` transition for that gate carrying `relayed_by` with those four values (REQ-history-010), and add one checkpoint decision `<decided_by> decided "<gate>": "<words>" (<where>); recorded by <recorded_by>; a relayed claim, not authenticated`. Its output SHALL carry the gate, the four values and the sentence `a relayed claim, not authenticated: it records who is said to have decided, not proof that they did`. Each refusal SHALL save nothing and write no transition: `refused:relay-incomplete` when any of the four is missing or blank, naming the missing flags and the complete command; `refused:relay-without-gate` when any of the four is given without `--close-gate`; `refused:gate-not-open` when no open gate has that exact text, naming `chough next`, which lists the open gates; and `refused:relay-too-long` when the decision would exceed 4096 bytes, saying to shorten `--words` or `--where`. `--close-gate` SHALL NOT be combined with `--clear-awaiting-human`, `--awaiting-human` or `--merge`. A relayed decision SHALL NOT write or change any SpecSync record and SHALL NOT satisfy or bypass any stage gate: reaching build, test, accept and done keeps reading native state. `--clear-awaiting-human` SHALL keep withdrawing every open gate with no relay and no decision. Chough SHALL NOT authenticate any of the four values. Trace: FLOW-16, FLOW-10, FLOW-13, FLOW-3.

Acceptance Criteria
- With gates A and B open, `--close-gate A` with the four values closes only A:
  - B stays open, and advance is still refused with `refused:awaiting-human` naming B;
  - one `gate-closed` event for A carries `relayed_by` with the four values byte for byte;
  - the decision appears in `chough next` from a fresh process, and the output carries the boundary sentence.
- A missing or blank value, a value without `--close-gate`, a gate that is not open and an over-long decision each refuse with the checkpoint and history unchanged, and the command each refusal names then succeeds.
- With a native change in draft, closing its scope gate with a relay leaves every SpecSync file byte-identical, and advance to build is still refused by the native gate.
- `--clear-awaiting-human` still closes every open gate and writes no `relayed_by` and no decision.

## MODIFIED

### REQUIREMENT REQ-cli-030

Open human gates hold forward moves. While the checkpoint's `awaiting_human` list is not empty, `chough stage advance` and `chough stage skip` SHALL be refused with `refused:awaiting-human` from every main stage and from release, before any native read; the refusal SHALL name at most 3 open gates, in checkpoint order, with the count of the rest, and SHALL say to record the human's decision on a gate with `chough checkpoint --close-gate "<gate>" --decided-by "<who>" --words "<their words>" --where "<where>" --recorded-by "<agent>"` (REQ-cli-050), and to withdraw gates no human decision closes with `chough checkpoint --clear-awaiting-human`, re-passing gates still waiting with `--awaiting-human`. No flag or reason SHALL bypass it. Iterate, back, reset, pause, resume and side-stage enter and return SHALL NOT be refused by it. Once the list is empty the same move SHALL be applied and gated as before. While a gate is open, the waiting next action in `chough next` and `chough status` SHALL say not to decide it for the human, SHALL name `chough checkpoint --close-gate` for recording their decision, and SHALL say that advance and skip are refused until it is cleared. Trace: FLOW-3, FLOW-5, FLOW-13, FLOW-2, FLOW-16.

Acceptance Criteria
- With one open gate, advance and skip are refused with `refused:awaiting-human` at intent, design, plan with SDD disabled, and release; the stage and the transitions log are unchanged and the message names the gate, the `--close-gate` command and the clearing command.
- With 5 open gates the message names the first 3 and says 2 more.
- After `chough checkpoint --clear-awaiting-human` the same advance is applied with its usual gate label; back, iterate, pause and resume, and side-stage enter and return, still work with a gate open.
- After `chough checkpoint --close-gate` closes the only open gate with a relay, the same advance is applied with its usual gate label.
- `chough next` and `chough status` give the same waiting action, which names `--close-gate` and says advance and skip are refused until the gate is cleared.
