---
kind: bug
id: 2026-11-01-initial-host-tools-do-not-match-ci-versions
aliases: [BUG-0022]
title: Initial host tools do not match CI versions
status: verified
classification: environment
scope: out-of-scope
impact: The first verify lane is not evidence from the exact CI toolchain
criteria: [LIFE-6]
requirements: []
cases: []
evidence:
  - kimi-cold-start-5af3f78
  - kimi-cold-start-5af3f78-scope
  - tc-cli-0106-kimi-original
  - OBS-tc-cli-bug-0022-toolchain-matches-ci-2858359e45fc0c99ab80ad5e
  - OBS-tc-cli-bug-0022-toolchain-matches-ci-24db0bde2060c1cadcfc949a
  - OBS-tc-cli-bug-0022-toolchain-matches-ci-738989a073f6e19b43e012a3
receipts:
  - id: 2026-11-01-original-kimi-code-cli-0-41-0-cold-start-discovered-the-agents
    aliases: kimi-cold-start-5af3f78
    claim: Original Kimi Code CLI 0.41.0 cold start discovered the .agents skill and completed local checks, but safe cold resume failed because next_valid_action repeated stale review and ship guidance and Kimi mapping was undocumented
    result: failed
    revision: 5af3f78d66b40a5af42607eafd5649c9b957d2ad
    environment: local-macos-arm64
    content: b877fa05caa2297084ca7a9beb1828f9a91d42c7c32675846fe9f624b6618d47
    fingerprint: sha256:66de8ea91e3c1f7a8c35f6958f2a4d134782b806b2ff77876ade31a8305b2c97
    observed: 2026-11-01T16:25:22Z
  - id: 2026-11-01-retrospective-review-of-the-original-kimi-code-cli-0-41-0-cold
    aliases: kimi-cold-start-5af3f78-scope
    claim: "Retrospective review of the original Kimi Code CLI 0.41.0 cold start on the completed scope definition: discovery and local checks succeeded, but safe cold resume failed on stale archived guidance and missing Kimi mapping"
    result: failed
    revision: 5af3f78d66b40a5af42607eafd5649c9b957d2ad
    environment: local-macos-arm64
    content: 0c308b91d4f20f4c9840bf918118bc459b38882bac949998052aab238cc35744
    fingerprint: sha256:66de8ea91e3c1f7a8c35f6958f2a4d134782b806b2ff77876ade31a8305b2c97
    observed: 2026-11-01T16:30:45Z
  - id: 2026-11-01-tc-cli-0106
    aliases: tc-cli-0106-kimi-original
    claim: "Verdicts 1=pass, 2=fail, 3=fail, 4=pass. Original retrospective Kimi cold-start review: the preserved run facts and human-gate limits pass, but Kimi mapping guidance and consistent corrected docs are absent at base"
    result: failed
    revision: 1d9264f71168f9f9cf2a50d45ab118e517b5a3ee
    environment: local-macos-arm64
    content: 7e03f73492d75b516a35048fce831115d2b1785a8651d0cb12a89d4f7b094702
    fingerprint: sha256:66de8ea91e3c1f7a8c35f6958f2a4d134782b806b2ff77876ade31a8305b2c97
    case: TC-cli-0106
    case_digest: 9306885ac0365a73944302c527dae2935465f1f55c932f4de386eb243eb9554f
    observed: 2026-11-01T16:43:04Z
  - id: 2026-11-11-tc-cli-bug-0022-toolchain-matches-ci
    aliases: OBS-tc-cli-bug-0022-toolchain-matches-ci-2858359e45fc0c99ab80ad5e
    claim: "Ran cargo test --locked --test package -- toolchain_task_names_a_version_that_differs toolchain_failure_names_each_copy_on_path_with_its_version for TC-cli-bug-0022-toolchain-matches-ci: passed (exit 0)."
    result: passed
    command: cargo test --locked --test package -- toolchain_task_names_a_version_that_differs toolchain_failure_names_each_copy_on_path_with_its_version
    revision: 504cecba6393f6b629f83a36ec3847842055da83
    environment: local-macos-arm64
    content: 879f7097ade0f326c1c45b5faa86603a11ce283581e3856ab68bb56a08f1e93a
    fingerprint: sha256:621dac0f6f0ab4a200872d49208d08eacaaa23a9ea75b6eeca2abd0c36607f42
    case: TC-cli-bug-0022-toolchain-matches-ci
    case_digest: 53a7ec2f1894f3d050ece157d1f43a6393eaac22aab78decb403fc224848c2d8
    observed: 2026-11-11T20:16:12Z
    duration_seconds: 6
  - id: 2026-11-12-tc-cli-bug-0022-toolchain-matches-ci
    aliases: OBS-tc-cli-bug-0022-toolchain-matches-ci-24db0bde2060c1cadcfc949a
    claim: "Ran cargo test --locked --test package -- toolchain_task_names_a_version_that_differs toolchain_failure_names_each_copy_on_path_with_its_version for TC-cli-bug-0022-toolchain-matches-ci: failed (exit 101)."
    result: failed
    command: cargo test --locked --test package -- toolchain_task_names_a_version_that_differs toolchain_failure_names_each_copy_on_path_with_its_version
    revision: bc8020474b181bbdbfc6671513016ecb1fd0bfb2
    environment: local-macos-arm64
    content: ccde5070f89f807682fe59e75ed34633c9b8d0d2c5b3cdd8a21533d45f2e20bc
    fingerprint: sha256:1f8d0e7e415056869fdb5a3f8d5fc4cbaedde38869a45fb076f0d1bc569f3e67
    case: TC-cli-bug-0022-toolchain-matches-ci
    case_digest: cb7fde202523d07f1a72335d9b4a6b4b72914a7ae05ac6c9669990725e7a10c9
    observed: 2026-11-12T00:19:17Z
    duration_seconds: 0
  - id: 2026-11-12-tc-cli-bug-0022-toolchain-matches-ci-320f74
    aliases: OBS-tc-cli-bug-0022-toolchain-matches-ci-738989a073f6e19b43e012a3
    claim: "Ran cargo test --locked --test package -- toolchain_task_names_a_version_that_differs toolchain_failure_names_each_copy_on_path_with_its_version for TC-cli-bug-0022-toolchain-matches-ci: passed (exit 0)."
    result: passed
    command: cargo test --locked --test package -- toolchain_task_names_a_version_that_differs toolchain_failure_names_each_copy_on_path_with_its_version
    revision: 54ed7c4cd8cfbbf8574a1b685b680bd25e767c27
    environment: local-macos-arm64
    content: f4706304eb513cc26319b7fb72787977781ccff31bfe4ba11e6a37108ffc82c5
    fingerprint: sha256:7f5c37ea0aba7808983276354e6f9266b9d27e1398d9dc439c16520013ca636d
    case: TC-cli-bug-0022-toolchain-matches-ci
    case_digest: 53a7ec2f1894f3d050ece157d1f43a6393eaac22aab78decb403fc224848c2d8
    observed: 2026-11-12T00:21:12Z
    duration_seconds: 4
environment: local-macos-arm64
revision: 5af3f78d66b40a5af42607eafd5649c9b957d2ad
found_by: manual report
related: []
created: 2026-11-01T15:16:42Z
updated: 2026-11-12T00:21:25Z
---
# Initial host tools do not match CI versions

**What happened:** The global HI and Fledge versions differ from the required CI versions
Actual: The initial PATH provides HI 0.5.0 and Fledge 1.8.0; SpecSync 6.1.1 matches

**Steps:** Run hi --version and fledge --version at the start of the exercise
1. Check tool versions before building

**Expected:** HI 0.8.0 and Fledge 1.7.2 match CI

**Why it matters:** The first verify lane is not evidence from the exact CI toolchain

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T15:16:42Z · Reported (manual report)
- 2026-11-01T15:33:00Z · Downloaded release-verified HI 0.8.0 and Fledge 1.7.2 into target/tmp/ci-tools for later commands; the global PATH still has the initial mismatch, and the baseline verify used the initial versions
- 2026-11-01T16:25:22Z · The original Kimi cold-start observation supplies failed evidence for this finding; its recorded scope and open status are unchanged; evidence: kimi-cold-start-5af3f78
- 2026-11-01T16:30:46Z · The retrospective scope-gate observation preserves the same original failure on the completed draft content; scope and open status are unchanged; evidence: kimi-cold-start-5af3f78-scope
- 2026-11-01T16:43:22Z · The original TC-cli-0106 retrospective observation preserves evidence for this finding before the approved fixes; scope and open status are unchanged; evidence: tc-cli-0106-kimi-original
- 2026-11-11T20:17:14Z · Fixed in plan step 12 (PR #238, t38b): the toolchain task, first in the verify lane, names each differing tool and every copy on PATH. Passed here; the red run on 0.5.0 is to be recorded before verification.; evidence: OBS-tc-cli-bug-0022-toolchain-matches-ci-2858359e45fc0c99ab80ad5e
- 2026-11-12T00:20:54Z · Red on 0.5.0 (t38b, bc802047 on leif/0.6.0-ledger-red): 0.5.0 has no toolchain task, so nothing compares the local tools with the CI pins or names a binary path and its PATH copies.; evidence: OBS-tc-cli-bug-0022-toolchain-matches-ci-24db0bde2060c1cadcfc949a
- 2026-11-12T00:21:25Z · status: open -> verified. Green on integration after its red run on 0.5.0: the toolchain step names each tool that differs from the CI pins, with the binary path and the other copies on PATH.; evidence: OBS-tc-cli-bug-0022-toolchain-matches-ci-738989a073f6e19b43e012a3
