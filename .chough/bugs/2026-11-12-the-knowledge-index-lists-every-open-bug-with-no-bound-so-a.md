---
kind: bug
id: 2026-11-12-the-knowledge-index-lists-every-open-bug-with-no-bound-so-a
aliases:
  - BUG-the-knowledge-index-lists-every-open-bug-with-no-bound-so-a-few-hundred-539000056d8d9cc6f09b429a
title: The knowledge index lists every open bug with no bound, so a few hundred open bugs put it over its byte bound for good
status: verified
classification: requirement-gap
scope: in-scope
impact: Large backlogs, such as spec-sync's, lose the index and fail the verify lane
criteria: []
requirements: [REQ-cli-043]
cases: []
evidence:
  - OBS-tc-cli-knowledge-index-bounded-open-bugs-ea0827c5ad2f87c839e961a5
  - OBS-tc-cli-knowledge-index-bounded-open-bugs-f9088b287fd3f64ad9fbbf8d
receipts:
  - id: 2026-11-12-tc-cli-knowledge-index-bounded-open-bugs
    aliases: OBS-tc-cli-knowledge-index-bounded-open-bugs-ea0827c5ad2f87c839e961a5
    claim: "Ran cargo test --locked --test vault -- --exact the_index_lists_at_most_twenty_open_bugs_blocking_first_then_newest for TC-cli-knowledge-index-bounded-open-bugs: failed (exit 101)."
    result: failed
    command: cargo test --locked --test vault -- --exact the_index_lists_at_most_twenty_open_bugs_blocking_first_then_newest
    revision: 3d45f137d8ca45912b33f2492362b4539bd21b56
    environment: local-macos-arm64
    content: 5d380164742d95eb5e2bdb7923ef31940a39119a904ebd6bb55233c92911d63d
    fingerprint: sha256:d88a28be0df1175fda5107fd39b71c903d85868ed31beb11b49963029d386821
    case: TC-cli-knowledge-index-bounded-open-bugs
    case_digest: 3cfa031e4847dba90e508ba9a9f6e6b87c24c88e6eb45d6e222ce0ac0b8a23bb
    observed: 2026-11-12T06:57:12Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-knowledge-index-bounded-open-bugs-55fec1
    aliases: OBS-tc-cli-knowledge-index-bounded-open-bugs-f9088b287fd3f64ad9fbbf8d
    claim: "Ran cargo test --locked --test vault -- --exact the_index_lists_at_most_twenty_open_bugs_blocking_first_then_newest for TC-cli-knowledge-index-bounded-open-bugs: passed (exit 0)."
    result: passed
    command: cargo test --locked --test vault -- --exact the_index_lists_at_most_twenty_open_bugs_blocking_first_then_newest
    revision: 8091890cb874cf2bf33f8d5e3a6467d3bbee0dbb
    environment: local-macos-arm64
    content: 0d709ba899b8dfcf2dc898f29ca4dbb97df391dcac2ed6abe29f19ea553c235d
    fingerprint: sha256:4788bd8e4fce6a3ec50ad4f3d9a8b8cea90babc13b0f847a904069d6e27dc6d5
    case: TC-cli-knowledge-index-bounded-open-bugs
    case_digest: 3cfa031e4847dba90e508ba9a9f6e6b87c24c88e6eb45d6e222ce0ac0b8a23bb
    observed: 2026-11-12T06:59:22Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t37f, test stage, area knowledge
related: []
created: 2026-11-12T01:38:48Z
updated: 2026-11-12T06:59:41Z
---
# The knowledge index lists every open bug with no bound, so a few hundred open bugs put it over its byte bound for good

**What happened:** With about 1,500 open bugs, library index and --check exit 1 naming the 65536-byte bound, init refuses the vault, and the move into done writes a post into a vault with no index
Actual: Each open bug adds about 230 bytes; from roughly 270 the index cannot be built

**Steps:** A project with a few hundred open bugs with long titles

**Expected:** The index stays within its bound (REQ-cli-043) however many bugs are open

**Why it matters:** Large backlogs, such as spec-sync's, lose the index and fail the verify lane

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:48Z · Reported (t37f, test stage, area knowledge)
- 2026-11-12T06:57:34Z · status: open -> investigating. Red at 3d45f137 (TC-cli-knowledge-index-bounded-open-bugs, before its fix, SpecSync 6.3.0 on PATH): failed for this bug's reason; fix in 9c6f7552.; evidence: OBS-tc-cli-knowledge-index-bounded-open-bugs-ea0827c5ad2f87c839e961a5
- 2026-11-12T06:59:41Z · status: investigating -> verified. Green at 8091890c: TC-cli-knowledge-index-bounded-open-bugs passed; full suite 638 passed, clippy and fmt clean on SpecSync 6.3.0 at 596ced67; t38b's black-box retest passed.; evidence: OBS-tc-cli-knowledge-index-bounded-open-bugs-f9088b287fd3f64ad9fbbf8d
