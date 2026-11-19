---
kind: session
id: 2026-11-19-doctor-fix-safety
aliases: [doctor-fix-safety]
objective: "doctor --fix never destroys or hangs: the archive finish keeps evidence its story does not hold (#381) and removes OS metadata (#377), next names the archive finish (#371), doctor names front-matter conflicts and gives no restore advice for a redaction (#374), and the spec walk stays inside the project and is bounded (#402)"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: doctor-fix-safety
pr: ""
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-cli-021, REQ-records-001, REQ-records-014]
cases: []
design: []
files: []
observations:
  - 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not
  - 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept
  - 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish
  - 2026-11-19-tc-history-a-front-matter-conflict-is-named
  - 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten
  - 2026-11-19-tc-history-the-spec-walk-follows-no-link-and-is-bounded
  - 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-cad975
  - 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-d21527
  - 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-40c735
  - 2026-11-19-tc-history-next-names-finishing-the-archive
  - 2026-11-19-tc-history-a-front-matter-conflict-is-named-67d461
  - 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-ac9ac5
  - 2026-11-19-tc-history-the-spec-walk-follows-no-link-and-is-bounded-9bfc91
  - 2026-11-19-tc-history-a-front-matter-conflict-is-named-f1e33c
  - 2026-11-19-tc-history-the-spec-walk-follows-no-link-and-is-bounded-672851
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Gate \"Re-approval of doctor-fix-safety for the #381 screenshot wording\" closed (approved); run `chough stage advance` to move on."
---
# doctor --fix never destroys or hangs: the archive finish keeps evidence its story does not hold (#381) and removes OS metadata (#377), next names the archive finish (#371), doctor names front-matter conflicts and gives no restore advice for a redaction (#374), and the spec walk stays inside the project and is bounded (#402)

## Goal

doctor --fix never destroys or hangs: the archive finish keeps evidence its story does not hold (#381) and removes OS metadata (#377), next names the archive finish (#371), doctor names front-matter conflicts and gives no restore advice for a redaction (#374), and the spec walk stays inside the project and is bounded (#402)

## Where it stands

## Decisions and why

- 0xLeif decided "Scope approval of doctor-fix-safety": "Approve, with the 3 recommended choices" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, his scope approval of D1 'doctor --fix must not destroy or hang' (#381, #402, #377, #371, #374) for the deltas at bf9f70c, with the three recommended choices: #381 keeps and names the entry, attaching it to the story being later and its own issue; #371 next says commit, then doctor --fix; #374 a Timeline entry changed only by redaction is not reported as rewritten, checked by redacting HEAD's text again); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve doctor-fix-safety as 0xLeif
- 0xLeif decided "Re-approval of doctor-fix-safety for the #381 criterion wording": "Yes, fix the wording" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18: REQ-history-012's #381 criterion now ends 'naming each and the way out by hand; the kept files are byte-identical, and after the way out the evidence entry resolves again (#381)', since a kept entry in an archived session's folder resolves only once moved into an open session; nothing else in the delta changed); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve doctor-fix-safety as 0xLeif
- 0xLeif decided "Re-approval of doctor-fix-safety for the #381 inbox way out": "Yes, fix the wording" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, on D1's #381 inbox way out: an unfolded inbox entry's way out is now 'add its text again in an open session with chough inbox add, then drop the kept copy with git rm in a reviewed commit', in REQ-history-012 and in the finding, since an inbox entry names its session and cannot be moved; nothing else changed); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve doctor-fix-safety as 0xLeif
- 0xLeif decided "Re-approval of doctor-fix-safety for the #381 screenshot wording": "Yes" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, on D1's #381 log wording: in both places in REQ-history-012, what doctor --fix keeps and the git mv way out, 'with its log and screenshot' became 'with the screenshot beside it', since an entry's log lives in .chough/local/logs and never in the session folder; nothing else changed); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve doctor-fix-safety as 0xLeif

## What was verified

- [Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not.md)
- [Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept.md)
- [Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish.md)
- [Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-a-front-matter-conflict-is-named.md)
- [Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten.md)
- [Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-history-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-the-spec-walk-follows-no-link-and-is-bounded.md)
- [Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-cad975.md)
- [Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-d21527.md)
- [Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-40c735.md)
- [Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-next-names-finishing-the-archive.md)
- [Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-a-front-matter-conflict-is-named-67d461.md)
- [Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-ac9ac5.md)
- [Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-history-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-the-spec-walk-follows-no-link-and-is-bounded-9bfc91.md)
- [Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-a-front-matter-conflict-is-named-f1e33c.md)
- [Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-history-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1).](evidence/2026-11-19-tc-history-the-spec-walk-follows-no-link-and-is-bounded-672851.md)

## Open threads

## Timeline

- 2026-11-19T02:52:53Z · start · intent → intent · doctor --fix never destroys or hangs: the archive finish keeps evidence its story does not hold (#381) and removes OS metadata (#377), next names the archive finish (#371), doctor names front-matter conflicts and gives no restore advice for a redaction (#374), and the spec walk stays inside the project and is bounded (#402) ^t-ff9a59
  - revision: 6936f88f47a2c69f9febf15822678f1d804ff027
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:52:53Z · advance · intent → design ·  ^t-83fe0e
  - revision: 6936f88f47a2c69f9febf15822678f1d804ff027
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T02:52:53Z · skip · design → plan · Doctor and next behaviour; nothing visual or interactive changes ^t-9a591f
  - revision: 6936f88f47a2c69f9febf15822678f1d804ff027
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:22:54Z · gate-opened · plan → plan · Scope approval of doctor-fix-safety ^t-8ed6a7
  - native_change: doctor-fix-safety
  - revision: bf9f70cdb6e723c070b4cb660d9b43aa4a705a58
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:22:54Z · gate-closed · plan → plan · Scope approval of doctor-fix-safety ^t-b60f44
  - native_change: doctor-fix-safety
  - revision: bf9f70cdb6e723c070b4cb660d9b43aa4a705a58
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve, with the 3 recommended choices
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, his scope approval of D1 'doctor --fix must not destroy or hang' (#381, #402, #377, #371, #374) for the deltas at bf9f70c, with the three recommended choices: #381 keeps and names the entry, attaching it to the story being later and its own issue; #371 next says commit, then doctor --fix; #374 a Timeline entry changed only by redaction is not reported as rewritten, checked by redacting HEAD's text again
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve doctor-fix-safety
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:22:55Z · advance · plan → build · native:approved ^t-38ef65
  - native_change: doctor-fix-safety
  - gate: native:approved
  - revision: bf9f70cdb6e723c070b4cb660d9b43aa4a705a58
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:25:36Z · gate-opened · build → build · Re-approval of doctor-fix-safety for the #381 criterion wording ^t-0f3884
  - native_change: doctor-fix-safety
  - revision: 98e8e0c35305897b5362a983015467df525db4e0
  - holds: build
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:25:36Z · gate-closed · build → build · Re-approval of doctor-fix-safety for the #381 criterion wording ^t-847ee6
  - native_change: doctor-fix-safety
  - revision: 98e8e0c35305897b5362a983015467df525db4e0
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, fix the wording
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18: REQ-history-012's #381 criterion now ends 'naming each and the way out by hand; the kept files are byte-identical, and after the way out the evidence entry resolves again (#381)', since a kept entry in an archived session's folder resolves only once moved into an open session; nothing else in the delta changed
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve doctor-fix-safety
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:32:19Z · gate-opened · build → build · Re-approval of doctor-fix-safety for the #381 inbox way out ^t-8ffd8f
  - native_change: doctor-fix-safety
  - revision: 18f4b1755d4d5999189fe2ebd1674bd381935d70
  - holds: build
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:32:20Z · gate-closed · build → build · Re-approval of doctor-fix-safety for the #381 inbox way out ^t-fb72df
  - native_change: doctor-fix-safety
  - revision: 18f4b1755d4d5999189fe2ebd1674bd381935d70
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, fix the wording
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, on D1's #381 inbox way out: an unfolded inbox entry's way out is now 'add its text again in an open session with chough inbox add, then drop the kept copy with git rm in a reviewed commit', in REQ-history-012 and in the finding, since an inbox entry names its session and cannot be moved; nothing else changed
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve doctor-fix-safety
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:37:42Z · gate-opened · build → build · Re-approval of doctor-fix-safety for the #381 screenshot wording ^t-1ba198
  - native_change: doctor-fix-safety
  - revision: a6c97eb528a176798513cd5792e929848a41ab36
  - holds: build
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T04:37:42Z · gate-closed · build → build · Re-approval of doctor-fix-safety for the #381 screenshot wording ^t-daf78c
  - native_change: doctor-fix-safety
  - revision: a6c97eb528a176798513cd5792e929848a41ab36
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes
    - where: Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, on D1's #381 log wording: in both places in REQ-history-012, what doctor --fix keeps and the git mv way out, 'with its log and screenshot' became 'with the screenshot beside it', since an entry's log lives in .chough/local/logs and never in the session folder; nothing else changed
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve doctor-fix-safety
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
