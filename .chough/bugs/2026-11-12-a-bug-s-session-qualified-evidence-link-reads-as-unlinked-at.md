---
kind: bug
id: 2026-11-12-a-bug-s-session-qualified-evidence-link-reads-as-unlinked-at
aliases:
  - BUG-a-bug-s-session-qualified-evidence-link-reads-as-unlinked-at-the-test-g-d7b6d63d2be07ec2dc18cf15
title: A bug's session-qualified evidence link reads as unlinked at the test gate and in next's last failure
status: verified
classification: product-defect
scope: blocking
impact: Any session with a linked failure cannot leave test
criteria: []
requirements: [REQ-cli-021]
cases: []
evidence:
  - OBS-tc-cli-qualified-link-leaves-test-db9b137477577e9d19416c85
  - OBS-tc-cli-qualified-link-last-failure-d95a707ab518bb9d07487756
  - OBS-tc-cli-qualified-link-leaves-test-b249031e5d6b0cb6cc02e5ce
  - OBS-tc-cli-qualified-link-last-failure-3739991ae6ee2368edab745d
receipts:
  - id: 2026-11-12-tc-cli-qualified-link-leaves-test
    aliases: OBS-tc-cli-qualified-link-leaves-test-db9b137477577e9d19416c85
    claim: "Ran cargo test --locked --test stages -- --exact a_failure_linked_by_its_session_qualified_id_lifts_the_unlinked_failure_refusal for TC-cli-qualified-link-leaves-test: failed (exit 101)."
    result: failed
    command: cargo test --locked --test stages -- --exact a_failure_linked_by_its_session_qualified_id_lifts_the_unlinked_failure_refusal
    revision: ef4540b027990eb8099b4a10b455975a19057870
    environment: local-macos-arm64
    content: 75bfc74101d1c0bc38684a2e8b6f534159124aa32491fc95f371db80dcc5a18a
    fingerprint: sha256:4ae9b800e1595a7d0aed8ee3d8ba969a2e9d7bca1a419704ef44e16fd0d3b917
    case: TC-cli-qualified-link-leaves-test
    case_digest: 4b02adff933ff55016884aa0710bcca12bd98647435279bd9213722e22264a50
    observed: 2026-11-12T02:02:06Z
    duration_seconds: 1
  - id: 2026-11-12-tc-cli-qualified-link-last-failure
    aliases: OBS-tc-cli-qualified-link-last-failure-d95a707ab518bb9d07487756
    claim: "Ran cargo test --locked --test next -- --exact last_failure_names_a_bug_linking_it_by_its_session_qualified_id_open_and_archived for TC-cli-qualified-link-last-failure: failed (exit 101)."
    result: failed
    command: cargo test --locked --test next -- --exact last_failure_names_a_bug_linking_it_by_its_session_qualified_id_open_and_archived
    revision: ef4540b027990eb8099b4a10b455975a19057870
    environment: local-macos-arm64
    content: 75bfc74101d1c0bc38684a2e8b6f534159124aa32491fc95f371db80dcc5a18a
    fingerprint: sha256:b589ed0cbaae0ea08fff5b06c2beb2340795344be982632b0d9246cd62e0bc08
    case: TC-cli-qualified-link-last-failure
    case_digest: 15390c1fa51fd37fdc10b6f072eab661a7f6fef32edb172d31f0f39ef69d275b
    observed: 2026-11-12T02:02:07Z
    duration_seconds: 0
  - id: 2026-11-12-tc-cli-qualified-link-leaves-test-55dc54
    aliases: OBS-tc-cli-qualified-link-leaves-test-b249031e5d6b0cb6cc02e5ce
    claim: "Ran cargo test --locked --test stages -- --exact a_failure_linked_by_its_session_qualified_id_lifts_the_unlinked_failure_refusal for TC-cli-qualified-link-leaves-test: passed (exit 0)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_failure_linked_by_its_session_qualified_id_lifts_the_unlinked_failure_refusal
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:6bf659ef371ea423ac126b4f03128d877c194041cee1666754cdb14bbcf70aa8
    case: TC-cli-qualified-link-leaves-test
    case_digest: 4b02adff933ff55016884aa0710bcca12bd98647435279bd9213722e22264a50
    observed: 2026-11-12T02:17:30Z
    duration_seconds: 2
  - id: 2026-11-12-tc-cli-qualified-link-last-failure-766eb7
    aliases: OBS-tc-cli-qualified-link-last-failure-3739991ae6ee2368edab745d
    claim: "Ran cargo test --locked --test next -- --exact last_failure_names_a_bug_linking_it_by_its_session_qualified_id_open_and_archived for TC-cli-qualified-link-last-failure: passed (exit 0)."
    result: passed
    command: cargo test --locked --test next -- --exact last_failure_names_a_bug_linking_it_by_its_session_qualified_id_open_and_archived
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:08a83dd38be306e5ddb8b01a9494dd4dda3745a83d755c1f4b07ae7493b179a2
    case: TC-cli-qualified-link-last-failure
    case_digest: 15390c1fa51fd37fdc10b6f072eab661a7f6fef32edb172d31f0f39ef69d275b
    observed: 2026-11-12T02:17:32Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: c226da4259bfafd37e83208bfe37a7636589e12c
found_by: t37j, test stage, area sessions
related: []
created: 2026-11-12T01:39:56Z
updated: 2026-11-12T02:18:07Z
---
# A bug's session-qualified evidence link reads as unlinked at the test gate and in next's last failure

**What happened:** Leaving test is refused with refused:unlinked-failure for a failure a bug links, and its remedy stores the same qualified link again, so it loops; next's last_failure lists no linking bug
Actual: bugs::observation_references returns links as stored, and the test gate (src/stage.rs) and last_failure (src/main.rs) compare them with bare observation IDs; introduced by 7667e63e (the F1 fix)

**Steps:** bug add or bug update --observation with a session entry, which since 7667e63e is stored as <session>/<id>, then stage advance out of test, or chough next

**Expected:** A failure a bug links does not block leaving test, and last_failure names each bug that links it (REQ-cli-021)

**Why it matters:** Any session with a linked failure cannot leave test

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:39:56Z · Reported (t37j, test stage, area sessions)
- 2026-11-12T02:02:52Z · status: open -> investigating. Red at ef4540b0 (TC-cli-qualified-link-leaves-test, on t37x's branch before its fix): failed for this bug's reason; fix in e942c1eb.; evidence: OBS-tc-cli-qualified-link-leaves-test-db9b137477577e9d19416c85
- 2026-11-12T02:02:52Z · Red at ef4540b0 (TC-cli-qualified-link-last-failure, on t37x's branch before its fix): failed for this bug's reason; fix in e942c1eb.; evidence: OBS-tc-cli-qualified-link-last-failure-d95a707ab518bb9d07487756
- 2026-11-12T02:18:06Z · Green at 0ee26ac0: TC-cli-qualified-link-leaves-test passed (fix e942c1eb).; evidence: OBS-tc-cli-qualified-link-leaves-test-b249031e5d6b0cb6cc02e5ce
- 2026-11-12T02:18:07Z · status: investigating -> verified. Green at 0ee26ac0: TC-cli-qualified-link-last-failure passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-cli-qualified-link-last-failure-3739991ae6ee2368edab745d
