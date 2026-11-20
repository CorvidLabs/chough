---
kind: session
id: 2026-11-20-change-leaks
aliases: [change-leaks]
objective: "A change's description, rationale and answers pass the leak check before SpecSync writes them, doctor --leaks scans the open changes, and the last echo sites name what a value holds (#412, rest of #367)"
intent_scope: ""
modules: [cli, records]
stage: build
stage_status: active
iteration: 1
callers: []
native_change: change-leaks
pr: ""
criteria: [LIFE-26]
requirements: [REQ-cli-057, REQ-records-006]
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue build
---
# A change's description, rationale and answers pass the leak check before SpecSync writes them, doctor --leaks scans the open changes, and the last echo sites name what a value holds (#412, rest of #367)

## Goal

A change's description, rationale and answers pass the leak check before SpecSync writes them, doctor --leaks scans the open changes, and the last echo sites name what a value holds (#412, rest of #367)

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of change-leaks": "A: refuse, --redact (Recommended); 1: open changes (Recommended)" (Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 15:08Z, relayed by hc t4c; the questions as shown: 'Chough #412: a change's description, rationale and answers aren't leak-checked today ... When one holds a leak, what should happen?' and 'Chough #412: what should doctor --leaks also scan?' (.specsync/changes/ but not each change's deltas/, and --fix --redact skips those findings); on change-leaks at 01b7952d with choices (A) and (1)); recorded by chough-fixer-t4n; a relayed claim, not authenticated; ran specsync change approve change-leaks as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-20T14:54:25Z · start · intent → intent · A change's description, rationale and answers pass the leak check before SpecSync writes them, doctor --leaks scans the open changes, and the last echo sites name what a value holds (#412, rest of #367) ^t-295bc8
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:54:26Z · advance · intent → design ·  ^t-72e0e1
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T14:54:26Z · skip · design → plan · Nothing visual changes: refusals before SpecSync runs, a wider doctor scan and plain error wording. ^t-072fbe
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:09:16Z · gate-opened · plan → plan · Approve the scope of change-leaks ^t-1d3ce9
  - native_change: change-leaks
  - revision: 01b7952d1fe5f422a610eb9a957aa62a686fd4ae
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:09:16Z · gate-closed · plan → plan · Approve the scope of change-leaks ^t-38d949
  - native_change: change-leaks
  - revision: 01b7952d1fe5f422a610eb9a957aa62a686fd4ae
  - relayed_by:
    - decided_by: 0xLeif
    - words: A: refuse, --redact (Recommended); 1: open changes (Recommended)
    - where: Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 15:08Z, relayed by hc t4c; the questions as shown: 'Chough #412: a change's description, rationale and answers aren't leak-checked today ... When one holds a leak, what should happen?' and 'Chough #412: what should doctor --leaks also scan?' (.specsync/changes/ but not each change's deltas/, and --fix --redact skips those findings); on change-leaks at 01b7952d with choices (A) and (1)
    - recorded_by: chough-fixer-t4n
    - native: specsync change approve change-leaks
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T16:50:32Z · advance · plan → build · native:approved ^t-1ea9cc
  - native_change: change-leaks
  - gate: native:approved
  - revision: f6536ca4bd92c7fd83642f0028b4f954a641254c
  - by: chough-fixer (claude-opus-5-5, hc t4n)
