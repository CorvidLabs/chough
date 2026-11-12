---
kind: bug
id: 2026-11-06-archived-reports-lack-bounded-authenticated-reuse
aliases: [BUG-archived-reports-lack-bounded-authenticated-reuse-0536fdd42da0bab4c25202cf]
title: Archived reports lack bounded authenticated reuse
status: verified
classification: product-defect
scope: in-scope
impact: Archived next and status retain their native-status latency; lifecycle authority is unchanged.
criteria: [DRIVE-15, DRIVE-2]
requirements: [REQ-cli-053]
cases:
  - TC-cli-archived-report-reuse
  - TC-cli-archive-cache-invalidation
  - TC-cli-archive-cache-timeout
evidence:
  - OBS-tc-cli-archived-report-reuse-caa35ad0d8067f0b20169c34
  - OBS-tc-cli-archived-report-reuse-59382721b25cc6f8e1ab97ce
  - OBS-tc-cli-archive-cache-invalidation-c6967565fd883fbf3ec5a0c3
  - OBS-tc-cli-archive-cache-timeout-85b73aed0ef8a3686b5ba33a
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
fixture: .chough/artifacts/issue-32-next-status-latency/failing-native-before.log
environment: local-macos-arm64
revision: cfa2d10422259d677e9e6f28ae49e7380628de78
found_by: preserved failing-first product test
related: []
created: 2026-11-06T23:18:14Z
updated: 2026-11-06T23:18:52Z
---
# Archived reports lack bounded authenticated reuse

**What happened:** Warm report reuse, invalidation, anti-spoof and timeout regressions fail because no authenticated live response is recorded.
Actual: Four regressions fail with live response count 0 instead of 1; the two independent controls pass.

**Steps:** Run the native cache regressions at the failing-first commit before the cache implementation.
1. At commit 1f94916d9a1c7b0c017b528838b9137e973cfa85, run the native test target before implementing the cache.

**Expected:** A completed authenticated archive read primes bounded read-only reuse; invalid or timed-out inputs stay live and write no authenticated entry.

**Why it matters:** Archived next and status retain their native-status latency; lifecycle authority is unchanged.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-06T23:18:14Z · Reported (preserved failing-first product test); evidence: OBS-tc-cli-archived-report-reuse-caa35ad0d8067f0b20169c34
- 2026-11-06T23:18:52Z · status: open -> verified. The implemented cache passes the warm-reuse, invalidation and timeout cases after the preserved failing-first receipt.; evidence: OBS-tc-cli-archived-report-reuse-59382721b25cc6f8e1ab97ce, OBS-tc-cli-archive-cache-invalidation-c6967565fd883fbf3ec5a0c3, OBS-tc-cli-archive-cache-timeout-85b73aed0ef8a3686b5ba33a
