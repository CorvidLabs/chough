---
kind: bug
id: 2026-11-11-a-human-s-gate-decision-does-not-drive-the-native-approval-as
aliases:
  - BUG-a-human-s-gate-decision-does-not-drive-the-native-approval-as-that-huma-a668c7a05a71710dbb283bdc
title: A human's gate decision does not drive the native approval as that human (#236)
status: verified
classification: product-defect
scope: in-scope
impact: "Human approvals are recorded under the wrong identity or not at all (GitHub #236)"
criteria: [FLOW-23, FLOW-24]
requirements: [REQ-cli-029]
cases: [TC-records-approval-as-the-human]
evidence:
  - OBS-tc-records-approval-as-the-human-f6d8c563d853b097d71c6f4c
  - OBS-dry-run-e43743fac8e3ffdcf0e1dd26
  - OBS-tc-records-approval-as-the-human-444192105d5fc75eaae55dc1
  - OBS-tc-cli-close-gate-drives-native-step-40befe9849980c3cda707578
  - OBS-tc-cli-ship-reviews-as-human-82437920965a295ab87c0fa5
  - OBS-dry-run-9db85eed3d0c73dbd63de8b1
receipts:
  - id: 2026-11-11-tc-records-approval-as-the-human
    aliases: OBS-tc-records-approval-as-the-human-f6d8c563d853b097d71c6f4c
    claim: "Ran cargo test --locked --test humans -- a_scope_gate_runs_the_approval_once_as_the_configured_human a_review_gate_needs_a_verdict_and_runs_the_review_with_it an_unknown_or_ambiguous_human_is_refused_before_anything_runs the_project_file_overrides_the_user_file no_source_names_a_person_as_the_default_human for TC-records-approval-as-the-human: failed (exit 101)."
    result: failed
    command: cargo test --locked --test humans -- a_scope_gate_runs_the_approval_once_as_the_configured_human a_review_gate_needs_a_verdict_and_runs_the_review_with_it an_unknown_or_ambiguous_human_is_refused_before_anything_runs the_project_file_overrides_the_user_file no_source_names_a_person_as_the_default_human
    revision: 82c85c99483082130a7bb306fad29262c7578b2c
    environment: local-macos-arm64
    content: 1a60927d1ea0f042ab8f9e6a210a484b93756612fd5d4975f6d790ee833764f1
    fingerprint: sha256:788c11c80b263c74f6015ef7c1c79f7cac5f94113210ad08fb2e4aac00f61d68
    case: TC-records-approval-as-the-human
    case_digest: fcb626c0dca4884cc45e23f20e6207f9c9069cfb78eac90f464383c6a8b7396b
    observed: 2026-11-11T19:06:58Z
    duration_seconds: 1
  - id: 2026-11-11-claude-code-claude-opus-5-5-auto-mode-hc-t382-ran-specsync
    aliases: OBS-dry-run-e43743fac8e3ffdcf0e1dd26
    claim: "Claude Code (claude-opus-5-5, auto mode, hc t382) ran specsync change approve <0.6.0 change> --actor 0xLeif --note <Leif's relayed words> directly, after Leif's relayed scope approval (\"ok i approve\", 2026-11-11). The permission classifier refused it before it ran: \"Permission for this action was denied by the Claude Code auto mode classifier. Reason: [Self-Approval].\" Leif then ran the same approve himself. The Chough-driven approve of REQ-records-008 does not exist yet; it is tried for real in plan step 6 (#236)."
    result: failed
    revision: 24545ca7ee6e23da00aa7f7fb1414f532062995a
    environment: local-macos-arm64
    content: 595878672b503937ab1387efb2c115787ecbdb1719ac6c88de9df5d42f19a978
    observed: 2026-11-11T15:48:18Z
  - id: 2026-11-11-tc-records-approval-as-the-human-4084fd
    aliases: OBS-tc-records-approval-as-the-human-444192105d5fc75eaae55dc1
    claim: "Ran cargo test --locked --test humans -- a_scope_gate_runs_the_approval_once_as_the_configured_human a_review_gate_needs_a_verdict_and_runs_the_review_with_it an_unknown_or_ambiguous_human_is_refused_before_anything_runs the_project_file_overrides_the_user_file no_source_names_a_person_as_the_default_human for TC-records-approval-as-the-human: passed (exit 0)."
    result: passed
    command: cargo test --locked --test humans -- a_scope_gate_runs_the_approval_once_as_the_configured_human a_review_gate_needs_a_verdict_and_runs_the_review_with_it an_unknown_or_ambiguous_human_is_refused_before_anything_runs the_project_file_overrides_the_user_file no_source_names_a_person_as_the_default_human
    revision: 82a7392dda5b59925f1dad704d2c6c2778b23b8b
    environment: local-macos-arm64
    content: c0ad4a455cdd87cc042886e343d1ad1a31016c6ee2b83c35f413758823f87006
    fingerprint: sha256:a040a3b36e62bf0f5cbe6092924ee237d8d7b7173a515311487b97f94845ad98
    case: TC-records-approval-as-the-human
    case_digest: fcb626c0dca4884cc45e23f20e6207f9c9069cfb78eac90f464383c6a8b7396b
    observed: 2026-11-11T19:09:17Z
    duration_seconds: 1
  - id: 2026-11-11-tc-cli-close-gate-drives-native-step
    aliases: OBS-tc-cli-close-gate-drives-native-step-40befe9849980c3cda707578
    claim: "Ran cargo test --locked --test humans -- a_scope_gate_runs_the_approval_once_as_the_configured_human a_review_gate_needs_a_verdict_and_runs_the_review_with_it no_native_closes_a_driving_gate_and_runs_nothing a_specsync_refusal_passes_through_and_nothing_is_written one_gate_record_runs_at_most_one_native_step for TC-cli-close-gate-drives-native-step: passed (exit 0)."
    result: passed
    command: cargo test --locked --test humans -- a_scope_gate_runs_the_approval_once_as_the_configured_human a_review_gate_needs_a_verdict_and_runs_the_review_with_it no_native_closes_a_driving_gate_and_runs_nothing a_specsync_refusal_passes_through_and_nothing_is_written one_gate_record_runs_at_most_one_native_step
    revision: 82a7392dda5b59925f1dad704d2c6c2778b23b8b
    environment: local-macos-arm64
    content: c0ad4a455cdd87cc042886e343d1ad1a31016c6ee2b83c35f413758823f87006
    fingerprint: sha256:62e95539c36cd1667de50d5c1de50111e3e4dc8c3b307e5933805643b1a42852
    case: TC-cli-close-gate-drives-native-step
    case_digest: 8e594a949445e6d02b48ecc64323b883155b4a76d1a4d148dee19013fb695e2c
    observed: 2026-11-11T19:09:13Z
    duration_seconds: 1
  - id: 2026-11-11-tc-cli-ship-reviews-as-human
    aliases: OBS-tc-cli-ship-reviews-as-human-82437920965a295ab87c0fa5
    claim: "Ran cargo test --locked --test change -- ship_with_the_review_relay_ships_closes_the_gate_and_moves_to_done ship_refuses_an_unknown_human_and_stops_on_a_refused_review for TC-cli-ship-reviews-as-human: passed (exit 0)."
    result: passed
    command: cargo test --locked --test change -- ship_with_the_review_relay_ships_closes_the_gate_and_moves_to_done ship_refuses_an_unknown_human_and_stops_on_a_refused_review
    revision: 82a7392dda5b59925f1dad704d2c6c2778b23b8b
    environment: local-macos-arm64
    content: c0ad4a455cdd87cc042886e343d1ad1a31016c6ee2b83c35f413758823f87006
    fingerprint: sha256:3612ecd4d75c49d603aaf7c8a7cc4a44591f7a8d978b7b51b168f751d1d1d3d0
    case: TC-cli-ship-reviews-as-human
    case_digest: da010aec65d78d5c634129a24838b384dbbf6a7ca3f59f929b19cf0a7c6273b7
    observed: 2026-11-11T19:09:15Z
    duration_seconds: 1
  - id: 2026-11-11-claude-code-claude-opus-5-5-auto-mode-hc-t382-ran-the-chough
    aliases: OBS-dry-run-9db85eed3d0c73dbd63de8b1
    claim: "Claude Code (claude-opus-5-5, auto mode, hc t382) ran the Chough-driven approve of plan step 6 (#236, REQ-records-008) itself: chough checkpoint --close-gate on a scope gate held at plan, built from this branch's step 6 source, in a throwaway project with real SpecSync 6.2.0 and one configured SYNTHETIC human. The permission classifier did not refuse it. Chough ran specsync change approve once with --actor SYNTHETIC-human and a note quoting the relayed words, place and recording agent; SpecSync recorded the definition approval (approved, approval_valid true); the gate closed with the relay and a decision naming the step; chough next then showed no pending direct native write."
    result: passed
    revision: 8032bb5a7d5bcabf2a67487378318f684f3a1ed1
    environment: local-macos-arm64
    content: 4c37930521649a9a5b56c117602da4fcf979b379ba6e9c0c6352c833506388aa
    observed: 2026-11-11T18:47:38Z
environment: local-macos-arm64
revision: 99b2664f637137a402777a2e023ee66321ffce62
found_by: "GitHub issue #236; red on 0.5.0 at leif/0.6.0-features-red 82c85c99"
related: []
created: 2026-11-11T19:07:33Z
updated: 2026-11-11T19:09:40Z
---
# A human's gate decision does not drive the native approval as that human (#236)

**What happened:** closing a scope gate with a human's relayed decision records the relay but runs no specsync change approve, so the agent records the approval itself under a hard-coded or agent identity
Actual: no SpecSync call is made; the approval is left to the agent, which the permission classifier refused as self-approval when tried directly as 0xLeif

**Steps:** Configure a human, open a scope gate, close it with --close-gate and that human's relay
1. chough init; chough checkpoint --native-change <id>
2. write .chough/local/identity.toml with one [[human]]
3. chough checkpoint --awaiting-human "Scope approval"
4. chough checkpoint --close-gate "Scope approval" with that human's relay

**Expected:** Chough runs specsync change approve once with the configured human's SpecSync actor and a note quoting their words, before it records the decision

**Why it matters:** Human approvals are recorded under the wrong identity or not at all (GitHub #236)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T19:07:33Z · Reported (GitHub issue #236; red on 0.5.0 at leif/0.6.0-features-red 82c85c99); evidence: OBS-tc-records-approval-as-the-human-f6d8c563d853b097d71c6f4c, OBS-dry-run-e43743fac8e3ffdcf0e1dd26
- 2026-11-11T19:09:40Z · status: open -> verified. Fixed by plan step 6 (PR #238, f70f5727): closing a driving gate runs specsync change approve or review as the configured human (identity from .chough/local/identity.toml or the user file), refusing an unknown or ambiguous human before anything runs. The approval, close-gate and ship cases passed after the 0.5.0 failure; the agent-run dogfood in a throwaway project was not refused by the classifier. The real check with Leif's relayed decision runs at plan step 14.; evidence: OBS-tc-records-approval-as-the-human-444192105d5fc75eaae55dc1, OBS-tc-cli-close-gate-drives-native-step-40befe9849980c3cda707578, OBS-tc-cli-ship-reviews-as-human-82437920965a295ab87c0fa5, OBS-dry-run-9db85eed3d0c73dbd63de8b1
