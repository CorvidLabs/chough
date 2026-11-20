---
kind: session
id: 2026-11-20-g2-gate-summary
aliases: [g2-gate-summary]
objective: "The gate summary counts evidence without a case, shows uncommitted work in its diff and a base for a session started before the first commit; doctor names the journal post to commit; without SpecSync, --to done across plan or accept says no human decided and needs --reason (#389, #390)"
stage: done
stage_status: archived
iteration: 1
native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
pr: "428"
observations:
  - 2026-11-20-tc-cli-g2-evidence-with-no-case-counts
  - 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line
  - 2026-11-20-tc-cli-g2-a-session-before-the-first-commit
  - 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason
  - 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided
  - 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post
  - 2026-11-20-tc-cli-g2-evidence-with-no-case-counts-05d119
  - 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-694a1e
  - 2026-11-20-tc-cli-g2-a-session-before-the-first-commit-1d6151
  - 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-d31721
  - 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-64807b
  - 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-52da8f
  - 2026-11-20-tc-cli-g2-evidence-with-no-case-counts-2af9e7
  - 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-19cf07
  - 2026-11-20-tc-cli-g2-a-session-before-the-first-commit-1afd11
  - 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-dd29b8
  - 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-7daf52
  - 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-7a867b
  - 2026-11-20-tc-cli-g2-a-note-and-a-reason-both-reach-the-timeline
  - 2026-11-20-tc-cli-g2-evidence-with-no-case-counts-a722d2
  - 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-09efc1
  - 2026-11-20-tc-cli-g2-a-session-before-the-first-commit-7014dc
  - 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-d02678
  - 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-96adc3
  - 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-941dd9
  - 2026-11-20-tc-cli-g2-a-note-and-a-reason-both-reach-the-timeline-b7a92d
  - 2026-11-20-tc-cli-g2-an-uncommitted-definition-in-the-diff-line
title: The gate summary tells the truth, and an unattended done needs a reason
summary:
  - "#389: a gate summary's Tests line counts evidence with no case; work not committed yet shows as git diff <base>; a session with no native change counts from the first commit and says why; the unfinished_archive fix names the journal post among the files to commit, and an untracked post never holds the finish (option B kept). #390, Leif's decision B: without SpecSync, a move past plan or accept with no human decision says so, and stage advance --to across one needs --reason, which lands in the Timeline."
  - "Verified: tester2's full PASS at b2031107 on macOS and Linux (F14 at a scope approval fixed); the tester's run of the refusal on 22 real repositories and Chough; corvid-agent's approval at 83ba5d1f and its notes taken; main 562ef738 merged in, own lines the same per file; full lane green at b2031107."
closed: 2026-11-20
outcome: done
---
# The gate summary counts evidence without a case, shows uncommitted work in its diff and a base for a session started before the first commit; doctor names the journal post to commit; without SpecSync, --to done across plan or accept says no human decided and needs --reason (#389, #390)

## Goal

The gate summary counts evidence without a case, shows uncommitted work in its diff and a base for a session started before the first commit; doctor names the journal post to commit; without SpecSync, --to done across plan or accept says no human decided and needs --reason (#389, #390)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason (native approval)": "Approve (Recommended)" (Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 16:04Z, to: 'Chough G2 (#389's gate summary and #390), scope approval. Four truth-telling fixes found on main ... It changes some of your approved words (REQ-cli-061's cases and its diff range) and adds that one refusal, so it's not a standing-OK case. Approve the scope as drafted?' Recorded by t4c itself, at the draft 17e01d68); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason as 0xLeif
- 0xLeif decided "One send-back of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason to narrow its history delta and keep option B": "A: narrow it, keep B (Recommended)" (Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 17:04Z, to: 'Chough G2, a correction (t4b's mistake, caught before building). Its approved change to REQ-history-012 says an untracked journal post holds up doctor --fix's finish. That reverses your option B from Nov 19 (D1: an untracked or ignored journal post does NOT hold the finish), which is on main. Fix by one send-back of the history delta?' The cli delta and the rest of the scope stand on his 'Approve (Recommended)' of about 16:04Z. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason with its history delta narrowed (native approval)": "A: narrow it, keep B (Recommended)" (Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 17:04Z, to: 'Chough G2, a correction (t4b's mistake, caught before building). Its approved change to REQ-history-012 says an untracked journal post holds up doctor --fix's finish. That reverses your option B from Nov 19 (D1: an untracked or ignored journal post does NOT hold the finish), which is on main. Fix by one send-back of the history delta?' The cli delta and the rest of the scope stand on his 'Approve (Recommended)' of about 16:04Z. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason as 0xLeif
- 0xLeif decided "One send-back of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason to add tests/next.rs to its paths": "Standing OK, tell me after" (Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: tests/next.rs joins G2's paths so one older test gives its stage advance --to the --reason G2's approved refusal asks for; no delta word changes, no new refusal. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason with tests/next.rs added to its paths (native approval)": "Standing OK, tell me after" (Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: tests/next.rs joins G2's paths so one older test gives its stage advance --to the --reason G2's approved refusal asks for; no delta word changes, no new refusal. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change approve fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason as 0xLeif
- 0xLeif decided "Implementation review of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason": "Go" (Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with a tester PASS that changes no approved sentence. tester2's full PASS at b2031107 on macOS and Linux: F14 fixed at a scope approval, F4, F5 and #390 passing, the full set 7 fixed and 0 new (the 6 rows that run --to done without --reason are #390's intended refusal). The tester's rule-3 run passed the new refusal (refused:no-human-decision) on the 22 real repositories and Chough, at f547313c. corvid-agent approved #428 at 83ba5d1f. After that: main 562ef738 (#426, #423) merged in at 0e771e62, tests/doctor.rs keeping both sides, t4b's script showing its own changed lines the same per file, the specs only renumbered (cli 132, history 37); corvid-agent's notes taken, a --note given with --reason kept beside it (test first, cfbc67a4) and a design.md note on the narrowed F5 line (eae8f99f); tester2's F14 finding fixed inside the approved words, the diff line counting what What changed counts (test first at a scope approval, 8f0f0403); no approved sentence changed; the full lane green at b2031107. Recorded by t4c); recorded by t4c; a relayed claim, not authenticated; ran specsync change review fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason as 0xLeif with verdict pass

## What was verified

- 19 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1). - 52cb696a0efd; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-g2-evidence-with-no-case-counts-a722d2
- Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1). - b95afc473a9f; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-09efc1
- Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1). - 6616c2471cce; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-g2-a-session-before-the-first-commit-7014dc
- Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1). - be1c7b288741; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-d02678
- Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1). - 4a01c0eadbdd; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-96adc3
- Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1). - afe31bb11024; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-941dd9
- Ran cargo test --locked --test stages -- --exact a_note_and_a_reason_both_reach_the_timeline for TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline: passed (exit 0, tests run 1). - fa7675a941a8; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-g2-a-note-and-a-reason-both-reach-the-timeline-b7a92d
- Ran cargo test --locked --test gates -- --exact an_uncommitted_definition_is_shown_by_the_diff_line_at_scope_approval for TC-cli-g2-an-uncommitted-definition-in-the-diff-line: passed (exit 0, tests run 1). - df45e1360254; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-g2-an-uncommitted-definition-in-the-diff-line

## Bugs

None.

## Open threads

## Timeline

- 2026-11-20T16:02:17Z · start · intent → intent · The gate summary counts evidence without a case, shows uncommitted work in its diff and a base for a session started before the first commit; doctor names the journal post to commit; without SpecSync, --to done across plan or accept says no human decided and needs --reason (#389, #390) ^t-f64d53
  - revision: 7bf110dcca70bdd3565007086aee40a286f40eff
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T16:04:34Z · gate-opened · intent → intent · Scope approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason (native approval) ^t-7422bc
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 17e01d681dd0a57fbc968d589825df64b52dcc7c
  - holds: intent
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T16:04:34Z · gate-closed · intent → intent · Scope approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason (native approval) ^t-a7d620
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 17e01d681dd0a57fbc968d589825df64b52dcc7c
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve (Recommended)
    - where: Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 16:04Z, to: 'Chough G2 (#389's gate summary and #390), scope approval. Four truth-telling fixes found on main ... It changes some of your approved words (REQ-cli-061's cases and its diff range) and adds that one refusal, so it's not a standing-OK case. Approve the scope as drafted?' Recorded by t4c itself, at the draft 17e01d68
    - recorded_by: t4c
    - native: specsync change approve fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T16:04:51Z · advance · intent → design ·  ^t-855b35
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: b614bf138f4021e1d61342f9b7734649e8740a1e
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T16:04:52Z · advance · design → plan ·  ^t-5ddef1
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: b614bf138f4021e1d61342f9b7734649e8740a1e
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T16:04:52Z · advance · plan → build · native:approved ^t-4c59ad
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - gate: native:approved
  - revision: b614bf138f4021e1d61342f9b7734649e8740a1e
  - by: fable (claude-opus-5-5, hc t85)
- 2026-11-20T17:05:12Z · gate-opened · build → build · One send-back of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason to narrow its history delta and keep option B ^t-8c3ed5
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 36e6c6c793d55ba678a5a24c3ffd56ace04fae2e
  - holds: build
  - drives: none
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T17:05:12Z · gate-closed · build → build · One send-back of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason to narrow its history delta and keep option B ^t-377bba
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 36e6c6c793d55ba678a5a24c3ffd56ace04fae2e
  - relayed_by:
    - decided_by: 0xLeif
    - words: A: narrow it, keep B (Recommended)
    - where: Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 17:04Z, to: 'Chough G2, a correction (t4b's mistake, caught before building). Its approved change to REQ-history-012 says an untracked journal post holds up doctor --fix's finish. That reverses your option B from Nov 19 (D1: an untracked or ignored journal post does NOT hold the finish), which is on main. Fix by one send-back of the history delta?' The cli delta and the rest of the scope stand on his 'Approve (Recommended)' of about 16:04Z. Recorded by t4c
    - recorded_by: t4c
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T17:05:13Z · gate-opened · build → build · Re-approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason with its history delta narrowed (native approval) ^t-044a5e
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 36e6c6c793d55ba678a5a24c3ffd56ace04fae2e
  - holds: build
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T17:05:14Z · direct-native-write · build → build · fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-d18206
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 36e6c6c793d55ba678a5a24c3ffd56ace04fae2e
  - direct_write:
    - change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T17:05:14Z · gate-closed · build → build · Re-approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason with its history delta narrowed (native approval) ^t-ac6094
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 36e6c6c793d55ba678a5a24c3ffd56ace04fae2e
  - relayed_by:
    - decided_by: 0xLeif
    - words: A: narrow it, keep B (Recommended)
    - where: Leif's own answer in a Claude Code question dialog in his planning session (hc t4c), 2026-11-20 about 17:04Z, to: 'Chough G2, a correction (t4b's mistake, caught before building). Its approved change to REQ-history-012 says an untracked journal post holds up doctor --fix's finish. That reverses your option B from Nov 19 (D1: an untracked or ignored journal post does NOT hold the finish), which is on main. Fix by one send-back of the history delta?' The cli delta and the rest of the scope stand on his 'Approve (Recommended)' of about 16:04Z. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change approve fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T18:05:11Z · gate-opened · build → build · One send-back of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason to add tests/next.rs to its paths ^t-a1584c
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 1609cd94b56166634cd32d077250ce3247aae4e8
  - holds: build
  - drives: none
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T18:05:11Z · gate-closed · build → build · One send-back of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason to add tests/next.rs to its paths ^t-1fe076
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 1609cd94b56166634cd32d077250ce3247aae4e8
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: tests/next.rs joins G2's paths so one older test gives its stage advance --to the --reason G2's approved refusal asks for; no delta word changes, no new refusal. Recorded by t4c
    - recorded_by: t4c
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T18:05:12Z · gate-opened · build → build · Re-approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason with tests/next.rs added to its paths (native approval) ^t-a68ea7
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 1609cd94b56166634cd32d077250ce3247aae4e8
  - holds: build
  - drives: approve
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T18:05:12Z · direct-native-write · build → build · fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-84efb4
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 1609cd94b56166634cd32d077250ce3247aae4e8
  - direct_write:
    - change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T18:05:12Z · gate-closed · build → build · Re-approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason with tests/next.rs added to its paths (native approval) ^t-65608b
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: 1609cd94b56166634cd32d077250ce3247aae4e8
  - relayed_by:
    - decided_by: 0xLeif
    - words: Standing OK, tell me after
    - where: Leif's standing OK for scope approvals ('Standing OK, tell me after', a Claude Code question dialog in his planning session hc t4c, 2026-11-20 about 04:17Z), confirmed by t4c for this path-only addition: tests/next.rs joins G2's paths so one older test gives its stage advance --to the --reason G2's approved refusal asks for; no delta word changes, no new refusal. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change approve fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T21:34:09Z · advance · build → test · native:verifying ^t-9da70d
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - gate: native:verifying
  - revision: b2031107b4baf73e8bd325b32c7287c3fc36ba69
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T21:34:09Z · advance · test → accept · native:verifying ^t-2df481
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - gate: native:verifying
  - revision: b2031107b4baf73e8bd325b32c7287c3fc36ba69
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T21:34:10Z · gate-opened · accept → accept · Implementation review of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason ^t-555ef4
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: b2031107b4baf73e8bd325b32c7287c3fc36ba69
  - holds: accept
  - drives: review
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T21:35:45Z · gate-closed · accept → accept · Implementation review of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason ^t-2c17b6
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - revision: de6e16cb7371d304c89446f2ec75e284372ca655
  - relayed_by:
    - decided_by: 0xLeif
    - words: Go
    - where: Standing OK 6(b): Leif's 'Go' to the six rules (Claude Code chat message of 2026-11-19, relayed by his planning session hc t4c), recording his implementation review of a batch with a tester PASS that changes no approved sentence. tester2's full PASS at b2031107 on macOS and Linux: F14 fixed at a scope approval, F4, F5 and #390 passing, the full set 7 fixed and 0 new (the 6 rows that run --to done without --reason are #390's intended refusal). The tester's rule-3 run passed the new refusal (refused:no-human-decision) on the 22 real repositories and Chough, at f547313c. corvid-agent approved #428 at 83ba5d1f. After that: main 562ef738 (#426, #423) merged in at 0e771e62, tests/doctor.rs keeping both sides, t4b's script showing its own changed lines the same per file, the specs only renumbered (cli 132, history 37); corvid-agent's notes taken, a --note given with --reason kept beside it (test first, cfbc67a4) and a design.md note on the narrowed F5 line (eae8f99f); tester2's F14 finding fixed inside the approved words, the diff line counting what What changed counts (test first at a scope approval, 8f0f0403); no approved sentence changed; the full lane green at b2031107. Recorded by t4c
    - recorded_by: t4c
    - native: specsync change review fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
    - actor: 0xLeif
  - by: t4c (claude-opus-5-5, hc t4c)
- 2026-11-20T21:35:48Z · advance · accept → done · native:archived ^t-313689
  - native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
  - gate: native:archived
  - revision: de6e16cb7371d304c89446f2ec75e284372ca655
  - by: t4c (claude-opus-5-5, hc t4c)

Closed 2026-11-20: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-20-tc-cli-g2-evidence-with-no-case-counts
    claim: "Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line
    revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
    environment: local-macos-arm64
    content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
    fingerprint: sha256:e5c8afb839e37ee31bbc9a899439281d10535b2d3d4cb36fb7e1f6b392338689
    case: TC-cli-g2-evidence-with-no-case-counts
    case_digest: 951df83095a2697734c253fa89bb31caa1a5ed68454d932a225584d2218ac821
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T17:43:15Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line
    claim: "Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line
    revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
    environment: local-macos-arm64
    content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
    fingerprint: sha256:ea3aa31c0e6195acbb212176525c296f0a6984e0193cdb2d18867dc4ab051289
    case: TC-cli-g2-uncommitted-work-in-the-diff-line
    case_digest: 4b5b405a8ca5af0e5003477e3576a039bec7072d9d9ac8cb244ad7de9d4ff878
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T17:43:18Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-a-session-before-the-first-commit
    claim: "Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it
    revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
    environment: local-macos-arm64
    content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
    fingerprint: sha256:99994a7fd5001b996ce22c591c58fd25accadcfcd58f0fe36258648fa8066e83
    case: TC-cli-g2-a-session-before-the-first-commit
    case_digest: 6aac022451b58ebc4595b293330f52a91eeb8d0eefdcccbe621685e39891842d
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T17:43:20Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason
    claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason
    revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
    environment: local-macos-arm64
    content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
    fingerprint: sha256:085c824626e3c5f36a8da2595ba1093483331d2b8c83509483d38339c6e63cbe
    case: TC-cli-g2-an-unattended-done-needs-a-reason
    case_digest: 9b7afffc63069734d84ed6c521a86bc957cd0b7fd4327fef7f0f55a2004d92c3
    criteria: DRIVE-3
    requirements: REQ-cli-031
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T17:43:23Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided
    claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided
    revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
    environment: local-macos-arm64
    content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
    fingerprint: sha256:3fed28a399110b77c89c4308bfe315e173a5513a02e1ea1c3f9b5c0be62e7991
    case: TC-cli-g2-a-move-past-plan-says-whether-a-human-decided
    case_digest: 2c7ef7f4a08553661e5c362f7169727fe426dffbae8c100f2ea9821c97f150c4
    criteria: DRIVE-3
    requirements: REQ-cli-031
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T17:43:25Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post
    claim: "Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post
    revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
    environment: local-macos-arm64
    content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
    fingerprint: sha256:f297402455e16a111c4602b64a34ca13c260a018292921e133f21be78fb99674
    case: TC-cli-g2-the-fix-names-the-journal-post
    case_digest: 2c32119ea0e9c733d9858ec464529dce4f8dccf61b3b23374207a309994e4dbd
    criteria: DRIVE-17
    requirements: REQ-cli-055
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T17:43:28Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-evidence-with-no-case-counts-05d119
    claim: "Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line
    revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
    environment: local-macos-arm64
    content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
    fingerprint: sha256:ac2b8de6df8ffa77ae680622ddf041ad5fb5c6ea142a0575fccef88b9a9b33e0
    case: TC-cli-g2-evidence-with-no-case-counts
    case_digest: 951df83095a2697734c253fa89bb31caa1a5ed68454d932a225584d2218ac821
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T18:43:42Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-694a1e
    claim: "Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line
    revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
    environment: local-macos-arm64
    content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
    fingerprint: sha256:94f6917d5a3cbbda8f4854144330648ba3ba55b1968508cd6525955355d57154
    case: TC-cli-g2-uncommitted-work-in-the-diff-line
    case_digest: 4b5b405a8ca5af0e5003477e3576a039bec7072d9d9ac8cb244ad7de9d4ff878
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T18:43:44Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-a-session-before-the-first-commit-1d6151
    claim: "Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it
    revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
    environment: local-macos-arm64
    content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
    fingerprint: sha256:27bedee362587b1a42d5f000a6690b800ffd1ee18433f09f6126da3dcb4d48ee
    case: TC-cli-g2-a-session-before-the-first-commit
    case_digest: 6aac022451b58ebc4595b293330f52a91eeb8d0eefdcccbe621685e39891842d
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T18:43:46Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-d31721
    claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason
    revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
    environment: local-macos-arm64
    content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
    fingerprint: sha256:83fba77d849958d305146f80766df4d90c568559cf35cbb979d560261f1df70d
    case: TC-cli-g2-an-unattended-done-needs-a-reason
    case_digest: 9b7afffc63069734d84ed6c521a86bc957cd0b7fd4327fef7f0f55a2004d92c3
    criteria: DRIVE-3
    requirements: REQ-cli-031
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T18:43:49Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-64807b
    claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided
    revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
    environment: local-macos-arm64
    content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
    fingerprint: sha256:0051d90e19884695a6d6fe6386c1aa9b797052eec24b1468a3e66e580ed348ec
    case: TC-cli-g2-a-move-past-plan-says-whether-a-human-decided
    case_digest: 2c7ef7f4a08553661e5c362f7169727fe426dffbae8c100f2ea9821c97f150c4
    criteria: DRIVE-3
    requirements: REQ-cli-031
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T18:43:51Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-52da8f
    claim: "Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post
    revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
    environment: local-macos-arm64
    content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
    fingerprint: sha256:115dbddadec8dc9d88c4a26b6d42209199cca9f7be8ae539537b23d84d71f192
    case: TC-cli-g2-the-fix-names-the-journal-post
    case_digest: 2c32119ea0e9c733d9858ec464529dce4f8dccf61b3b23374207a309994e4dbd
    criteria: DRIVE-17
    requirements: REQ-cli-055
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T18:43:54Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-evidence-with-no-case-counts-2af9e7
    claim: "Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line
    revision: b0006dd3acf176c6e05b870c0213084d93116a71
    environment: local-macos-arm64
    content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
    fingerprint: sha256:11bee782b9751c7d57d8b5ccbed4810123bd27f71ebfcca4d69ddf16e2abfab7
    case: TC-cli-g2-evidence-with-no-case-counts
    case_digest: 951df83095a2697734c253fa89bb31caa1a5ed68454d932a225584d2218ac821
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T19:56:56Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-19cf07
    claim: "Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line
    revision: b0006dd3acf176c6e05b870c0213084d93116a71
    environment: local-macos-arm64
    content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
    fingerprint: sha256:e2887ba56b8776981f5d60d7cac80b354dd83c56ed71e8b538cbff5b409764ab
    case: TC-cli-g2-uncommitted-work-in-the-diff-line
    case_digest: 4b5b405a8ca5af0e5003477e3576a039bec7072d9d9ac8cb244ad7de9d4ff878
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T19:56:59Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-a-session-before-the-first-commit-1afd11
    claim: "Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it
    revision: b0006dd3acf176c6e05b870c0213084d93116a71
    environment: local-macos-arm64
    content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
    fingerprint: sha256:27bedee362587b1a42d5f000a6690b800ffd1ee18433f09f6126da3dcb4d48ee
    case: TC-cli-g2-a-session-before-the-first-commit
    case_digest: 6aac022451b58ebc4595b293330f52a91eeb8d0eefdcccbe621685e39891842d
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T19:57:01Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-dd29b8
    claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason
    revision: b0006dd3acf176c6e05b870c0213084d93116a71
    environment: local-macos-arm64
    content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
    fingerprint: sha256:45c57fce3c351ce48a63c37736e22a419e01e4e0fb4b39514b44083b0b9c7b23
    case: TC-cli-g2-an-unattended-done-needs-a-reason
    case_digest: 9b7afffc63069734d84ed6c521a86bc957cd0b7fd4327fef7f0f55a2004d92c3
    criteria: DRIVE-3
    requirements: REQ-cli-031
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T19:57:04Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-7daf52
    claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided
    revision: b0006dd3acf176c6e05b870c0213084d93116a71
    environment: local-macos-arm64
    content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
    fingerprint: sha256:fd4335e1d7a4ad070c43421a612956fdf5f7c8a82f4294c4dea4b66b700e4e41
    case: TC-cli-g2-a-move-past-plan-says-whether-a-human-decided
    case_digest: 2c7ef7f4a08553661e5c362f7169727fe426dffbae8c100f2ea9821c97f150c4
    criteria: DRIVE-3
    requirements: REQ-cli-031
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T19:57:06Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-7a867b
    claim: "Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post
    revision: b0006dd3acf176c6e05b870c0213084d93116a71
    environment: local-macos-arm64
    content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
    fingerprint: sha256:bfdae2a0504e2942b8b032e131b5abc596b8b7645f0fafe842cab87406d0b3bc
    case: TC-cli-g2-the-fix-names-the-journal-post
    case_digest: 2c32119ea0e9c733d9858ec464529dce4f8dccf61b3b23374207a309994e4dbd
    criteria: DRIVE-17
    requirements: REQ-cli-055
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T19:57:09Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-a-note-and-a-reason-both-reach-the-timeline
    claim: "Ran cargo test --locked --test stages -- --exact a_note_and_a_reason_both_reach_the_timeline for TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_note_and_a_reason_both_reach_the_timeline
    revision: b0006dd3acf176c6e05b870c0213084d93116a71
    environment: local-macos-arm64
    content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
    fingerprint: sha256:6db6a50ec044a267f5a488499b760123a1732a2b2642355f6b394e93b20bfbe2
    case: TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline
    case_digest: 9ddeedc00bc2af4e31acf464ca345e745a6eb90be21189316fdfe1e3228ab0b5
    criteria: DRIVE-3
    requirements: REQ-cli-031
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T19:57:11Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-evidence-with-no-case-counts-a722d2
    claim: "Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line
    revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
    environment: local-macos-arm64
    content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
    fingerprint: sha256:52cb696a0efd0a58231d3fd6430c2148af7d9d4f54d1fabd447b0bbd353dd83f
    case: TC-cli-g2-evidence-with-no-case-counts
    case_digest: 951df83095a2697734c253fa89bb31caa1a5ed68454d932a225584d2218ac821
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T21:09:38Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-09efc1
    claim: "Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line
    revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
    environment: local-macos-arm64
    content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
    fingerprint: sha256:b95afc473a9f4f1a719a3d5eff2fc994f655abc1b78f0f130f685967cc4e9cf7
    case: TC-cli-g2-uncommitted-work-in-the-diff-line
    case_digest: 4b5b405a8ca5af0e5003477e3576a039bec7072d9d9ac8cb244ad7de9d4ff878
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T21:09:41Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-a-session-before-the-first-commit-7014dc
    claim: "Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it
    revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
    environment: local-macos-arm64
    content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
    fingerprint: sha256:6616c2471cce2e1b8a914f4a2436dc8028740460132bdd5b1f4b9d899deaa1a6
    case: TC-cli-g2-a-session-before-the-first-commit
    case_digest: 6aac022451b58ebc4595b293330f52a91eeb8d0eefdcccbe621685e39891842d
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T21:09:43Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-d02678
    claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason
    revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
    environment: local-macos-arm64
    content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
    fingerprint: sha256:be1c7b2887419e7d3ecde1449fd019c5d24b06e0934536210e35e0bc17b63a2f
    case: TC-cli-g2-an-unattended-done-needs-a-reason
    case_digest: 9b7afffc63069734d84ed6c521a86bc957cd0b7fd4327fef7f0f55a2004d92c3
    criteria: DRIVE-3
    requirements: REQ-cli-031
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T21:09:45Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-96adc3
    claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided
    revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
    environment: local-macos-arm64
    content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
    fingerprint: sha256:4a01c0eadbdd98c6f12ba69a8c1c56856bc8a3506f4021864673fbd9f35755c6
    case: TC-cli-g2-a-move-past-plan-says-whether-a-human-decided
    case_digest: 2c7ef7f4a08553661e5c362f7169727fe426dffbae8c100f2ea9821c97f150c4
    criteria: DRIVE-3
    requirements: REQ-cli-031
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T21:09:47Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-941dd9
    claim: "Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post
    revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
    environment: local-macos-arm64
    content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
    fingerprint: sha256:afe31bb11024c7218d5f3a8acf2a53018d7e78cdffdbf7736ef7ad6be2732c1a
    case: TC-cli-g2-the-fix-names-the-journal-post
    case_digest: 2c32119ea0e9c733d9858ec464529dce4f8dccf61b3b23374207a309994e4dbd
    criteria: DRIVE-17
    requirements: REQ-cli-055
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T21:09:50Z
    duration_seconds: 1
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-a-note-and-a-reason-both-reach-the-timeline-b7a92d
    claim: "Ran cargo test --locked --test stages -- --exact a_note_and_a_reason_both_reach_the_timeline for TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact a_note_and_a_reason_both_reach_the_timeline
    revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
    environment: local-macos-arm64
    content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
    fingerprint: sha256:fa7675a941a855ad50481a7bb46a9ace57e4cb472b94942e044652277a2db6f3
    case: TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline
    case_digest: 9ddeedc00bc2af4e31acf464ca345e745a6eb90be21189316fdfe1e3228ab0b5
    criteria: DRIVE-3
    requirements: REQ-cli-031
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T21:09:52Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
  - id: 2026-11-20-tc-cli-g2-an-uncommitted-definition-in-the-diff-line
    claim: "Ran cargo test --locked --test gates -- --exact an_uncommitted_definition_is_shown_by_the_diff_line_at_scope_approval for TC-cli-g2-an-uncommitted-definition-in-the-diff-line: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test gates -- --exact an_uncommitted_definition_is_shown_by_the_diff_line_at_scope_approval
    revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
    environment: local-macos-arm64
    content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
    fingerprint: sha256:df45e136025426f6755051fb5244ddee93b5231871f5cf1a00190ffc57e16036
    case: TC-cli-g2-an-uncommitted-definition-in-the-diff-line
    case_digest: 67e7c44996f40f1e5d4511dc7ee370aa407802ca65c362e0732c7d6e905721db
    criteria: DRIVE-24
    requirements: REQ-cli-061
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T21:09:54Z
    duration_seconds: 0
    actor: fable (claude-opus-5-5, hc t85)
```
