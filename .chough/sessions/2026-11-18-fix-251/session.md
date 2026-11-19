---
kind: session
id: 2026-11-18-fix-251
aliases: [fix-251]
objective: "When SpecSync's source_dirs or meaningful_paths covers the whole repository, chough init and chough doctor advise narrowing to the code folders and say why, instead of a list SpecSync refuses (REQ-records-014, #251, #279)"
intent_scope: ""
modules: []
stage: done
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
observations:
  - 2026-11-18-tc-records-a-whole-repository-specsync-gets-the-narrowing
  - 2026-11-18-tc-records-a-whole-repository-specsync-gets-the-narrowing-f04562
  - 2026-11-19-tc-records-a-whole-repository-specsync-gets-the-narrowing
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
---
# When SpecSync's source_dirs or meaningful_paths covers the whole repository, chough init and chough doctor advise narrowing to the code folders and say why, instead of a list SpecSync refuses (REQ-records-014, #251, #279)

## Goal

When SpecSync's source_dirs or meaningful_paths covers the whole repository, chough init and chough doctor advise narrowing to the code folders and say why, instead of a list SpecSync refuses (REQ-records-014, #251, #279)

## Where it stands

- Failing test first: a_whole_repository_specsync_gets_the_narrowing_advice failed on main (init and doctor advised a list SpecSync refuses); exclusions() reads source_dirs and meaningful_paths, and the advice names narrowing first and says why, the skill bundle included; doctor, next, cli and migrate suites pass
- #354 tester BLOCK on the NFTMix shape fixed: with the specified code at the repository root, init and doctor advise adding .chough and the installed skill folder to exclude_dirs and say the ignored list needs the code in a folder or CorvidLabs/spec-sync#1083; root_level_code_gets_the_exclude_advice_instead_of_narrowing failed before and passes; doctor (20), next, cli and migrate pass

## Decisions and why

- 0xLeif decided "Scope approval of fix-251": "Chough advises narrowing. When SpecSync's source_dirs is ["."], chough init and chough doctor advise narrowing it to the real code folders (for example ["src"]) and say why, instead of advice SpecSync refuses as overlapping. This fixes #251 and #279 together, without changing SpecSync's overlap rule. … Yes, both" (Leif's decision comment on CorvidLabs/chough#251 and #279 from his 2026-11-18 planning interview, relayed by his planning session (hc t4c) to chough-fixer (hc fixer) as the scope approval; then 'Yes, both', relayed by hc t4c on 2026-11-18: init and doctor also advise removing '.' from meaningful_paths in .specsync/sdd.json, with the same why); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-251 as 0xLeif
- 0xLeif decided "Re-approval of fix-251 for code at the repository root": "Chough advises narrowing. … Yes, both … Both: better advice + SpecSync" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, with his #251/#279 decision comment and 'Yes, both' as the scope: after the tester's BLOCK on the NFTMix shape, he chose that when the specified code sits at the repository root (no code folder to narrow to), init and doctor advise adding the skill bundle and Chough's folders to exclude_dirs, and say the ignored list needs the code moved into a folder or SpecSync's fix, CorvidLabs/spec-sync#1083; the folder-repo advice stays as it is); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-251 as 0xLeif
- 0xLeif decided "Implementation review of fix-251": "Approve + follow-up" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his implementation review of #354 after the tester's recheck PASS at a059d88 on real NFTMix and arcron; the follow-up is #357 (root code in every files: style, and a folder repo with one root file)); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change review fix-251 as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test doctor -- --exact a_whole_repository_specsync_gets_the_narrowing_advice for TC-records-a-whole-repository-specsync-gets-the-narrowing-advice: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-whole-repository-specsync-gets-the-narrowing.md)
- [Ran cargo test --locked --test doctor -- --exact a_whole_repository_specsync_gets_the_narrowing_advice for TC-records-a-whole-repository-specsync-gets-the-narrowing-advice: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-records-a-whole-repository-specsync-gets-the-narrowing-f04562.md)
- [Ran cargo test --locked --test doctor -- --exact a_whole_repository_specsync_gets_the_narrowing_advice for TC-records-a-whole-repository-specsync-gets-the-narrowing-advice: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-records-a-whole-repository-specsync-gets-the-narrowing.md)

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
- 2026-11-18T21:35:21Z · gate-opened · accept → accept · Re-approval of fix-251 for code at the repository root ^t-bd884c
  - native_change: fix-251
  - revision: 0214525c0b72726f90300fbcc416b11ee869a2ad
  - holds: accept
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-18T21:35:21Z · gate-closed · accept → accept · Re-approval of fix-251 for code at the repository root ^t-cf0ca6
  - native_change: fix-251
  - revision: 0214525c0b72726f90300fbcc416b11ee869a2ad
  - relayed_by:
    - decided_by: 0xLeif
    - words: Chough advises narrowing. … Yes, both … Both: better advice + SpecSync
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, with his #251/#279 decision comment and 'Yes, both' as the scope: after the tester's BLOCK on the NFTMix shape, he chose that when the specified code sits at the repository root (no code folder to narrow to), init and doctor advise adding the skill bundle and Chough's folders to exclude_dirs, and say the ignored list needs the code moved into a folder or SpecSync's fix, CorvidLabs/spec-sync#1083; the folder-repo advice stays as it is
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve fix-251
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T00:32:42Z · gate-closed · accept → accept · Implementation review of fix-251 ^t-055565
  - native_change: fix-251
  - revision: ef41d3bbed04b18dd1819ee95c8b289c354fda33
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve + follow-up
    - where: Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his implementation review of #354 after the tester's recheck PASS at a059d88 on real NFTMix and arcron; the follow-up is #357 (root code in every files: style, and a folder repo with one root file)
    - recorded_by: chough-fixer-t4g
    - native: specsync change review fix-251
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T00:32:43Z · advance · accept → done · native:archived ^t-3c7863
  - native_change: fix-251
  - gate: native:archived
  - revision: ef41d3bbed04b18dd1819ee95c8b289c354fda33
  - by: chough-fixer (claude-opus-5-5, hc t4n)
