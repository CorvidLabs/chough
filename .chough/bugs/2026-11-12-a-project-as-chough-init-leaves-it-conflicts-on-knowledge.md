---
kind: bug
id: 2026-11-12-a-project-as-chough-init-leaves-it-conflicts-on-knowledge
aliases:
  - BUG-a-project-as-chough-init-leaves-it-conflicts-on-knowledge-index-md-and-4390e553d9921f2d81c48108
title: A project as chough init leaves it conflicts on knowledge/index.md and feed.xml at every merge of two finished sessions
status: verified
classification: requirement-gap
scope: blocking
impact: Every project that finishes sessions on two branches hits the conflict
criteria: []
requirements: [REQ-cli-047]
cases: []
evidence:
  - OBS-tc-cli-init-sets-up-merge-drivers-c80aae4f9ae0fc0fd1582321
  - OBS-tc-cli-init-appends-only-missing-merge-lines-d913104da5e7419d149c5f81
  - OBS-tc-cli-doctor-merge-drivers-94c47290140fd3dc5794d767
  - OBS-tc-cli-init-sets-up-merge-drivers-5e4791b139c2cb1c5763f531
  - OBS-tc-cli-init-sets-up-merge-drivers-e131a616df6317747142b432
  - OBS-tc-cli-init-appends-only-missing-merge-lines-fea655d42dffdbafdca2eb18
  - OBS-tc-history-doctor-merge-drivers-aa529b8d7136855b732daadc
  - OBS-tc-cli-doctor-merge-drivers-071d776393914339fae22380
receipts:
  - id: 2026-11-12-tc-cli-init-sets-up-merge-drivers
    aliases: OBS-tc-cli-init-sets-up-merge-drivers-c80aae4f9ae0fc0fd1582321
    claim: "Ran cargo test --locked --test journal -- --exact init_sets_up_the_merge_drivers_so_two_branches_merge_clean for TC-cli-init-sets-up-merge-drivers: failed (exit 101)."
    result: failed
    command: cargo test --locked --test journal -- --exact init_sets_up_the_merge_drivers_so_two_branches_merge_clean
    revision: 6ee5ec3769b5f5c3bdddb86e1071fee01e89585f
    environment: local-macos-arm64
    content: 33380448c306a25c8982205bf364cead8a95f3b442fbebf544628e83bab69663
    fingerprint: sha256:87c7b4a59b716ce0064fffd7cd31597cf9ad213595918e458d78eb29473664e6
    case: TC-cli-init-sets-up-merge-drivers
    case_digest: 35bb076ad219ff2b67e13f42b7c1c91d52dc10d755b6cd304bdf53d0d3d8db81
    observed: 2026-11-12T06:57:04Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-init-appends-only-missing-merge-lines
    aliases: OBS-tc-cli-init-appends-only-missing-merge-lines-d913104da5e7419d149c5f81
    claim: "Ran cargo test --locked --test journal -- --exact init_appends_only_the_missing_merge_lines_and_keeps_a_set_driver for TC-cli-init-appends-only-missing-merge-lines: failed (exit 101)."
    result: failed
    command: cargo test --locked --test journal -- --exact init_appends_only_the_missing_merge_lines_and_keeps_a_set_driver
    revision: 6ee5ec3769b5f5c3bdddb86e1071fee01e89585f
    environment: local-macos-arm64
    content: 33380448c306a25c8982205bf364cead8a95f3b442fbebf544628e83bab69663
    fingerprint: sha256:f1fe279f79ab9ea1d029bbedf5d5ab689d786545208f28c1eb29f8e8ffbf325c
    case: TC-cli-init-appends-only-missing-merge-lines
    case_digest: e99fee83b52391f8ff8cc35dc97478249cfac20cd69748b524748c2e935e623c
    observed: 2026-11-12T06:57:05Z
    duration_seconds: 0
  - id: 2026-11-12-tc-cli-doctor-merge-drivers
    aliases: OBS-tc-cli-doctor-merge-drivers-94c47290140fd3dc5794d767
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_reports_missing_merge_drivers_and_fix_sets_them for TC-cli-doctor-merge-drivers: failed (exit 101)."
    result: failed
    command: cargo test --locked --test doctor -- --exact doctor_reports_missing_merge_drivers_and_fix_sets_them
    revision: 6ee5ec3769b5f5c3bdddb86e1071fee01e89585f
    environment: local-macos-arm64
    content: 33380448c306a25c8982205bf364cead8a95f3b442fbebf544628e83bab69663
    fingerprint: sha256:5fc41e78e6315a31eeb8fae0902f60a756c78dc8ae572841b9211d523f85a5dd
    case: TC-cli-doctor-merge-drivers
    case_digest: 404821704c148e6b83f611b02dc7416c9a71341643e75205ed5e331431018046
    observed: 2026-11-12T06:57:07Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-init-sets-up-merge-drivers-b657dd
    aliases: OBS-tc-cli-init-sets-up-merge-drivers-5e4791b139c2cb1c5763f531
    claim: "Ran cargo test --locked --test journal -- --exact init_sets_up_the_merge_drivers_so_two_branches_merge_clean for TC-cli-init-sets-up-merge-drivers: failed (exit 101)."
    result: failed
    command: cargo test --locked --test journal -- --exact init_sets_up_the_merge_drivers_so_two_branches_merge_clean
    revision: 6ee5ec3769b5f5c3bdddb86e1071fee01e89585f
    environment: local-macos-arm64
    content: 33380448c306a25c8982205bf364cead8a95f3b442fbebf544628e83bab69663
    fingerprint: sha256:e1258d99d71f80d4b554153a931cc6313d44598c04ea74c73bbbe88678c0c951
    case: TC-cli-init-sets-up-merge-drivers
    case_digest: 35bb076ad219ff2b67e13f42b7c1c91d52dc10d755b6cd304bdf53d0d3d8db81
    observed: 2026-11-12T06:58:23Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-init-sets-up-merge-drivers-9a6a63
    aliases: OBS-tc-cli-init-sets-up-merge-drivers-e131a616df6317747142b432
    claim: "Ran cargo test --locked --test journal -- --exact init_sets_up_the_merge_drivers_so_two_branches_merge_clean for TC-cli-init-sets-up-merge-drivers: passed (exit 0)."
    result: passed
    command: cargo test --locked --test journal -- --exact init_sets_up_the_merge_drivers_so_two_branches_merge_clean
    revision: 8091890cb874cf2bf33f8d5e3a6467d3bbee0dbb
    environment: local-macos-arm64
    content: 0d709ba899b8dfcf2dc898f29ca4dbb97df391dcac2ed6abe29f19ea553c235d
    fingerprint: sha256:8b2e1549d3ee9560e71e937130d9a186be2445032ce719824b87ab73dc5c70a6
    case: TC-cli-init-sets-up-merge-drivers
    case_digest: 35bb076ad219ff2b67e13f42b7c1c91d52dc10d755b6cd304bdf53d0d3d8db81
    observed: 2026-11-12T06:59:14Z
    duration_seconds: 2
  - id: 2026-11-12-tc-cli-init-appends-only-missing-merge-lines-5a3a4a
    aliases: OBS-tc-cli-init-appends-only-missing-merge-lines-fea655d42dffdbafdca2eb18
    claim: "Ran cargo test --locked --test journal -- --exact init_appends_only_the_missing_merge_lines_and_keeps_a_set_driver for TC-cli-init-appends-only-missing-merge-lines: passed (exit 0)."
    result: passed
    command: cargo test --locked --test journal -- --exact init_appends_only_the_missing_merge_lines_and_keeps_a_set_driver
    revision: 8091890cb874cf2bf33f8d5e3a6467d3bbee0dbb
    environment: local-macos-arm64
    content: 0d709ba899b8dfcf2dc898f29ca4dbb97df391dcac2ed6abe29f19ea553c235d
    fingerprint: sha256:705ed6bfe1e79c7b1c7f199505c0e9fb3e7c8175942b977e1ceb85d0ecb379f6
    case: TC-cli-init-appends-only-missing-merge-lines
    case_digest: e99fee83b52391f8ff8cc35dc97478249cfac20cd69748b524748c2e935e623c
    observed: 2026-11-12T06:59:15Z
    duration_seconds: 0
  - id: 2026-11-12-tc-history-doctor-merge-drivers
    aliases: OBS-tc-history-doctor-merge-drivers-aa529b8d7136855b732daadc
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_reports_missing_merge_drivers_and_fix_sets_them for TC-history-doctor-merge-drivers: passed (exit 0)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_reports_missing_merge_drivers_and_fix_sets_them
    revision: 8091890cb874cf2bf33f8d5e3a6467d3bbee0dbb
    environment: local-macos-arm64
    content: 0d709ba899b8dfcf2dc898f29ca4dbb97df391dcac2ed6abe29f19ea553c235d
    fingerprint: sha256:f75b13e95a5f78d3f0f0daa210e3c332fef8d6145c533f3901053a8732415c5f
    case: TC-history-doctor-merge-drivers
    case_digest: 01ad5bd1b1a177966cd19f484cf53fb2f1a5a669ca89ce9381135a430c451ded
    observed: 2026-11-12T06:59:19Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-doctor-merge-drivers-dacc42
    aliases: OBS-tc-cli-doctor-merge-drivers-071d776393914339fae22380
    claim: "Ran cargo test --locked --test doctor -- --exact doctor_reports_missing_merge_drivers_and_fix_sets_them for TC-cli-doctor-merge-drivers: passed (exit 0)."
    result: passed
    command: cargo test --locked --test doctor -- --exact doctor_reports_missing_merge_drivers_and_fix_sets_them
    revision: 8091890cb874cf2bf33f8d5e3a6467d3bbee0dbb
    environment: local-macos-arm64
    content: 0d709ba899b8dfcf2dc898f29ca4dbb97df391dcac2ed6abe29f19ea553c235d
    fingerprint: sha256:f1fd5377ecac4848c851368d255f30bcbc71e3bd664b53b318e9fb550f20f2f5
    case: TC-cli-doctor-merge-drivers
    case_digest: 404821704c148e6b83f611b02dc7416c9a71341643e75205ed5e331431018046
    observed: 2026-11-12T06:59:17Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t37f, test stage, area knowledge
related: []
created: 2026-11-12T01:38:48Z
updated: 2026-11-12T06:59:41Z
---
# A project as chough init leaves it conflicts on knowledge/index.md and feed.xml at every merge of two finished sessions

**What happened:** Two branches that each add a journal post, or each finish a session, conflict on knowledge/index.md and knowledge/journal/feed.xml; the merged feed holds conflict markers and is not XML
Actual: init writes no .gitattributes lines and configures no merge driver, and a merge= attribute naming an undefined driver falls back to Git's text merge; AGENTS.md here asks only for the bug driver

**Steps:** chough init, then a journal post on each of two branches, then git merge

**Expected:** No hand-resolved conflict in a file Chough writes (REQ-cli-047, DRIVE-12)

**Why it matters:** Every project that finishes sessions on two branches hits the conflict

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:48Z · Reported (t37f, test stage, area knowledge)
- 2026-11-12T06:57:33Z · status: open -> investigating. Red at 6ee5ec37 (TC-cli-init-sets-up-merge-drivers, before its fix, SpecSync 6.3.0 on PATH): failed for this bug's reason; fix in 78f9a77b.; evidence: OBS-tc-cli-init-sets-up-merge-drivers-c80aae4f9ae0fc0fd1582321
- 2026-11-12T06:57:33Z · Red at 6ee5ec37 (TC-cli-init-appends-only-missing-merge-lines, before its fix, SpecSync 6.3.0 on PATH): failed for this bug's reason; fix in 78f9a77b.; evidence: OBS-tc-cli-init-appends-only-missing-merge-lines-d913104da5e7419d149c5f81
- 2026-11-12T06:57:34Z · Red at 6ee5ec37 (TC-cli-doctor-merge-drivers, before its fix, SpecSync 6.3.0 on PATH): failed for this bug's reason; fix in 78f9a77b.; evidence: OBS-tc-cli-doctor-merge-drivers-94c47290140fd3dc5794d767
- 2026-11-12T06:58:33Z · Red again at 6ee5ec37 with its test target prebuilt: TC-cli-init-sets-up-merge-drivers failed for this bug's reason. It replaces OBS-tc-cli-init-sets-up-merge-drivers-c80aae4f9ae0fc0fd1582321, whose committed log held a local scratch path from a cargo Compiling line; that path is now redacted in place (REQ-records-006), so the old log no longer matches its recorded digest.; evidence: OBS-tc-cli-init-sets-up-merge-drivers-5e4791b139c2cb1c5763f531
- 2026-11-12T06:59:41Z · Green at 8091890c on 6.3.0: TC-cli-init-sets-up-merge-drivers passed.; evidence: OBS-tc-cli-init-sets-up-merge-drivers-e131a616df6317747142b432
- 2026-11-12T06:59:41Z · Green at 8091890c on 6.3.0: TC-cli-init-appends-only-missing-merge-lines passed.; evidence: OBS-tc-cli-init-appends-only-missing-merge-lines-fea655d42dffdbafdca2eb18
- 2026-11-12T06:59:41Z · Green at 8091890c on 6.3.0: TC-history-doctor-merge-drivers passed.; evidence: OBS-tc-history-doctor-merge-drivers-aa529b8d7136855b732daadc
- 2026-11-12T06:59:41Z · status: investigating -> verified. Green at 8091890c: TC-cli-doctor-merge-drivers passed; full suite 638 passed, clippy and fmt clean on SpecSync 6.3.0 at 596ced67; t38b's black-box retest passed.; evidence: OBS-tc-cli-doctor-merge-drivers-071d776393914339fae22380
