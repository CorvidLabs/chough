---
kind: bug
id: 2026-11-12-the-knowledge-index-links-folders-while-req-cli-044-says-every
aliases:
  - BUG-the-knowledge-index-links-folders-while-req-cli-044-says-every-link-res-d8090478910f5decf5e61d42
title: The knowledge index links folders while REQ-cli-044 says every link resolves to an existing file
status: verified
classification: requirement-gap
scope: in-scope
impact: Folder links do nothing in Obsidian
criteria: []
requirements: [REQ-cli-043, REQ-cli-044]
cases: []
evidence:
  - OBS-tc-cli-send-back-wording-in-the-specs-673576ff2d3dc6e1e67499a8
  - OBS-tc-cli-send-back-wording-in-the-specs-c38beba2a983bdab7eedeb68
receipts:
  - id: 2026-11-12-tc-cli-send-back-wording-in-the-specs
    aliases: OBS-tc-cli-send-back-wording-in-the-specs-673576ff2d3dc6e1e67499a8
    claim: "Ran cargo test --locked --test spec_split -- --exact the_send_back_wording_is_in_the_cli_requirements for TC-cli-send-back-wording-in-the-specs: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test spec_split -- --exact the_send_back_wording_is_in_the_cli_requirements
    revision: 18c04614cfc4318fd4a219d4c8f42a3bf00f7e0b
    environment: local-macos-arm64
    content: 5fcbeae90b2dfe5515964344c355f099de6ef2bb570bb17f4719da618bc59404
    fingerprint: sha256:798b58f11c8ecbccbd40b08c89624559ca824579ef2bb4e6138c1e932a4cf310
    case: TC-cli-send-back-wording-in-the-specs
    case_digest: 7fc001a83d34dbfa573b0b9dc5b005b74f43fe51fc753b687d196cdfeef53afa
    observed: 2026-11-12T14:19:38Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-send-back-wording-in-the-specs-6c58b8
    aliases: OBS-tc-cli-send-back-wording-in-the-specs-c38beba2a983bdab7eedeb68
    claim: "Ran cargo test --locked --test spec_split -- --exact the_send_back_wording_is_in_the_cli_requirements for TC-cli-send-back-wording-in-the-specs: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test spec_split -- --exact the_send_back_wording_is_in_the_cli_requirements
    revision: 7adc35323f48643b94e5d82f88e755f14d1f7502
    environment: local-macos-arm64
    content: 5bbc7880678e784498a781e7ab0ebbcbaf51c926b767ec1eada090347f295e41
    fingerprint: sha256:a0e638fed3c18d3ebd68bf3523a983fc851b528a28d3dd8691e496857263c98e
    case: TC-cli-send-back-wording-in-the-specs
    case_digest: 7fc001a83d34dbfa573b0b9dc5b005b74f43fe51fc753b687d196cdfeef53afa
    observed: 2026-11-12T14:21:03Z
    duration_seconds: 0
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t37f, test stage, area knowledge
related: []
created: 2026-11-12T01:38:48Z
updated: 2026-11-12T14:21:03Z
---
# The knowledge index links folders while REQ-cli-044 says every link resolves to an existing file

**What happened:** index.md links journal/, hi/, specs/, .specsync/archive/ and .specsync/archive/changes/ as REQ-cli-043 asks; Obsidian does not open folder links
Actual: REQ-cli-043 and REQ-cli-044 disagree

**Steps:** Open the index in Obsidian, or check links against REQ-cli-044's acceptance criterion

**Expected:** One consistent contract: file links, or the AC says file or folder

**Why it matters:** Folder links do nothing in Obsidian

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:48Z · Reported (t37f, test stage, area knowledge)
- 2026-11-12T14:19:45Z · status: open -> investigating. Red at 18c04614, after Leif's re-approval and before change check --commit: the cli requirements do not yet hold the approved wording (TC-cli-send-back-wording-in-the-specs).; evidence: OBS-tc-cli-send-back-wording-in-the-specs-673576ff2d3dc6e1e67499a8
- 2026-11-12T14:21:03Z · status: investigating -> verified. Green at 7adc3532, after change check --commit materialized the approved deltas on SpecSync 6.3.0: TC-cli-send-back-wording-in-the-specs passed.; evidence: OBS-tc-cli-send-back-wording-in-the-specs-c38beba2a983bdab7eedeb68
