---
kind: bug
id: 2026-11-12-a-migrated-bug-does-not-answer-to-its-0-5-x-alias-in-bug-list
aliases:
  - BUG-a-migrated-bug-does-not-answer-to-its-0-5-x-alias-in-bug-list-or-bug-se-348e392ec63519f8f1e69d95
title: A migrated bug does not answer to its 0.5.x alias in bug list or bug search
status: verified
classification: product-defect
scope: in-scope
impact: A person holding an old bug ID cannot find it from list or search
criteria: []
requirements: [REQ-cli-012]
cases: []
evidence:
  - OBS-tc-cli-bug-alias-list-and-search-3ea8b1c5417343048199f77a
  - OBS-tc-cli-bug-alias-list-and-search-65505ed225fd2c4a099481ed
receipts:
  - id: 2026-11-12-tc-cli-bug-alias-list-and-search
    aliases: OBS-tc-cli-bug-alias-list-and-search-3ea8b1c5417343048199f77a
    claim: "Ran cargo test --locked --test bugs -- --exact a_migrated_bug_answers_to_its_alias_in_bug_list_and_bug_search for TC-cli-bug-alias-list-and-search: failed (exit 101)."
    result: failed
    command: cargo test --locked --test bugs -- --exact a_migrated_bug_answers_to_its_alias_in_bug_list_and_bug_search
    revision: fbe098cd023fcb95c18b8ebe931b2a4afc3480c4
    environment: local-macos-arm64
    content: 48e83de38e91250577a8d05d5a06483e6dcd3d15369b64727951ad2212081a32
    fingerprint: sha256:9289695ebb2b6e0d92184774b851c7474cbc3aa7ff590f5cec572a095dbdbe2b
    case: TC-cli-bug-alias-list-and-search
    case_digest: 26070a4cb3e855aa34d460303b6f5e6c83acbd3f8f46dfdfaa0a1b8ee326ad37
    observed: 2026-11-12T02:02:16Z
    duration_seconds: 0
  - id: 2026-11-12-tc-cli-bug-alias-list-and-search-061fc4
    aliases: OBS-tc-cli-bug-alias-list-and-search-65505ed225fd2c4a099481ed
    claim: "Ran cargo test --locked --test bugs -- --exact a_migrated_bug_answers_to_its_alias_in_bug_list_and_bug_search for TC-cli-bug-alias-list-and-search: passed (exit 0)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_migrated_bug_answers_to_its_alias_in_bug_list_and_bug_search
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:d5e04237a60d53b60c109b2372983a18a6d4aaef902319e76da06586266e4558
    case: TC-cli-bug-alias-list-and-search
    case_digest: 26070a4cb3e855aa34d460303b6f5e6c83acbd3f8f46dfdfaa0a1b8ee326ad37
    observed: 2026-11-12T02:17:37Z
    duration_seconds: 0
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 18025ad47c730af7589317f76608634b2487896f
found_by: t37j, test stage, areas sessions and bugs
related: []
created: 2026-11-12T01:41:12Z
updated: 2026-11-12T02:18:08Z
---
# A migrated bug does not answer to its 0.5.x alias in bug list or bug search

**What happened:** bug list --all shows no aliases, and bug search for the BUG-NNNN alias returns no match, while show, update and report by the alias work
Actual: list and search ignore aliases

**Steps:** A bug whose front matter has aliases [BUG-0007], then chough bug search BUG-0007 or bug list --all

**Expected:** A migrated bug answers to its BUG-NNNN alias in list, search, show, update and report (REQ-cli-012)

**Why it matters:** A person holding an old bug ID cannot find it from list or search

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:41:12Z · Reported (t37j, test stage, areas sessions and bugs)
- 2026-11-12T02:02:52Z · status: open -> investigating. Red at fbe098cd (TC-cli-bug-alias-list-and-search, on t37x's branch before its fix): failed for this bug's reason; fix in b3714dd3.; evidence: OBS-tc-cli-bug-alias-list-and-search-3ea8b1c5417343048199f77a
- 2026-11-12T02:18:08Z · status: investigating -> verified. Green at 0ee26ac0: TC-cli-bug-alias-list-and-search passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-cli-bug-alias-list-and-search-65505ed225fd2c4a099481ed
