---
kind: bug
id: 2026-11-12-doctor-reports-timeline-rewritten-for-a-session-archived-since
aliases:
  - BUG-doctor-reports-timeline-rewritten-for-a-session-archived-since-head-bec-5237c60f52bdfc8462d503c6
title: doctor reports timeline_rewritten for a session archived since HEAD, because the story's Closed line joins its last Timeline entry
status: verified
classification: product-defect
scope: in-scope
impact: A false finding with a 'restore by hand' remedy after every archive
criteria: []
requirements: [REQ-history-012]
cases: []
evidence:
  - OBS-tc-history-an-archived-story-is-no-rewritten-timeline-bfc342bfa43aef017807aa33
  - OBS-tc-history-an-archived-story-is-no-rewritten-timeline-e2d8ce2bdd5041fccfae4d21
receipts:
  - id: 2026-11-12-tc-history-an-archived-story-is-no-rewritten-timeline
    aliases: OBS-tc-history-an-archived-story-is-no-rewritten-timeline-bfc342bfa43aef017807aa33
    claim: "Ran cargo test --locked --test doctor -- --exact a_session_archived_since_head_is_not_a_rewritten_timeline for TC-history-an-archived-story-is-no-rewritten-timeline: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test doctor -- --exact a_session_archived_since_head_is_not_a_rewritten_timeline
    revision: 9a8f2d236fb5ed69a2ec6b03a0f147a6c32825d0
    environment: local-macos-arm64
    content: 39e050e51f4ea53ba2e0657775849eed3be653aec9e5995a200385d77bfb6fe8
    fingerprint: sha256:f670fc8cfbaa975c7810060b46698d14f1769d759d35d4a18f20890f9edb7b0e
    case: TC-history-an-archived-story-is-no-rewritten-timeline
    case_digest: 98da34fb4ef49d626dcc187ba75799597ceb3c2515e00a16e070fbfaea7d50cb
    observed: 2026-11-12T07:06:38Z
    duration_seconds: 1
  - id: 2026-11-12-tc-history-an-archived-story-is-no-rewritten-timeline-e1c437
    aliases: OBS-tc-history-an-archived-story-is-no-rewritten-timeline-e2d8ce2bdd5041fccfae4d21
    claim: "Ran cargo test --locked --test doctor -- --exact a_session_archived_since_head_is_not_a_rewritten_timeline for TC-history-an-archived-story-is-no-rewritten-timeline: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_session_archived_since_head_is_not_a_rewritten_timeline
    revision: f32abb7663c87268536e9a6970ece97b0e6e2655
    environment: local-macos-arm64
    content: eff4dd5b2cb71f3fd7a00a69476766c719a4a3e2feb8bd907b74d68db7890b65
    fingerprint: sha256:ce3c17fa32ddace98fd531d223b09f6579942ffaee9af41d12c4b4fdc510110f
    case: TC-history-an-archived-story-is-no-rewritten-timeline
    case_digest: 98da34fb4ef49d626dcc187ba75799597ceb3c2515e00a16e070fbfaea7d50cb
    observed: 2026-11-12T07:06:55Z
    duration_seconds: 1
environment: local-macos-arm64, tester build at 596ced67, debug, SpecSync 6.3.0
revision: 6e8a7b9d8ad0421009c60717bb8f532d3e0ddea5
found_by: t37f, 6.3.0 re-run, area knowledge
related: []
created: 2026-11-12T07:05:00Z
updated: 2026-11-12T07:06:55Z
---
# doctor reports timeline_rewritten for a session archived since HEAD, because the story's Closed line joins its last Timeline entry

**What happened:** Right after a session is archived (by close, done or doctor --fix) and before the removal is committed, doctor reports timeline_rewritten on the story's last entry and exits 1, though no entry changed
Actual: timeline_entries appends any non-entry line, including the story's 'Closed <date>: <outcome>.' paragraph, to the last entry

**Steps:** Commit a session, archive it, then chough doctor before committing

**Expected:** timeline_rewritten only when an entry at HEAD differs (REQ-history-012, REQ-history-002)

**Why it matters:** A false finding with a 'restore by hand' remedy after every archive

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T07:05:00Z · Reported (t37f, 6.3.0 re-run, area knowledge)
- 2026-11-12T07:06:55Z · status: open -> investigating. Red at 9a8f2d23 (before its fix): doctor reported timeline_rewritten for a story archived since HEAD; fix in f32abb76.; evidence: OBS-tc-history-an-archived-story-is-no-rewritten-timeline-bfc342bfa43aef017807aa33
- 2026-11-12T07:06:55Z · status: investigating -> verified. Green at f32abb76: TC-history-an-archived-story-is-no-rewritten-timeline passed; doctor, correct and history tests pass.; evidence: OBS-tc-history-an-archived-story-is-no-rewritten-timeline-e2d8ce2bdd5041fccfae4d21
