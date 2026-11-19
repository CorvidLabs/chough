---
kind: session
id: 2026-11-18-fix-361
aliases: [fix-361]
objective: chough doctor and chough next say when installed skill files are known-outdated and name chough skill update, and status JSON carries no_native_change (#361)
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-361
pr: ""
criteria: [LIFE-12]
requirements: [REQ-cli-016]
cases: [TC-cli-doctor-and-next-name-skill-update-for-outdated-files]
design: []
files: [src/skill.rs, src/doctor.rs, src/main.rs, tests/package.rs]
observations:
  - 2026-11-18-tc-cli-doctor-and-next-name-skill-update-for-outdated-files
  - 2026-11-19-tc-cli-doctor-and-next-name-skill-update-for-outdated-files
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-361
    holds: accept
    drives: review
    opened: 2026-11-18T23:44:05Z
run_consents: []
next: Wait for the tester's check and Leif's implementation review of fix-361
---
# chough doctor and chough next say when installed skill files are known-outdated and name chough skill update, and status JSON carries no_native_change (#361)

## Goal

chough doctor and chough next say when installed skill files are known-outdated and name chough skill update, and status JSON carries no_native_change (#361)

## Where it stands

- Doctor and next name chough skill update for known-outdated skill files; the test fails on main and passes with the change

## Decisions and why

- 0xLeif decided "Scope approval of fix-361": "doctor and next" (GitHub issue #361, which records Leif's decision of 2026-11-18 (chough doctor reports each known-outdated skill file naming chough skill update, chough next shows one line, and status JSON carries no_native_change); his planning session (hc t4c) relayed it to fable-t4b as his scope approval); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-361 as 0xLeif

## What was verified

- [Ran cargo test --locked --test package -- --exact doctor_and_next_say_when_skill_files_are_known_outdated for TC-cli-doctor-and-next-name-skill-update-for-outdated-files: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-doctor-and-next-name-skill-update-for-outdated-files.md)
- [Ran cargo test --locked --test package -- --exact doctor_and_next_say_when_skill_files_are_known_outdated for TC-cli-doctor-and-next-name-skill-update-for-outdated-files: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-cli-doctor-and-next-name-skill-update-for-outdated-files.md)

## Open threads

## Timeline

- 2026-11-18T23:36:02Z · start · intent → intent · chough doctor and chough next say when installed skill files are known-outdated and name chough skill update, and status JSON carries no_native_change (#361) ^t-c1adcc
  - revision: 66b82f22a2eb775ece9ab823b4f2698137474a68
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:36:02Z · advance · intent → design ·  ^t-f25b2f
  - revision: 66b82f22a2eb775ece9ab823b4f2698137474a68
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:36:02Z · skip · design → plan · Guidance text change; nothing visual or interactive changes ^t-8aa741
  - revision: 66b82f22a2eb775ece9ab823b4f2698137474a68
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:40:10Z · gate-opened · plan → plan · Scope approval of fix-361 ^t-23226c
  - native_change: fix-361
  - revision: 66b82f22a2eb775ece9ab823b4f2698137474a68
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:40:10Z · gate-closed · plan → plan · Scope approval of fix-361 ^t-42d1dc
  - native_change: fix-361
  - revision: 66b82f22a2eb775ece9ab823b4f2698137474a68
  - relayed_by:
    - decided_by: 0xLeif
    - words: doctor and next
    - where: GitHub issue #361, which records Leif's decision of 2026-11-18 (chough doctor reports each known-outdated skill file naming chough skill update, chough next shows one line, and status JSON carries no_native_change); his planning session (hc t4c) relayed it to fable-t4b as his scope approval
    - recorded_by: fable-t4b
    - native: specsync change approve fix-361
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:40:11Z · advance · plan → build · native:approved ^t-26e607
  - native_change: fix-361
  - gate: native:approved
  - revision: 66b82f22a2eb775ece9ab823b4f2698137474a68
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:40:11Z · advance · build → test · native:approved ^t-e358ba
  - native_change: fix-361
  - gate: native:approved
  - revision: 66b82f22a2eb775ece9ab823b4f2698137474a68
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:44:05Z · advance · test → accept · native:verifying ^t-7d694b
  - native_change: fix-361
  - gate: native:verifying
  - revision: 298d00547f969f18b7aa4b0075d39f6b3f97a637
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:44:05Z · gate-opened · accept → accept · Implementation review of fix-361 ^t-0df171
  - native_change: fix-361
  - revision: 298d00547f969f18b7aa4b0075d39f6b3f97a637
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
