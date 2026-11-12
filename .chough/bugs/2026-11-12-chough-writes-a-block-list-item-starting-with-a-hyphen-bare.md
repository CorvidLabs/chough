---
kind: bug
id: 2026-11-12-chough-writes-a-block-list-item-starting-with-a-hyphen-bare
aliases:
  - BUG-chough-writes-a-block-list-item-starting-with-a-hyphen-bare-which-its-o-4abb00c8c2477730b405c808
title: Chough writes a block list item starting with a hyphen bare, which its own reader refuses, so status and next fail
status: verified
classification: product-defect
scope: blocking
impact: sh -c, bash -lc, python -m or node -e bindings leave status and next failing until a hand edit
criteria: []
requirements: [REQ-records-001]
cases: []
evidence:
  - OBS-tc-records-hyphen-led-block-items-read-back-4a8a2adfb9d913d0dca8b673
  - OBS-tc-records-hyphen-led-block-items-read-back-658111dcb507e5e6585fd16b
receipts:
  - id: 2026-11-12-tc-records-hyphen-led-block-items-read-back
    aliases: OBS-tc-records-hyphen-led-block-items-read-back-4a8a2adfb9d913d0dca8b673
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_block_list_item_starting_with_a_hyphen_reads_back for TC-records-hyphen-led-block-items-read-back: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test front_matter_writes -- --exact a_block_list_item_starting_with_a_hyphen_reads_back
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:5719818a4d05f7833db6bc0f79897a83a8b8696a21db6d8fb655604662a81e6f
    case: TC-records-hyphen-led-block-items-read-back
    case_digest: 275a75966b6899b4c2b6dc20a0d9b75e6a433e20e24c4a736299cd0e6df4c8d9
    observed: 2026-11-12T01:59:25Z
    duration_seconds: 1
  - id: 2026-11-12-tc-records-hyphen-led-block-items-read-back-f83cd4
    aliases: OBS-tc-records-hyphen-led-block-items-read-back-658111dcb507e5e6585fd16b
    claim: "Ran cargo test --locked --test front_matter_writes -- --exact a_block_list_item_starting_with_a_hyphen_reads_back for TC-records-hyphen-led-block-items-read-back: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test front_matter_writes -- --exact a_block_list_item_starting_with_a_hyphen_reads_back
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:19f8e1f0f341acdac1642512fd1fa3f900ab14137275a7b97b02ca95aa08111f
    case: TC-records-hyphen-led-block-items-read-back
    case_digest: 275a75966b6899b4c2b6dc20a0d9b75e6a433e20e24c4a736299cd0e6df4c8d9
    observed: 2026-11-12T02:17:12Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 18025ad47c730af7589317f76608634b2487896f
found_by: t38b, test stage, area records
related: []
created: 2026-11-12T01:40:37Z
updated: 2026-11-12T02:18:06Z
---
# Chough writes a block list item starting with a hyphen bare, which its own reader refuses, so status and next fail

**What happened:** After one cases run of a command binding such as sh -c with a quoted argument, chough status and chough next exit 1: the evidence entry's command list has a bare '- -c' item, read as a nested list outside the subset; doctor reports nothing
Actual: plain_safe lets an item starting with '-' stay bare in a block list, and parse_block refuses it

**Steps:** A command-bound case whose argv needs a block list (an item needing quotes) and holds an item starting with a hyphen

**Expected:** Chough never writes front matter outside the subset it reads (REQ-records-001); a record Chough cannot read is named by doctor

**Why it matters:** sh -c, bash -lc, python -m or node -e bindings leave status and next failing until a hand edit

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:40:37Z · Reported (t38b, test stage, area records)
- 2026-11-12T02:01:22Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-records-hyphen-led-block-items-read-back failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-records-hyphen-led-block-items-read-back-4a8a2adfb9d913d0dca8b673
- 2026-11-12T02:18:06Z · status: investigating -> verified. Green at 0ee26ac0: TC-records-hyphen-led-block-items-read-back passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-records-hyphen-led-block-items-read-back-658111dcb507e5e6585fd16b
