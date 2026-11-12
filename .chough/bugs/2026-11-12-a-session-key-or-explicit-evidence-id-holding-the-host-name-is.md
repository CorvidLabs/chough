---
kind: bug
id: 2026-11-12-a-session-key-or-explicit-evidence-id-holding-the-host-name-is
aliases:
  - BUG-a-session-key-or-explicit-evidence-id-holding-the-host-name-is-not-leak-78050401d16bcb3096ed350d
title: A session key or explicit evidence ID holding the host name is not leak-checked and becomes a committed name
status: verified
classification: product-defect
scope: in-scope
impact: A person-given name commits the machine's host name
criteria: []
requirements: [REQ-records-006]
cases: []
evidence:
  - OBS-tc-records-given-names-hold-no-host-name-09fb36432053106a7c9e4582
  - OBS-tc-records-given-names-hold-no-host-name-164ecedfe171c04489fe9ce1
receipts:
  - id: 2026-11-12-tc-records-given-names-hold-no-host-name
    aliases: OBS-tc-records-given-names-hold-no-host-name-09fb36432053106a7c9e4582
    claim: "Ran cargo test --locked --test leaks -- --exact a_given_name_holding_the_host_name_is_refused for TC-records-given-names-hold-no-host-name: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test leaks -- --exact a_given_name_holding_the_host_name_is_refused
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:9f981b51638430d798501f9afc5cca09ef533c687f2ec440560fea8beab5d229
    case: TC-records-given-names-hold-no-host-name
    case_digest: c44dfe5dfd4d50c216c1e2a7d533145f2475673534fb77c153a942fe8602acec
    observed: 2026-11-12T01:59:23Z
    duration_seconds: 0
  - id: 2026-11-12-tc-records-given-names-hold-no-host-name-a00bd4
    aliases: OBS-tc-records-given-names-hold-no-host-name-164ecedfe171c04489fe9ce1
    claim: "Ran cargo test --locked --test leaks -- --exact a_given_name_holding_the_host_name_is_refused for TC-records-given-names-hold-no-host-name: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test leaks -- --exact a_given_name_holding_the_host_name_is_refused
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:1b4d26be4a3526890fc790389f891a94a7d4885d57e4a274a08dfb429a1322b9
    case: TC-records-given-names-hold-no-host-name
    case_digest: c44dfe5dfd4d50c216c1e2a7d533145f2475673534fb77c153a942fe8602acec
    observed: 2026-11-12T02:17:10Z
    duration_seconds: 0
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t38b, test stage, area records
related: []
created: 2026-11-12T01:38:28Z
updated: 2026-11-12T02:18:06Z
---
# A session key or explicit evidence ID holding the host name is not leak-checked and becomes a committed name

**What happened:** session new with a key holding the host name, and evidence add with an explicit ID holding it, both write; the folder or file name, id and alias carry the host name
Actual: Exit 0; the name is written

**Steps:** chough session new <key with host name> or chough evidence add <dated ID with host name>

**Expected:** The write is refused with refused:leak naming the field and host-name (REQ-records-006), as the same host name in a value already is

**Why it matters:** A person-given name commits the machine's host name

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:28Z · Reported (t38b, test stage, area records)
- 2026-11-12T02:01:36Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-records-given-names-hold-no-host-name failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-records-given-names-hold-no-host-name-09fb36432053106a7c9e4582
- 2026-11-12T02:18:06Z · status: investigating -> verified. Green at 0ee26ac0: TC-records-given-names-hold-no-host-name passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-records-given-names-hold-no-host-name-164ecedfe171c04489fe9ce1
