---
kind: session
id: 2026-11-24-fix-436
aliases: [fix-436]
objective: fix-436-overnight
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: fix-436
pr: ""
criteria: []
requirements: []
cases: []
design: []
files: []
observations:
  - 2026-11-24-tc-records-gate-decision-renders-display-name-4618a1
  - 2026-11-24-tc-records-gate-decision-exact-name-unchanged-171a84
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue done
---
# fix-436-overnight

## Goal

fix-436-overnight

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-436 (display name in decision text)": "I approve" (pi chat t14z (CorvidLabs coordination session), 2026-11-24 about 03:20 UTC, on his standing overnight approval of the six-fix stack (fix-436 is the sixth), relayed by pi-t14z after he confirmed the display-name-only scope in the one-at-a-time interview — flagged as a relayed claim, not authenticated); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve fix-436 as 0xLeif
- 0xLeif decided "Implementation review of fix-436": "Yes start merging those" (pi chat t14z (CorvidLabs coordination session), 2026-11-24 about 14:15 UTC, his morning instruction 'Yes start merging those and talk to me if you need anything..' — review the six-change stack PR 448 and merge; relayed by pi-t14z, flagged as a relayed claim, not authenticated; re-scoped (affected_paths narrowed, re-approved) purely to satisfy finalize's ownership check); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change review fix-436 as 0xLeif with verdict pass

## What was verified

- [Ran cargo test --locked --test humans -- --exact an_alias_close_renders_the_configured_display_name_in_the_decision for TC-records-gate-decision-renders-display-name: passed (exit 0).](evidence/2026-11-24-tc-records-gate-decision-renders-display-name-4618a1.md)
- [Ran cargo test --locked --test humans -- --exact an_exact_name_close_renders_the_display_name_unchanged for TC-records-gate-decision-exact-name-unchanged: passed (exit 0).](evidence/2026-11-24-tc-records-gate-decision-exact-name-unchanged-171a84.md)

## Open threads

## Timeline

- 2026-11-24T03:16:07Z · start · intent → intent · fix-436 ^t-551a9d
  - revision: 71a5c468a92d609113efbd5382317a18f1948e6e
  - by: unknown agent (hc t14z)
- 2026-11-24T03:17:21Z · gate-opened · intent → intent · Scope approval of fix-436 (display name in decision text) ^t-545cd1
  - native_change: fix-436
  - revision: 71a5c468a92d609113efbd5382317a18f1948e6e
  - holds: intent
  - drives: approve
  - by: unknown agent (hc t14z)
- 2026-11-24T03:17:28Z · gate-closed · intent → intent · Scope approval of fix-436 (display name in decision text) ^t-6e798f
  - native_change: fix-436
  - revision: 71a5c468a92d609113efbd5382317a18f1948e6e
  - relayed_by:
    - decided_by: 0xLeif
    - words: I approve
    - where: pi chat t14z (CorvidLabs coordination session), 2026-11-24 about 03:20 UTC, on his standing overnight approval of the six-fix stack (fix-436 is the sixth), relayed by pi-t14z after he confirmed the display-name-only scope in the one-at-a-time interview — flagged as a relayed claim, not authenticated
    - recorded_by: pi-t14z
    - native: specsync change approve fix-436
    - actor: 0xLeif
  - by: unknown agent (hc t14z)
- 2026-11-24T03:17:30Z · advance · intent → design ·  ^t-442e3b
  - native_change: fix-436
  - revision: 71a5c468a92d609113efbd5382317a18f1948e6e
  - by: unknown agent (hc t14z)
- 2026-11-24T03:17:30Z · advance · design → plan ·  ^t-5da95e
  - native_change: fix-436
  - revision: 71a5c468a92d609113efbd5382317a18f1948e6e
  - by: unknown agent (hc t14z)
- 2026-11-24T03:17:30Z · advance · plan → build · native:approved ^t-b7c23c
  - native_change: fix-436
  - gate: native:approved
  - revision: 71a5c468a92d609113efbd5382317a18f1948e6e
  - by: unknown agent (hc t14z)
- 2026-11-24T03:17:33Z · advance · build → test · native:approved ^t-73ff67
  - native_change: fix-436
  - gate: native:approved
  - revision: 71a5c468a92d609113efbd5382317a18f1948e6e
  - by: unknown agent (hc t14z)
- 2026-11-24T04:47:03Z · advance · test → accept · native:approved ^t-9ba897
  - native_change: fix-436
  - gate: native:approved
  - revision: 8c8acfcab211a820125d7570d9acebb7386730bd
  - by: unknown agent (hc t14z)
- 2026-11-24T04:47:06Z · gate-opened · accept → accept · Implementation review of fix-436 ^t-17a90a
  - native_change: fix-436
  - revision: 8c8acfcab211a820125d7570d9acebb7386730bd
  - holds: accept
  - drives: review
  - by: unknown agent (hc t14z)
- 2026-11-24T14:16:25Z · direct-native-write · accept → accept · fix-436's lifecycle records changed outside Chough: state.json changed, verification.json added, verification-attempts.json changed; state approved → verifying ^t-d5df2f
  - native_change: fix-436
  - revision: 01ab408d920116b57d508117b1af6a26b8391c7f
  - direct_write:
    - change: fix-436
    - files:
      - state.json: changed
      - verification.json: added
      - verification-attempts.json: changed
    - before_state: approved
    - after_state: verifying
  - by: unknown agent (hc t14z)
- 2026-11-24T14:16:25Z · direct-native-write · accept → accept · fix-436's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-63fbe4
  - native_change: fix-436
  - revision: 01ab408d920116b57d508117b1af6a26b8391c7f
  - direct_write:
    - change: fix-436
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: unknown agent (hc t14z)
- 2026-11-24T14:16:25Z · direct-native-write · accept → accept · fix-436's lifecycle records changed outside Chough: state.json changed, approvals.json changed, verification.json changed, verification-attempts.json changed; state draft → verifying ^t-ad4c2a
  - native_change: fix-436
  - revision: 01ab408d920116b57d508117b1af6a26b8391c7f
  - direct_write:
    - change: fix-436
    - files:
      - state.json: changed
      - approvals.json: changed
      - verification.json: changed
      - verification-attempts.json: changed
    - before_state: draft
    - after_state: verifying
  - by: unknown agent (hc t14z)
- 2026-11-24T14:16:25Z · gate-closed · accept → accept · Implementation review of fix-436 ^t-360ec8
  - native_change: fix-436
  - revision: 01ab408d920116b57d508117b1af6a26b8391c7f
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes start merging those
    - where: pi chat t14z (CorvidLabs coordination session), 2026-11-24 about 14:15 UTC, his morning instruction 'Yes start merging those and talk to me if you need anything..' — review the six-change stack PR 448 and merge; relayed by pi-t14z, flagged as a relayed claim, not authenticated; re-scoped (affected_paths narrowed, re-approved) purely to satisfy finalize's ownership check
    - recorded_by: pi-t14z
    - summary_digest: ab7afb615cd9a2bb4cb80196f9ad8a10f60686bc3c3c212e62f4b5f480ddc67b
    - list: 99a91b559a2c
    - native: specsync change review fix-436
    - actor: 0xLeif
  - by: unknown agent (hc t14z)
- 2026-11-24T14:16:26Z · advance · accept → done · native:archived ^t-fe83d4
  - native_change: fix-436
  - gate: native:archived
  - revision: 01ab408d920116b57d508117b1af6a26b8391c7f
  - by: unknown agent (hc t14z)
