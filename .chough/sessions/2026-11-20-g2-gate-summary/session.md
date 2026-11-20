---
kind: session
id: 2026-11-20-g2-gate-summary
aliases: [g2-gate-summary]
objective: "The gate summary counts evidence without a case, shows uncommitted work in its diff and a base for a session started before the first commit; doctor names the journal post to commit; without SpecSync, --to done across plan or accept says no human decided and needs --reason (#389, #390)"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
pr: "428"
criteria: []
requirements: []
cases: []
design: []
files: []
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
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Re-approval of fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason with tests/next.rs added to its paths (native approval)\" closed (approved); run `chough stage advance` to move on."
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

## What was verified

- [Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-evidence-with-no-case-counts.md)
- [Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line.md)
- [Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-a-session-before-the-first-commit.md)
- [Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason.md)
- [Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided.md)
- [Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-the-fix-names-the-journal-post.md)
- [Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-evidence-with-no-case-counts-05d119.md)
- [Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-694a1e.md)
- [Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-a-session-before-the-first-commit-1d6151.md)
- [Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-d31721.md)
- [Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-64807b.md)
- [Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-52da8f.md)
- [Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-evidence-with-no-case-counts-2af9e7.md)
- [Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-19cf07.md)
- [Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-a-session-before-the-first-commit-1afd11.md)
- [Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-dd29b8.md)
- [Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-7daf52.md)
- [Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-7a867b.md)
- [Ran cargo test --locked --test stages -- --exact a_note_and_a_reason_both_reach_the_timeline for TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-a-note-and-a-reason-both-reach-the-timeline.md)
- [Ran cargo test --locked --test gates -- --exact evidence_with_no_case_counts_in_the_tests_line for TC-cli-g2-evidence-with-no-case-counts: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-evidence-with-no-case-counts-a722d2.md)
- [Ran cargo test --locked --test gates -- --exact uncommitted_work_is_shown_by_the_diff_line for TC-cli-g2-uncommitted-work-in-the-diff-line: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-uncommitted-work-in-the-diff-line-09efc1.md)
- [Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-a-session-before-the-first-commit-7014dc.md)
- [Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-d02678.md)
- [Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-96adc3.md)
- [Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-941dd9.md)
- [Ran cargo test --locked --test stages -- --exact a_note_and_a_reason_both_reach_the_timeline for TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-a-note-and-a-reason-both-reach-the-timeline-b7a92d.md)
- [Ran cargo test --locked --test gates -- --exact an_uncommitted_definition_is_shown_by_the_diff_line_at_scope_approval for TC-cli-g2-an-uncommitted-definition-in-the-diff-line: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-g2-an-uncommitted-definition-in-the-diff-line.md)

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
