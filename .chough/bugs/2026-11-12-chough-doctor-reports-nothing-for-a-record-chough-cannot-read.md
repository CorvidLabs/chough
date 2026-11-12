---
kind: bug
id: 2026-11-12-chough-doctor-reports-nothing-for-a-record-chough-cannot-read
aliases:
  - BUG-chough-doctor-reports-nothing-for-a-record-chough-cannot-read-while-sta-070fab337107164182cb40f2
title: chough doctor reports nothing for a record Chough cannot read, while status and next refuse the project
status: verified
classification: requirement-gap
scope: in-scope
impact: An agent told to run doctor sees a clean project that cannot be read
criteria: []
requirements: [REQ-history-012]
cases: []
evidence:
  - OBS-tc-history-doctor-reports-an-unreadable-record-9ec8b51fd182a0bc22682785
  - OBS-tc-history-doctor-reports-an-unreadable-record-cb13b6075c9ee456ae1b5100
receipts:
  - id: 2026-11-12-tc-history-doctor-reports-an-unreadable-record
    aliases: OBS-tc-history-doctor-reports-an-unreadable-record-9ec8b51fd182a0bc22682785
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_reports_a_committed_record_its_reader_refuses for TC-history-doctor-reports-an-unreadable-record: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test doctor -- --exact doctor_reports_a_committed_record_its_reader_refuses
    revision: 2a93de1ef1ef2925428bc2d5c1fa197f35957e9b
    environment: local-macos-arm64
    content: af30284b9051e49f6b0ef8baef52cecb50202288519151ea4c6f97ce70f7917f
    fingerprint: sha256:8776d4b64227f7a6dbc52621430abab1df1bbb7a359f808cc02b4abdf16c9af2
    case: TC-history-doctor-reports-an-unreadable-record
    case_digest: 1159f8f73e0d9c54f45095e186216b3b4465d3af29fe8369c7b9153bda32100e
    observed: 2026-11-12T06:56:57Z
    duration_seconds: 1
  - id: 2026-11-12-tc-history-doctor-reports-an-unreadable-record-3b7595
    aliases: OBS-tc-history-doctor-reports-an-unreadable-record-cb13b6075c9ee456ae1b5100
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_reports_a_committed_record_its_reader_refuses for TC-history-doctor-reports-an-unreadable-record: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_reports_a_committed_record_its_reader_refuses
    revision: 8091890cb874cf2bf33f8d5e3a6467d3bbee0dbb
    environment: local-macos-arm64
    content: 0d709ba899b8dfcf2dc898f29ca4dbb97df391dcac2ed6abe29f19ea553c235d
    fingerprint: sha256:47a541408022376ecef72c570c64787d73e59d2c97fa5a99cbc1c4876d14be41
    case: TC-history-doctor-reports-an-unreadable-record
    case_digest: 1159f8f73e0d9c54f45095e186216b3b4465d3af29fe8369c7b9153bda32100e
    observed: 2026-11-12T06:59:11Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 0ee26ac0, debug build, SpecSync 6.2.0
revision: e71b11b32167cad7ff8e298f9eff04a13069c3eb
found_by: t38b, test stage, area records (R4 retest)
related: []
created: 2026-11-12T02:19:35Z
updated: 2026-11-12T06:59:41Z
---
# chough doctor reports nothing for a record Chough cannot read, while status and next refuse the project

**What happened:** With an evidence entry whose front matter is outside the subset, status and next exit 1 naming the line, and doctor exits 0 with no findings
Actual: REQ-history-012 lists no unreadable-record finding

**Steps:** Any committed Chough record that the reader refuses, such as one written by the R4 bug

**Expected:** doctor names the debt that blocks status and next

**Why it matters:** An agent told to run doctor sees a clean project that cannot be read

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T02:19:35Z · Reported (t38b, test stage, area records (R4 retest))
- 2026-11-12T06:57:33Z · status: open -> investigating. Red at 2a93de1e (TC-history-doctor-reports-an-unreadable-record, before its fix, SpecSync 6.3.0 on PATH): failed for this bug's reason; fix in 64db6bf8.; evidence: OBS-tc-history-doctor-reports-an-unreadable-record-9ec8b51fd182a0bc22682785
- 2026-11-12T06:59:41Z · status: investigating -> verified. Green at 8091890c: TC-history-doctor-reports-an-unreadable-record passed; full suite 638 passed, clippy and fmt clean on SpecSync 6.3.0 at 596ced67; t38b's black-box retest passed.; evidence: OBS-tc-history-doctor-reports-an-unreadable-record-cb13b6075c9ee456ae1b5100
