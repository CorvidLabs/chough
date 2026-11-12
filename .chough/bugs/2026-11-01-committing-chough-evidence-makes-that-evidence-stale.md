---
kind: bug
id: 2026-11-01-committing-chough-evidence-makes-that-evidence-stale
aliases: [BUG-0015]
title: Committing Chough evidence makes that evidence stale
status: verified
classification: product-defect
scope: in-scope
impact: Committed evidence can never read as current
criteria: [LIFE-2, LIFE-7]
requirements: [REQ-cli-004]
cases: []
evidence:
  - bug-0015-repro-1
  - bug-0015-fixed-1
  - tc-cli-0067-1793498630
  - tc-cli-0068-1793498632
  - tc-cli-0069-1793498635
  - tc-cli-0073-1793498650
  - tc-cli-0074-1793498652
receipts:
  - id: 2026-11-01-before-the-fix-a-new-observation-reads-stale-revision-or
    aliases: bug-0015-repro-1
    claim: Before the fix, a new observation reads stale-revision-or-inputs after committing only .chough/, after an empty commit, and after committing edits that were already present when it ran
    result: failed
    revision: a386c132598916c6572a633da455b169b825d3e9
    environment: local-macos-arm64
    fingerprint: sha256:efd75aec7aa2df60fef447246b9f4809b44ebde0d5c47d9c82f60ff29b319150
    observed: 2026-11-01T01:31:33Z
  - id: 2026-11-01-tc-cli-0067
    aliases: bug-0015-fixed-1
    claim: After the fix, a new observation stays current-observation after committing only .chough/, after an empty commit and after committing edits already present when it ran; restoring a deleted file stales it
    result: passed
    revision: e806f783b34792c76961edfb24d42aa73764e6fa
    environment: local-macos-arm64
    content: 7eb66be2577f9e78895e0a93951cf202ab0bfe83611034719cfce6dfd4190e27
    fingerprint: sha256:2a5503e27555231221af92c2b4c286ecc4e6ed3c983586a2891232d45a3979bb
    case: TC-cli-0067
    case_digest: ae0a214ac7ea18f81c98ced40f22340666a1b039f1532c3a539fdcaef2e9c7ac
    observed: 2026-11-01T01:40:32Z
  - id: 2026-11-01-tc-cli-0067-22937a
    aliases: tc-cli-0067-1793498630
    claim: "Ran cargo test --locked --test cli -- --exact committing_only_chough_records_or_an_empty_commit_keeps_an_observation_current for TC-cli-0067: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact committing_only_chough_records_or_an_empty_commit_keeps_an_observation_current
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:d2167322e7df8c92eb94f05751aa721ba4c57c855a2234bc918318c7d7f0af4f
    case: TC-cli-0067
    case_digest: ae0a214ac7ea18f81c98ced40f22340666a1b039f1532c3a539fdcaef2e9c7ac
    observed: 2026-11-01T02:03:52Z
  - id: 2026-11-01-tc-cli-0068
    aliases: tc-cli-0068-1793498632
    claim: "Ran cargo test --locked --test cli -- --exact committing_edits_already_present_when_observed_keeps_an_observation_current for TC-cli-0068: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact committing_edits_already_present_when_observed_keeps_an_observation_current
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:370c56ce0749c23435ec54802132c8f55eb8169eff87c66d238a241216bcae2d
    case: TC-cli-0068
    case_digest: 5f73ff87c94ac173fe495b26f673966d6a350f24b5ba74d7fac9a86ac6ae8075
    observed: 2026-11-01T02:03:54Z
  - id: 2026-11-01-tc-cli-0069
    aliases: tc-cli-0069-1793498635
    claim: "Ran cargo test --locked --test cli -- --exact any_fingerprinted_content_change_stales_an_observation for TC-cli-0069: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact any_fingerprinted_content_change_stales_an_observation
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:a66e7b0b5d7eca6c65a35ce0c4c9ab87acfef773ac326ada8a62ef0522a015ee
    case: TC-cli-0069
    case_digest: 6c737d65a07bae2ee3a6544cfcb6604b45d303c47f93ee97aebe7fbcbf8bfca8
    observed: 2026-11-01T02:03:58Z
  - id: 2026-11-01-tc-cli-0073
    aliases: tc-cli-0073-1793498650
    claim: "Ran cargo test --locked --test cli -- --exact observations_without_a_content_identity_stay_stale_after_a_commit for TC-cli-0073: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact observations_without_a_content_identity_stay_stale_after_a_commit
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:17ba2f751813874ba4d4a4d6dd5d5e1ac1a51a3b3790d9de2640c82dfd4c9731
    case: TC-cli-0073
    case_digest: 9e6a6dc2156be99c5c1634f2d29d390b0f1cc51aab691242195ece316ad58869
    observed: 2026-11-01T02:04:12Z
  - id: 2026-11-01-tc-cli-0074
    aliases: tc-cli-0074-1793498652
    claim: "Ran cargo test --locked --test cli -- --exact a_malformed_content_identity_is_refused for TC-cli-0074: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cli -- --exact a_malformed_content_identity_is_refused
    revision: 7c4278b49181a7c71c2487364b6bce5c8622dc9a
    environment: local-macos-arm64
    content: fc88d131a5f7e1925f59db06927bbc602dcdff663f96904cc2246edf5cf702f5
    fingerprint: sha256:9c188d6572085bf01d36410d954ce9a1bedcbb2dafd1a4b1671de78df470b759
    case: TC-cli-0074
    case_digest: cddec2d484d0dc69057661018116dd433e00770ddb178cb35c2d4be4f88a3962
    observed: 2026-11-01T02:04:14Z
environment: local-macos-arm64
revision: 2756dc24a821bbb92021455c704ef7a177123ffa
found_by: CorvidLabs/chough#10 (Raven run, CorvidLabs/raven#644); reproduced on main 2756dc2
related: []
created: 2026-11-01T00:57:53Z
updated: 2026-11-01T02:05:06Z
---
# Committing Chough evidence makes that evidence stale

**What happened:** After cases run and committing .chough/, next reports the observation stale-revision-or-inputs; an empty commit does too
Actual: snapshot() in src/evidence.rs hashes the HEAD revision into the fingerprint

**Steps:** Any commit after recording an observation
1. chough cases run <case>
2. git add .chough && git commit
3. chough next: stale-revision-or-inputs

**Expected:** Only a change to fingerprinted content stales an observation

**Why it matters:** Committed evidence can never read as current

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-01T00:57:53Z · Reported (CorvidLabs/chough#10 (Raven run, CorvidLabs/raven#644); reproduced on main 2756dc2)
- 2026-11-01T01:31:34Z · Reproduced before the fix by the new tests; see .chough/artifacts/bug-0015-reproduction.log.; evidence: bug-0015-repro-1
- 2026-11-01T01:40:33Z · status: open -> verified. Fixed in 4bf46da; the reproduction tests pass at e806f78 (.chough/artifacts/bug-0015-fixed-reproduction.log).; evidence: bug-0015-fixed-1
- 2026-11-01T02:05:06Z · The reproduction was a test-log observation recorded with evidence add before the fix, not a case-run observation: the regression case definitions TC-cli-0067, 0068, 0069, 0073 and 0074 arrived with the fix commit 4bf46da, so they could not run at the unfixed tip. This note links their current passing case-run observations at the review-fix head.; evidence: tc-cli-0067-1793498630, tc-cli-0068-1793498632, tc-cli-0069-1793498635, tc-cli-0073-1793498650, tc-cli-0074-1793498652
