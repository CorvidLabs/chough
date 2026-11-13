---
kind: session
id: 2026-11-13-fix-246-migrate-preview
aliases: [fix-246-migrate-preview]
objective: "Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply"
stage: done
stage_status: archived
iteration: 1
native_change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
observations:
  - 2026-11-13-tc-records-migrate-preview-names-what-doctor-reports
  - 2026-11-13-tc-records-migrate-preview-names-what-doctor-reports-9609f3
  - 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
bugs: [2026-11-13-chough-migrate-s-preview-omits-the-finished-sessions-and-stale]
title: "Fix #246: migrate's preview names what doctor will report"
summary:
  - "chough migrate's preview and receipt name each session chough doctor will report right after the apply: finished sessions to archive with chough doctor --fix, and stale-gated ones with each gate's text and chough doctor --fix --reason, from doctor's own session check (REQ-records-009, REQ-cli-055)."
  - "Verified: the case red then green, the bug verified, and the full suite 736 of 736. Next: Leif's review and merge."
closed: 2026-11-13
outcome: done
---
# Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply

## Goal

Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply

## Where it stands

## Decisions and why

- Leif decided "Approve fixing #246: the migration preview and receipt also list the sessions doctor will flag right after the apply": "Chough #246 migrate preview" (coordinator chat (Claude Code, hc t370), 2026-11-13T04:37Z, answering: 'Approve these two?', he selected 'Chough #246 migrate preview' (option text: 'The migration preview and receipt also list the sessions doctor will flag right after, so one consent round covers both. Small migration wording change, with a failing test ready.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply": "Chough #246 migrate preview" (coordinator chat (Claude Code, hc t370), 2026-11-13T04:37Z, answering: 'Approve these two?', he selected 'Chough #246 migrate preview' (option text: 'The migration preview and receipt also list the sessions doctor will flag right after, so one consent round covers both. Small migration wording change, with a failing test ready.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply as 0xLeif
- Leif decided "Implementation review of fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply": "#259 migrate preview (#246)" (coordinator chat (Claude Code, hc t370), 2026-11-13T14:14Z, answering: 'Chough PRs passed by the reviewer, part 1: approve the implementation? Each ships after your yes, then you'll get a merge it.', he selected '#256 record polish (0.6.1)', '#258 CI flake fix (#247)', '#259 migrate preview (#246)' and '#261 relay details (#249)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change review fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply as 0xLeif with verdict pass

## What was verified

- 1 earlier run of the cases below is kept in this story's receipts, not listed here.
- Ran cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply for TC-records-migrate-preview-names-what-doctor-reports: passed (exit 0, tests run 1). — b1b230bf6de6 ^2026-11-13-tc-records-migrate-preview-names-what-doctor-reports-9609f3
- The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 9a081e0c, #246's final head; clippy --all-targets -D warnings and rustfmt --check were clean ^2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo

## Bugs

- [chough migrate's preview omits the finished sessions and stale gates chough doctor reports right after the apply](../../bugs/2026-11-13-chough-migrate-s-preview-omits-the-finished-sessions-and-stale.md): verified

## Open threads

## Timeline

- 2026-11-13T04:37:59Z · start · intent → intent · Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply ^t-89916f
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t382)
- 2026-11-13T04:37:59Z · gate-opened · intent → intent · Approve fixing #246: the migration preview and receipt also list the sessions doctor will flag right after the apply ^t-33e3ed
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T04:37:59Z · gate-closed · intent → intent · Approve fixing #246: the migration preview and receipt also list the sessions doctor will flag right after the apply ^t-77eb9d
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - relayed_by:
    - decided_by: Leif
    - words: Chough #246 migrate preview
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T04:37Z, answering: 'Approve these two?', he selected 'Chough #246 migrate preview' (option text: 'The migration preview and receipt also list the sessions doctor will flag right after, so one consent round covers both. Small migration wording change, with a failing test ready.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T04:37:59Z · advance · intent → design · Leif approved the intent (03:44Z, relayed by t370) ^t-5b8d0e
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t382)
- 2026-11-13T04:37:59Z · skip · design → plan · No visuals or interactions change ^t-c2fd70
  - revision: 38fe24c063e3afde49a5be5093cd69791bd06126
  - by: claude-code (hc t382)
- 2026-11-13T04:38:28Z · correction · plan → plan · The note's time is wrong: Leif approved this intent at 2026-11-13T04:37Z (relayed by t370), as the gate-closed entry's where says, not 03:44Z; the setup script reused the time of the #247 and #249 approvals ^t-d6f2d1
  - corrects: ^t-5b8d0e
  - by: claude-code (hc t382)
- 2026-11-13T04:40:18Z · gate-opened · plan → plan · Approve the scope of fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply ^t-850603
  - native_change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
  - revision: 25af1398b2627c339a5ce0a0b3f3cd78a79e261c
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T04:40:18Z · gate-closed · plan → plan · Approve the scope of fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply ^t-579cd4
  - native_change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
  - revision: 25af1398b2627c339a5ce0a0b3f3cd78a79e261c
  - relayed_by:
    - decided_by: Leif
    - words: Chough #246 migrate preview
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T04:37Z, answering: 'Approve these two?', he selected 'Chough #246 migrate preview' (option text: 'The migration preview and receipt also list the sessions doctor will flag right after, so one consent round covers both. Small migration wording change, with a failing test ready.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T04:40:21Z · advance · plan → build · Leif approved the scope (04:37Z, relayed by t370) ^t-685e6e
  - native_change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
  - gate: native:approved
  - revision: 25af1398b2627c339a5ce0a0b3f3cd78a79e261c
  - by: claude-code (hc t382)
- 2026-11-13T07:47:29Z · advance · build → test · Built: the red and the fix, the cases red then green, the bug verified ^t-0149db
  - native_change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
  - gate: native:approved
  - revision: ca5685cdd311a905b1747b8464b1abf881e62c07
  - by: claude-code (hc t382)
- 2026-11-13T07:59:29Z · advance · test → accept · Tested: the cases green and the full suite 736/736 ^t-820b83
  - native_change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
  - gate: native:verifying
  - revision: 9a081e0c6317beacb6ea4afa7365d1f0884914e6
  - by: claude-code (hc t382)
- 2026-11-13T07:59:29Z · gate-opened · accept → accept · Implementation review of fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply ^t-be33f6
  - native_change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
  - revision: 9a081e0c6317beacb6ea4afa7365d1f0884914e6
  - holds: accept
  - drives: review
  - by: claude-code (hc t382)
- 2026-11-13T14:17:16Z · gate-closed · accept → accept · Implementation review of fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply ^t-f8c7b7
  - native_change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
  - revision: 61b24a4163736e03e03071ace41022b73f3b64ae
  - relayed_by:
    - decided_by: Leif
    - words: #259 migrate preview (#246)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T14:14Z, answering: 'Chough PRs passed by the reviewer, part 1: approve the implementation? Each ships after your yes, then you'll get a merge it.', he selected '#256 record polish (0.6.1)', '#258 CI flake fix (#247)', '#259 migrate preview (#246)' and '#261 relay details (#249)'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change review fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T14:17:18Z · advance · accept → done · native:archived ^t-4869c4
  - native_change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
  - gate: native:archived
  - revision: 61b24a4163736e03e03071ace41022b73f3b64ae
  - by: claude-code (hc t382)

Closed 2026-11-13: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-13-tc-records-migrate-preview-names-what-doctor-reports
    claim: "Ran cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply for TC-records-migrate-preview-names-what-doctor-reports: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply
    revision: bc6d4479ffdc60d4b84891542c8949325f8886a7
    environment: local-macos-arm64
    content: bc32e0a8290da75323b46576c288e2da274d9e5f14fd04175a01dffb2ab183ce
    fingerprint: sha256:d4e9fff1c249051c0f861738a51f5199058899e10457ea025e0fee0af906535c
    case: TC-records-migrate-preview-names-what-doctor-reports
    case_digest: 6c9a069cdb9cbfb5a53789f87c9a5b593cc427539e89a3364028853ea22c7af5
    observed: 2026-11-13T07:45:52Z
    duration_seconds: 2
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-migrate-preview-names-what-doctor-reports-9609f3
    claim: "Ran cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply for TC-records-migrate-preview-names-what-doctor-reports: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply
    revision: ca5685cdd311a905b1747b8464b1abf881e62c07
    environment: local-macos-arm64
    content: 7a7aea84dc1e09e6ea513c87077793df46508d7c62b826b2330d1abcc6d7bc1c
    fingerprint: sha256:b1b230bf6de63d0a93001212ace6fb39eded6a0a78bff6417b01d5b460ff87fa
    case: TC-records-migrate-preview-names-what-doctor-reports
    case_digest: 6c9a069cdb9cbfb5a53789f87c9a5b593cc427539e89a3364028853ea22c7af5
    observed: 2026-11-13T07:46:07Z
    duration_seconds: 2
    actor: claude-code (hc t382)
  - id: 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
    claim: "The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 9a081e0c, #246's final head; clippy --all-targets -D warnings and rustfmt --check were clean"
    result: passed
    revision: 9a081e0c6317beacb6ea4afa7365d1f0884914e6
    environment: local-macos-arm64
    content: 31575271f78a6fda3693493a400c10f02b053187d68cc82330a44690f043bf84
    observed: 2026-11-13T07:59:26Z
    actor: claude-code (hc t382)
```
