---
kind: bug
id: 2026-11-02-stage-references-and-installed-workflows-contradict-agent
aliases:
  - BUG-stage-references-and-installed-workflows-contradict-agent-authority-gui-17c8cf5e8275ded08fcacc38
title: Stage references and installed workflows contradict agent authority guidance
status: verified
classification: product-defect
scope: blocking
impact: Blocks the requested conversation-to-PR workflow consistency
criteria: [FLOW-5, LIFE-6]
requirements: [REQ-cli-029]
cases: []
evidence:
  - OBS-product-test-ebe8dcc6161354afe79b4f95
  - OBS-product-test-fa1399785a5046d25484052c
  - OBS-product-test-42eaa88e1ebc9f69e4d357b5
receipts:
  - id: 2026-11-02-the-structural-cross-bundle-native-writer-and-installed-copy
    aliases: OBS-product-test-ebe8dcc6161354afe79b4f95
    claim: "The structural cross-bundle native-writer and installed-copy regression fails: plan/accept command examples still use human actor/reviewer placeholders, and both tracked host installations retain stale SKILL/lifecycle guidance."
    result: failed
    revision: 7cd739219ff667dcf6d153a6f439d6c3661b1fce
    environment: local-macos-arm64
    content: 4cba4165d14bb58ee296fcc29fcf2a20ba04e04cc79ceab2854800b173c3e458
    fingerprint: sha256:fa0d71adff195ff6668a8b04377edb02f27eeacf9bfe626c05e694d717b4d326
    observed: 2026-11-02T17:34:02Z
  - id: 2026-11-02-both-workflow-regressions-pass-structural-native-writer-actor
    aliases: OBS-product-test-fa1399785a5046d25484052c
    claim: "Both workflow regressions pass: structural native-writer actor/reviewer argument checks cover canonical Markdown and README, all4guidance files match both tracked host copies, and exact4file predecessor upgrades preserve all edited variants. Safe updater classified all target copies known-outdated before refresh."
    result: passed
    revision: 7cd739219ff667dcf6d153a6f439d6c3661b1fce
    environment: local-macos-arm64
    content: dac563ce07a09bd197690a74b94a9efba1e6355fbda2f90de28b878f6337ab13
    fingerprint: sha256:5068309b809e65668c0472e77137450a8deafadaef2b2e4f566765b070a60de9
    observed: 2026-11-02T17:40:52Z
  - id: OBS-product-test-42eaa88e1ebc9f69e4d357b5
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
environment: local-macos-arm64
revision: 7cd739219ff667dcf6d153a6f439d6c3661b1fce
found_by: PR44 peer review and failing structural package regression
related: []
created: 2026-11-02T17:34:03Z
updated: 2026-11-02T17:47:39Z
---
# Stage references and installed workflows contradict agent authority guidance

**What happened:** The same shipped workflow has conflicting native actor and approval instructions
Actual: Human actor/reviewer placeholders and stale installed SKILL/lifecycle copies fail the cross-bundle regression

**Steps:** Loading plan or accept references or a tracked host installation after reading the updated main skill

**Expected:** All native-writer examples use the acting agent account and installed copies match canonical guidance while preserving edits

**Why it matters:** Blocks the requested conversation-to-PR workflow consistency

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T17:34:03Z · Reported (PR44 peer review and failing structural package regression); evidence: OBS-product-test-ebe8dcc6161354afe79b4f95
- 2026-11-02T17:40:52Z · status: open -> verified. Plan applies existing authority before asking; native examples use agent accounts; exact old copies update and all tracked target host files match canonical bytes. Structural regression catches the earlier conflicting argument and stale-copy wiring.; evidence: OBS-product-test-fa1399785a5046d25484052c
- 2026-11-02T17:47:39Z · Complete198test verification and both bound cases pass after the cross-bundle reference and installed-copy correction.; evidence: OBS-product-test-42eaa88e1ebc9f69e4d357b5
