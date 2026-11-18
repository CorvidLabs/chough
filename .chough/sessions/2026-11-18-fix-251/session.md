---
kind: session
id: 2026-11-18-fix-251
aliases: [fix-251]
objective: "When SpecSync's source_dirs or meaningful_paths covers the whole repository, chough init and chough doctor advise narrowing to the code folders and say why, instead of a list SpecSync refuses (REQ-records-014, #251, #279)"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-251
pr: ""
criteria: [DRIVE-4]
requirements: [REQ-records-014, REQ-cli-001]
cases: [TC-records-a-whole-repository-specsync-gets-the-narrowing-advice]
design: []
files: []
observations: [2026-11-18-tc-records-a-whole-repository-specsync-gets-the-narrowing]
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-251
    holds: accept
    drives: review
    opened: 2026-11-18T20:54:37Z
run_consents: []
next: Continue accept
---
# When SpecSync's source_dirs or meaningful_paths covers the whole repository, chough init and chough doctor advise narrowing to the code folders and say why, instead of a list SpecSync refuses (REQ-records-014, #251, #279)

## Goal

When SpecSync's source_dirs or meaningful_paths covers the whole repository, chough init and chough doctor advise narrowing to the code folders and say why, instead of a list SpecSync refuses (REQ-records-014, #251, #279)

## Where it stands

- Failing test first: a_whole_repository_specsync_gets_the_narrowing_advice failed on main (init and doctor advised a list SpecSync refuses); exclusions() reads source_dirs and meaningful_paths, and the advice names narrowing first and says why, the skill bundle included; doctor, next, cli and migrate suites pass

## Decisions and why

- 0xLeif decided "Scope approval of fix-251": "Chough advises narrowing. When SpecSync's source_dirs is ["."], chough init and chough doctor advise narrowing it to the real code folders (for example ["src"]) and say why, instead of advice SpecSync refuses as overlapping. This fixes #251 and #279 together, without changing SpecSync's overlap rule. … Yes, both" (Leif's decision comment on CorvidLabs/chough#251 and #279 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval; then 'Yes, both', relayed by hc t4c on 2026-11-18: init and doctor also advise removing '.' from meaningful_paths in .specsync/sdd.json, with the same why); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-251 as 0xLeif

## What was verified

- [Ran cargo test --locked --test doctor -- --exact a_whole_repository_specsync_gets_the_narrowing_advice for TC-records-a-whole-repository-specsync-gets-the-narrowing-advice: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-whole-repository-specsync-gets-the-narrowing.md)

## Open threads

## Timeline

- 2026-11-18T20:28:18Z · start · intent → intent · When SpecSync's source_dirs or meaningful_paths covers the whole repository, chough init and chough doctor advise narrowing to the code folders and say why, instead of a list SpecSync refuses (REQ-records-014, #251, #279) ^t-4b0445
  - revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:28:18Z · advance · intent → design ·  ^t-d9ca13
  - revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:28:18Z · skip · design → plan · Advice text from init and doctor; nothing visual or interactive changes ^t-2df685
  - revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:29:52Z · gate-opened · plan → plan · Scope approval of fix-251 ^t-a148ac
  - native_change: fix-251
  - revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:29:52Z · gate-closed · plan → plan · Scope approval of fix-251 ^t-1df1a3
  - native_change: fix-251
  - revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
  - relayed_by:
    - decided_by: 0xLeif
    - words: Chough advises narrowing. When SpecSync's source_dirs is ["."], chough init and chough doctor advise narrowing it to the real code folders (for example ["src"]) and say why, instead of advice SpecSync refuses as overlapping. This fixes #251 and #279 together, without changing SpecSync's overlap rule. … Yes, both
    - where: Leif's decision comment on CorvidLabs/chough#251 and #279 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval; then 'Yes, both', relayed by hc t4c on 2026-11-18: init and doctor also advise removing '.' from meaningful_paths in .specsync/sdd.json, with the same why
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-251
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:54:27Z · advance · plan → build · native:approved ^t-c9e3ba
  - native_change: fix-251
  - gate: native:approved
  - revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:54:28Z · advance · build → test · native:approved ^t-69b17a
  - native_change: fix-251
  - gate: native:approved
  - revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:54:37Z · advance · test → accept · native:verifying ^t-4a1ac8
  - native_change: fix-251
  - gate: native:verifying
  - revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T20:54:37Z · gate-opened · accept → accept · Implementation review of fix-251 ^t-345d7e
  - native_change: fix-251
  - revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
