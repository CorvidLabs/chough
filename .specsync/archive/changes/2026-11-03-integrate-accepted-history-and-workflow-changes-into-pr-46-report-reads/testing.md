---
change: integrate-accepted-history-and-workflow-changes-into-pr-46-report-reads
artifact: testing
---

# Testing

Initial37 integration: run focused report case-read/invalidations/trace/typed-error regressions and history tests, plus format/lint as needed. Do not run full Fledge, MSRV, final cases or committed verification until root provides final44 source.

Final combined source: run all nine mandatory Fledge verification steps, all tests on Rust1.95 and Rust1.88, and applicable bound history, report and workflow cases. Use actual execution counts. Compare immutable archives, evidence, artifacts and cold-start blobs from every parent byte-for-byte. Check requirements, checkpoint collections and transition occurrence preservation. Compare final44 main squash source identity with its premerged qualified tip. Refresh native checks and audit; root alone records review/ship. Hosted checks are separate after an authorized push. Preserve failures without weakened assertions.
