---
kind: session
id: 2026-11-18-fix-294
aliases: [fix-294]
objective: "A test asserts the gate summary's Change line separator, with no em dash (REQ-cli-061, #294)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-294
pr: ""
criteria: [LIFE-21]
requirements: [REQ-cli-061]
cases: [TC-cli-gate-summary-names-its-change-with-a-plain-dash]
design: []
files: []
observations:
  - 2026-11-18-tc-cli-gate-summary-names-its-change-with-a-plain-dash
  - 2026-11-18-tc-cli-gate-summary-names-its-change-with-a-plain-dash-fb30c0
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-294
    holds: accept
    drives: review
    opened: 2026-11-18T16:11:04Z
run_consents: []
next: Continue accept
---
# A test asserts the gate summary's Change line separator, with no em dash (REQ-cli-061, #294)

## Goal

A test asserts the gate summary's Change line separator, with no em dash (REQ-cli-061, #294)

## Where it stands

- Added a_gate_summary_names_its_change_with_a_plain_dash: it passes on main and fails with the em dash put back into src/gates.rs, a local edit that was reverted and never committed

## Decisions and why

- Test-only: REQ-cli-061 already names the plain-dash Change line and src/gates.rs already writes it, so no requirement or product code changes. The test's guard is shown by failing against a local, uncommitted em-dash edit of src/gates.rs.
- 0xLeif decided "Scope approval of fix-294": "Approve all three" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g), 2026-11-18, in answer to: Approve the scope for all three A-list fixes: #268 (checkpoint --file/--design skip the write-time leak guard), #274 (feature-bug-loop example: no pipefail), #294 (test-only: the gate summary Change line)?); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-294 as 0xLeif

## What was verified

- [Ran cargo test --locked --test gates -- --exact a_gate_summary_names_its_change_with_a_plain_dash for TC-cli-gate-summary-names-its-change-with-a-plain-dash: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-gate-summary-names-its-change-with-a-plain-dash.md)
- [Ran cargo test --locked --test gates -- --exact a_gate_summary_names_its_change_with_a_plain_dash for TC-cli-gate-summary-names-its-change-with-a-plain-dash: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-gate-summary-names-its-change-with-a-plain-dash-fb30c0.md)

## Open threads

## Timeline

- 2026-11-18T16:09:35Z · start · intent → intent · A test asserts the gate summary's Change line separator, with no em dash (REQ-cli-061, #294) ^t-cdb91a
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:09:45Z · advance · intent → design ·  ^t-52966b
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:09:45Z · skip · design → plan · A test-only change; nothing visual or interactive changes ^t-0d19f1
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:10:04Z · gate-opened · plan → plan · Scope approval of fix-294 ^t-4bf1db
  - native_change: fix-294
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:10:04Z · gate-closed · plan → plan · Scope approval of fix-294 ^t-5b74c5
  - native_change: fix-294
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve all three
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g), 2026-11-18, in answer to: Approve the scope for all three A-list fixes: #268 (checkpoint --file/--design skip the write-time leak guard), #274 (feature-bug-loop example: no pipefail), #294 (test-only: the gate summary Change line)?
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-294
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:11:00Z · advance · plan → build · native:approved ^t-2754d0
  - native_change: fix-294
  - gate: native:approved
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:11:00Z · advance · build → test · native:approved ^t-38669b
  - native_change: fix-294
  - gate: native:approved
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:11:04Z · advance · test → accept · native:approved ^t-98d2c7
  - native_change: fix-294
  - gate: native:approved
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:11:04Z · gate-opened · accept → accept · Implementation review of fix-294 ^t-0a6656
  - native_change: fix-294
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4g)
