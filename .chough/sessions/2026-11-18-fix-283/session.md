---
kind: session
id: 2026-11-18-fix-283
aliases: [fix-283]
objective: cases check returns early for a retired case before checking its fledge-task binding, so the original binding stays as history (#283)
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: fix-283
pr: ""
criteria: [LIFE-8]
requirements: [REQ-cli-009]
cases: [TC-cli-a-retired-case-keeps-its-fledge-task-binding]
design: []
files: []
observations:
  - 2026-11-18-tc-cli-a-retired-case-keeps-its-fledge-task-binding
  - 2026-11-18-tc-cli-a-retired-case-keeps-its-fledge-task-binding-37d595
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
---
# cases check returns early for a retired case before checking its fledge-task binding, so the original binding stays as history (#283)

## Goal

cases check returns early for a retired case before checking its fledge-task binding, so the original binding stays as history (#283)

## Where it stands

- Failing test first: a_retired_case_keeps_a_binding_to_a_removed_fledge_task failed on main (cases check refused the retired case's missing fledge task); the retired-case early return now comes before the fledge-task check; all 39 cases tests pass

## Decisions and why

- 0xLeif decided "Scope approval of fix-283": "skip the check for retired cases. cases check returns early for a retired case before checking its fledge-task binding, so the original binding stays as history." (Leif's decision comment on CorvidLabs/chough#283 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval for the 0.6.2 list); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-283 as 0xLeif
- 0xLeif decided "Implementation review of fix-283": "Approve, merge when green" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his implementation review of #346 after the tester's PASS); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change review fix-283 as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test cases -- --exact a_retired_case_keeps_a_binding_to_a_removed_fledge_task for TC-cli-a-retired-case-keeps-its-fledge-task-binding: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-retired-case-keeps-its-fledge-task-binding.md)
- [Ran cargo test --locked --test cases -- --exact a_retired_case_keeps_a_binding_to_a_removed_fledge_task for TC-cli-a-retired-case-keeps-its-fledge-task-binding: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-retired-case-keeps-its-fledge-task-binding-37d595.md)

## Open threads

## Timeline

- 2026-11-18T20:09:01Z · start · intent → intent · cases check returns early for a retired case before checking its fledge-task binding, so the original binding stays as history (#283) ^t-88b533
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:09:01Z · advance · intent → design ·  ^t-468938
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:09:01Z · skip · design → plan · A check order in cases check; nothing visual or interactive changes ^t-1edc74
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:09:19Z · gate-opened · plan → plan · Scope approval of fix-283 ^t-5eb8ff
  - native_change: fix-283
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:09:19Z · gate-closed · plan → plan · Scope approval of fix-283 ^t-36160b
  - native_change: fix-283
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - relayed_by:
    - decided_by: 0xLeif
    - words: skip the check for retired cases. cases check returns early for a retired case before checking its fledge-task binding, so the original binding stays as history.
    - where: Leif's decision comment on CorvidLabs/chough#283 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval for the 0.6.2 list
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-283
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:15:45Z · advance · plan → build · native:approved ^t-e6df0d
  - native_change: fix-283
  - gate: native:approved
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:15:45Z · advance · build → test · native:approved ^t-9bf947
  - native_change: fix-283
  - gate: native:approved
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:15:51Z · advance · test → accept · native:verifying ^t-a586cc
  - native_change: fix-283
  - gate: native:verifying
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:15:51Z · gate-opened · accept → accept · Implementation review of fix-283 ^t-e6aced
  - native_change: fix-283
  - revision: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:19:25Z · gate-closed · accept → accept · Implementation review of fix-283 ^t-140ce2
  - native_change: fix-283
  - revision: e5dc357b7ee73902b8eebd56f6ec095c1cc72139
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve, merge when green
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his implementation review of #346 after the tester's PASS
    - recorded_by: chough-fixer-t4g
    - native: specsync change review fix-283
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:19:26Z · advance · accept → done · native:archived ^t-1ddbe9
  - native_change: fix-283
  - gate: native:archived
  - revision: e5dc357b7ee73902b8eebd56f6ec095c1cc72139
  - by: chough-fixer (claude-opus-5-5, hc t4n)
