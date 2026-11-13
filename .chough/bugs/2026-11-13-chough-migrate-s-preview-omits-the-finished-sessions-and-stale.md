---
kind: bug
id: 2026-11-13-chough-migrate-s-preview-omits-the-finished-sessions-and-stale
aliases: []
title: chough migrate's preview omits the finished sessions and stale gates chough doctor reports right after the apply
status: verified
classification: product-defect
scope: in-scope
impact: Adopting 0.6.0 in a project with finished or stale-gated sessions needs a second human round after consenting to the migration (seen in CorvidLabs/spec-sync and raven)
criteria: []
requirements: [REQ-records-009, REQ-cli-055]
cases: []
evidence:
  - 2026-11-13-fix-246-migrate-preview/2026-11-13-tc-records-migrate-preview-names-what-doctor-reports
  - 2026-11-13-fix-246-migrate-preview/2026-11-13-tc-records-migrate-preview-names-what-doctor-reports-9609f3
receipts: []
environment: local-macos-arm64
revision: bc6d4479ffdc60d4b84891542c8949325f8886a7
found_by: "claude-code (hc t382) (GitHub #246 (corvid-agent, adopting Chough 0.6.0 in CorvidLabs/spec-sync and raven))"
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-13T07:44:14Z
updated: 2026-11-13T07:47:29Z
---
# chough migrate's preview omits the finished sessions and stale gates chough doctor reports right after the apply

**What happened:** The migration preview's Follow-ups listed records_in_verification, the merge-driver lines and chough skill update only; right after chough migrate --apply, chough doctor exited 1 with finished_session and stale_gates findings the preview never named
Actual: The preview names none of them; the person consenting sees the stale-gate decisions only after the apply, and the adoption PR has to go back to the human

**Steps:** Trigger: chough migrate on a 0.5.x tree holding a finished session and a session whose native change is archived but which still waits on human gates, then chough migrate --apply and chough doctor
1. Build a 0.5.x fixture with a finished session and an archived-change session waiting on two gates
2. Run chough migrate and read its Follow-ups
3. Apply and run chough doctor: it reports stale_gates and finished_session

**Expected:** The preview's and the receipt's follow-ups name each session doctor will report: finished_session ones to archive with chough doctor --fix, stale_gates ones with each open gate's text and chough doctor --fix --reason, so one consent round covers both

**Why it matters:** Adopting 0.6.0 in a project with finished or stale-gated sessions needs a second human round after consenting to the migration (seen in CorvidLabs/spec-sync and raven)

**Fix:** ca5685cd (t38b's 019d97e1): doctor's per-session check is one function, doctor::session_check; doctor::session_follow_ups runs it on each session the migration plan writes and each open session it leaves, giving a finished_session follow-up (chough doctor --fix, then commit) and a stale_gates one with each gate's text and chough doctor --fix --reason

## Log

- 2026-11-13T07:44:14Z · Reported (GitHub #246 (corvid-agent, adopting Chough 0.6.0 in CorvidLabs/spec-sync and raven)) · by claude-code (hc t382)
- 2026-11-13T07:44:14Z · Added despite similar reports: Distinct from earlier migrate and doctor bugs: this is the preview not naming doctor's post-apply findings · by claude-code (hc t382)
- 2026-11-13T07:46:00Z · evidence: 2026-11-13-fix-246-migrate-preview/2026-11-13-tc-records-migrate-preview-names-what-doctor-reports. Red at bc6d4479 (t38b's test): the preview's only follow-up is the merge-driver line; it names neither finished_session 2026-11-13-finished nor the stale-gated session · by claude-code (hc t382)
- 2026-11-13T07:47:29Z · status: open -> verified; evidence: 2026-11-13-fix-246-migrate-preview/2026-11-13-tc-records-migrate-preview-names-what-doctor-reports-9609f3; fix: ca5685cd (t38b's 019d97e1): doctor's per-session check is one function, doctor::session_check; doctor::session_follow_ups runs it on each session the migration plan writes and each open session it leaves, giving a finished_session follow-up (chough doctor --fix, then commit) and a stale_gates one with each gate's text and chough doctor --fix --reason. Green at ca5685cd: the cases pass · by claude-code (hc t382)
