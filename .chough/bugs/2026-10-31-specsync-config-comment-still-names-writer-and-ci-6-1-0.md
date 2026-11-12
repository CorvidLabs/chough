---
kind: bug
id: 2026-10-31-specsync-config-comment-still-names-writer-and-ci-6-1-0
aliases: [BUG-0001]
title: SpecSync config comment still names writer and CI 6.1.0
status: verified
classification: evidence-gap
scope: out-of-scope
impact: Misleading comment only; no behavior or gate depends on it. Fixing it needs a native change that covers .specsync/config.toml, which the approved scope does not.
criteria: [LIFE-6]
requirements: [REQ-cli-003]
cases: []
evidence:
  - OBS-tc-cli-0110-d401e774e0492b43be1a2895
  - OBS-tc-cli-0110-6f332c7ddda57139d7373d58
  - OBS-tc-cli-0110-246ea544b0723c91fb40903a
receipts:
  - id: 2026-11-11-tc-cli-0110
    aliases: OBS-tc-cli-0110-d401e774e0492b43be1a2895
    claim: "Ran cargo test --locked --test package -- --exact versions_agree_across_readme_agents_ci_and_specsync_config for TC-cli-0110: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact versions_agree_across_readme_agents_ci_and_specsync_config
    revision: 504cecba6393f6b629f83a36ec3847842055da83
    environment: local-macos-arm64
    content: 879f7097ade0f326c1c45b5faa86603a11ce283581e3856ab68bb56a08f1e93a
    fingerprint: sha256:a2a3cd32a32cb455e09c2e60ea05a1af96452e160121aaa027b0ca9b7f88bb7a
    case: TC-cli-0110
    case_digest: fb271ed0d2aa1ab6eff085b69636c8703ba2092bd1042f98e878166dbf8cc726
    observed: 2026-11-11T20:16:13Z
    duration_seconds: 0
  - id: 2026-11-11-tc-cli-0110-493a97
    aliases: OBS-tc-cli-0110-6f332c7ddda57139d7373d58
    claim: "Ran cargo test --locked --test package -- --exact versions_agree_across_readme_agents_ci_and_specsync_config for TC-cli-0110: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test package -- --exact versions_agree_across_readme_agents_ci_and_specsync_config
    revision: 370d221d68840f2e263e90b927a200438a7a0173
    environment: local-macos-arm64
    content: 843a083f60a67703f38514e1fcffa147b0e3c500ec307ca879614ca5ee672a4c
    fingerprint: sha256:67e1e55ef211d36911aa2e8ad112d3d5b84c70be77355466f60fc8a6ae1a4206
    case: TC-cli-0110
    case_digest: fb271ed0d2aa1ab6eff085b69636c8703ba2092bd1042f98e878166dbf8cc726
    observed: 2026-11-11T20:36:09Z
    duration_seconds: 0
  - id: 2026-11-11-tc-cli-0110-efca69
    aliases: OBS-tc-cli-0110-246ea544b0723c91fb40903a
    claim: "Ran cargo test --locked --test package -- --exact versions_agree_across_readme_agents_ci_and_specsync_config for TC-cli-0110: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact versions_agree_across_readme_agents_ci_and_specsync_config
    revision: 7b4cd3eb827f70c821255c0efb0326f17d7ca23a
    environment: local-macos-arm64
    content: e287cef970a2e70b5d716af6bc20f127df4d7275ee92871b760314f7393345e9
    fingerprint: sha256:db0af1b88d6f30119f621028039893288f51ae8fb19eaea976c2fe0e2bba57dc
    case: TC-cli-0110
    case_digest: fb271ed0d2aa1ab6eff085b69636c8703ba2092bd1042f98e878166dbf8cc726
    observed: 2026-11-11T20:39:30Z
    duration_seconds: 1
environment: repository-text
revision: 12c8d4009995e13a42a354e4bc59f641ad9d0dfb
found_by: specsync change audit on feat/resumable-stage-workflow
related: []
created: 2026-10-31T18:59:09Z
updated: 2026-11-11T20:39:43Z
---
# SpecSync config comment still names writer and CI 6.1.0

**What happened:** The header comment in .specsync/config.toml says writer/CI 6.1.0 while lifecycle writers and CI use SpecSync 6.1.1
Actual: writer/CI: 6.1.0

**Steps:** Reading .specsync/config.toml after the 6.1.1 alignment (D-009)
1. Read the first line of .specsync/config.toml
2. Compare with .github/workflows/ci.yml and docs/DECISIONS.md D-009

**Expected:** The comment names the version actually used, or no version

**Why it matters:** Misleading comment only; no behavior or gate depends on it. Fixing it needs a native change that covers .specsync/config.toml, which the approved scope does not.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T18:59:09Z · Reported (specsync change audit on feat/resumable-stage-workflow)
- 2026-11-11T20:17:14Z · Fixed in plan step 12 (PR #238, t38b): the SpecSync config comment names 6.2.0, HI 0.8.0 and Fledge 1.8.0 and joins the version check. Passed here; the red run on 0.5.0 is to be recorded before verification.; evidence: OBS-tc-cli-0110-d401e774e0492b43be1a2895
- 2026-11-11T20:36:57Z · Red on 0.5.0 (red on 0.5.0 at leif/0.6.0-batch-red 370d221d): the SpecSync config comment names only SpecSync's pin, not HI's and Fledge's, so the pin check cannot read it.; evidence: OBS-tc-cli-0110-6f332c7ddda57139d7373d58
- 2026-11-11T20:39:43Z · status: open -> verified. Fixed in plan step 12 (PR #238, dc0a6e5d); the case passed after its 0.5.0 failure.; evidence: OBS-tc-cli-0110-246ea544b0723c91fb40903a
