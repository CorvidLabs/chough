---
kind: bug
id: 2026-11-07-approved-prune-requirement-delta-leaves-split-digest-stale
aliases: [BUG-approved-prune-requirement-delta-leaves-split-digest-stale-51af3f652531624b8e771636]
title: Approved prune requirement delta leaves split digest stale
status: verified
classification: evidence-gap
scope: blocking
impact: Not yet assessed.
criteria: [LIFE-7]
requirements: [REQ-history-004]
cases: []
evidence: [OBS-product-test-f87bec1f3802297bc62bebe7, OBS-product-test-1a8d3cd9e2fff7f17314bd35]
receipts:
  - id: 2026-11-07-fledge-run-linux-verify-at
    aliases: OBS-product-test-f87bec1f3802297bc62bebe7
    claim: "fledge run linux-verify at 7febd53a86d1a007565ff38638b41abd760c123f failed test step 3: moved_requirements_keep_their_wording_under_their_new_ids still pins old REQ-history-004 body; exact old/new SHA256 in log. No assertion changed."
    result: failed
    revision: 7febd53a86d1a007565ff38638b41abd760c123f
    environment: local-linux-aarch64
    content: 4c668870ced52a9c7a062d91efe5eab149c877b1b996c41748c2cf558353ab87
    fingerprint: sha256:a69c9b00e67effaeb197f7158f3ccf5b1505acad866afb29fbdce654ca6754d8
    observed: 2026-11-07T06:50:48Z
  - id: 2026-11-07-pinned-focused-validation-fledge-1-8-0-hi-0-8-0-specsync-6-2-0
    aliases: OBS-product-test-1a8d3cd9e2fff7f17314bd35
    claim: "Pinned focused validation: Fledge 1.8.0, HI 0.8.0, SpecSync 6.2.0, Rust 1.95.0. Build, all 4 spec_split tests, 11 evidence unit tests, 30 evidence integration tests, 25 history tests and global case-definition validation passed. The formerly failing strict REQ-history-004 assertion now passes with the exact approved digest."
    result: passed
    revision: 4ef6b8e29913077a6f557d1371a930540e02884b
    environment: local-macos-arm64
    content: 7357063ec647a2c8493d700dc99e2ebaa99d0f01043e3bd400bdacef867bc44e
    fingerprint: sha256:9dc22d60f02451b332ba4c856bd41f10b7e2a0211707384932c7bf882c123009
    observed: 2026-11-07T14:54:44Z
environment: local-linux-aarch64
revision: 7febd53a86d1a007565ff38638b41abd760c123f
found_by: Declared Linux verification at 7febd53a86d1a007565ff38638b41abd760c123f; scope amendment directed by orc t19f
related: []
created: 2026-11-07T06:51:16Z
updated: 2026-11-07T14:55:25Z
---
# Approved prune requirement delta leaves split digest stale

**What happened:** Linux verify fails the strict split-wording assertion for REQ-history-004 after the approved prune delta
Actual: tests/spec_split.rs expects 128c08a8a763ba404099891f17da8c28e53da365375a44decfd8755c78208fd2 but approved body computes d807b65dc985f3d8971b83fcb6c1e1cc4615beb80ca30dee2ca595e847acf0d2; test file omitted from affected_paths

**Steps:** Run fledge run linux-verify at 7febd53a86d1a007565ff38638b41abd760c123f

**Expected:** The assertion pins the exact body authorized by the approved native delta

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-07T06:51:16Z · Reported (Declared Linux verification at 7febd53a86d1a007565ff38638b41abd760c123f; scope amendment directed by orc t19f); evidence: OBS-product-test-f87bec1f3802297bc62bebe7
- 2026-11-07T14:55:25Z · status: open -> verified. Leif scope reapproval was recorded by orc t19f at 4ef6b8e29913077a6f557d1371a930540e02884b. Only the approved digest literal changed; all four strict spec_split tests now pass under pinned tools, with the failure receipt retained.; evidence: OBS-product-test-1a8d3cd9e2fff7f17314bd35
