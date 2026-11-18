---
kind: session
id: 2026-11-18-fix-316
aliases: [fix-316]
objective: "A slug with -sk- is not a model-provider key again, while the distinctive token forms stay refused after a hyphen (REQ-records-006, #316)"
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: fix-316
pr: ""
criteria: [LIFE-26]
requirements: [REQ-records-006]
cases: [TC-records-sk-after-a-hyphen-is-not-a-key]
design: []
files: []
observations: [2026-11-18-tc-records-sk-after-a-hyphen-is-not-a-key]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
---
# A slug with -sk- is not a model-provider key again, while the distinctive token forms stay refused after a hyphen (REQ-records-006, #316)

## Goal

A slug with -sk- is not a model-provider key again, while the distinctive token forms stay refused after a hyphen (REQ-records-006, #316)

## Where it stands

- Failing tests first: leaks::tests::sk_after_a_hyphen_continues_a_word and a_slug_holding_sk_is_not_a_key_and_a_glued_token_still_is failed on main (the sk-learn bug ID was refused as a token); sk- now keeps 0.6.0's boundary and both pass, with #267's test still green

## Decisions and why

- Leif chose to narrow #303's boundary rather than revert it: sk- goes back to 0.6.0's rule, where a hyphen before the prefix continues a word, and the distinctive forms (GitHub, glpat-, xox*-, AKIA/ASIA) keep the hyphen boundary. No requirement text changes; REQ-records-006 does not fix the byte before a prefix.
- 0xLeif decided "Scope approval of fix-316": "Narrow it: not for sk-" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g), 2026-11-18, his answer on how to handle the #316 regression, with t4c's instruction to record it as his scope approval for this change); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-316 as 0xLeif
- 0xLeif decided "Implementation review of fix-316": "Approve, merge when green" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g), 2026-11-18, his implementation review of #318 after the tester's PASS and t4c's read of the leaks.rs diff for him); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change review fix-316 as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test leaks -- --exact a_slug_holding_sk_is_not_a_key_and_a_glued_token_still_is for TC-records-sk-after-a-hyphen-is-not-a-key: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-sk-after-a-hyphen-is-not-a-key.md)

## Open threads

## Timeline

- 2026-11-18T15:45:16Z · start · intent → intent · A slug with -sk- is not a model-provider key again, while the distinctive token forms stay refused after a hyphen (REQ-records-006, #316) ^t-80f368
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:45:26Z · advance · intent → design ·  ^t-a4853b
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:45:26Z · skip · design → plan · Leak-matcher boundary for one token form; nothing visual or interactive changes ^t-911846
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:45:52Z · gate-opened · plan → plan · Scope approval of fix-316 ^t-d170b7
  - native_change: fix-316
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:45:52Z · gate-closed · plan → plan · Scope approval of fix-316 ^t-ac6e6a
  - native_change: fix-316
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - relayed_by:
    - decided_by: 0xLeif
    - words: Narrow it: not for sk-
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g), 2026-11-18, his answer on how to handle the #316 regression, with t4c's instruction to record it as his scope approval for this change
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-316
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:45:59Z · advance · plan → build · native:approved ^t-9265dd
  - native_change: fix-316
  - gate: native:approved
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:47:41Z · advance · build → test · native:approved ^t-f1a59c
  - native_change: fix-316
  - gate: native:approved
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:47:43Z · advance · test → accept · native:approved ^t-f5b9a6
  - native_change: fix-316
  - gate: native:approved
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T15:47:43Z · gate-opened · accept → accept · Implementation review of fix-316 ^t-1c4a3a
  - native_change: fix-316
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:00:59Z · gate-closed · accept → accept · Implementation review of fix-316 ^t-adbf61
  - native_change: fix-316
  - revision: c91031762b3cf5dcf436d161c68a08e16a6b4bbc
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve, merge when green
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc t4g), 2026-11-18, his implementation review of #318 after the tester's PASS and t4c's read of the leaks.rs diff for him
    - recorded_by: chough-fixer-t4g
    - native: specsync change review fix-316
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4g)
- 2026-11-18T16:01:02Z · advance · accept → done · native:archived ^t-45e47b
  - native_change: fix-316
  - gate: native:archived
  - revision: c91031762b3cf5dcf436d161c68a08e16a6b4bbc
  - by: chough-fixer (claude-opus-5-5, hc t4g)
