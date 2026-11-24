---
kind: session
id: 2026-11-23-leak-host-equals-user
aliases: [leak-host-equals-user]
objective: When a host label equals the local user name, as this Mac's 'Tob' for user 'tob', the host-name rule must not re-flag the plain folder the user-name rules deliberately let through; REQ-records-006 gains that exception
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
pr: ""
criteria: [LIFE-26]
requirements: [REQ-records-006]
cases: []
design: []
files: []
observations: [2026-11-24-cargo-test-full-suite-1040-passed-0-failed-new-unit-test-a-ho]
bugs: []
blockers: []
waiting_on_human:
  - gate: "Implementation review of leak-host-equals-user: REQ-records-006 exception (host label equal to the local user name does not count as a path component; URL, <user>@<host> and scp contexts still count), implemented in src/leaks.rs with the unit test; both model reviews pass, both checker runs clean after your confirmations, 1040/1040 tests, native check committed"
    holds: accept
    drives: review
    opened: 2026-11-24T02:02:39Z
run_consents: []
next: Continue accept
---
# When a host label equals the local user name, as this Mac's 'Tob' for user 'tob', the host-name rule must not re-flag the plain folder the user-name rules deliberately let through; REQ-records-006 gains that exception

## Goal

When a host label equals the local user name, as this Mac's 'Tob' for user 'tob', the host-name rule must not re-flag the plain folder the user-name rules deliberately let through; REQ-records-006 gains that exception

## Where it stands

## Decisions and why

- Intent review (product, QA, engineering, architecture passes, 2026-11-23): the exception is path-component context only, so a URL to the bare label, <user>@<host> and scp-style hosts still refuse; the dotted and .local names keep their existing rules. QA edge accepted: an absolute path whose first component is that label is also exempt, consistent with the user rule exempting system-root first components. No product-changing decisions remain for the human.
- 0xLeif decided "Scope approval of leak-host-equals-user: REQ-records-006 gains one exception — a host label equal to the local user name (case-insensitive) does not count as a host-name leak as a path component, while the same name still counts in a URL, as <user>@<host> and as the host of an scp-style remote path; full delta in the change's deltas/records.md": "approve" (pi chat, 2026-11-23, after the one-scope summary of the REQ-records-006 exception); recorded by pi-t14g; a relayed claim, not authenticated; ran specsync change approve a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user as 0xLeif
- Design (2026-11-23): in src/leaks.rs, names() computes per host whether the label equals the local user name (case-insensitive) and, when so, names_a_host() stops counting the path-component context while keeping the URL, <user>@<host> and scp-style contexts; check_name() applies the same per-host exception. No new state; users and hosts are already in Local.
- Diff review (qwen38-reviewer, t15z, 2026-11-24): pass, no blocking code finding; two test gaps fixed in the unit test (a dotted label that only equals the user after another label is still refused; the equality is tested mixed-case). Recorded corrections: my design note said the name check gains the exception, but the implementation and tasks.md keep check_name's rule, which the approved requirement's path-component scoping requires; testing.md names tests/leaks.rs, but the new tests are unit tests in src/leaks.rs, where the leaks unit tests live; I will not edit the approved change artifacts, so that stays as written.
- 0xLeif confirmed in the pi chat (2026-11-24 questionnaire): the scope-approval words 'approve' were his, resolving the checker's and the first reviewer's blocking governance finding on the relayed claim.
- 0xLeif decided (2026-11-24 questionnaire): the REQ-records-006 exception stays unconditional, as the approved delta sentence says, including an 8+ character host equal to the user name as a path component; reviewer finding F3 is resolved, the code keeps the approved text's behavior.
- 0xLeif decided "Scope re-approval of leak-host-equals-user after send-back: affected_paths gains changelog.d/2026-11-23-leak-host-equals-user.md (the planned fragment, per repo convention); the requirement delta is unchanged": "Add it to scope (Recommended)" (pi chat questionnaire, 2026-11-24, the changelog-scope question after the checker named the fragment); recorded by pi-t14g; a relayed claim, not authenticated; ran specsync change approve a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user as 0xLeif
- Checker blocking findings resolved (2026-11-24): (1) 0xLeif confirmed in the pi chat questionnaire that the scope-approval words 'approve' were his, so the relay stands; (2) the planned changelog fragment now sits in affected_paths, re-approved by his decision 'Add it to scope' through a send-back and fresh approval, and the native check passed and committed the materialized spec plus verification evidence.
- Disclosed before the implementation-review gate (2026-11-24, from checker note): the cargo-test evidence entry was misnamed with a wrong count (213) by the recording agent and renamed in place to its 1040 name before its first commit; REQ-records-003 says a name SHALL NOT change after creation, so this is a grey edge kept open here on purpose, and no 213 was ever committed. Also disclosed: testing.md names tests/leaks.rs while the new tests are unit tests in src/leaks.rs, and tasks.md was reworded after approval to match the approved delta, with the delta digest unchanged.

## What was verified

- [cargo test: full suite 1040 passed, 0 failed; new unit test a_host_label_equal_to_the_user_name_is_not_a_leak_as_a_path_component red on main, green with the fix; the built binary's doctor --leaks on the tob repository reports 0 findings (6 host-name findings before the fix)](evidence/2026-11-24-cargo-test-full-suite-1040-passed-0-failed-new-unit-test-a-ho.md)

## Open threads

## Timeline

- 2026-11-23T23:48:03Z · start · intent → intent · When a host label equals the local user name, as this Mac's 'Tob' for user 'tob', the host-name rule must not re-flag the plain folder the user-name rules deliberately let through; REQ-records-006 gains that exception ^t-627ceb
  - revision: 6e00b813a0d52be20a361b625cb96db03be171d6
  - by: unknown agent (hc t14g)
- 2026-11-23T23:58:24Z · gate-opened · intent → intent · Scope approval of leak-host-equals-user: REQ-records-006 gains one exception — a host label equal to the local user name (case-insensitive) does not count as a host-name leak as a path component, while the same name still counts in a URL, as <user>@<host> and as the host of an scp-style remote path; full delta in the change's deltas/records.md ^t-13a6fe
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - revision: 6e00b813a0d52be20a361b625cb96db03be171d6
  - holds: intent
  - drives: approve
  - by: unknown agent (hc t14g)
- 2026-11-23T23:59:52Z · gate-closed · intent → intent · Scope approval of leak-host-equals-user: REQ-records-006 gains one exception — a host label equal to the local user name (case-insensitive) does not count as a host-name leak as a path component, while the same name still counts in a URL, as <user>@<host> and as the host of an scp-style remote path; full delta in the change's deltas/records.md ^t-54c7c1
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - revision: 6e00b813a0d52be20a361b625cb96db03be171d6
  - relayed_by:
    - decided_by: 0xLeif
    - words: approve
    - where: pi chat, 2026-11-23, after the one-scope summary of the REQ-records-006 exception
    - recorded_by: pi-t14g
    - native: specsync change approve a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
    - actor: 0xLeif
  - by: unknown agent (hc t14g)
- 2026-11-24T00:00:10Z · advance · intent → design ·  ^t-a778a6
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - revision: 6e00b813a0d52be20a361b625cb96db03be171d6
  - by: unknown agent (hc t14g)
- 2026-11-24T00:00:28Z · advance · design → plan ·  ^t-3b3feb
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - revision: 6e00b813a0d52be20a361b625cb96db03be171d6
  - by: unknown agent (hc t14g)
- 2026-11-24T00:00:39Z · advance · plan → build · native:approved ^t-e4858d
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - gate: native:approved
  - revision: 6e00b813a0d52be20a361b625cb96db03be171d6
  - by: unknown agent (hc t14g)
- 2026-11-24T01:45:46Z · gate-opened · build → build · Scope re-approval of leak-host-equals-user after send-back: affected_paths gains changelog.d/2026-11-23-leak-host-equals-user.md (the planned fragment, per repo convention); the requirement delta is unchanged ^t-6bdbc2
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - revision: 6e00b813a0d52be20a361b625cb96db03be171d6
  - holds: build
  - drives: approve
  - by: unknown agent (hc t14g)
- 2026-11-24T01:45:54Z · direct-native-write · build → build · a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-ddbcc4
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - revision: 6e00b813a0d52be20a361b625cb96db03be171d6
  - direct_write:
    - change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: unknown agent (hc t14g)
- 2026-11-24T01:45:54Z · gate-closed · build → build · Scope re-approval of leak-host-equals-user after send-back: affected_paths gains changelog.d/2026-11-23-leak-host-equals-user.md (the planned fragment, per repo convention); the requirement delta is unchanged ^t-fa6357
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - revision: 6e00b813a0d52be20a361b625cb96db03be171d6
  - relayed_by:
    - decided_by: 0xLeif
    - words: Add it to scope (Recommended)
    - where: pi chat questionnaire, 2026-11-24, the changelog-scope question after the checker named the fragment
    - recorded_by: pi-t14g
    - native: specsync change approve a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
    - actor: 0xLeif
  - by: unknown agent (hc t14g)
- 2026-11-24T01:46:20Z · direct-native-write · build → build · a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user's lifecycle records changed outside Chough: state.json changed, verification.json added, verification-attempts.json added; state approved → verifying ^t-fd1e17
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - revision: 80791c9a6296773575af708c74af96f04a551ac3
  - direct_write:
    - change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
    - files:
      - state.json: changed
      - verification.json: added
      - verification-attempts.json: added
    - before_state: approved
    - after_state: verifying
  - by: unknown agent (hc t14g)
- 2026-11-24T01:46:20Z · advance · build → test · native:verifying ^t-a242c8
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - gate: native:verifying
  - revision: 80791c9a6296773575af708c74af96f04a551ac3
  - by: unknown agent (hc t14g)
- 2026-11-24T02:02:39Z · gate-opened · test → test · Implementation review of leak-host-equals-user: REQ-records-006 exception (host label equal to the local user name does not count as a path component; URL, <user>@<host> and scp contexts still count), implemented in src/leaks.rs with the unit test; both model reviews pass, both checker runs clean after your confirmations, 1040/1040 tests, native check committed ^t-9e0577
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - revision: 80791c9a6296773575af708c74af96f04a551ac3
  - holds: accept
  - drives: review
  - by: unknown agent (hc t14g)
- 2026-11-24T03:09:13Z · advance · test → accept · native:verifying ^t-1f0644
  - native_change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
  - gate: native:verifying
  - revision: 80791c9a6296773575af708c74af96f04a551ac3
  - by: unknown agent (hc t14z)
