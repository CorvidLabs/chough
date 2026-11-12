---
kind: bug
id: 2026-11-12-gate-list-prints-an-unusable-git-diff-unknown-sha-line-when
aliases:
  - BUG-gate-list-prints-an-unusable-git-diff-unknown-sha-line-when-there-is-no-bcb37c77b5e1ebc9ccd78e90
title: gate list prints an unusable 'git diff unknown..<sha>' line when there is no merge base or base commit
status: verified
classification: product-defect
scope: in-scope
impact: A reviewer is given a command that fails
criteria: []
requirements: [REQ-cli-061]
cases: []
evidence:
  - OBS-tc-cli-gate-summary-says-when-there-is-no-base-c6dd518c1d39631bc2c740da
  - OBS-tc-cli-gate-summary-says-when-there-is-no-base-247dc555ccebbb89715bfec1
receipts:
  - id: 2026-11-12-tc-cli-gate-summary-says-when-there-is-no-base
    aliases: OBS-tc-cli-gate-summary-says-when-there-is-no-base-c6dd518c1d39631bc2c740da
    claim: "Ran cargo test --locked --test gates -- --exact a_summary_with_no_base_says_so_instead_of_a_broken_diff_command for TC-cli-gate-summary-says-when-there-is-no-base: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test gates -- --exact a_summary_with_no_base_says_so_instead_of_a_broken_diff_command
    revision: f2625e5b38e1c5b52b75f6ce898504ff844383f9
    environment: local-macos-arm64
    content: 1aea8bdfa5b288c9662d6b19d6da52ba50c79e1f9211d8b171ae8c12c462799c
    fingerprint: sha256:974a44bd6e33bbbfde7fe06c89655baa6718cff609f8a54759aed608779519b0
    case: TC-cli-gate-summary-says-when-there-is-no-base
    case_digest: e789971eda848287157badb8ea5dbb03c250941cd4a27deca441c954baf1abf3
    observed: 2026-11-12T02:30:03Z
    duration_seconds: 4
  - id: 2026-11-12-tc-cli-gate-summary-says-when-there-is-no-base-2a2cd3
    aliases: OBS-tc-cli-gate-summary-says-when-there-is-no-base-247dc555ccebbb89715bfec1
    claim: "Ran cargo test --locked --test gates -- --exact a_summary_with_no_base_says_so_instead_of_a_broken_diff_command for TC-cli-gate-summary-says-when-there-is-no-base: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact a_summary_with_no_base_says_so_instead_of_a_broken_diff_command
    revision: 46a4cb691c1e339abc16d65e24d884ea0e10f54f
    environment: local-macos-arm64
    content: e01775b3355fc98a482ee0ce7077c21f65e48e6fe1aa51f00b2d25ed4efb7496
    fingerprint: sha256:dd82cb6ad8ad79f257f2cfb25ea0869afbb49008bad65430a473462977308062
    case: TC-cli-gate-summary-says-when-there-is-no-base
    case_digest: e789971eda848287157badb8ea5dbb03c250941cd4a27deca441c954baf1abf3
    observed: 2026-11-12T02:30:33Z
    duration_seconds: 2
environment: local-macos-arm64, tester build at 367dc86b, debug, specsync stub (6.2.0 CLI)
revision: 30ac761f13feef4cc3ac765ad077d30630e4650a
found_by: t37b, test stage, area approvals and gates
related: []
created: 2026-11-12T02:24:03Z
updated: 2026-11-12T02:30:33Z
---
# gate list prints an unusable 'git diff unknown..<sha>' line when there is no merge base or base commit

**What happened:** In a single-commit project the gate summary's Diff line is 'git diff unknown..<sha>'
Actual: The placeholder 'unknown' is pasted into a command

**Steps:** chough gate list in a project with no merge base and no recorded base commit

**Expected:** A compare URL or 'git diff <base>..<HEAD>' (REQ-cli-061), and an honest line when there is no base

**Why it matters:** A reviewer is given a command that fails

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T02:24:03Z · Reported (t37b, test stage, area approvals and gates)
- 2026-11-12T02:30:06Z · status: open -> investigating. Red at f2625e5b (leif/0.6.0-test1-red: d854ce1a plus the regression tests): the Diff line was git diff unknown..<sha>; fix in 08ccf14d.; evidence: OBS-tc-cli-gate-summary-says-when-there-is-no-base-c6dd518c1d39631bc2c740da
- 2026-11-12T02:30:33Z · status: investigating -> verified. Green at 46a4cb69: TC-cli-gate-summary-says-when-there-is-no-base passed; tests/gates.rs 8 of 8.; evidence: OBS-tc-cli-gate-summary-says-when-there-is-no-base-247dc555ccebbb89715bfec1
