---
kind: session
id: 2026-11-20-closed-pipe-and-help
aliases: [closed-pipe-and-help]
objective: "A closed standard output ends a command quietly with its own exit code, and help works after global options, with -V documented as the top-level version flag (#383's closed pipe, #399)"
intent_scope: ""
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: closed-pipe-and-help
pr: "425"
criteria: [LIFE-4]
requirements: [REQ-cli-021, REQ-cli-019]
cases: []
design: []
files: []
observations:
  - 2026-11-20-tc-cli-a-closed-pipe-ends-a-command-with-its-own-code
  - 2026-11-20-tc-cli-a-write-into-a-closed-pipe-saves-and-exits-0
  - 2026-11-20-tc-cli-a-refusal-with-both-streams-closed-exits-1
  - 2026-11-20-tc-cli-help-after-the-global-options
  - 2026-11-20-tc-cli-a-closed-pipe-ends-a-command-with-its-own-code-ea01b6
  - 2026-11-20-tc-cli-a-write-into-a-closed-pipe-saves-and-exits-0-0e91de
  - 2026-11-20-tc-cli-a-refusal-with-both-streams-closed-exits-1-b38e60
  - 2026-11-20-tc-cli-help-after-the-global-options-7ca4da
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
---
# A closed standard output ends a command quietly with its own exit code, and help works after global options, with -V documented as the top-level version flag (#383's closed pipe, #399)

## Goal

A closed standard output ends a command quietly with its own exit code, and help works after global options, with -V documented as the top-level version flag (#383's closed pipe, #399)

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of closed-pipe-and-help": "Standing OK, tell me after" (Leif's standing OK for scope approvals, from his Claude Code question dialog in his planning session (hc t4c), 2026-11-20 ('Standing OK, tell me after'), relayed via t4c, which confirmed on 2026-11-20 that E1 closed-pipe-and-help at 8e562c06 has only the recommended choice, changes no requirement wording and adds no refusal; recorded after one interview correction SpecSync required for a no-spec change (public_contract yes to no), which changes none of those three); recorded by chough-fixer-t4n; a relayed claim, not authenticated; ran specsync change approve closed-pipe-and-help as 0xLeif
- 0xLeif decided "Implementation review of closed-pipe-and-help": "Go" (Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #425 at de0f1dc7: the tester's harness, run by the fixer at de0f1dc7, 26 of 26 (the four E1 rows fixed against main, among them a 22-command closed-pipe sweep with no exit 101 or panic, and the 22 real repositories unchanged); corvid-agent APPROVED de0f1dc7 (2026-11-20, after its nit was fixed in e60f1978). A no-spec change, so no approved sentence exists to change; its scope carries Leif's standing OK for scope approvals, relayed via t4c. fable-lead is paused until about 2026-11-23, so t4c is told after.); recorded by chough-fixer-t4n; a relayed claim, not authenticated; ran specsync change review closed-pipe-and-help as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test cli -- --exact a_command_whose_reader_has_gone_exits_with_its_own_code for TC-cli-a-closed-pipe-ends-a-command-with-its-own-code: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-closed-pipe-ends-a-command-with-its-own-code.md)
- [Ran cargo test --locked --test cli -- --exact a_write_into_a_closed_pipe_saves_and_exits_0 for TC-cli-a-write-into-a-closed-pipe-saves-and-exits-0: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-write-into-a-closed-pipe-saves-and-exits-0.md)
- [Ran cargo test --locked --test cli -- --exact a_refusal_with_both_streams_closed_exits_as_it_would for TC-cli-a-refusal-with-both-streams-closed-exits-1: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-refusal-with-both-streams-closed-exits-1.md)
- [Ran cargo test --locked --test cli -- --exact help_after_the_global_options_prints_the_top_level_help for TC-cli-help-after-the-global-options: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-help-after-the-global-options.md)
- [Ran cargo test --locked --test cli -- --exact a_command_whose_reader_has_gone_exits_with_its_own_code for TC-cli-a-closed-pipe-ends-a-command-with-its-own-code: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-closed-pipe-ends-a-command-with-its-own-code-ea01b6.md)
- [Ran cargo test --locked --test cli -- --exact a_write_into_a_closed_pipe_saves_and_exits_0 for TC-cli-a-write-into-a-closed-pipe-saves-and-exits-0: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-write-into-a-closed-pipe-saves-and-exits-0-0e91de.md)
- [Ran cargo test --locked --test cli -- --exact a_refusal_with_both_streams_closed_exits_as_it_would for TC-cli-a-refusal-with-both-streams-closed-exits-1: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-refusal-with-both-streams-closed-exits-1-b38e60.md)
- [Ran cargo test --locked --test cli -- --exact help_after_the_global_options_prints_the_top_level_help for TC-cli-help-after-the-global-options: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-help-after-the-global-options-7ca4da.md)

## Open threads

## Timeline

- 2026-11-20T15:10:25Z · start · intent → intent · A closed standard output ends a command quietly with its own exit code, and help works after global options, with -V documented as the top-level version flag (#383's closed pipe, #399) ^t-a1be46
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:10:25Z · advance · intent → design ·  ^t-31a106
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:10:25Z · skip · design → plan · Nothing visual changes: quiet exits on a closed pipe, help parsing and a docs sentence. ^t-386d1c
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:11:41Z · gate-opened · plan → plan · Approve the scope of closed-pipe-and-help ^t-1b9ce3
  - native_change: closed-pipe-and-help
  - revision: 8e562c06eb6a5acfb2d3759e47991f67973b7ab4
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T15:12:13Z · gate-closed · plan → plan · Approve the scope of closed-pipe-and-help ^t-763f84
  - native_change: closed-pipe-and-help
  - revision: 8e562c06eb6a5acfb2d3759e47991f67973b7ab4
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Leif's standing OK for scope approvals, from his Claude Code question dialog in his planning session (hc t4c), 2026-11-20 ('Standing OK, tell me after'), relayed via t4c, which confirmed on 2026-11-20 that E1 closed-pipe-and-help at 8e562c06 has only the recommended choice, changes no requirement wording and adds no refusal; recorded after one interview correction SpecSync required for a no-spec change (public_contract yes to no), which changes none of those three
    - recorded_by: chough-fixer-t4n
    - native: specsync change approve closed-pipe-and-help
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T16:30:10Z · advance · plan → build · native:approved ^t-11dd30
  - native_change: closed-pipe-and-help
  - gate: native:approved
  - revision: 49217bbc1569c1c0e0f080ab7995cd42fd96492b
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T17:33:15Z · advance · build → test · native:verifying ^t-4da8d8
  - native_change: closed-pipe-and-help
  - gate: native:verifying
  - revision: de0f1dc7ec89772141d8f938e73cb490dce2ffef
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T17:33:16Z · advance · test → accept · native:verifying ^t-b23f7a
  - native_change: closed-pipe-and-help
  - gate: native:verifying
  - revision: de0f1dc7ec89772141d8f938e73cb490dce2ffef
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T17:33:32Z · gate-opened · accept → accept · Implementation review of closed-pipe-and-help ^t-f506b6
  - native_change: closed-pipe-and-help
  - revision: de0f1dc7ec89772141d8f938e73cb490dce2ffef
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T17:34:16Z · gate-closed · accept → accept · Implementation review of closed-pipe-and-help ^t-8c84ec
  - native_change: closed-pipe-and-help
  - revision: 99e618bb2b3d29adc7ad0514cb680b255e447e6b
  - relayed_by:
    - decided_by: 0xLeif
    - words: Go
    - where: Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #425 at de0f1dc7: the tester's harness, run by the fixer at de0f1dc7, 26 of 26 (the four E1 rows fixed against main, among them a 22-command closed-pipe sweep with no exit 101 or panic, and the 22 real repositories unchanged); corvid-agent APPROVED de0f1dc7 (2026-11-20, after its nit was fixed in e60f1978). A no-spec change, so no approved sentence exists to change; its scope carries Leif's standing OK for scope approvals, relayed via t4c. fable-lead is paused until about 2026-11-23, so t4c is told after.
    - recorded_by: chough-fixer-t4n
    - native: specsync change review closed-pipe-and-help
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T17:34:18Z · advance · accept → done · native:archived ^t-cb4128
  - native_change: closed-pipe-and-help
  - gate: native:archived
  - revision: 99e618bb2b3d29adc7ad0514cb680b255e447e6b
  - by: chough-fixer (claude-opus-5-5, hc t4n)
