---
change: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply
artifact: requirements
---

# Requirements

REQ-records-009 (modified): the plan's and the receipt's follow-ups also name what
`chough doctor` will report on the migrated tree: each `finished_session`, to archive
with `chough doctor --fix`, and each `stale_gates` session with each open gate's text
and `chough doctor --fix --reason`. They're computed by doctor's own session check
(REQ-history-012), whose findings are unchanged.
REQ-cli-055 (modified): the doctor's per-session findings come from that one check, so a
migration's follow-ups name what the doctor will report.
