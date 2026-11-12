---
kind: session
id: 2026-11-06-issue-32-next-status-latency
aliases: [issue-32-next-status-latency]
objective: Make archived next and status fast within explicit warm budgets (#32/C8)
intent_scope: ""
modules: []
stage: done
stage_status: active
iteration: 1
callers: []
native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
pr: ""
criteria: [DRIVE-2, DRIVE-15]
requirements: [REQ-cli-053]
cases:
  - TC-cli-archived-report-reuse
  - TC-cli-archive-cache-invalidation
  - TC-cli-archive-cache-timeout
  - TC-cli-snapshot-worker-identity
  - TC-cli-specsync-symlink-reuse
design: []
files: []
observations:
  - OBS-tc-cli-archived-report-reuse-caa35ad0d8067f0b20169c34
  - OBS-tc-cli-specsync-symlink-reuse-ff73f0fe4f4b6d80220a43e7
  - OBS-tc-cli-archived-report-reuse-59382721b25cc6f8e1ab97ce
  - OBS-tc-cli-archive-cache-invalidation-c6967565fd883fbf3ec5a0c3
  - OBS-tc-cli-archive-cache-timeout-85b73aed0ef8a3686b5ba33a
  - OBS-tc-cli-snapshot-worker-identity-5eaeb6789f2e89031c77c344
  - OBS-tc-cli-specsync-symlink-reuse-b5fb5bc36c4a751af0c59fb3
  - OBS-tc-cli-archived-report-reuse-20f0b488f31ec8fa7fbbff14
  - OBS-structural-d609d7d731efad7e2db11df1
bugs:
  - BUG-archived-reports-lack-bounded-authenticated-reuse-0536fdd42da0bab4c25202cf
  - BUG-specsync-symlink-cache-key-uses-unresolved-path-9205c0afb5fb77ec63d04630
blockers: []
waiting_on_human: []
run_consents: []
next: Push the archive tip, observe green CI, and merge only at the approved pinned head
closed: 2026-11-06
outcome: done
receipts:
  - id: 2026-11-06-tc-cli-archived-report-reuse
    aliases: OBS-tc-cli-archived-report-reuse-caa35ad0d8067f0b20169c34
    claim: "Preserved failing-first receipt from commit 1f94916d9a1c7b0c017b528838b9137e973cfa85: warm reuse, invalidation, anti-spoof and timeout regressions failed before the cache implementation; two controls passed. This is a historical record, not a fresh execution."
    result: failed
    revision: cfa2d10422259d677e9e6f28ae49e7380628de78
    environment: local-macos-arm64
    content: d9381ff305c9a69260e3adee50f95ee7a92fbc834d6f621a6ef70e9d66e0cc5f
    fingerprint: sha256:f6c8e42e11af68cd19d61a40d20ad81e4a26ce1a6dbf9d33ed19b9b011d75916
    case: TC-cli-archived-report-reuse
    case_digest: a7e515d39d4c7f118dfc374c016bab8061faafc6640d768e9cb683d04335e1a1
    observed: 2026-11-06T23:17:53Z
  - id: 2026-11-06-tc-cli-specsync-symlink-reuse
    aliases: OBS-tc-cli-specsync-symlink-reuse-ff73f0fe4f4b6d80220a43e7
    claim: "Preserved failing-first receipt: the resolved-SpecSync-symlink regression keyed <redacted:home-path> instead of the fixture's resolved real-specsync executable. This is a historical record, not a fresh execution."
    result: failed
    revision: cfa2d10422259d677e9e6f28ae49e7380628de78
    environment: local-macos-arm64
    content: d9381ff305c9a69260e3adee50f95ee7a92fbc834d6f621a6ef70e9d66e0cc5f
    fingerprint: sha256:de70aa00d8d598dec8b2155283f0bc8158548e5de991c32aa59768d0b0859778
    case: TC-cli-specsync-symlink-reuse
    case_digest: 74c5e86c830c2f021be2bcb4d8f12bba871e3539e0f2bbf5a1020756cbdc06d8
    observed: 2026-11-06T23:17:59Z
  - id: 2026-11-06-tc-cli-archived-report-reuse-a89a21
    aliases: OBS-tc-cli-archived-report-reuse-59382721b25cc6f8e1ab97ce
    claim: "Ran cargo test --locked --test native -- --exact warm_reports_reuse_but_every_lifecycle_read_stays_live for TC-cli-archived-report-reuse: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact warm_reports_reuse_but_every_lifecycle_read_stays_live
    revision: cfa2d10422259d677e9e6f28ae49e7380628de78
    environment: local-macos-arm64
    content: d9381ff305c9a69260e3adee50f95ee7a92fbc834d6f621a6ef70e9d66e0cc5f
    fingerprint: sha256:479d742f9b7a9efd1ed01c6c2399596399d2c207be920f8d091d6d3c48dab9fd
    case: TC-cli-archived-report-reuse
    case_digest: a7e515d39d4c7f118dfc374c016bab8061faafc6640d768e9cb683d04335e1a1
    observed: 2026-11-06T23:18:31Z
    duration_seconds: 4
  - id: 2026-11-06-tc-cli-archive-cache-invalidation
    aliases: OBS-tc-cli-archive-cache-invalidation-c6967565fd883fbf3ec5a0c3
    claim: "Ran cargo test --locked --test native -- --exact every_cache_identity_mismatch_or_untrusted_entry_goes_live for TC-cli-archive-cache-invalidation: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact every_cache_identity_mismatch_or_untrusted_entry_goes_live
    revision: cfa2d10422259d677e9e6f28ae49e7380628de78
    environment: local-macos-arm64
    content: d9381ff305c9a69260e3adee50f95ee7a92fbc834d6f621a6ef70e9d66e0cc5f
    fingerprint: sha256:f2092e485ea1748abcadc7f5e602fda7f42d23b5a61de5cacf3fc4835d104f7a
    case: TC-cli-archive-cache-invalidation
    case_digest: 25b538b6805b1209eb7c181bb11ba3731d2e877cc85abd77e175db5a4256c06a
    observed: 2026-11-06T23:18:37Z
    duration_seconds: 6
  - id: 2026-11-06-tc-cli-archive-cache-timeout
    aliases: OBS-tc-cli-archive-cache-timeout-85b73aed0ef8a3686b5ba33a
    claim: "Ran cargo test --locked --test native -- --exact a_timed_out_prime_writes_nothing_kills_children_and_retries_cold for TC-cli-archive-cache-timeout: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact a_timed_out_prime_writes_nothing_kills_children_and_retries_cold
    revision: cfa2d10422259d677e9e6f28ae49e7380628de78
    environment: local-macos-arm64
    content: d9381ff305c9a69260e3adee50f95ee7a92fbc834d6f621a6ef70e9d66e0cc5f
    fingerprint: sha256:a7bbbc405dbcd3c2d7004a98c1d127a4875d1fde9a15cef79007aed84422316c
    case: TC-cli-archive-cache-timeout
    case_digest: 733e65ee61a0af0a69b1b62d5ab1f08e27ad6dfafdf92ec647d4d5234d28af59
    observed: 2026-11-06T23:18:40Z
    duration_seconds: 2
  - id: 2026-11-06-tc-cli-snapshot-worker-identity
    aliases: OBS-tc-cli-snapshot-worker-identity-5eaeb6789f2e89031c77c344
    claim: "Ran cargo test --locked --test native -- --exact snapshot_identities_and_change_front_matter_rules_stay_exact for TC-cli-snapshot-worker-identity: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact snapshot_identities_and_change_front_matter_rules_stay_exact
    revision: cfa2d10422259d677e9e6f28ae49e7380628de78
    environment: local-macos-arm64
    content: d9381ff305c9a69260e3adee50f95ee7a92fbc834d6f621a6ef70e9d66e0cc5f
    fingerprint: sha256:980ccf4ef2dc1cc44ffaf2e099968576db115182508b7f64253ac21ee9e923e4
    case: TC-cli-snapshot-worker-identity
    case_digest: 5aa8127e34076938fa3a54b64e2a8eb6444d6966106f46af6cd95de9fd69af2b
    observed: 2026-11-06T23:18:42Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-specsync-symlink-reuse-0125a9
    aliases: OBS-tc-cli-specsync-symlink-reuse-b5fb5bc36c4a751af0c59fb3
    claim: "Ran cargo test --locked --test native -- --exact a_resolved_specsync_symlink_can_prime_and_reuse for TC-cli-specsync-symlink-reuse: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact a_resolved_specsync_symlink_can_prime_and_reuse
    revision: cfa2d10422259d677e9e6f28ae49e7380628de78
    environment: local-macos-arm64
    content: d9381ff305c9a69260e3adee50f95ee7a92fbc834d6f621a6ef70e9d66e0cc5f
    fingerprint: sha256:b359b806f91d8da1ea5318b7feea4a9443c1be6732e068bd15113fe53cff1f9d
    case: TC-cli-specsync-symlink-reuse
    case_digest: 74c5e86c830c2f021be2bcb4d8f12bba871e3539e0f2bbf5a1020756cbdc06d8
    observed: 2026-11-06T23:18:44Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-archived-report-reuse-c790a1
    aliases: OBS-tc-cli-archived-report-reuse-20f0b488f31ec8fa7fbbff14
    claim: "Final integrated release benchmark at cfa2d104 on 147 archives: next p95/max 0.66/0.66 s and status 0.67/0.67 s over ten warm runs each after a 5.35 s authenticated prime; separate wrapper proof made zero warm SpecSync calls for each command."
    result: passed
    revision: cfa2d10422259d677e9e6f28ae49e7380628de78
    environment: local-macos-arm64
    content: d9381ff305c9a69260e3adee50f95ee7a92fbc834d6f621a6ef70e9d66e0cc5f
    fingerprint: sha256:8619b27cfa7f6129099f5538fef9d085ad3cd07135f19c04dd8609c290829a0d
    case: TC-cli-archived-report-reuse
    case_digest: a7e515d39d4c7f118dfc374c016bab8061faafc6640d768e9cb683d04335e1a1
    observed: 2026-11-06T23:22:15Z
  - id: OBS-structural-d609d7d731efad7e2db11df1
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
---
# Make archived next and status fast within explicit warm budgets (#32/C8)

## Goal

Make archived next and status fast within explicit warm budgets (#32/C8)

## Where it stands

- Profiled no-active, active and archived native status plus fresh debug/release next/status on the 121-archive repository; measurements and limits are in .chough/artifacts/issue-32-next-status-latency/PROFILE.md.
- Failing tests first: 1f94916d9a1c7b0c017b528838b9137e973cfa85. Implemented the read-only cache and exact snapshot workers at c1d4223b43c818027c377e90b04c97828548f889, then merged #141/main at ce9e140f0d86e0342531c45dd6e92af66c9a7e52.
- All five focused cases passed on cfa2d104; both preserved failure receipts are linked to verified local bugs. Full Fledge verify passed 10/10 before the latest main merge.
- Final integrated release benchmark passed at cfa2d104 on 147 archives: next p95/max 0.66/0.66 s, status 0.67/0.67 s, with zero warm SpecSync calls.

## Decisions and why

- proposed: warm archived next/status budgets are 2000 ms p95 and 3000 ms maximum over ten release runs because archived native status alone measured 4.36-4.84 s and local hashing can exceed 2 s.
- rejected: cached stage transitions, cached active native state, or Chough-parsed archive validity because lifecycle mutations must keep live SpecSync authority and Chough must not become a second authenticator.
- decided: the PR 69 split assigns archive caching to issue 32 and batched stage advance to a separate PR because the orc coordinated that ownership with ch-pr69; the ID audit assigns REQ-cli-052 and DRIVE-15 above every existing claim.
- decided: issue 32 uses REQ-cli-053 instead of REQ-cli-052 because ch-pr69 subsequently claimed REQ-cli-052 for the separate stage-advance PR; DRIVE-15 remains unclaimed.
- decided: the warm budget begins only after a successful authenticated prime because ch-pr69 measured cold SpecSync reads beyond Chough's 120 second timeout under load; a timeout must write no entry, leave no native descendants, and produce no warm claim.
- decided: Leif said "Approve" at 22:00 MST through the orc for the exact REQ-cli-053/DRIVE-15 scope at de8e0cd3; the archive cache is read-only and all lifecycle mutations stay live.
- Leif decided "Scope correction for make-archived-next-and-status-fast-within-explicit-warm-budgets: native audit requires adding specs/cli/cli.spec.md and specs/cli/testing.md to affected_paths and fresh approval": "add only specs/cli/cli.spec.md and specs/cli/testing.md to affected_paths (they carry the approved REQ-cli-053), no behavior change." (relayed by orc t18 on 2026-11-06; native re-approval recorded by orc at 4f8ceb74); recorded by kimi-ch-perf32; a relayed claim, not authenticated
- Leif decided "Implementation review of make-archived-next-and-status-fast-within-explicit-warm-budgets": "pass" (PR #180 review recorded as 0xLeif by orc t18 at 18af4851 on 2026-11-06); recorded by kimi-ch-perf32; a relayed claim, not authenticated

## What was verified

- OBS-tc-cli-archived-report-reuse-caa35ad0d8067f0b20169c34
- OBS-tc-cli-specsync-symlink-reuse-ff73f0fe4f4b6d80220a43e7
- OBS-tc-cli-archived-report-reuse-59382721b25cc6f8e1ab97ce
- OBS-tc-cli-archive-cache-invalidation-c6967565fd883fbf3ec5a0c3
- OBS-tc-cli-archive-cache-timeout-85b73aed0ef8a3686b5ba33a
- OBS-tc-cli-snapshot-worker-identity-5eaeb6789f2e89031c77c344
- OBS-tc-cli-specsync-symlink-reuse-b5fb5bc36c4a751af0c59fb3
- OBS-tc-cli-archived-report-reuse-20f0b488f31ec8fa7fbbff14
- OBS-structural-d609d7d731efad7e2db11df1

## Open threads

## Timeline

- 2026-11-06T03:37:39Z · reset · intent → intent · Issue #32 / C8: profile next and status latency and obtain approval for explicit budgets and a focused fix ^t-2cadbe
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: e99dc7961323e5c4e2aff1a2cf13333b0c47037e
  - refresh: intent: HI criteria and FRD; append new criteria, never rewrite approved ones
- 2026-11-06T04:49:55Z · gate-opened · intent → intent · Intent and scope decision of make-archived-next-and-status-fast-within-explicit-warm-budgets: approve the numeric warm budgets, read-only archive cache, exact snapshot workers, and supersession of stale PR 69 ^t-282302
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: e99dc7961323e5c4e2aff1a2cf13333b0c47037e
- 2026-11-06T04:53:36Z · gate-closed · intent → intent · Intent and scope decision of make-archived-next-and-status-fast-within-explicit-warm-budgets: approve the numeric warm budgets, read-only archive cache, exact snapshot workers, and supersession of stale PR 69 ^t-ffeb48
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: a350ff5a95b6c5479475458393e93c06995c7cf3
- 2026-11-06T04:53:36Z · gate-opened · intent → intent · Intent and scope decision of make-archived-next-and-status-fast-within-explicit-warm-budgets: approve the numeric warm budgets, read-only archive cache using REQ-cli-052 and DRIVE-15, and exact snapshot workers ^t-3d5157
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: a350ff5a95b6c5479475458393e93c06995c7cf3
- 2026-11-06T04:54:30Z · gate-closed · intent → intent · Intent and scope decision of make-archived-next-and-status-fast-within-explicit-warm-budgets: approve the numeric warm budgets, read-only archive cache using REQ-cli-052 and DRIVE-15, and exact snapshot workers ^t-442309
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: 51f4ab3af71faceb6e0404cd3f39713b73f15552
- 2026-11-06T04:54:30Z · gate-opened · intent → intent · Intent and scope decision of make-archived-next-and-status-fast-within-explicit-warm-budgets: approve the numeric warm budgets, read-only archive cache using REQ-cli-053 and DRIVE-15, and exact snapshot workers ^t-6155ca
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: 51f4ab3af71faceb6e0404cd3f39713b73f15552
- 2026-11-06T05:31:19Z · gate-closed · intent → intent · Intent and scope decision of make-archived-next-and-status-fast-within-explicit-warm-budgets: approve the numeric warm budgets, read-only archive cache using REQ-cli-053 and DRIVE-15, and exact snapshot workers ^t-407dd7
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: ce9e140f0d86e0342531c45dd6e92af66c9a7e52
- 2026-11-06T05:31:19Z · advance · intent → design ·  ^t-f55154
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: ce9e140f0d86e0342531c45dd6e92af66c9a7e52
- 2026-11-06T05:31:19Z · skip · design → plan · CLI performance work has no visual or interaction design ^t-069a5a
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: ce9e140f0d86e0342531c45dd6e92af66c9a7e52
- 2026-11-06T05:31:24Z · advance · plan → build · native:approved ^t-3e2a14
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - gate: native:approved
  - revision: ce9e140f0d86e0342531c45dd6e92af66c9a7e52
- 2026-11-06T05:48:27Z · gate-opened · build → build · Scope correction for make-archived-next-and-status-fast-within-explicit-warm-budgets: native audit requires adding specs/cli/cli.spec.md and specs/cli/testing.md to affected_paths and fresh approval ^t-0c3ff7
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: fcade057c380f85503fad6304eb78c4685b601c3
- 2026-11-06T05:48:30Z · back · build → plan · Native audit found that the approved affected_paths omitted the two cli companions already changed by the approved scope; fresh scope authorization is required ^t-f8a308
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: fcade057c380f85503fad6304eb78c4685b601c3
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, native: the definition is approved; changing intent, contract, acceptance criteria or scope needs native reapproval. Follow `specsync change status make-archived-next-and-status-fast-within-explicit-warm-budgets`; Chough does not change native state.
- 2026-11-06T22:50:53Z · gate-closed · plan → plan · Scope correction for make-archived-next-and-status-fast-within-explicit-warm-budgets: native audit requires adding specs/cli/cli.spec.md and specs/cli/testing.md to affected_paths and fresh approval ^t-45661d
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: 76390c786ce86d653e4a7a44fbd032e275d9c685
  - relayed_by: {"decided_by":"Leif","words":"add only specs/cli/cli.spec.md and specs/cli/testing.md to affected_paths (they carry the approved REQ-cli-053), no behavior change.","where":"relayed by orc t18 on 2026-11-06; native re-approval recorded by orc at 4f8ceb74","recorded_by":"kimi-ch-perf32"}
- 2026-11-06T22:51:04Z · advance · plan → build · native:approved ^t-4e8314
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - gate: native:approved
  - revision: 76390c786ce86d653e4a7a44fbd032e275d9c685
- 2026-11-06T23:17:44Z · advance · build → test · native:verifying ^t-261c68
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - gate: native:verifying
  - revision: cfa2d10422259d677e9e6f28ae49e7380628de78
- 2026-11-06T23:35:00Z · advance · test → accept · native:verifying ^t-5ea305
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - gate: native:verifying
  - revision: cfa2d10422259d677e9e6f28ae49e7380628de78
- 2026-11-06T23:35:06Z · gate-opened · accept → accept · Implementation review of make-archived-next-and-status-fast-within-explicit-warm-budgets ^t-dee180
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: cfa2d10422259d677e9e6f28ae49e7380628de78
- 2026-11-06T23:54:41Z · gate-closed · accept → accept · Implementation review of make-archived-next-and-status-fast-within-explicit-warm-budgets ^t-313c90
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - revision: c1aff90dea76b697b3adb6b395cb73ec78aeb68f
  - relayed_by: {"decided_by":"Leif","words":"pass","where":"PR #180 review recorded as 0xLeif by orc t18 at 18af4851 on 2026-11-06","recorded_by":"kimi-ch-perf32"}
- 2026-11-06T23:55:14Z · advance · accept → done · native:archived ^t-9555ac
  - native_change: make-archived-next-and-status-fast-within-explicit-warm-budgets
  - gate: native:archived
  - revision: c1aff90dea76b697b3adb6b395cb73ec78aeb68f


Closed 2026-11-06: done.
