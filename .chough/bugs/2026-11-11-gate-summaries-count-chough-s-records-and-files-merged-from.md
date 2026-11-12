---
kind: bug
id: 2026-11-11-gate-summaries-count-chough-s-records-and-files-merged-from
aliases:
  - BUG-gate-summaries-count-chough-s-records-and-files-merged-from-main-190-d4703cc336a1f7af8f4b2787
title: Gate summaries count Chough's records and files merged from main (#190)
status: verified
classification: product-defect
scope: in-scope
impact: "The human deciding a gate cannot see what the change touched (GitHub #190)"
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-061]
cases: [TC-cli-190-gate-summary-counts-the-change]
evidence:
  - OBS-tc-cli-190-gate-summary-counts-the-change-99cf6eeb353b024999329d9c
  - OBS-tc-cli-190-gate-summary-counts-the-change-a4077f7200f7c230117b9e16
receipts:
  - id: 2026-11-11-tc-cli-190-gate-summary-counts-the-change
    aliases: OBS-tc-cli-190-gate-summary-counts-the-change-99cf6eeb353b024999329d9c
    claim: "Ran cargo test --locked --test gates -- --exact gate_summary_counts_only_the_changes_own_files for TC-cli-190-gate-summary-counts-the-change: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test gates -- --exact gate_summary_counts_only_the_changes_own_files
    revision: 603456f3a22a5ff926090e40e81bbca2e4b2a21c
    environment: local-macos-arm64
    content: 4b690ed0cf4105249ce2089c70c61b0744e8550f3401d507e59fc0067e9ba6e2
    fingerprint: sha256:ec8b28006f25073fdfd8de4a6d30f52f3a2029b023bef4e52b25ea4a0ec58653
    case: TC-cli-190-gate-summary-counts-the-change
    case_digest: 087d32b333cbca67a875dfa17244ee8a3a29082ac164da481a66c35093023750
    observed: 2026-11-11T19:19:36Z
    duration_seconds: 8
  - id: 2026-11-11-tc-cli-190-gate-summary-counts-the-change-066dce
    aliases: OBS-tc-cli-190-gate-summary-counts-the-change-a4077f7200f7c230117b9e16
    claim: "Ran cargo test --locked --test gates -- --exact gate_summary_counts_only_the_changes_own_files for TC-cli-190-gate-summary-counts-the-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact gate_summary_counts_only_the_changes_own_files
    revision: f9c0ae674c89f3a4c4a6c5b9d29057f22f7b9bf0
    environment: local-macos-arm64
    content: 4adc6e184b41d042b73284db8f143439ad1bd2bdc6fc389e3a5266068b430ba2
    fingerprint: sha256:03f2836295b24ac2df23052bd2e1e22b0f39637d912fc92e4444583db7623141
    case: TC-cli-190-gate-summary-counts-the-change
    case_digest: 087d32b333cbca67a875dfa17244ee8a3a29082ac164da481a66c35093023750
    observed: 2026-11-11T19:42:49Z
    duration_seconds: 2
environment: local-macos-arm64
revision: b27a2c9a9c687b01899de6878b062d8e1d298235
found_by: "GitHub issue #190; red on 0.5.0 at leif/0.6.0-190-red 603456f3"
related: []
created: 2026-11-11T19:20:02Z
updated: 2026-11-11T19:43:31Z
---
# Gate summaries count Chough's records and files merged from main (#190)

**What happened:** a gate summary's What changed line counts every Chough record, knowledge note and file merged from main since the change's base commit
Actual: 509 files, with Chough's evidence entries among the top five

**Steps:** Make a one-line product edit, record evidence and knowledge, merge main into the branch, then chough gate list
1. chough session new; chough checkpoint --native-change <id>
2. one product edit; 300 records under .chough/ and knowledge/; 200 files on main merged in
3. chough gate list --json

**Expected:** the summary counts from the merge base with the default branch and leaves out .chough/, the knowledge directory and SpecSync bookkeeping: one file, named

**Why it matters:** The human deciding a gate cannot see what the change touched (GitHub #190)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T19:20:02Z · Reported (GitHub issue #190; red on 0.5.0 at leif/0.6.0-190-red 603456f3); evidence: OBS-tc-cli-190-gate-summary-counts-the-change-99cf6eeb353b024999329d9c
- 2026-11-11T19:43:31Z · status: open -> verified. Fixed in plan step 12 (PR #238, 657dc532); the case passed after its 0.5.0 failure.; evidence: OBS-tc-cli-190-gate-summary-counts-the-change-a4077f7200f7c230117b9e16
