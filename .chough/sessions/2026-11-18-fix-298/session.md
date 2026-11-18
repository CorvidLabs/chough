---
kind: session
id: 2026-11-18-fix-298
aliases: [fix-298]
objective: "chough change ship refuses before review when the change's history holds a change both open and archived in one commit (REQ-cli-059, #298)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-298
pr: ""
criteria: [LIFE-21]
requirements: [REQ-cli-059]
cases: [TC-cli-ship-refuses-a-corrupt-history]
design: []
files: []
observations:
  - 2026-11-18-tc-cli-ship-refuses-a-corrupt-history
  - 2026-11-18-tc-cli-ship-refuses-a-corrupt-history-00abbb
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-298
    holds: accept
    drives: review
    opened: 2026-11-18T18:44:54Z
run_consents: []
next: Continue accept
---
# chough change ship refuses before review when the change's history holds a change both open and archived in one commit (REQ-cli-059, #298)

## Goal

chough change ship refuses before review when the change's history holds a change both open and archived in one commit (REQ-cli-059, #298)

## Where it stands

- Failing test first: ship_refuses_a_history_with_a_change_both_open_and_archived failed on main (the ship ran SpecSync); change ship now checks base..HEAD for a change both open and archived in one commit before anything runs, marked interim until spec-sync#1082, and all 23 change tests pass
- #333 tester BLOCK fixed: the corrupt-history check counts a change only by its state.json on both sides, as SpecSync 6.3.0 does, so a leftover tasks.md beside an archive no longer refuses; the refusal names the commit that brought the duplicate in and how many hold it; ship_runs_with_a_leftover_file_of_an_archived_change and the updated refusal test failed on the previous check and pass

## Decisions and why

- SpecSync reports terminal-evidence validity only after it archives, and its ship --dry-run stops before the review, so Chough checks the change's own history (base..HEAD) for a change both open and archived in one commit, as an interim until SpecSync's preflight (spec-sync#1082).
- 0xLeif decided "Scope approval of fix-298": "Both: Chough check now + SpecSync" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his choice of option (3) for #298, refusing before review when base..HEAD shows a change both open and archived in one commit, with the SpecSync preflight filed as CorvidLabs/spec-sync#1082; it follows his decision comment on #298, 'check before shipping'); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-298 as 0xLeif

## What was verified

- [Ran cargo test --locked --test change -- --exact ship_refuses_a_history_with_a_change_both_open_and_archived for TC-cli-ship-refuses-a-corrupt-history: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-ship-refuses-a-corrupt-history.md)
- [Ran cargo test --locked --test change -- --exact ship_refuses_a_history_with_a_change_both_open_and_archived for TC-cli-ship-refuses-a-corrupt-history: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-ship-refuses-a-corrupt-history-00abbb.md)

## Open threads

## Timeline

- 2026-11-18T18:42:50Z · start · intent → intent · chough change ship refuses before review when the change's history holds a change both open and archived in one commit (REQ-cli-059, #298) ^t-0a8947
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:42:50Z · advance · intent → design ·  ^t-0d3009
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:42:50Z · skip · design → plan · A ship preflight; nothing visual or interactive changes ^t-f371ec
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:43:20Z · gate-opened · plan → plan · Scope approval of fix-298 ^t-7bc698
  - native_change: fix-298
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:43:20Z · gate-closed · plan → plan · Scope approval of fix-298 ^t-9c35ee
  - native_change: fix-298
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - relayed_by:
    - decided_by: 0xLeif
    - words: Both: Chough check now + SpecSync
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his choice of option (3) for #298, refusing before review when base..HEAD shows a change both open and archived in one commit, with the SpecSync preflight filed as CorvidLabs/spec-sync#1082; it follows his decision comment on #298, 'check before shipping'
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-298
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:43:22Z · advance · plan → build · native:approved ^t-445052
  - native_change: fix-298
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:44:49Z · advance · build → test · native:approved ^t-998621
  - native_change: fix-298
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:44:54Z · advance · test → accept · native:verifying ^t-939d29
  - native_change: fix-298
  - gate: native:verifying
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:44:54Z · gate-opened · accept → accept · Implementation review of fix-298 ^t-f4661b
  - native_change: fix-298
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
