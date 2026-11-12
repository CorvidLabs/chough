---
kind: bug
id: 2026-11-12-the-migration-plan-and-receipt-do-not-say-what-each-carried-0
aliases:
  - BUG-the-migration-plan-and-receipt-do-not-say-what-each-carried-0-5-x-file-b25e1b190f3d3647d18c218e
title: The migration plan and receipt do not say what each carried 0.5.x file becomes
status: verified
classification: product-defect
scope: in-scope
impact: A reviewer cannot see where session.json or the EVT files went before consenting
criteria: []
requirements: [REQ-records-009]
cases: []
evidence:
  - OBS-tc-records-migrate-plan-says-what-each-file-becomes-f7b471a433f8e08a07239738
  - OBS-tc-records-migrate-plan-says-what-each-file-becomes-86f0b37e83ff32491a3a6d51
receipts:
  - id: 2026-11-12-tc-records-migrate-plan-says-what-each-file-becomes
    aliases: OBS-tc-records-migrate-plan-says-what-each-file-becomes-f7b471a433f8e08a07239738
    claim: "Ran cargo test --locked --test migrate -- --exact the_plan_and_receipt_say_what_each_carried_file_becomes for TC-records-migrate-plan-says-what-each-file-becomes: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test migrate -- --exact the_plan_and_receipt_say_what_each_carried_file_becomes
    revision: 57b88a8a127c0796d49cb2a0b3950226dbc1d21a
    environment: local-macos-arm64
    content: de0c45417537500b3853fa028ac92fbdb75163962b09fd42d9de7882da6eece8
    fingerprint: sha256:7b8bdf52923e29acd0896e70186ca14e968e8c28c74b64f18b4fd7b776f71311
    case: TC-records-migrate-plan-says-what-each-file-becomes
    case_digest: 60f3d3b79c8280f37bb048b6c4096783bf53c63c3bce71a348b15c09374fa39d
    observed: 2026-11-12T02:11:49Z
    duration_seconds: 2
  - id: 2026-11-12-tc-records-migrate-plan-says-what-each-file-becomes-08b829
    aliases: OBS-tc-records-migrate-plan-says-what-each-file-becomes-86f0b37e83ff32491a3a6d51
    claim: "Ran cargo test --locked --test migrate -- --exact the_plan_and_receipt_say_what_each_carried_file_becomes for TC-records-migrate-plan-says-what-each-file-becomes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact the_plan_and_receipt_say_what_each_carried_file_becomes
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:bffc1270b2052bc1ed2f34f7ee4d8fdca7ff3f987197736af206fcf758a278f8
    case: TC-records-migrate-plan-says-what-each-file-becomes
    case_digest: 60f3d3b79c8280f37bb048b6c4096783bf53c63c3bce71a348b15c09374fa39d
    observed: 2026-11-12T02:17:04Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 4c450dd587bc89738118fb50a0883855e708fe4c
found_by: t384, test stage, area upgrade
related: []
created: 2026-11-12T02:03:38Z
updated: 2026-11-12T02:18:06Z
---
# The migration plan and receipt do not say what each carried 0.5.x file becomes

**What happened:** chough migrate's sources is a bare path-to-SHA-256 map; carried files name no target, and the receipt's Removed section says only carried into the 0.6.0 records above
Actual: Only removed files say why; carried ones can be traced only by ID through aliases

**Steps:** chough migrate on any committed 0.5.0 project

**Expected:** Each 0.5.x file with its SHA-256 and what it becomes (REQ-records-009, docs/cli.md)

**Why it matters:** A reviewer cannot see where session.json or the EVT files went before consenting

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T02:03:38Z · Reported (t384, test stage, area upgrade)
- 2026-11-12T02:12:00Z · status: open -> investigating. Red at 57b88a8a (leif/0.6.0-test1-red: d854ce1a plus the regression tests): TC-records-migrate-plan-says-what-each-file-becomes failed, the plan has no becomes; fix in 2de0e7cd.; evidence: OBS-tc-records-migrate-plan-says-what-each-file-becomes-f7b471a433f8e08a07239738
- 2026-11-12T02:18:06Z · status: investigating -> verified. Green at 0ee26ac0: TC-records-migrate-plan-says-what-each-file-becomes passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-records-migrate-plan-says-what-each-file-becomes-86f0b37e83ff32491a3a6d51
