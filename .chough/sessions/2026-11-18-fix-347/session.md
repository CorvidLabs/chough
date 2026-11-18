---
kind: session
id: 2026-11-18-fix-347
aliases: [fix-347]
objective: A new bug's Log links its evidence with relative links, and chough doctor reports and fixes a bug link left stale when its session is archived (#347)
intent_scope: ""
modules: []
stage: test
stage_status: active
iteration: 1
callers: []
native_change: fix-347
pr: ""
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
cases:
  - TC-cli-a-new-bugs-log-links-its-evidence
  - TC-cli-doctor-relinks-a-bugs-evidence-after-archiving
design: []
files: [src/bugs.rs, src/doctor.rs, tests/bugs.rs]
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue test
---
# A new bug's Log links its evidence with relative links, and chough doctor reports and fixes a bug link left stale when its session is archived (#347)

## Goal

A new bug's Log links its evidence with relative links, and chough doctor reports and fixes a bug link left stale when its session is archived (#347)

## Where it stands

- New bugs link their evidence and doctor relinks after archiving; both tests fail on main and pass with the change

## Decisions and why

- 0xLeif decided "Scope approval of fix-347": "both" (GitHub issue #347, which records 'Leif chose both' on 2026-11-18 for its two follow-ups (new bugs get real links; links follow archiving, by repointing or by doctor reporting and fixing); his planning session (hc t4c) relayed it to fable-t4b as his scope approval and asked for the smaller honest option on the second, which is the doctor fix); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-347 as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-18T23:00:35Z · start · intent → intent · A new bug's Log links its evidence with relative links, and chough doctor reports and fixes a bug link left stale when its session is archived (#347) ^t-c54c0d
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:00:35Z · advance · intent → design ·  ^t-e82acb
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:00:35Z · skip · design → plan · Record content change; nothing visual or interactive changes ^t-f0ebf6
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:08:07Z · gate-opened · plan → plan · Scope approval of fix-347 ^t-87ba23
  - native_change: fix-347
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:08:07Z · gate-closed · plan → plan · Scope approval of fix-347 ^t-1d0cc8
  - native_change: fix-347
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - relayed_by:
    - decided_by: 0xLeif
    - words: both
    - where: GitHub issue #347, which records 'Leif chose both' on 2026-11-18 for its two follow-ups (new bugs get real links; links follow archiving, by repointing or by doctor reporting and fixing); his planning session (hc t4c) relayed it to fable-t4b as his scope approval and asked for the smaller honest option on the second, which is the doctor fix
    - recorded_by: fable-t4b
    - native: specsync change approve fix-347
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:08:09Z · advance · plan → build · native:approved ^t-e50f6f
  - native_change: fix-347
  - gate: native:approved
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:08:09Z · advance · build → test · native:approved ^t-e49786
  - native_change: fix-347
  - gate: native:approved
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - by: fable (claude-fable-5-2, hc t4b)
