---
kind: session
id: 2026-11-06-name-bug-merge-in-the-cli-reference-s-command-table-with-a
aliases:
  - name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
objective: Name bug merge in the CLI reference's command table, with a check that the table matches --help (#173, FLOW-18)
intent_scope: FLOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
pr: ""
criteria: [FLOW-18]
requirements: [REQ-cli-019]
cases: [TC-cli-cli-command-table-matches-help]
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "After Leif's review (recorded by orc): ship, close the review gate with his relayed words, stage advance, commit, audit --strict and pre-push, push, CI, merge pinned"
closed: 2026-11-06
outcome: done
receipts: []
---
# Name bug merge in the CLI reference's command table, with a check that the table matches --help (#173, FLOW-18)

## Goal

Name bug merge in the CLI reference's command table, with a check that the table matches --help (#173, FLOW-18)

## Where it stands

- Fix: docs/cli.md's bug row names merge (#173); the Commands table check failed first at 46ce740a and passes; all eleven docs cases and cargo test (533) pass

## Decisions and why

- decided: the #173 check is a separate test in tests/docs.rs because docs/cli.md's command table writes subcommands with escaped pipes and bracketed options, unlike the README's helper table
- Leif (0xLeif) decided "Scope approval of name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18": "approved" (orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy as 'Leif approved #173' (orc's wording; Leif's own words were not quoted); orc recorded the native definition approval as 0xLeif at 46ce740a); recorded by claude (ch-docs-accuracy); a relayed claim, not authenticated
- Leif (0xLeif) decided "Implementation review of name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18": "pass" (orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy; orc recorded the native scoped review as 0xLeif, verdict pass at 77579661 on PR #176); recorded by claude (ch-docs-accuracy); a relayed claim, not authenticated

## What was verified

## Open threads

## Timeline

- 2026-11-06T20:59:16Z · reset · intent → plan · FLOW-18 (approved by Leif, 'FLOW-18, one PR') already covers #173; orc assigned it under FLOW-18 on 2026-11-06 ('#173 under FLOW-18 (already captured)') ^t-794fa0
  - native_change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
  - revision: 69caa0e721d0a150652fd50b7104fcb6f4e444a7
  - refresh: plan: native definition; scope or contract changes need native reapproval
- 2026-11-06T20:59:17Z · gate-opened · plan → plan · Scope approval of name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18 ^t-5cc517
  - native_change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
  - revision: 69caa0e721d0a150652fd50b7104fcb6f4e444a7
- 2026-11-06T21:02:43Z · gate-closed · plan → plan · Scope approval of name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18 ^t-539b5c
  - native_change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
  - revision: 46ce740ac98c9448c5dcf6e1e2422bd5081fe28e
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"approved","where":"orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy as 'Leif approved #173' (orc's wording; Leif's own words were not quoted); orc recorded the native definition approval as 0xLeif at 46ce740a","recorded_by":"claude (ch-docs-accuracy)"}
- 2026-11-06T21:02:45Z · advance · plan → build · native:approved ^t-cccdc5
  - native_change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
  - gate: native:approved
  - revision: 46ce740ac98c9448c5dcf6e1e2422bd5081fe28e
- 2026-11-06T21:14:38Z · advance · build → test · native:approved ^t-a279cc
  - native_change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
  - gate: native:approved
  - revision: 50e070e102338bac00f7cbfab4aaf24742a3277d
- 2026-11-06T21:14:39Z · advance · test → accept · native:approved ^t-56cabe
  - native_change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
  - gate: native:approved
  - revision: 50e070e102338bac00f7cbfab4aaf24742a3277d
- 2026-11-06T21:14:55Z · gate-opened · accept → accept · Implementation review of name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18 ^t-803f2f
  - native_change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
  - revision: 50e070e102338bac00f7cbfab4aaf24742a3277d
- 2026-11-06T21:49:17Z · gate-closed · accept → accept · Implementation review of name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18 ^t-aa6d9d
  - native_change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
  - revision: 66dac7f577c3b34eeeef15fcbf93ef0bc8295b45
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"pass","where":"orc tab, 2026-11-06, relayed by orc (hc t18) to ch-docs-accuracy; orc recorded the native scoped review as 0xLeif, verdict pass at 77579661 on PR #176","recorded_by":"claude (ch-docs-accuracy)"}
- 2026-11-06T21:49:42Z · advance · accept → done · native:archived ^t-7f5bc4
  - native_change: name-bug-merge-in-the-cli-reference-s-command-table-with-a-check-that-the-table-matches-help-173-flow-18
  - gate: native:archived
  - revision: 66dac7f577c3b34eeeef15fcbf93ef0bc8295b45


Closed 2026-11-06: done.
