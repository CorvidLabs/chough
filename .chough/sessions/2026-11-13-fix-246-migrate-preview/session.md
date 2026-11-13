---
kind: session
id: 2026-11-13-fix-246-migrate-preview
aliases: [fix-246-migrate-preview]
objective: "Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
pr: ""
criteria: []
requirements: []
cases: []
design: []
files: []
observations:
  - 2026-11-13-tc-records-migrate-preview-names-what-doctor-reports
  - 2026-11-13-tc-records-migrate-preview-names-what-doctor-reports-9609f3
  - 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
bugs: [2026-11-13-chough-migrate-s-preview-omits-the-finished-sessions-and-stale]
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
    holds: accept
    drives: review
    opened: 2026-11-13T07:59:29Z
run_consents: []
next: Continue accept
title: "Fix #246: migrate's preview names what doctor will report"
summary:
  - "chough migrate's preview and receipt name each session chough doctor will report right after the apply: finished sessions to archive with chough doctor --fix, and stale-gated ones with each gate's text and chough doctor --fix --reason, from doctor's own session check (REQ-records-009, REQ-cli-055)."
  - "Verified: the case red then green, the bug verified, and the full suite 736 of 736. Next: Leif's review and merge."
---
# Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply

## Goal

Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply

## Where it stands

## Decisions and why

- Leif decided "Approve fixing #246: the migration preview and receipt also list the sessions doctor will flag right after the apply": "Chough #246 migrate preview" (coordinator chat (Claude Code, hc t370), 2026-11-13T04:37Z, answering: 'Approve these two?', he selected 'Chough #246 migrate preview' (option text: 'The migration preview and receipt also list the sessions doctor will flag right after, so one consent round covers both. Small migration wording change, with a failing test ready.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply": "Chough #246 migrate preview" (coordinator chat (Claude Code, hc t370), 2026-11-13T04:37Z, answering: 'Approve these two?', he selected 'Chough #246 migrate preview' (option text: 'The migration preview and receipt also list the sessions doctor will flag right after, so one consent round covers both. Small migration wording change, with a failing test ready.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply as 0xLeif

## What was verified

- [Ran cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply for TC-records-migrate-preview-names-what-doctor-reports: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-records-migrate-preview-names-what-doctor-reports.md)
- [Ran cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply for TC-records-migrate-preview-names-what-doctor-reports: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-records-migrate-preview-names-what-doctor-reports-9609f3.md)
- [The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 9a081e0c, #246's final head; clippy --all-targets -D warnings and rustfmt --check were clean](evidence/2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo.md)

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
