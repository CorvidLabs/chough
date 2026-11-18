---
kind: session
id: 2026-11-18-fix-266
aliases: [fix-266]
objective: "next, status and doctor warn during a git merge or rebase, and writes refuse while .chough paths are unmerged (REQ-cli-069, #266)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-266
pr: ""
criteria: [DRIVE-17]
requirements: [REQ-cli-069]
cases: [TC-cli-git-merge-in-progress-warns-and-refuses-writes]
design: []
files: []
observations:
  - 2026-11-18-tc-cli-git-merge-in-progress-warns-and-refuses-writes
  - 2026-11-18-tc-cli-git-merge-in-progress-warns-and-refuses-writes-882968
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-266
    holds: accept
    drives: review
    opened: 2026-11-18T18:26:42Z
run_consents: []
next: Continue accept
---
# next, status and doctor warn during a git merge or rebase, and writes refuse while .chough paths are unmerged (REQ-cli-069, #266)

## Goal

next, status and doctor warn during a git merge or rebase, and writes refuse while .chough paths are unmerged (REQ-cli-069, #266)

## Where it stands

- Failing tests first: a_merge_conflict_in_chough_warns_and_refuses_writes and a_merge_conflict_outside_chough_warns_without_refusing failed on main (next named no merge); doctor::git_operation reads the merge or rebase and the unmerged paths, next and status warn and give only the merge while .chough/ is unmerged, doctor reports git_operation, and the writer lock and observation_room refuse as refused:unmerged; both pass

## Decisions and why

- The rule is a new REQ-cli-069, with the doctor finding defined there beside REQ-history-012's, since #250 and #320 also change REQ-history-012. Writes are refused at the writer lock and before an evidence entry is written, so a refused writer leaves nothing; the merge drivers take no lock and still run.
- 0xLeif decided "Scope approval of fix-266": "warn, and refuse .chough writes. next, status and doctor report an in-progress git merge or rebase (MERGE_HEAD, REBASE_HEAD, unmerged paths) with 'finish or abort it first'. Write commands refuse only while paths under .chough/ are unmerged." (Leif's decision comment on CorvidLabs/chough#266 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval for this change); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-266 as 0xLeif

## What was verified

- [Ran cargo test --locked --test git_merge -- --exact a_merge_conflict_in_chough_warns_and_refuses_writes for TC-cli-git-merge-in-progress-warns-and-refuses-writes: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-git-merge-in-progress-warns-and-refuses-writes.md)
- [Ran cargo test --locked --test git_merge -- --exact a_merge_conflict_in_chough_warns_and_refuses_writes for TC-cli-git-merge-in-progress-warns-and-refuses-writes: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-git-merge-in-progress-warns-and-refuses-writes-882968.md)

## Open threads

## Timeline

- 2026-11-18T18:16:03Z · start · intent → intent · next, status and doctor warn during a git merge or rebase, and writes refuse while .chough paths are unmerged (REQ-cli-069, #266) ^t-fb310c
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:16:21Z · advance · intent → design ·  ^t-a8cef4
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:16:21Z · skip · design → plan · Git merge-state warnings and a write refusal; nothing visual or interactive changes ^t-e5966b
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:17:14Z · gate-opened · plan → plan · Scope approval of fix-266 ^t-d16790
  - native_change: fix-266
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:17:14Z · gate-closed · plan → plan · Scope approval of fix-266 ^t-393903
  - native_change: fix-266
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - relayed_by:
    - decided_by: 0xLeif
    - words: warn, and refuse .chough writes. next, status and doctor report an in-progress git merge or rebase (MERGE_HEAD, REBASE_HEAD, unmerged paths) with 'finish or abort it first'. Write commands refuse only while paths under .chough/ are unmerged.
    - where: Leif's decision comment on CorvidLabs/chough#266 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval for this change
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-266
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:17:15Z · advance · plan → build · native:approved ^t-76fb0e
  - native_change: fix-266
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:26:37Z · advance · build → test · native:approved ^t-202277
  - native_change: fix-266
  - gate: native:approved
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:26:42Z · advance · test → accept · native:verifying ^t-d58bba
  - native_change: fix-266
  - gate: native:verifying
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T18:26:42Z · gate-opened · accept → accept · Implementation review of fix-266 ^t-ed6795
  - native_change: fix-266
  - revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
