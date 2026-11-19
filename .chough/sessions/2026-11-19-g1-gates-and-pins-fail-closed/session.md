---
kind: session
id: 2026-11-19-g1-gates-and-pins-fail-closed
aliases: [g1-gates-and-pins-fail-closed]
objective: "Gates and session pins fail closed: a hand-edited no_native_change cannot unlock the ungated move, change ship checks its whole history and writes only to its pinned session, inbox list and session shelve honor the pin and aliases, and session use says when CHOUGH_SESSION still wins (#401, #400, #369, #393, #372)"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: fix-g1-gates-and-pins-fail-closed
pr: ""
criteria: [DRIVE-3, DRIVE-24, DRIVE-25]
requirements: [REQ-cli-007, REQ-cli-037, REQ-cli-057, REQ-cli-059]
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Scope approval of fix-g1-gates-and-pins-fail-closed with the narrowed #400 sentence (native approval)\" closed (approved); run `chough stage advance` to move on."
---
# Gates and session pins fail closed: a hand-edited no_native_change cannot unlock the ungated move, change ship checks its whole history and writes only to its pinned session, inbox list and session shelve honor the pin and aliases, and session use says when CHOUGH_SESSION still wins (#401, #400, #369, #393, #372)

## Goal

Gates and session pins fail closed: a hand-edited no_native_change cannot unlock the ungated move, change ship checks its whole history and writes only to its pinned session, inbox list and session shelve honor the pin and aliases, and session use says when CHOUGH_SESSION still wins (#401, #400, #369, #393, #372)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-g1-gates-and-pins-fail-closed": "Approve, both as recommended / Approve / Yes, covered" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53). Three answers in order: 'Approve, both as recommended' (Claude Code chat answer, 2026-11-19 about 05:21Z, on G1 at 6a7c7d56 with #372 say-so and #400 whole-range); 'Approve' (about 06:12Z, on declaring history, dropping src/session_md.rs, adding the history spec files and moving the inbox criterion); 'Yes, covered' (2026-11-19 morning MST, on the two case files in place of .chough/cases/ and the re-answered constraints). Scope as checked by fable-lead at e219ab98); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-g1-gates-and-pins-fail-closed as 0xLeif
- 0xLeif decided "Leif's word on the #400 sentence and the G1 send-back": "yes to both" (Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 14:47Z on 2026-11-19 (morning MST): yes to the replacement #400 sentence and criterion in REQ-cli-059, and yes to t4b running G1's one send-back on his word, relayed via t4c); recorded by fable-t4b; a relayed claim, not authenticated
- 0xLeif decided "Scope approval of fix-g1-gates-and-pins-fail-closed with the narrowed #400 sentence (native approval)": "yes to both" (Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 14:47Z on 2026-11-19: yes to the replacement #400 sentence and criterion in REQ-cli-059 as sent (g1-400-wording), and to t4b running this one send-back. The send-back cancelled his three earlier answers on G1, which stand for the rest of the delta: 'Approve, both as recommended' (about 05:21Z), 'Approve' (about 06:12Z), 'Yes, covered' (morning MST)); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-g1-gates-and-pins-fail-closed as 0xLeif

## What was verified

## Open threads

## Timeline

- 2026-11-19T05:13:40Z · start · intent → intent · Gates and session pins fail closed: a hand-edited no_native_change cannot unlock the ungated move, change ship checks its whole history and writes only to its pinned session, inbox list and session shelve honor the pin and aliases, and session use says when CHOUGH_SESSION still wins (#401, #400, #369, #393, #372) ^t-d0790c
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T05:13:50Z · advance · intent → design ·  ^t-b558e1
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T05:13:50Z · skip · design → plan · Bug fixes to validation, pins and refusals; nothing visual or interactive changes ^t-30ad58
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T05:21:53Z · gate-opened · plan → plan · Scope approval of fix-g1-gates-and-pins-fail-closed ^t-feef99
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 6a7c7d569106d0d6f3045470f58790650de1cbfc
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T14:05:52Z · direct-native-write · plan → plan · fix-g1-gates-and-pins-fail-closed's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-66fdb4
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: e219ab986110afa03610666eabf7cbaed212e503
  - direct_write:
    - change: fix-g1-gates-and-pins-fail-closed
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:05:52Z · gate-closed · plan → plan · Scope approval of fix-g1-gates-and-pins-fail-closed ^t-a68821
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: e219ab986110afa03610666eabf7cbaed212e503
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve, both as recommended / Approve / Yes, covered
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53). Three answers in order: 'Approve, both as recommended' (Claude Code chat answer, 2026-11-19 about 05:21Z, on G1 at 6a7c7d56 with #372 say-so and #400 whole-range); 'Approve' (about 06:12Z, on declaring history, dropping src/session_md.rs, adding the history spec files and moving the inbox criterion); 'Yes, covered' (2026-11-19 morning MST, on the two case files in place of .chough/cases/ and the re-answered constraints). Scope as checked by fable-lead at e219ab98
    - recorded_by: fable-t4b
    - native: specsync change approve fix-g1-gates-and-pins-fail-closed
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:18:34Z · advance · plan → build · native:approved ^t-883f9b
  - native_change: fix-g1-gates-and-pins-fail-closed
  - gate: native:approved
  - revision: 966758794f24e6ef21befbb20bfe209b0ea668de
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:47:47Z · gate-opened · build → build · Leif's word on the #400 sentence and the G1 send-back ^t-ca7a35
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 531e2329aee336d3175b564dc4c0a7065eaa7a1a
  - holds: build
  - drives: none
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:47:47Z · gate-closed · build → build · Leif's word on the #400 sentence and the G1 send-back ^t-3d9620
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 531e2329aee336d3175b564dc4c0a7065eaa7a1a
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes to both
    - where: Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 14:47Z on 2026-11-19 (morning MST): yes to the replacement #400 sentence and criterion in REQ-cli-059, and yes to t4b running G1's one send-back on his word, relayed via t4c
    - recorded_by: fable-t4b
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:48:14Z · gate-opened · build → build · Scope approval of fix-g1-gates-and-pins-fail-closed with the narrowed #400 sentence (native approval) ^t-200a2b
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 531e2329aee336d3175b564dc4c0a7065eaa7a1a
  - holds: build
  - drives: approve
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:48:14Z · direct-native-write · build → build · fix-g1-gates-and-pins-fail-closed's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-dfe690
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 531e2329aee336d3175b564dc4c0a7065eaa7a1a
  - direct_write:
    - change: fix-g1-gates-and-pins-fail-closed
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: fable (claude-opus-5-5, hc t4b)
- 2026-11-19T14:48:14Z · gate-closed · build → build · Scope approval of fix-g1-gates-and-pins-fail-closed with the narrowed #400 sentence (native approval) ^t-b6a3e1
  - native_change: fix-g1-gates-and-pins-fail-closed
  - revision: 531e2329aee336d3175b564dc4c0a7065eaa7a1a
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes to both
    - where: Claude Code chat message relayed by hc t4c through fable-lead (hc t53), about 14:47Z on 2026-11-19: yes to the replacement #400 sentence and criterion in REQ-cli-059 as sent (g1-400-wording), and to t4b running this one send-back. The send-back cancelled his three earlier answers on G1, which stand for the rest of the delta: 'Approve, both as recommended' (about 05:21Z), 'Approve' (about 06:12Z), 'Yes, covered' (morning MST)
    - recorded_by: fable-t4b
    - native: specsync change approve fix-g1-gates-and-pins-fail-closed
    - actor: 0xLeif
  - by: fable (claude-opus-5-5, hc t4b)
