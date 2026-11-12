---
kind: bug
id: 2026-11-12-the-leak-guard-counts-trailing-markup-as-part-of-a-credential
aliases:
  - BUG-the-leak-guard-counts-trailing-markup-as-part-of-a-credential-value-ref-306ffa88f21178ada95cb341
title: The leak guard counts trailing markup as part of a credential value, refusing short assignments and failing doctor on Chough's own lines
status: verified
classification: product-defect
scope: in-scope
impact: A legitimate write is blocked, and a write the guard allowed leaves the verify lane's leaks step failing
criteria: []
requirements: [REQ-records-006]
cases: []
evidence:
  - OBS-tc-records-credential-value-ends-at-markup-578953ac793f79c4268ecd17
  - OBS-tc-records-credential-value-ends-at-markup-565ff709a12577f43e77dbcc
receipts:
  - id: 2026-11-12-tc-records-credential-value-ends-at-markup
    aliases: OBS-tc-records-credential-value-ends-at-markup-578953ac793f79c4268ecd17
    claim: "Ran cargo test --locked --test leaks -- --exact a_short_assignment_ending_a_claim_is_written_and_leaves_no_leak_finding for TC-records-credential-value-ends-at-markup: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test leaks -- --exact a_short_assignment_ending_a_claim_is_written_and_leaves_no_leak_finding
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:68ac372a0953db2f8e404a01b2114788f3ccc4e69aae2df791d1750f4326e62c
    case: TC-records-credential-value-ends-at-markup
    case_digest: 0ace816592858e7ca1ae9cbf3861967070260b2114de2c8cf1815b790e03e377
    observed: 2026-11-12T01:59:13Z
    duration_seconds: 5
  - id: 2026-11-12-tc-records-credential-value-ends-at-markup-53a2dd
    aliases: OBS-tc-records-credential-value-ends-at-markup-565ff709a12577f43e77dbcc
    claim: "Ran cargo test --locked --test leaks -- --exact a_short_assignment_ending_a_claim_is_written_and_leaves_no_leak_finding for TC-records-credential-value-ends-at-markup: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_short_assignment_ending_a_claim_is_written_and_leaves_no_leak_finding
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:093da0cd828c5961c8b573d0aae48812955ae015d9171d6ef7d0796b4e341ed7
    case: TC-records-credential-value-ends-at-markup
    case_digest: 0ace816592858e7ca1ae9cbf3861967070260b2114de2c8cf1815b790e03e377
    observed: 2026-11-12T02:17:07Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t38b, test stage, area records
related: []
created: 2026-11-12T01:38:28Z
updated: 2026-11-12T02:18:06Z
---
# The leak guard counts trailing markup as part of a credential value, refusing short assignments and failing doctor on Chough's own lines

**What happened:** A claim ending in a 5 to 7 character password, secret or token assignment is refused as body holds credential; an accepted 4 character one makes doctor --leaks report the session.md What was verified line
Actual: The value runs on through the closing markup, so the rendered body or link line reaches 8 characters

**Steps:** evidence add --summary ending in a short credential assignment; then chough doctor --leaks

**Expected:** Only assignments whose value has 8 or more characters count (REQ-records-006), measured without the bold, period, bracket or parenthesis markup Chough adds

**Why it matters:** A legitimate write is blocked, and a write the guard allowed leaves the verify lane's leaks step failing

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:28Z · Reported (t38b, test stage, area records)
- 2026-11-12T02:01:36Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-records-credential-value-ends-at-markup failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-records-credential-value-ends-at-markup-578953ac793f79c4268ecd17
- 2026-11-12T02:18:06Z · status: investigating -> verified. Green at 0ee26ac0: TC-records-credential-value-ends-at-markup passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-records-credential-value-ends-at-markup-565ff709a12577f43e77dbcc
