---
kind: session
id: 2026-11-18-fix-329
aliases: [fix-329]
objective: "chough bug add while a session is active relates the new bug to that session, so a blocking bug stops it; with no session active it stays unlinked (REQ-cli-012, REQ-cli-054, #329)"
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: fix-329
pr: ""
criteria: [LIFE-5]
requirements: [REQ-cli-012, REQ-cli-054]
cases: [TC-cli-bug-add-joins-the-active-session]
design: []
files: []
observations:
  - 2026-11-18-tc-cli-bug-add-joins-the-active-session-f05d70
  - 2026-11-18-tc-cli-bug-add-joins-the-active-session-3e5107
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
---
# chough bug add while a session is active relates the new bug to that session, so a blocking bug stops it; with no session active it stays unlinked (REQ-cli-012, REQ-cli-054, #329)

## Goal

chough bug add while a session is active relates the new bug to that session, so a blocking bug stops it; with no session active it stays unlinked (REQ-cli-012, REQ-cli-054, #329)

## Where it stands

- Failing tests first: a_blocking_bug_added_in_a_session_stops_it and #304's a_blocking_bug_stops_only_a_session_it_is_related_to failed on main; bug add now joins the active session whenever one is active; the out-of-scope, checkpoint-and-story and records alias tests are updated to the new rule; bugs (36) and records (8) pass

## Decisions and why

- 0xLeif decided "Scope approval of fix-329": "auto-link. chough bug add while a session is active relates the new bug to that session, so a blocking bug stops it. A bug filed with no session active stays unlinked and gets the existing warning. … Recreate on my word" (Leif's decision comment on CorvidLabs/chough#329 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval; then, relayed by hc t4c on 2026-11-18, 'Recreate on my word' to recreate fix-329 with tests/records.rs declared, since its alias test reads the session's bugs list; the first draft was never pushed); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-329 as 0xLeif
- 0xLeif decided "Implementation review of fix-329": "Approve, merge when green" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his implementation review of #343 after the tester's PASS; he accepted that a bug filed during session A that links session B's failure blocks both); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change review fix-329 as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-bug-add-joins-the-active-session-f05d70.md)
- [Ran cargo test --locked --test bugs -- --exact a_blocking_bug_added_in_a_session_stops_it for TC-cli-bug-add-joins-the-active-session: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-bug-add-joins-the-active-session-3e5107.md)

## Open threads

## Timeline

- 2026-11-18T19:53:44Z · start · intent → intent · chough bug add while a session is active relates the new bug to that session, so a blocking bug stops it; with no session active it stays unlinked (REQ-cli-012, REQ-cli-054, #329) ^t-eb60f9
  - revision: 275fe4dae6d7a083751186bee89627af9f8140fe
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:53:44Z · advance · intent → design ·  ^t-45c57c
  - revision: 275fe4dae6d7a083751186bee89627af9f8140fe
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:53:44Z · skip · design → plan · A bug-to-session link; nothing visual or interactive changes ^t-9f097c
  - revision: 275fe4dae6d7a083751186bee89627af9f8140fe
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:54:06Z · gate-opened · plan → plan · Scope approval of fix-329 ^t-c04662
  - native_change: fix-329
  - revision: 275fe4dae6d7a083751186bee89627af9f8140fe
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:54:06Z · gate-closed · plan → plan · Scope approval of fix-329 ^t-d35791
  - native_change: fix-329
  - revision: 275fe4dae6d7a083751186bee89627af9f8140fe
  - relayed_by:
    - decided_by: 0xLeif
    - words: auto-link. chough bug add while a session is active relates the new bug to that session, so a blocking bug stops it. A bug filed with no session active stays unlinked and gets the existing warning. … Recreate on my word
    - where: Leif's decision comment on CorvidLabs/chough#329 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval; then, relayed by hc t4c on 2026-11-18, 'Recreate on my word' to recreate fix-329 with tests/records.rs declared, since its alias test reads the session's bugs list; the first draft was never pushed
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-329
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:54:43Z · advance · plan → build · native:approved ^t-3f3c5e
  - native_change: fix-329
  - gate: native:approved
  - revision: 275fe4dae6d7a083751186bee89627af9f8140fe
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:54:43Z · advance · build → test · native:approved ^t-97f557
  - native_change: fix-329
  - gate: native:approved
  - revision: 275fe4dae6d7a083751186bee89627af9f8140fe
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:54:51Z · advance · test → accept · native:verifying ^t-a9ce3d
  - native_change: fix-329
  - gate: native:verifying
  - revision: 275fe4dae6d7a083751186bee89627af9f8140fe
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:54:51Z · gate-opened · accept → accept · Implementation review of fix-329 ^t-012713
  - native_change: fix-329
  - revision: 275fe4dae6d7a083751186bee89627af9f8140fe
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:04:24Z · direct-native-write · accept → accept · fix-329's lifecycle records changed outside Chough: state.json changed, verification.json changed, verification-attempts.json changed; state stays verifying ^t-4be245
  - native_change: fix-329
  - revision: f1964aa3b3f6297adb11cf8b0ea78029da860a9a
  - direct_write:
    - change: fix-329
    - files:
      - state.json: changed
      - verification.json: changed
      - verification-attempts.json: changed
    - before_state: verifying
    - after_state: verifying
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:04:24Z · gate-closed · accept → accept · Implementation review of fix-329 ^t-ba6f1e
  - native_change: fix-329
  - revision: f1964aa3b3f6297adb11cf8b0ea78029da860a9a
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve, merge when green
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his implementation review of #343 after the tester's PASS; he accepted that a bug filed during session A that links session B's failure blocks both
    - recorded_by: chough-fixer-t4g
    - native: specsync change review fix-329
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T23:04:25Z · advance · accept → done · native:archived ^t-364f10
  - native_change: fix-329
  - gate: native:archived
  - revision: f1964aa3b3f6297adb11cf8b0ea78029da860a9a
  - by: chough-fixer (claude-opus-5-5, hc t4n)
