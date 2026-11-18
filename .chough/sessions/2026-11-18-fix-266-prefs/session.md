---
kind: session
id: 2026-11-18-fix-266-prefs
aliases: [fix-266-prefs]
objective: pref set and pref unset take the writer lock, so they refuse while a .chough path is unmerged, as REQ-cli-069 promises (#330 review P2)
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-266-prefs
pr: ""
criteria: [DRIVE-17]
requirements: [REQ-cli-069]
cases: []
design: []
files: []
observations:
  - 2026-11-18-tc-cli-git-merge-in-progress-warns-and-refuses-writes-227cca
  - 2026-11-18-cargo-test-locked-test-git-merge-preference-writes-refuse
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-266-prefs
    holds: accept
    drives: review
    opened: 2026-11-18T19:36:28Z
run_consents: []
next: Continue accept
---
# pref set and pref unset take the writer lock, so they refuse while a .chough path is unmerged, as REQ-cli-069 promises (#330 review P2)

## Goal

pref set and pref unset take the writer lock, so they refuse while a .chough path is unmerged, as REQ-cli-069 promises (#330 review P2)

## Where it stands

- Failing test first: preference_writes_refuse_while_chough_is_unmerged failed on fix-266 (pref set ran mid-merge and changed .chough/preferences.json); pref set and pref unset now take the writer lock before reading, so they refuse as refused:unmerged with every file unchanged while pref show still runs; all 5 git_merge tests pass

## Decisions and why

- 0xLeif decided "Scope approval of fix-266-prefs": "Fix first, you resolve" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his answer to his own P2 review thread on #330 ('Cover preference writers before relying on this lock as the merge guard … regress pref set/unset'): a small follow-up native change on #330's branch in which pref set and pref unset take the writer lock, with a regression test red first); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-266-prefs as 0xLeif

## What was verified

- [Ran cargo test --locked --test git_merge -- --exact a_merge_conflict_in_chough_warns_and_refuses_writes for TC-cli-git-merge-in-progress-warns-and-refuses-writes: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-git-merge-in-progress-warns-and-refuses-writes-227cca.md)
- [cargo test --locked --test git_merge -- preference_writes_refuse_while_chough_is_unmerged: 1 passed. With .chough/README.md unmerged in a stopped git merge, pref set and pref unset refuse as refused:unmerged naming it with every file unchanged, pref show runs, and after git merge --abort pref unset writes (#330 review P2).](evidence/2026-11-18-cargo-test-locked-test-git-merge-preference-writes-refuse.md)

## Open threads

## Timeline

- 2026-11-18T19:34:31Z · start · intent → intent · pref set and pref unset take the writer lock, so they refuse while a .chough path is unmerged, as REQ-cli-069 promises (#330 review P2) ^t-1c5adf
  - revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:34:32Z · advance · intent → design ·  ^t-7d4efc
  - revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:34:32Z · skip · design → plan · A lock on two preference writes; nothing visual or interactive changes ^t-6af0f3
  - revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:34:52Z · gate-opened · plan → plan · Scope approval of fix-266-prefs ^t-cd6569
  - native_change: fix-266-prefs
  - revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:34:52Z · gate-closed · plan → plan · Scope approval of fix-266-prefs ^t-5b2f24
  - native_change: fix-266-prefs
  - revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
  - relayed_by:
    - decided_by: 0xLeif
    - words: Fix first, you resolve
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his answer to his own P2 review thread on #330 ('Cover preference writers before relying on this lock as the merge guard … regress pref set/unset'): a small follow-up native change on #330's branch in which pref set and pref unset take the writer lock, with a regression test red first
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-266-prefs
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:34:53Z · advance · plan → build · native:approved ^t-0791c5
  - native_change: fix-266-prefs
  - gate: native:approved
  - revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:36:08Z · advance · build → test · native:approved ^t-914be4
  - native_change: fix-266-prefs
  - gate: native:approved
  - revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:36:28Z · advance · test → accept · native:verifying ^t-8b6e44
  - native_change: fix-266-prefs
  - gate: native:verifying
  - revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T19:36:28Z · gate-opened · accept → accept · Implementation review of fix-266-prefs ^t-9e46c9
  - native_change: fix-266-prefs
  - revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
