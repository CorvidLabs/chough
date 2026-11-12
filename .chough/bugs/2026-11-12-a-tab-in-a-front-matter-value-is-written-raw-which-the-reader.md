---
kind: bug
id: 2026-11-12-a-tab-in-a-front-matter-value-is-written-raw-which-the-reader
aliases:
  - BUG-a-tab-in-a-front-matter-value-is-written-raw-which-the-reader-refuses-s-275ddb164e77d008e045f3f9
title: A tab in a front matter value is written raw, which the reader refuses, so one write makes the record or session unreadable
status: verified
classification: product-defect
scope: blocking
impact: One write leaves the session unreadable until a hand edit
criteria: []
requirements: [REQ-records-001]
cases: []
evidence:
  - OBS-tc-records-tabs-in-values-read-back-6368563e517ea218f885b516
  - OBS-tc-records-tabs-in-values-read-back-b53b5ef436ac3de271bc1994
receipts:
  - id: 2026-11-12-tc-records-tabs-in-values-read-back
    aliases: OBS-tc-records-tabs-in-values-read-back-6368563e517ea218f885b516
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_value_is_stored_as_a_space_and_reads_back for TC-records-tabs-in-values-read-back: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_value_is_stored_as_a_space_and_reads_back
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:4abda95388c343b58bd02e97e83cf41512947e6962e163371147c7c972f97a2f
    case: TC-records-tabs-in-values-read-back
    case_digest: 6a9724e12b8d79f4a10cdcba8e04d4d746d8a65c19637e87e6dc88f6e13ecf03
    observed: 2026-11-12T01:59:26Z
    duration_seconds: 0
  - id: 2026-11-12-tc-records-tabs-in-values-read-back-71ca9e
    aliases: OBS-tc-records-tabs-in-values-read-back-b53b5ef436ac3de271bc1994
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_value_is_stored_as_a_space_and_reads_back for TC-records-tabs-in-values-read-back: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test front_matter_writes -- --exact a_tab_in_a_value_is_stored_as_a_space_and_reads_back
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:64d658ee5d30818ee830a3284ef90fc6fe454df7637d0f19c4ac2e1684bba25a
    case: TC-records-tabs-in-values-read-back
    case_digest: 6a9724e12b8d79f4a10cdcba8e04d4d746d8a65c19637e87e6dc88f6e13ecf03
    observed: 2026-11-12T02:17:14Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 94023bc8edc863230805a2f60799e34a7a231948
found_by: t38b, test stage, area records
related: []
created: 2026-11-12T01:41:53Z
updated: 2026-11-12T02:18:07Z
---
# A tab in a front matter value is written raw, which the reader refuses, so one write makes the record or session unreadable

**What happened:** checkpoint --objective or --next-action, evidence add --summary or --limitation, and bug add --title with a tab exit 0, then status and next exit 1: a tab is outside the front matter subset
Actual: render_scalar quotes the value but keeps the raw tab, and the reader refuses any tab

**Steps:** Any front matter value holding a tab, such as pasted terminal output

**Expected:** Chough never writes front matter outside the subset it reads (REQ-records-001)

**Why it matters:** One write leaves the session unreadable until a hand edit

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:41:53Z · Reported (t38b, test stage, area records)
- 2026-11-12T02:01:22Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-records-tabs-in-values-read-back failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-records-tabs-in-values-read-back-6368563e517ea218f885b516
- 2026-11-12T02:18:07Z · status: investigating -> verified. Green at 0ee26ac0: TC-records-tabs-in-values-read-back passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-records-tabs-in-values-read-back-b53b5ef436ac3de271bc1994
