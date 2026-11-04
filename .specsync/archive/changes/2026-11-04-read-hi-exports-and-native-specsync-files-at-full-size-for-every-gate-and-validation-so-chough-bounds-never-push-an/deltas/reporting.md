## ADDED

### REQUIREMENT REQ-reporting-004

Timing reads native records at full size. `chough timing` SHALL read each native SpecSync lifecycle record it takes times from, such as a change's `approvals.json`, `verification.json`, `review.json` and `finalization.json`, within SpecSync's 4 MiB change-artifact bound and never Chough's 65,536-byte record bound, so a long native ledger still gives its times; only a record over SpecSync's bound SHALL be reported unknown as over its bound. The report stays read-only. Human decision: Leif (0xLeif), issue #100 and his instruction of 2026-11-04: "Chough should only reduce and do all that for its own files in .chough and not the specs, not the human intent." Trace: LIFE-15, LIFE-10.

Acceptance Criteria
- With the current change's `approvals.json` larger than 65,536 bytes, `chough timing --json` lists both of its approvals with their times and reports no `approvals_unknown`.
