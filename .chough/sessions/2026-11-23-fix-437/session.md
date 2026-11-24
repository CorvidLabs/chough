---
kind: session
id: 2026-11-23-fix-437
aliases: [fix-437]
objective: "Fix chough#437: a direct-write entry writes before_state and after_state as none when the state is absent, in the Timeline and its text"
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: fix-437
pr: ""
criteria: [FLOW-2]
requirements: []
cases: [TC-cli-direct-write-absent-state-is-none]
design: []
files: []
observations:
  - 2026-11-23-tc-records-inbox-add-without-hard-links-8d250e
  - 2026-11-23-tc-cli-direct-write-absent-state-is-none
  - 2026-11-23-tc-cli-direct-write-absent-state-is-none-2a2c0d
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
---
# Fix chough#437: a direct-write entry writes before_state and after_state as none when the state is absent, in the Timeline and its text

## Goal

Fix chough#437: a direct-write entry writes before_state and after_state as none when the state is absent, in the Timeline and its text

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of fix-437": "I approve" (pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve fix-437 as 0xLeif
- 0xLeif decided "Implementation review of fix-437": "Yes start merging those" (pi chat t14z (CorvidLabs coordination session), 2026-11-24 about 14:15 UTC, his morning instruction 'Yes start merging those and talk to me if you need anything..' — review the six-change stack PR 448 and merge; relayed by pi-t14z, flagged as a relayed claim, not authenticated); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change review fix-437 as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test no_hard_links -- --ignored --exact new_records_are_written_on_a_volume_without_hard_links for TC-records-inbox-add-without-hard-links: passed (exit 0).](evidence/2026-11-23-tc-records-inbox-add-without-hard-links-8d250e.md)
- [Ran cargo test --locked --lib -- --exact direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none for TC-cli-direct-write-absent-state-is-none: failed (exit 101).](evidence/2026-11-23-tc-cli-direct-write-absent-state-is-none.md)
- [Ran cargo test --locked --bin chough -- --exact direct_writes::tests::a_direct_write_with_an_absent_state_says_state_stays_none for TC-cli-direct-write-absent-state-is-none: passed (exit 0).](evidence/2026-11-23-tc-cli-direct-write-absent-state-is-none-2a2c0d.md)

## Open threads

## Timeline

- 2026-11-23T14:44:33Z · start · intent → intent · fix-437 ^t-87f09a
  - revision: 35828a1df3d308677a38b138e7aebe0e7b6770ef
  - by: unknown agent (hc t14z)
- 2026-11-23T14:55:50Z · gate-opened · intent → intent · Approve the scope of fix-437 ^t-b5dbe6
  - native_change: fix-437
  - revision: a067aa560974bb904dd07c18e56ded5c038fd77d
  - holds: intent
  - drives: approve
  - by: unknown agent (hc t14z)
- 2026-11-23T14:56:25Z · direct-native-write · intent → intent · fix-437's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-cdc034
  - native_change: fix-437
  - revision: f64a87d77ce3e6075051119797060f9885db93b6
  - direct_write:
    - change: fix-437
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: unknown agent (hc t14z)
- 2026-11-23T14:56:25Z · gate-closed · intent → intent · Approve the scope of fix-437 ^t-f7a84f
  - native_change: fix-437
  - revision: f64a87d77ce3e6075051119797060f9885db93b6
  - relayed_by:
    - decided_by: 0xLeif
    - words: I approve
    - where: pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch
    - recorded_by: pi-t14z
    - summary_digest: c2c7464289420feb18e087b292f366f2a861900ac9fe922206bd001529848a12
    - list: 30451888bc48
    - native: specsync change approve fix-437
    - actor: 0xLeif
  - by: unknown agent (hc t14z)
- 2026-11-23T14:56:27Z · advance · intent → design ·  ^t-9c87e6
  - native_change: fix-437
  - revision: f64a87d77ce3e6075051119797060f9885db93b6
  - by: unknown agent (hc t14z)
- 2026-11-23T14:56:27Z · advance · design → plan ·  ^t-ae5e7b
  - native_change: fix-437
  - revision: f64a87d77ce3e6075051119797060f9885db93b6
  - by: unknown agent (hc t14z)
- 2026-11-23T15:29:18Z · advance · plan → build · native:approved ^t-6c4308
  - native_change: fix-437
  - gate: native:approved
  - revision: 4fbb7766ad13298491229d97ade73bf9515af354
  - by: unknown agent (hc t14z)
- 2026-11-23T15:29:19Z · advance · build → test · native:approved ^t-7ee0f0
  - native_change: fix-437
  - gate: native:approved
  - revision: 4fbb7766ad13298491229d97ade73bf9515af354
  - by: unknown agent (hc t14z)
- 2026-11-23T15:29:19Z · advance · test → accept · native:approved ^t-92eff5
  - native_change: fix-437
  - gate: native:approved
  - revision: 4fbb7766ad13298491229d97ade73bf9515af354
  - by: unknown agent (hc t14z)
- 2026-11-23T15:29:24Z · gate-opened · accept → accept · Implementation review of fix-437 ^t-9afe64
  - native_change: fix-437
  - revision: 4fbb7766ad13298491229d97ade73bf9515af354
  - holds: accept
  - drives: review
  - by: unknown agent (hc t14z)
- 2026-11-24T14:18:37Z · direct-native-write · accept → accept · fix-437's lifecycle records changed outside Chough: state.json changed, verification.json changed, verification-attempts.json changed; state stays verifying ^t-f7fbbe
  - native_change: fix-437
  - revision: a9395948ffd7485faddc5b6839acb99155e10d60
  - direct_write:
    - change: fix-437
    - files:
      - state.json: changed
      - verification.json: changed
      - verification-attempts.json: changed
    - before_state: verifying
    - after_state: verifying
  - by: unknown agent (hc t14z)
- 2026-11-24T14:18:37Z · gate-closed · accept → accept · Implementation review of fix-437 ^t-6b712b
  - native_change: fix-437
  - revision: a9395948ffd7485faddc5b6839acb99155e10d60
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes start merging those
    - where: pi chat t14z (CorvidLabs coordination session), 2026-11-24 about 14:15 UTC, his morning instruction 'Yes start merging those and talk to me if you need anything..' — review the six-change stack PR 448 and merge; relayed by pi-t14z, flagged as a relayed claim, not authenticated
    - recorded_by: pi-t14z
    - summary_digest: 61e6c776cf7782f08bdb758537739e88bd5a7056854434c3972e48bd6fe80384
    - list: 54c96801433f
    - native: specsync change review fix-437
    - actor: 0xLeif
  - by: unknown agent (hc t14z)
- 2026-11-24T14:18:39Z · advance · accept → done · native:archived ^t-dbd7c9
  - native_change: fix-437
  - gate: native:archived
  - revision: a9395948ffd7485faddc5b6839acb99155e10d60
  - by: unknown agent (hc t14z)
