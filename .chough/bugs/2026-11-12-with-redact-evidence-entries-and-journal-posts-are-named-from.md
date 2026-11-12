---
kind: bug
id: 2026-11-12-with-redact-evidence-entries-and-journal-posts-are-named-from
aliases:
  - BUG-with-redact-evidence-entries-and-journal-posts-are-named-from-the-unred-f1ee348405f1a8c0c0ea3b2f
title: With --redact, evidence entries and journal posts are named from the unredacted text, so a leak survives in the committed name
status: verified
classification: product-defect
scope: blocking
impact: A committed file name and id carry the local user name, host name, tokens or emails; lowercasing does not make them safe
criteria: []
requirements: [REQ-records-006, REQ-records-003]
cases: []
evidence:
  - OBS-tc-records-redacted-writes-are-named-from-the-redacted-text-64675e91c417ed90250c9918
  - OBS-tc-records-redacted-writes-are-named-from-the-redacted-text-c50b925a6f29b24593c25c98
receipts:
  - id: 2026-11-12-tc-records-redacted-writes-are-named-from-the-redacted-text
    aliases: OBS-tc-records-redacted-writes-are-named-from-the-redacted-text-64675e91c417ed90250c9918
    claim: "Ran cargo test --locked --test leaks -- --exact redacted_writes_are_named_from_the_redacted_text for TC-records-redacted-writes-are-named-from-the-redacted-text: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test leaks -- --exact redacted_writes_are_named_from_the_redacted_text
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:ac1dd458a6f478a36ee2d6086546e76d3e2a6440bc5e630e939793b630af3e05
    case: TC-records-redacted-writes-are-named-from-the-redacted-text
    case_digest: 81e36475bb4794b3bce34d47453af7af12c8f4c65f2e00c2bb8a8fd734f12a70
    observed: 2026-11-12T01:59:22Z
    duration_seconds: 7
  - id: 2026-11-12-tc-records-redacted-writes-are-named-from-the-redacted-text-afd310
    aliases: OBS-tc-records-redacted-writes-are-named-from-the-redacted-text-c50b925a6f29b24593c25c98
    claim: "Ran cargo test --locked --test leaks -- --exact redacted_writes_are_named_from_the_redacted_text for TC-records-redacted-writes-are-named-from-the-redacted-text: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact redacted_writes_are_named_from_the_redacted_text
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:52f4632cffcb8dfca007509d4fa5b0f9f32962182726c65f19104aec437b826b
    case: TC-records-redacted-writes-are-named-from-the-redacted-text
    case_digest: 81e36475bb4794b3bce34d47453af7af12c8f4c65f2e00c2bb8a8fd734f12a70
    observed: 2026-11-12T02:17:08Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t38b, test stage, area records
related: []
created: 2026-11-12T01:38:28Z
updated: 2026-11-12T02:18:06Z
---
# With --redact, evidence entries and journal posts are named from the unredacted text, so a leak survives in the committed name

**What happened:** evidence add --redact and journal add --redact store the redacted claim or title, but the file name, id, observations list, index link and feed id hold the raw value in slug form
Actual: The name is slugged from the raw text before redaction

**Steps:** chough --redact evidence add or journal add with a home path, user, host, token, credential assignment or email in --summary or --title

**Expected:** Every value written, the name and id included, holds <redacted:kind> instead (REQ-records-006); bug add and inbox add already name from the redacted text

**Why it matters:** A committed file name and id carry the local user name, host name, tokens or emails; lowercasing does not make them safe

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:28Z · Reported (t38b, test stage, area records)
- 2026-11-12T02:01:36Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-records-redacted-writes-are-named-from-the-redacted-text failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-records-redacted-writes-are-named-from-the-redacted-text-64675e91c417ed90250c9918
- 2026-11-12T02:18:06Z · status: investigating -> verified. Green at 0ee26ac0: TC-records-redacted-writes-are-named-from-the-redacted-text passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-records-redacted-writes-are-named-from-the-redacted-text-c50b925a6f29b24593c25c98
