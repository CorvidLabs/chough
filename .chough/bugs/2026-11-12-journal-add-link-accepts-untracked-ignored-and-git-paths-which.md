---
kind: bug
id: 2026-11-12-journal-add-link-accepts-untracked-ignored-and-git-paths-which
aliases:
  - BUG-journal-add-link-accepts-untracked-ignored-and-git-paths-which-are-dead-2adf94a75196d7f71102d782
title: journal add --link accepts untracked, ignored and .git paths, which are dead links in a clone
status: verified
classification: product-defect
scope: in-scope
impact: Posts carry links that resolve nowhere in a clone or on GitHub
criteria: []
requirements: [REQ-cli-044]
cases: []
evidence:
  - OBS-tc-cli-journal-link-tracked-only-e616203f178de12d5d50482e
  - OBS-tc-cli-journal-link-tracked-only-eb41d50cbdf2e80cee530a57
receipts:
  - id: 2026-11-12-tc-cli-journal-link-tracked-only
    aliases: OBS-tc-cli-journal-link-tracked-only-e616203f178de12d5d50482e
    claim: "Ran cargo test --locked --test journal -- --exact journal_links_name_only_project_paths_git_tracks for TC-cli-journal-link-tracked-only: failed (exit 101)."
    result: failed
    command: cargo test --locked --test journal -- --exact journal_links_name_only_project_paths_git_tracks
    revision: 17ad11b24ce40cb24f5043023f53f77e7ab9c503
    environment: local-macos-arm64
    content: 3768feef3984617a59ab27a6ce51c6212e86082d3c7951af175e30e71c29d322
    fingerprint: sha256:dc51433afa01324e739be23d3739463b006a83f617f53f5af82f27bcc8ae2ce8
    case: TC-cli-journal-link-tracked-only
    case_digest: 6a16a469d900a2899eb217e18cf2cc71098546afb157efd15aba0722cd4a08b2
    observed: 2026-11-12T02:02:28Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-journal-link-tracked-only-56eb8c
    aliases: OBS-tc-cli-journal-link-tracked-only-eb41d50cbdf2e80cee530a57
    claim: "Ran cargo test --locked --test journal -- --exact journal_links_name_only_project_paths_git_tracks for TC-cli-journal-link-tracked-only: passed (exit 0)."
    result: passed
    command: cargo test --locked --test journal -- --exact journal_links_name_only_project_paths_git_tracks
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:f166e19b0f5221702f5cda8d78d5fba951b467745e59715bd65d435c3b258890
    case: TC-cli-journal-link-tracked-only
    case_digest: 451a7f35dab817c6849491a01bcd18a19565ee2f7d3e887522a10447ab810519
    observed: 2026-11-12T02:17:42Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t37f, test stage, area knowledge
related: []
created: 2026-11-12T01:38:48Z
updated: 2026-11-12T02:18:08Z
---
# journal add --link accepts untracked, ignored and .git paths, which are dead links in a clone

**What happened:** Links to .chough/local files, .git/config and an untracked file are accepted and committed
Actual: Any path that exists on disk is accepted

**Steps:** chough journal add --link with a path that exists on disk but not in the repository

**Expected:** Every relative Markdown link in the vault resolves to an existing file in the repository (REQ-cli-044)

**Why it matters:** Posts carry links that resolve nowhere in a clone or on GitHub

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:48Z · Reported (t37f, test stage, area knowledge)
- 2026-11-12T02:02:52Z · status: open -> investigating. Red at 17ad11b2 (TC-cli-journal-link-tracked-only, on t37x's branch before its fix): failed for this bug's reason; fix in ec1f58b0.; evidence: OBS-tc-cli-journal-link-tracked-only-e616203f178de12d5d50482e
- 2026-11-12T02:18:08Z · status: investigating -> verified. Green at 0ee26ac0: TC-cli-journal-link-tracked-only passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-cli-journal-link-tracked-only-eb41d50cbdf2e80cee530a57
