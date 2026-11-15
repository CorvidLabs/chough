---
kind: session
id: 2026-11-15-fix-278-receipt-dash
aliases: [fix-278-receipt-dash]
objective: "Fix #278: Chough's records join a receipt's claim to its evidence with a dash, not an em dash, and still read the old join"
stage: done
stage_status: archived
iteration: 1
native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
criteria: [DRIVE-7]
requirements: [REQ-history-006]
cases: [TC-history-archived-stories-join-evidence-without-an-em-dash]
observations:
  - 2026-11-15-tc-history-archived-stories-join-evidence-without-an-em-dash
  - 2026-11-15-tc-history-archived-stories-join-evidence-without-an-em-dash-94e09a
closed: 2026-11-15
outcome: done
---
# Describe the current objective before beginning work.

## Goal

Describe the current objective before beginning work.

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with": "yes" (Claude Code chat (hc t28), 2026-11-14, reply to the #278 scope summary); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with as 0xLeif
- 0xLeif decided "Re-approve the scope of chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with after review fixes": "Yes" (Claude Code chat (hc t28), 2026-11-14, reply to the #278 re-approval summary after Grok's review fixes); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change approve chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with as 0xLeif
- 0xLeif decided "Implementation review of chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with": "yI approve" (Claude Code chat (hc t28), 2026-11-14, reply to the PR #293 implementation review summary); recorded by claude-code (hc t28); a relayed claim, not authenticated; ran specsync change review chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with as 0xLeif with verdict pass

## What was verified

- 1 earlier run of the cases below is kept in this story's receipts, not listed here.
- Ran cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt for TC-history-archived-stories-join-evidence-without-an-em-dash: passed (exit 0, tests run 1). — 4ae983ffc913 ^2026-11-15-tc-history-archived-stories-join-evidence-without-an-em-dash-94e09a

## Bugs

None.

## Open threads

## Timeline

- 2026-11-15T02:52:04Z · start · intent → intent · fix-278-receipt-dash ^t-8d0a3b
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: claude-code (hc t28)
- 2026-11-15T02:52:04Z · advance · intent → design ·  ^t-7cbb90
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: claude-code (hc t28)
- 2026-11-15T02:52:04Z · skip · design → plan · Nothing visual changes: a separator in generated Markdown records ^t-82dc3f
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: claude-code (hc t28)
- 2026-11-15T02:54:52Z · gate-opened · plan → plan · Approve the scope of chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with ^t-0ca254
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - holds: plan
  - drives: approve
  - by: claude-code (hc t28)
- 2026-11-15T02:55:38Z · direct-native-write · plan → plan · chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with's lifecycle records changed outside Chough: state.json changed; state stays draft ^t-18b222
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - direct_write:
    - change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
    - files:
      - state.json: changed
    - before_state: draft
    - after_state: draft
  - by: claude-code (hc t28)
- 2026-11-15T02:55:38Z · gate-closed · plan → plan · Approve the scope of chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with ^t-6b3523
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes
    - where: Claude Code chat (hc t28), 2026-11-14, reply to the #278 scope summary
    - recorded_by: claude-code (hc t28)
    - summary_digest: 09cd9e181a2e2bfe547eda89af92b6f9fad9e02f4910707e35ca646c72f39544
    - list: dd016d1e95f2
    - native: specsync change approve chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
    - actor: 0xLeif
  - by: claude-code (hc t28)
- 2026-11-15T02:55:43Z · advance · plan → build · native:approved ^t-71cb8f
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - gate: native:approved
  - revision: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
  - by: claude-code (hc t28)
- 2026-11-15T03:04:02Z · advance · build → test · native:approved ^t-6e63e4
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - gate: native:approved
  - revision: 9bdae1013b8a5dd5b009543d27ce846037200189
  - by: claude-code (hc t28)
- 2026-11-15T03:07:38Z · advance · test → accept · native:verifying ^t-6809a9
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - gate: native:verifying
  - revision: a92ef3d52c0786cc8c1cdf45390bdebeb86c5c72
  - by: claude-code (hc t28)
- 2026-11-15T03:33:44Z · back · accept → plan · Grok's review: the REQ-history-006 and REQ-records-011 deltas changed after approval and need the human's approval again ^t-4fb0ee
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - revision: 2a4c1d6bc7edd2c38796e417603ac87fe4335775
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, accept: native verification and the human implementation review, checkpoint references: 1 case(s) and 1 observation(s) to re-check with `chough status`, native: the definition is verifying; changing intent, contract, acceptance criteria or scope needs native reapproval. Follow `specsync change status chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with`; Chough does not change native state.
  - by: claude-code (hc t28)
- 2026-11-15T03:33:44Z · gate-opened · plan → plan · Re-approve the scope of chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with after review fixes ^t-7b2d72
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - revision: 2a4c1d6bc7edd2c38796e417603ac87fe4335775
  - holds: plan
  - drives: approve
  - by: claude-code (hc t28)
- 2026-11-15T03:39:01Z · gate-closed · plan → plan · Re-approve the scope of chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with after review fixes ^t-5f4951
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - revision: 2a4c1d6bc7edd2c38796e417603ac87fe4335775
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes
    - where: Claude Code chat (hc t28), 2026-11-14, reply to the #278 re-approval summary after Grok's review fixes
    - recorded_by: claude-code (hc t28)
    - native: specsync change approve chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
    - actor: 0xLeif
  - by: claude-code (hc t28)
- 2026-11-15T03:39:10Z · advance · plan → build · native:verifying ^t-dfd146
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - gate: native:verifying
  - revision: 2a4c1d6bc7edd2c38796e417603ac87fe4335775
  - by: claude-code (hc t28)
- 2026-11-15T03:39:10Z · advance · build → test · native:verifying ^t-2234ca
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - gate: native:verifying
  - revision: 2a4c1d6bc7edd2c38796e417603ac87fe4335775
  - by: claude-code (hc t28)
- 2026-11-15T03:39:10Z · advance · test → accept · native:verifying ^t-3c9949
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - gate: native:verifying
  - revision: 2a4c1d6bc7edd2c38796e417603ac87fe4335775
  - by: claude-code (hc t28)
- 2026-11-15T03:39:20Z · gate-opened · accept → accept · Implementation review of chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with ^t-45a151
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - revision: 2a4c1d6bc7edd2c38796e417603ac87fe4335775
  - holds: accept
  - drives: review
  - by: claude-code (hc t28)
- 2026-11-15T03:45:42Z · gate-closed · accept → accept · Implementation review of chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with ^t-37c3e4
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - revision: 199f2941eb1734ce0c7fac00b7dc83330596fb4b
  - relayed_by:
    - decided_by: 0xLeif
    - words: yI approve
    - where: Claude Code chat (hc t28), 2026-11-14, reply to the PR #293 implementation review summary
    - recorded_by: claude-code (hc t28)
    - native: specsync change review chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
    - actor: 0xLeif
  - by: claude-code (hc t28)
- 2026-11-15T03:45:43Z · advance · accept → done · native:archived ^t-420ca0
  - native_change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
  - gate: native:archived
  - revision: 199f2941eb1734ce0c7fac00b7dc83330596fb4b
  - by: claude-code (hc t28)

Closed 2026-11-15: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-15-tc-history-archived-stories-join-evidence-without-an-em-dash
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt for TC-history-archived-stories-join-evidence-without-an-em-dash: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt
    revision: 9bdae1013b8a5dd5b009543d27ce846037200189
    environment: local-macos-arm64
    content: 2f78bb446b2610e5a31daa31d2b523f0bdb6210046a81665846ac17aa631edfe
    fingerprint: sha256:1dce59554ece2cd6b0af9c4fb8de506a09d433b859b3ca46a8adda20e95cf054
    case: TC-history-archived-stories-join-evidence-without-an-em-dash
    case_digest: 4e818ba81bc305fd85ad341fdcd94a12acab6d81f15d107fb483233c3afb9c4f
    observed: 2026-11-15T03:04:34Z
    duration_seconds: 0
    actor: claude-code (hc t28)
  - id: 2026-11-15-tc-history-archived-stories-join-evidence-without-an-em-dash-94e09a
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt for TC-history-archived-stories-join-evidence-without-an-em-dash: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt
    revision: 2a4c1d6bc7edd2c38796e417603ac87fe4335775
    environment: local-macos-arm64
    content: 001e32db55bf23251f55a66232187ab8797bcc93eb57a584e480d67bbc9364e0
    fingerprint: sha256:4ae983ffc913caf2cb3f9c979db9ee05b5ac19414b30ed98ad146493cb257c5e
    case: TC-history-archived-stories-join-evidence-without-an-em-dash
    case_digest: 4e818ba81bc305fd85ad341fdcd94a12acab6d81f15d107fb483233c3afb9c4f
    observed: 2026-11-15T03:39:12Z
    duration_seconds: 1
    actor: claude-code (hc t28)
```
