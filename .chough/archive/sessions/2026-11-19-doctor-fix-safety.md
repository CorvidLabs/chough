---
kind: session
id: 2026-11-19-doctor-fix-safety
aliases: [doctor-fix-safety]
objective: "doctor --fix never destroys or hangs: the archive finish keeps evidence its story does not hold (#381) and removes OS metadata (#377), next names the archive finish (#371), doctor names front-matter conflicts and gives no restore advice for a redaction (#374), and the spec walk stays inside the project and is bounded (#402)"
stage: done
stage_status: archived
iteration: 1
native_change: doctor-fix-safety
criteria: [DRIVE-17]
requirements: [REQ-history-012, REQ-cli-021, REQ-records-001, REQ-records-014]
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
  - 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-38c262
  - 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-4ba00c
  - 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-a0e4bb
  - 2026-11-19-tc-history-next-names-finishing-the-archive-fc13d1
  - 2026-11-19-tc-history-a-front-matter-conflict-is-named-cb1377
  - 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-e220ba
  - 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its
  - 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor
  - 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed
  - 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish
  - 2026-11-19-tc-cli-next-names-finishing-the-archive
  - 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open
  - 2026-11-19-tc-cli-the-finish-command-quotes-each-path
  - 2026-11-19-tc-cli-the-archive-line-runs-as-printed
  - 2026-11-19-tc-cli-the-printed-command-commits-the-story
  - 2026-11-19-tc-records-a-front-matter-conflict-is-named
  - 2026-11-19-tc-records-the-spec-walk-follows-no-link-and-is-bounded
  - 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-e7c642
  - 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-b7d61f
  - 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-0c69e2
  - 2026-11-19-tc-history-next-names-finishing-the-archive-8afceb
  - 2026-11-19-tc-history-a-front-matter-conflict-is-named-5d2141
  - 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-8f0184
  - 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-ba11f2
  - 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-3cb989
  - 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-2cd41f
  - 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-a6651f
  - 2026-11-19-tc-history-an-ignored-story-holds-the-finish
  - 2026-11-19-tc-history-an-ignored-journal-post-holds-the-finish
  - 2026-11-19-tc-history-a-failed-git-read-holds-the-finish
  - 2026-11-19-tc-cli-next-names-finishing-the-archive-a7d9b2
  - 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-914865
  - 2026-11-19-tc-cli-the-finish-command-quotes-each-path-ca5f3c
  - 2026-11-19-tc-cli-the-archive-line-runs-as-printed-744a60
  - 2026-11-19-tc-cli-the-printed-command-commits-the-story-5bc003
  - 2026-11-19-tc-records-a-front-matter-conflict-is-named-d7da54
  - 2026-11-19-tc-records-the-spec-walk-follows-no-link-and-is-bounded-d43aba
  - 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-72a5d9
  - 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-557452
  - 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-a0e63d
  - 2026-11-19-tc-history-next-names-finishing-the-archive-c8420c
  - 2026-11-19-tc-history-a-front-matter-conflict-is-named-326c94
  - 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-456e2b
  - 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-422bab
  - 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-657ce6
  - 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-73688d
  - 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-86035d
  - 2026-11-19-tc-history-an-ignored-story-holds-the-finish-2f1763
  - 2026-11-19-tc-history-an-ignored-journal-post-holds-the-finish-52422e
  - 2026-11-19-tc-history-a-failed-git-read-holds-the-finish-662b6d
  - 2026-11-19-tc-history-an-unreadable-entry-is-dropped-by-the-printed
  - 2026-11-19-tc-history-a-redacting-save-leaves-no-timeline-rewrite
  - 2026-11-19-tc-cli-next-names-finishing-the-archive-c58ccb
  - 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-fb684d
  - 2026-11-19-tc-cli-the-finish-command-quotes-each-path-57dcf0
  - 2026-11-19-tc-cli-the-archive-line-runs-as-printed-0eeb56
  - 2026-11-19-tc-cli-the-printed-command-commits-the-story-e0f5f6
  - 2026-11-19-tc-records-a-front-matter-conflict-is-named-137168
  - 2026-11-19-tc-records-the-spec-walk-follows-no-link-and-is-bounded-6ce997
  - 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-ab3fd3
  - 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-ea5b56
  - 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-a709b2
  - 2026-11-19-tc-history-next-names-finishing-the-archive-ae56ea
  - 2026-11-19-tc-history-a-front-matter-conflict-is-named-0d34a5
  - 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-1ec7fb
  - 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-f81635
  - 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-614d92
  - 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-c9319e
  - 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-970e31
  - 2026-11-19-tc-history-an-ignored-story-holds-the-finish-00280e
  - 2026-11-19-tc-history-an-ignored-journal-post-does-not-hold-the-finish
  - 2026-11-19-tc-history-a-failed-git-read-holds-the-finish-6107ca
  - 2026-11-19-tc-history-an-unreadable-entry-is-dropped-by-the-printed-e874c0
  - 2026-11-19-tc-history-a-redacting-save-leaves-no-timeline-rewrite-d78011
  - 2026-11-19-tc-history-a-kept-entry-is-kept-by-its-file
  - 2026-11-19-tc-history-next-finishes-an-ignored-story-as-printed
  - 2026-11-19-tc-history-the-way-out-names-the-open-sessions
  - 2026-11-19-tc-cli-next-names-finishing-the-archive-3b567b
  - 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-bf2efa
  - 2026-11-19-tc-cli-the-finish-command-quotes-each-path-0ab8d3
  - 2026-11-19-tc-cli-the-archive-line-runs-as-printed-4c90e0
  - 2026-11-19-tc-cli-the-printed-command-commits-the-story-16ebcd
  - 2026-11-19-tc-records-a-front-matter-conflict-is-named-8aa642
  - 2026-11-19-tc-records-the-spec-walk-follows-no-link-and-is-bounded-20c91e
  - 2026-11-19-tc-cli-next-names-finishing-the-archive-fc1037
  - 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-c680de
  - 2026-11-19-tc-cli-the-finish-command-quotes-each-path-c735ad
  - 2026-11-19-tc-cli-the-archive-line-runs-as-printed-5b625b
  - 2026-11-19-tc-cli-the-printed-command-commits-the-story-a2638f
  - 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-cbe02e
  - 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-3f536d
  - 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-55bf2f
  - 2026-11-19-tc-history-next-names-finishing-the-archive-6264fb
  - 2026-11-19-tc-history-a-front-matter-conflict-is-named-e19237
  - 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-a6832a
  - 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-3cffa7
  - 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-3b0644
  - 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-0c0669
  - 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-67f44b
  - 2026-11-19-tc-history-an-ignored-story-holds-the-finish-201e6d
  - 2026-11-19-tc-history-an-ignored-journal-post-does-not-hold-the-finish-82b94f
  - 2026-11-19-tc-history-a-failed-git-read-holds-the-finish-7038fa
  - 2026-11-19-tc-history-an-unreadable-entry-is-dropped-by-the-printed-edd175
  - 2026-11-19-tc-history-a-redacting-save-leaves-no-timeline-rewrite-46f04e
  - 2026-11-19-tc-history-a-kept-entry-is-kept-by-its-file-4c635b
  - 2026-11-19-tc-history-next-finishes-an-ignored-story-as-printed-4b7c95
  - 2026-11-19-tc-history-the-way-out-names-the-open-sessions-4107d9
  - 2026-11-19-tc-history-an-untracked-kept-entry-is-committed-first
  - 2026-11-19-tc-history-an-inbox-entry-is-kept-whatever-its-name
  - 2026-11-19-tc-history-a-folded-inbox-entry-does-not-hold-the-finish
  - 2026-11-19-tc-history-an-inbox-file-inbox-add-would-not-take-is-only
  - 2026-11-19-tc-records-a-front-matter-conflict-is-named-610b0b
  - 2026-11-19-tc-records-the-spec-walk-follows-no-link-and-is-bounded-0cea68
  - 2026-11-20-tc-cli-next-names-finishing-the-archive
  - 2026-11-20-tc-cli-next-names-the-archive-with-a-session-open
  - 2026-11-20-tc-cli-the-finish-command-quotes-each-path
  - 2026-11-20-tc-cli-the-archive-line-runs-as-printed
  - 2026-11-20-tc-cli-the-printed-command-commits-the-story
  - 2026-11-20-tc-history-the-archive-finish-keeps-what-the-story-does-not
  - 2026-11-20-tc-history-an-entry-that-differs-from-its-receipt-is-kept
  - 2026-11-20-tc-history-macos-metadata-never-blocks-the-archive-finish
  - 2026-11-20-tc-history-next-names-finishing-the-archive
  - 2026-11-20-tc-history-a-front-matter-conflict-is-named
  - 2026-11-20-tc-history-a-redacted-timeline-entry-is-not-rewritten
  - 2026-11-20-tc-history-a-story-an-earlier-release-wrote-still-holds-its
  - 2026-11-20-tc-history-an-unreadable-file-never-stops-doctor
  - 2026-11-20-tc-history-the-finish-waits-until-the-story-is-committed
  - 2026-11-20-tc-history-a-story-edited-after-its-commit-holds-the-finish
  - 2026-11-20-tc-history-an-ignored-story-holds-the-finish
  - 2026-11-20-tc-history-an-ignored-journal-post-does-not-hold-the-finish
  - 2026-11-20-tc-history-a-failed-git-read-holds-the-finish
  - 2026-11-20-tc-history-an-unreadable-entry-is-dropped-by-the-printed
  - 2026-11-20-tc-history-a-redacting-save-leaves-no-timeline-rewrite
  - 2026-11-20-tc-history-a-kept-entry-is-kept-by-its-file
  - 2026-11-20-tc-history-next-finishes-an-ignored-story-as-printed
  - 2026-11-20-tc-history-the-way-out-names-the-open-sessions
  - 2026-11-20-tc-history-an-untracked-kept-entry-is-committed-first
  - 2026-11-20-tc-history-an-inbox-entry-is-kept-whatever-its-name
  - 2026-11-20-tc-history-a-folded-inbox-entry-does-not-hold-the-finish
  - 2026-11-20-tc-history-an-inbox-file-inbox-add-would-not-take-is-only
  - 2026-11-20-tc-records-a-front-matter-conflict-is-named
  - 2026-11-20-tc-records-the-spec-walk-follows-no-link-and-is-bounded
closed: 2026-11-20
outcome: done
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
- 0xLeif decided "Send back doctor-fix-safety for the two sentences and four paths, run by the fixer": "The fixer runs them, on my word" (Claude Code chat answer, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 14:17Z); recorded by chough-fixer-t4g; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of doctor-fix-safety for the receipt and #389 sentences and four paths": "Yes" (Claude Code chat answer, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 14:08Z, to: may doctor-fix-safety be sent back once for the two sentences (the receipt-fields sentence and the #389 sentence with its criterion) and the four paths tests/reduce.rs, src/evidence.rs, .chough/cases/records.doctor-fix-safety.cases.toml and .chough/cases/cli.doctor-fix-safety.cases.toml); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve doctor-fix-safety as 0xLeif
- 0xLeif decided "Standing OK (d): send back doctor-fix-safety to rebase its delta onto main after #407": "yes to both" (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:02Z, to t4c's proposed standing OK (d): when an approved delta goes stale only because main changed the same requirement, the builder rebases it onto main's text with exactly the same added and removed words, the lead verifies by script that nothing else changed, the builder runs the send-back with '(standing OK (d), relayed via t4c)' in its reason, puts in the rebased text and records the re-approval citing the rule); recorded by chough-fixer-t4g; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of doctor-fix-safety rebased onto main after #407 (standing OK (d))": "yes to both" (Standing OK (d), Leif's Claude Code chat message relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:02Z; the delta was rebased onto main's REQ-records-001 and REQ-history-006 after #407 with exactly #407's added and removed words, and fable-lead verified by script that nothing else changed); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve doctor-fix-safety as 0xLeif
- 0xLeif decided "Send back doctor-fix-safety to drop the journal post from the #389 guard, run by the fixer": "B, fixer runs the send-back" (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:46Z); recorded by chough-fixer-t4g; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of doctor-fix-safety with the journal post dropped from the #389 guard": "B, fixer runs the send-back" (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:46Z, choosing option B (drop the journal post from the #389 guard, with the criterion that an untracked or ignored journal post does not hold the finish) on this text; the earlier approvals stand for the rest of the delta: 'Approve, with the 3 recommended choices', the three wording answers, 'Yes' for the two sentences and four paths, and standing OK (d) for the rebase onto main after #407); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve doctor-fix-safety as 0xLeif
- 0xLeif decided "Send back doctor-fix-safety for the three inbox edits to REQ-history-012, run by the fixer": "a, all yes.. im back" (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 00:38Z, answering item 4 of t4c's eight-item round: #414's three edits to REQ-history-012 (any file under agent/inbox/ outside its folded/ folder is kept; the way out for a kept inbox file chough inbox add would not take; one new criterion), with the fixer running the send-back); recorded by chough-fixer-t4g; a relayed claim, not authenticated
- 0xLeif decided "Re-approval of doctor-fix-safety for the three inbox edits to REQ-history-012": "a, all yes.. im back" (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 00:38Z, answering item 4 of t4c's eight-item round, on this text: REQ-history-012 keeps any file under agent/inbox/ outside its folded/ folder, gives a kept inbox file that chough inbox add would not take a drop-only way out, and adds one criterion for both; nothing else in any delta changed, and main through #413 changed none of this change's requirements, so standing OK (d) was not used; the earlier approvals stand for the rest of the delta: 'Approve, with the 3 recommended choices', the three wording answers, 'Yes' for the two sentences and four paths, standing OK (d) for the rebase onto main after #407, and 'B, fixer runs the send-back' for the #389 guard); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve doctor-fix-safety as 0xLeif
- 0xLeif decided "Implementation review of doctor-fix-safety": "Go" (Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #414 at 8a586283: the tester's recheck PASS at 8a586283, corvid-agent APPROVED 8a586283 (2026-11-20 about 01:00Z, no finding), and fable-lead's read of 8a586283 passed; every sentence change in this batch carries Leif's own words (the scope approval, the three wording answers, the send-backs and standing OK (d)); relayed by fable-lead (hc t53), 2026-11-20); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change review doctor-fix-safety as 0xLeif with verdict pass

## What was verified

- 126 earlier runs of the cases below are kept in this story's receipts, not listed here.
- Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-history-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1). - a36175960ac3; limits: inherited environment variables were not recorded ^2026-11-19-tc-history-the-spec-walk-follows-no-link-and-is-bounded-672851
- Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_holds_the_finish for TC-history-an-ignored-journal-post-holds-the-finish: passed (exit 0, tests run 1). - 719b5267c466; limits: inherited environment variables were not recorded ^2026-11-19-tc-history-an-ignored-journal-post-holds-the-finish-52422e
- Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1). - abe6711401da; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-next-names-finishing-the-archive
- Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1). - a2d87af52e4e; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-next-names-the-archive-with-a-session-open
- Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1). - e2de77eb0071; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-the-finish-command-quotes-each-path
- Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1). - 2be04fa07213; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-the-archive-line-runs-as-printed
- Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1). - 25ef80036164; limits: inherited environment variables were not recorded ^2026-11-20-tc-cli-the-printed-command-commits-the-story
- Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1). - eb0516b24dd3; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-the-archive-finish-keeps-what-the-story-does-not
- Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1). - 2044b2453fdf; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-an-entry-that-differs-from-its-receipt-is-kept
- Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1). - b6f29907e9c4; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-macos-metadata-never-blocks-the-archive-finish
- Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1). - ceda34ac0a5d; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-next-names-finishing-the-archive
- Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1). - c4e85ecdb929; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-a-front-matter-conflict-is-named
- Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1). - 8f7aee315aaa; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-a-redacted-timeline-entry-is-not-rewritten
- Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1). - 6646d2e9c392; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-a-story-an-earlier-release-wrote-still-holds-its
- Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1). - 137469c4c48a; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-an-unreadable-file-never-stops-doctor
- Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1). - 80e51b74fda1; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-the-finish-waits-until-the-story-is-committed
- Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1). - 2bca1aff93a0; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-a-story-edited-after-its-commit-holds-the-finish
- Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1). - 8399fc341185; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-an-ignored-story-holds-the-finish
- Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish for TC-history-an-ignored-journal-post-does-not-hold-the-finish: passed (exit 0, tests run 1). - fc0874804be1; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-an-ignored-journal-post-does-not-hold-the-finish
- Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1). - 34a22531c7ee; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-a-failed-git-read-holds-the-finish
- Ran cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands for TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands: passed (exit 0, tests run 1). - 089692d5c1fb; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-an-unreadable-entry-is-dropped-by-the-printed
- Ran cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite for TC-history-a-redacting-save-leaves-no-timeline-rewrite: passed (exit 0, tests run 1). - de4928d60123; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-a-redacting-save-leaves-no-timeline-rewrite
- Ran cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id for TC-history-a-kept-entry-is-kept-by-its-file: passed (exit 0, tests run 1). - 569f8657b90c; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-a-kept-entry-is-kept-by-its-file
- Ran cargo test --locked --test sessions -- --exact next_finishes_an_ignored_story_as_printed for TC-history-next-finishes-an-ignored-story-as-printed: passed (exit 0, tests run 1). - e21f89c4479d; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-next-finishes-an-ignored-story-as-printed
- Ran cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active for TC-history-the-way-out-names-the-open-sessions: passed (exit 0, tests run 1). - fee6ad68e2b7; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-the-way-out-names-the-open-sessions
- Ran cargo test --locked --test sessions -- --exact an_untracked_kept_entry_is_committed_first_so_its_way_out_runs for TC-history-an-untracked-kept-entry-is-committed-first: passed (exit 0, tests run 1). - f5c568dc3363; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-an-untracked-kept-entry-is-committed-first
- Ran cargo test --locked --test sessions -- --exact an_inbox_entry_is_kept_whatever_its_name for TC-history-an-inbox-entry-is-kept-whatever-its-name: passed (exit 0, tests run 1). - 400aa9a143d3; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-an-inbox-entry-is-kept-whatever-its-name
- Ran cargo test --locked --test sessions -- --exact a_folded_inbox_entry_does_not_hold_the_finish for TC-history-a-folded-inbox-entry-does-not-hold-the-finish: passed (exit 0, tests run 1). - d6c0cff8b9d5; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-a-folded-inbox-entry-does-not-hold-the-finish
- Ran cargo test --locked --test sessions -- --exact an_inbox_file_inbox_add_would_not_take_is_only_dropped for TC-history-an-inbox-file-inbox-add-would-not-take-is-only-dropped: passed (exit 0, tests run 1). - 4accd8c08219; limits: inherited environment variables were not recorded ^2026-11-20-tc-history-an-inbox-file-inbox-add-would-not-take-is-only
- Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1). - b1e9dcfc0300; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-a-front-matter-conflict-is-named
- Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-records-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1). - 508800631c28; limits: inherited environment variables were not recorded ^2026-11-20-tc-records-the-spec-walk-follows-no-link-and-is-bounded

## Bugs

None.

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
- 2026-11-19T14:29:31Z · gate-opened · build → build · Send back doctor-fix-safety for the two sentences and four paths, run by the fixer ^t-993718
  - native_change: doctor-fix-safety
  - revision: 424a761f5faa817431e5f65763ab271469bd7f33
  - holds: build
  - drives: none
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T14:29:31Z · gate-closed · build → build · Send back doctor-fix-safety for the two sentences and four paths, run by the fixer ^t-44909f
  - native_change: doctor-fix-safety
  - revision: 424a761f5faa817431e5f65763ab271469bd7f33
  - relayed_by:
    - decided_by: 0xLeif
    - words: The fixer runs them, on my word
    - where: Claude Code chat answer, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 14:17Z
    - recorded_by: chough-fixer-t4g
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T14:30:01Z · gate-opened · build → build · Re-approval of doctor-fix-safety for the receipt and #389 sentences and four paths ^t-99b9d3
  - native_change: doctor-fix-safety
  - revision: 424a761f5faa817431e5f65763ab271469bd7f33
  - holds: build
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T14:30:02Z · direct-native-write · build → build · doctor-fix-safety's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-6e26ad
  - native_change: doctor-fix-safety
  - revision: 424a761f5faa817431e5f65763ab271469bd7f33
  - direct_write:
    - change: doctor-fix-safety
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T14:30:02Z · gate-closed · build → build · Re-approval of doctor-fix-safety for the receipt and #389 sentences and four paths ^t-825197
  - native_change: doctor-fix-safety
  - revision: 424a761f5faa817431e5f65763ab271469bd7f33
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes
    - where: Claude Code chat answer, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 14:08Z, to: may doctor-fix-safety be sent back once for the two sentences (the receipt-fields sentence and the #389 sentence with its criterion) and the four paths tests/reduce.rs, src/evidence.rs, .chough/cases/records.doctor-fix-safety.cases.toml and .chough/cases/cli.doctor-fix-safety.cases.toml
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve doctor-fix-safety
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T15:03:47Z · gate-opened · build → build · Standing OK (d): send back doctor-fix-safety to rebase its delta onto main after #407 ^t-71f9e3
  - native_change: doctor-fix-safety
  - revision: 5fbfe8a0e748f99151660263a73dea941db7bb8e
  - holds: build
  - drives: none
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T15:03:48Z · gate-closed · build → build · Standing OK (d): send back doctor-fix-safety to rebase its delta onto main after #407 ^t-36caca
  - native_change: doctor-fix-safety
  - revision: 5fbfe8a0e748f99151660263a73dea941db7bb8e
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes to both
    - where: Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:02Z, to t4c's proposed standing OK (d): when an approved delta goes stale only because main changed the same requirement, the builder rebases it onto main's text with exactly the same added and removed words, the lead verifies by script that nothing else changed, the builder runs the send-back with '(standing OK (d), relayed via t4c)' in its reason, puts in the rebased text and records the re-approval citing the rule
    - recorded_by: chough-fixer-t4g
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T15:04:17Z · gate-opened · build → build · Re-approval of doctor-fix-safety rebased onto main after #407 (standing OK (d)) ^t-04a7fb
  - native_change: doctor-fix-safety
  - revision: 5fbfe8a0e748f99151660263a73dea941db7bb8e
  - holds: build
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T15:04:17Z · direct-native-write · build → build · doctor-fix-safety's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-d13027
  - native_change: doctor-fix-safety
  - revision: 5fbfe8a0e748f99151660263a73dea941db7bb8e
  - direct_write:
    - change: doctor-fix-safety
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T15:04:17Z · gate-closed · build → build · Re-approval of doctor-fix-safety rebased onto main after #407 (standing OK (d)) ^t-ebce16
  - native_change: doctor-fix-safety
  - revision: 5fbfe8a0e748f99151660263a73dea941db7bb8e
  - relayed_by:
    - decided_by: 0xLeif
    - words: yes to both
    - where: Standing OK (d), Leif's Claude Code chat message relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:02Z; the delta was rebased onto main's REQ-records-001 and REQ-history-006 after #407 with exactly #407's added and removed words, and fable-lead verified by script that nothing else changed
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve doctor-fix-safety
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T15:55:49Z · gate-opened · build → build · Send back doctor-fix-safety to drop the journal post from the #389 guard, run by the fixer ^t-27e70a
  - native_change: doctor-fix-safety
  - revision: 45deab1d1f19a608f5e65d4994cef5afe01520bc
  - holds: build
  - drives: none
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T15:55:49Z · gate-closed · build → build · Send back doctor-fix-safety to drop the journal post from the #389 guard, run by the fixer ^t-f076fd
  - native_change: doctor-fix-safety
  - revision: 45deab1d1f19a608f5e65d4994cef5afe01520bc
  - relayed_by:
    - decided_by: 0xLeif
    - words: B, fixer runs the send-back
    - where: Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:46Z
    - recorded_by: chough-fixer-t4g
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T15:56:09Z · gate-opened · build → build · Re-approval of doctor-fix-safety with the journal post dropped from the #389 guard ^t-e67e76
  - native_change: doctor-fix-safety
  - revision: 45deab1d1f19a608f5e65d4994cef5afe01520bc
  - holds: build
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T15:56:09Z · direct-native-write · build → build · doctor-fix-safety's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-1fd5df
  - native_change: doctor-fix-safety
  - revision: 45deab1d1f19a608f5e65d4994cef5afe01520bc
  - direct_write:
    - change: doctor-fix-safety
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T15:56:09Z · gate-closed · build → build · Re-approval of doctor-fix-safety with the journal post dropped from the #389 guard ^t-a39b7c
  - native_change: doctor-fix-safety
  - revision: 45deab1d1f19a608f5e65d4994cef5afe01520bc
  - relayed_by:
    - decided_by: 0xLeif
    - words: B, fixer runs the send-back
    - where: Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19 about 15:46Z, choosing option B (drop the journal post from the #389 guard, with the criterion that an untracked or ignored journal post does not hold the finish) on this text; the earlier approvals stand for the rest of the delta: 'Approve, with the 3 recommended choices', the three wording answers, 'Yes' for the two sentences and four paths, and standing OK (d) for the rebase onto main after #407
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve doctor-fix-safety
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T00:41:06Z · gate-opened · build → build · Send back doctor-fix-safety for the three inbox edits to REQ-history-012, run by the fixer ^t-2f72a1
  - native_change: doctor-fix-safety
  - revision: 011b33f5c2cb1c55d17e09d723facc34b358b8cd
  - holds: build
  - drives: none
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T00:41:07Z · gate-closed · build → build · Send back doctor-fix-safety for the three inbox edits to REQ-history-012, run by the fixer ^t-840be2
  - native_change: doctor-fix-safety
  - revision: 011b33f5c2cb1c55d17e09d723facc34b358b8cd
  - relayed_by:
    - decided_by: 0xLeif
    - words: a, all yes.. im back
    - where: Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 00:38Z, answering item 4 of t4c's eight-item round: #414's three edits to REQ-history-012 (any file under agent/inbox/ outside its folded/ folder is kept; the way out for a kept inbox file chough inbox add would not take; one new criterion), with the fixer running the send-back
    - recorded_by: chough-fixer-t4g
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T00:43:22Z · gate-opened · build → build · Re-approval of doctor-fix-safety for the three inbox edits to REQ-history-012 ^t-f4f812
  - native_change: doctor-fix-safety
  - revision: a35f3c394e636ac4d18802f1c4b4ce8d2e983d72
  - holds: build
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T00:43:23Z · direct-native-write · build → build · doctor-fix-safety's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-30b543
  - native_change: doctor-fix-safety
  - revision: a35f3c394e636ac4d18802f1c4b4ce8d2e983d72
  - direct_write:
    - change: doctor-fix-safety
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T00:43:23Z · gate-closed · build → build · Re-approval of doctor-fix-safety for the three inbox edits to REQ-history-012 ^t-2ec03f
  - native_change: doctor-fix-safety
  - revision: a35f3c394e636ac4d18802f1c4b4ce8d2e983d72
  - relayed_by:
    - decided_by: 0xLeif
    - words: a, all yes.. im back
    - where: Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-20 about 00:38Z, answering item 4 of t4c's eight-item round, on this text: REQ-history-012 keeps any file under agent/inbox/ outside its folded/ folder, gives a kept inbox file that chough inbox add would not take a drop-only way out, and adds one criterion for both; nothing else in any delta changed, and main through #413 changed none of this change's requirements, so standing OK (d) was not used; the earlier approvals stand for the rest of the delta: 'Approve, with the 3 recommended choices', the three wording answers, 'Yes' for the two sentences and four paths, standing OK (d) for the rebase onto main after #407, and 'B, fixer runs the send-back' for the #389 guard
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve doctor-fix-safety
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T01:14:25Z · advance · build → test · native:verifying ^t-ba1ed7
  - native_change: doctor-fix-safety
  - gate: native:verifying
  - revision: 8a586283d6787f2b128ec9760d8c8cef31f26ed2
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T01:14:38Z · advance · test → accept · native:verifying ^t-6002d8
  - native_change: doctor-fix-safety
  - gate: native:verifying
  - revision: 8a586283d6787f2b128ec9760d8c8cef31f26ed2
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T01:14:52Z · gate-opened · accept → accept · Implementation review of doctor-fix-safety ^t-29ba33
  - native_change: doctor-fix-safety
  - revision: 8a586283d6787f2b128ec9760d8c8cef31f26ed2
  - holds: accept
  - drives: review
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T01:16:05Z · gate-closed · accept → accept · Implementation review of doctor-fix-safety ^t-321c4d
  - native_change: doctor-fix-safety
  - revision: 8434469ab4eb3554a65bfd15e4a0152422adee8f
  - relayed_by:
    - decided_by: 0xLeif
    - words: Go
    - where: Standing OK 6(b): Leif's 'Go' to t4c's six rules (Claude Code chat message, relayed by hc t4c through fable-lead (hc t53), 2026-11-19), under which the builder records his implementation review of a batch with a tester PASS whose build changes no approved sentence (t4c's ruling of 2026-11-19: the review only; every merge still needs his yes after CI). For #414 at 8a586283: the tester's recheck PASS at 8a586283, corvid-agent APPROVED 8a586283 (2026-11-20 about 01:00Z, no finding), and fable-lead's read of 8a586283 passed; every sentence change in this batch carries Leif's own words (the scope approval, the three wording answers, the send-backs and standing OK (d)); relayed by fable-lead (hc t53), 2026-11-20
    - recorded_by: chough-fixer-t4g
    - native: specsync change review doctor-fix-safety
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T01:16:08Z · advance · accept → done · native:archived ^t-6d35b8
  - native_change: doctor-fix-safety
  - gate: native:archived
  - revision: 8434469ab4eb3554a65bfd15e4a0152422adee8f
  - by: chough-fixer (claude-opus-5-5, hc t4n)

Closed 2026-11-20: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not
    claim: "Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold
    revision: 73cf8c7aaca29e4495ad0d6461db9f7b7fd64fb5
    environment: local-macos-arm64
    content: e0e1ce89a4414c7225189af8ad1965a6d132cfc4115a1598607a2421e13c2113
    fingerprint: sha256:f9b2e66f627b266b39f0953ac65e54b81b23dbea6faa8ffd48dc8c6a42fd0437
    case: TC-history-the-archive-finish-keeps-what-the-story-does-not-hold
    case_digest: 0e25ff1ec2d3cd5ea644afe46498c1654fa51f47e04e5a061e848c4a94de4a4a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:33:44Z
    duration_seconds: 15
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept
    claim: "Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept
    revision: 73cf8c7aaca29e4495ad0d6461db9f7b7fd64fb5
    environment: local-macos-arm64
    content: e0e1ce89a4414c7225189af8ad1965a6d132cfc4115a1598607a2421e13c2113
    fingerprint: sha256:513de9a6a2889ae6576a1156a4086fb964d4158b4924ad01f3f13b92bfba243d
    case: TC-history-an-entry-that-differs-from-its-receipt-is-kept
    case_digest: 040df95e4eb215fb43ddf52a93138667b7245be119fa97244642d30cb7c6e83e
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:34:01Z
    duration_seconds: 13
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish
    claim: "Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish
    revision: 73cf8c7aaca29e4495ad0d6461db9f7b7fd64fb5
    environment: local-macos-arm64
    content: e0e1ce89a4414c7225189af8ad1965a6d132cfc4115a1598607a2421e13c2113
    fingerprint: sha256:46555f63b5fd7a2b516cdcfb012f296c263b202eb255ff2087a2edc22262008f
    case: TC-history-macos-metadata-never-blocks-the-archive-finish
    case_digest: 9abb5c17fb680ed8e373ae90f5e4b4e5457b4fb3d916834ccf5ab92b2cb7f26b
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:34:10Z
    duration_seconds: 6
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-front-matter-conflict-is-named
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 73cf8c7aaca29e4495ad0d6461db9f7b7fd64fb5
    environment: local-macos-arm64
    content: e0e1ce89a4414c7225189af8ad1965a6d132cfc4115a1598607a2421e13c2113
    fingerprint: sha256:d287131568c407e7e7ad7af09c1c61535923ba1a2bbf16ff5ffa177066930a37
    case: TC-history-a-front-matter-conflict-is-named
    case_digest: a52a4bd47d867f61e33e4c18ef7edaa7111334bd8785dcb18063c54cde3feb8e
    criteria: DRIVE-17
    requirements: REQ-records-001, REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:34:20Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is
    revision: 73cf8c7aaca29e4495ad0d6461db9f7b7fd64fb5
    environment: local-macos-arm64
    content: e0e1ce89a4414c7225189af8ad1965a6d132cfc4115a1598607a2421e13c2113
    fingerprint: sha256:c847cba0de4a43dc4dac70fd779458d5f82e090b1515211eaacfa9a124bea268
    case: TC-history-a-redacted-timeline-entry-is-not-rewritten
    case_digest: ecdb628002127dd0bac95a871c821db001b94787f8905b438849e19d8ab78eb4
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:34:33Z
    duration_seconds: 9
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-spec-walk-follows-no-link-and-is-bounded
    claim: "Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-history-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded
    revision: 73cf8c7aaca29e4495ad0d6461db9f7b7fd64fb5
    environment: local-macos-arm64
    content: e0e1ce89a4414c7225189af8ad1965a6d132cfc4115a1598607a2421e13c2113
    fingerprint: sha256:ccaee29e8c4dd9060608cad957b7ff278236cab2986d3aa04b90bbccbe29ce1a
    case: TC-history-the-spec-walk-follows-no-link-and-is-bounded
    case_digest: 8d3c36492cc97804f457fe0525ca89be16c60c193a49e1d873200b184e8f7b7b
    criteria: DRIVE-17
    requirements: REQ-records-014
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:34:40Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-cad975
    claim: "Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold
    revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
    environment: local-macos-arm64
    content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
    fingerprint: sha256:b1b97f78dafb1bb773a31cace91aa25dce8b860ef7055c4292cd832e17334904
    case: TC-history-the-archive-finish-keeps-what-the-story-does-not-hold
    case_digest: 0e25ff1ec2d3cd5ea644afe46498c1654fa51f47e04e5a061e848c4a94de4a4a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:37:48Z
    duration_seconds: 15
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-d21527
    claim: "Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept
    revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
    environment: local-macos-arm64
    content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
    fingerprint: sha256:33ba80814cc380e115291a2ed3f7edcb9107a4e402e8a8130e60c1878306b79d
    case: TC-history-an-entry-that-differs-from-its-receipt-is-kept
    case_digest: 040df95e4eb215fb43ddf52a93138667b7245be119fa97244642d30cb7c6e83e
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:38:00Z
    duration_seconds: 10
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-40c735
    claim: "Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish
    revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
    environment: local-macos-arm64
    content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
    fingerprint: sha256:3ba8a3f36967369851eec56730a72ba444c4b2bbd54ab2675bdbe64ef08ea169
    case: TC-history-macos-metadata-never-blocks-the-archive-finish
    case_digest: 9abb5c17fb680ed8e373ae90f5e4b4e5457b4fb3d916834ccf5ab92b2cb7f26b
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:38:09Z
    duration_seconds: 6
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-next-names-finishing-the-archive
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
    environment: local-macos-arm64
    content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
    fingerprint: sha256:da3aa0fb5fadb792e773e055e66794f18a91c2968987044af784727afa9357ef
    case: TC-history-next-names-finishing-the-archive
    case_digest: 070774317b2ea37170632459474d531da442a09db24605e729171fff018eeb1a
    criteria: DRIVE-17
    requirements: REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:38:18Z
    duration_seconds: 6
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-front-matter-conflict-is-named-67d461
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
    environment: local-macos-arm64
    content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
    fingerprint: sha256:6c34efeda417ed93a9e6aa1f10bb8fefcefbccf0cc698a8668865395f63dcdc0
    case: TC-history-a-front-matter-conflict-is-named
    case_digest: a52a4bd47d867f61e33e4c18ef7edaa7111334bd8785dcb18063c54cde3feb8e
    criteria: DRIVE-17
    requirements: REQ-records-001, REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:38:25Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-ac9ac5
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is
    revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
    environment: local-macos-arm64
    content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
    fingerprint: sha256:ffd513fb01ea1f6f9e831d18ca95b69bae6dfb856b85b6e37896da30beca18d8
    case: TC-history-a-redacted-timeline-entry-is-not-rewritten
    case_digest: ecdb628002127dd0bac95a871c821db001b94787f8905b438849e19d8ab78eb4
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:38:35Z
    duration_seconds: 7
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-spec-walk-follows-no-link-and-is-bounded-9bfc91
    claim: "Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-history-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded
    revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
    environment: local-macos-arm64
    content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
    fingerprint: sha256:b66012b46f64b061020039b5b5b243f79883fe6e31ddd13bc78099a8c0f87ce1
    case: TC-history-the-spec-walk-follows-no-link-and-is-bounded
    case_digest: e7a7cbf67ea3cdf83e4710d0f766c27a5bc8bb8ac35785fbfe77cea47abb1ee4
    criteria: DRIVE-17
    requirements: REQ-records-014, REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:38:41Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-front-matter-conflict-is-named-f1e33c
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 040932308089ebaaa888b6b00bc5b04187e303d4
    environment: local-macos-arm64
    content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
    fingerprint: sha256:9b4ecab67166dc8d7c4b97dc7d97113f39c77ae93408fcec29fbb0afbf442172
    case: TC-history-a-front-matter-conflict-is-named
    case_digest: c28d317bab6b5af2bf356176cf7b7c972412547511c18dba7abc93ca1bafeac0
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:56:10Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-spec-walk-follows-no-link-and-is-bounded-672851
    claim: "Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-history-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded
    revision: 040932308089ebaaa888b6b00bc5b04187e303d4
    environment: local-macos-arm64
    content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
    fingerprint: sha256:a36175960ac3d2a0e898408b8b37083a9466cf895b48215281f4e34fc8afc374
    case: TC-history-the-spec-walk-follows-no-link-and-is-bounded
    case_digest: 4cebad664014882e0b40c3afd359000943a57fdcd404f8d5239adcb3beacf6ba
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T05:56:15Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-38c262
    claim: "Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:8c523b66c1fc68718f18738dbcddbbcb6f3b559550e61d9a4df684b56922e9f1
    case: TC-history-the-archive-finish-keeps-what-the-story-does-not-hold
    case_digest: 7416481d236addab0948f663b50ee255bee4899d1a4fef1357dc85ec8049d528
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:26Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-4ba00c
    claim: "Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:84aa9566597477b1a9f211e60b1a1e266f0ced09fb426beb6bf732045fe9e40a
    case: TC-history-an-entry-that-differs-from-its-receipt-is-kept
    case_digest: 6b4fe2900d7a4a2f73e65527f91b42c9950373e5e9a1e32fd0643f55501b7d93
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:31Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-a0e4bb
    claim: "Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:f39163581b36f699157ed8c2bd584859baea4d42acec17666475ec4783560b51
    case: TC-history-macos-metadata-never-blocks-the-archive-finish
    case_digest: 9abb5c17fb680ed8e373ae90f5e4b4e5457b4fb3d916834ccf5ab92b2cb7f26b
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:34Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-next-names-finishing-the-archive-fc13d1
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:73e9af94c0716728a1aea6a8d91b8e7a96512df8ebf10de2fc5695a6fa5387ea
    case: TC-history-next-names-finishing-the-archive
    case_digest: 070774317b2ea37170632459474d531da442a09db24605e729171fff018eeb1a
    criteria: DRIVE-17
    requirements: REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:37Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-front-matter-conflict-is-named-cb1377
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:0b80e436c79855aba2db6e5648161d5234bfa9203b1d02bd3f75794d32337bb7
    case: TC-history-a-front-matter-conflict-is-named
    case_digest: c28d317bab6b5af2bf356176cf7b7c972412547511c18dba7abc93ca1bafeac0
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:39Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-e220ba
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:cca777f8e53ab99c8290de98436f395df6d553df15727558ff09b1fdf2a7aa81
    case: TC-history-a-redacted-timeline-entry-is-not-rewritten
    case_digest: ecdb628002127dd0bac95a871c821db001b94787f8905b438849e19d8ab78eb4
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:42Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:42b6c9556ee992f68454edf9555ff8ba0250d390cbc2cc5af7ccfde33c418a1d
    case: TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries
    case_digest: 519c9da40455e7b15f76ac7bafd083659e6d02313e2d8cb0e4544c18738cc147
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:45Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor
    claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:61b88e9fc75301e0a40dd0d447d952274cfb5234dd57d01b04326e46af22597f
    case: TC-history-an-unreadable-file-never-stops-doctor
    case_digest: 623dd39576587f705f31f4fe1d7e0752b8b2357215804119aedac7c392922313
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:51Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed
    claim: "Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:57e6c6e636ef360382f2c02bdf871422baf1f62292cc7caed06e85a39f9918f3
    case: TC-history-the-finish-waits-until-the-story-is-committed
    case_digest: 8c422734c0abfe1244f590414ca6d32cb06ec08f39d05f89e13eff559232cec9
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:54Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:4a813b3cd5a99e82df1544c60803bbc32e353e57dffbb7aa952d0a6eb685dc10
    case: TC-history-a-story-edited-after-its-commit-holds-the-finish
    case_digest: 2caa4b3395afbcb07ad09017b5940a9f729d0b486ef2cf2abe9933fad7c32711
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:57Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-next-names-finishing-the-archive
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:34a0a6c88d647cc505a525bb0db85b8f504d85730d3f3016befd158c2008fb7e
    case: TC-cli-next-names-finishing-the-archive
    case_digest: cca9f365ab26edce72dc89919cdadc945421fdf1426e40ca5fce3a75bb9eada8
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:31:59Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open
    claim: "Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:b2ffdb7b8993ffc791c8fcdecb9ea1b1e777ceb375c0a7efc850028ccb26fdb2
    case: TC-cli-next-names-the-archive-with-a-session-open
    case_digest: cfe06814bffcaac6430899b4f9068f845bf06dbf349b5f500ac568e80d84361f
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:32:03Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-finish-command-quotes-each-path
    claim: "Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:40c0415732f6c74d1aeffc73893400bd4b44d2017ff293b4a7e99b1f804aa248
    case: TC-cli-the-finish-command-quotes-each-path
    case_digest: d68ddc8fbac742b62d0ddb225cac20b80eb7dc0468253d538d1ea05adb2ffaf5
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:32:05Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-archive-line-runs-as-printed
    claim: "Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:a61f9b204a43e828c91d6cac1473b5083e6588d2e1c0e7f05baa907dd52da949
    case: TC-cli-the-archive-line-runs-as-printed
    case_digest: 413ca9241d303fc7cca2d3290c0406c7a4a723929d546d0dfef5bf5520ee6980
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:32:08Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-printed-command-commits-the-story
    claim: "Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:e2bee2768c7b6588630164ad58906cd7aa6d246b4ab8d1e6b32da206158ba182
    case: TC-cli-the-printed-command-commits-the-story
    case_digest: b75f11fceab686a56c7bb4889f42258c3e5427cf589f33f7ba10823be063e9fa
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:32:11Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-front-matter-conflict-is-named
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:45629ef0b43e83f317f0da430aaf1ba3a6d0eb39b5fd16adf4f857cca0e44e91
    case: TC-records-a-front-matter-conflict-is-named
    case_digest: 18be79babe813b859103021cf7d5c686461a85549f7a415e35e90cdaa6e93862
    criteria: DRIVE-17
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:32:14Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-spec-walk-follows-no-link-and-is-bounded
    claim: "Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-records-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded
    revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
    environment: local-macos-arm64
    content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
    fingerprint: sha256:93d2b8a029d40bab3df5401a635fc239d96ea779f8241140ad32ebb789294b7b
    case: TC-records-the-spec-walk-follows-no-link-and-is-bounded
    case_digest: a2a6be7a89d4cea6de791695ddccb9168054bccd04d1c9b18581f35882507f55
    criteria: DRIVE-17
    requirements: REQ-records-014
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:32:16Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-e7c642
    claim: "Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:87d2138b5c9faedad3f17adb31cf3aaf8924f96e618ad77ba87bb9af21ee4ae4
    case: TC-history-the-archive-finish-keeps-what-the-story-does-not-hold
    case_digest: 7416481d236addab0948f663b50ee255bee4899d1a4fef1357dc85ec8049d528
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:48:55Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-b7d61f
    claim: "Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:794dfae31c6db759bc183b5585d2cefd56f8ea8a3636b89aa5eb01363f071ee0
    case: TC-history-an-entry-that-differs-from-its-receipt-is-kept
    case_digest: 6b4fe2900d7a4a2f73e65527f91b42c9950373e5e9a1e32fd0643f55501b7d93
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:48:58Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-0c69e2
    claim: "Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:b1e59e5ca2a4769daba24fee8889cea67fdffa69065fe87f1e145de90b726361
    case: TC-history-macos-metadata-never-blocks-the-archive-finish
    case_digest: 9abb5c17fb680ed8e373ae90f5e4b4e5457b4fb3d916834ccf5ab92b2cb7f26b
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:00Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-next-names-finishing-the-archive-8afceb
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:22cadc73be9cc15456919162c88459417354384657204eac747082f1c4b75b12
    case: TC-history-next-names-finishing-the-archive
    case_digest: 070774317b2ea37170632459474d531da442a09db24605e729171fff018eeb1a
    criteria: DRIVE-17
    requirements: REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:04Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-front-matter-conflict-is-named-5d2141
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:c6423a1a85a6989add674a7a2b5de93257653d683dc40ea0d498e52545ae19e8
    case: TC-history-a-front-matter-conflict-is-named
    case_digest: c28d317bab6b5af2bf356176cf7b7c972412547511c18dba7abc93ca1bafeac0
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:06Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-8f0184
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:905472e9b4ab21c8dfc598274351cd4fff020a67d28f7935ede6f394edfa60c2
    case: TC-history-a-redacted-timeline-entry-is-not-rewritten
    case_digest: ecdb628002127dd0bac95a871c821db001b94787f8905b438849e19d8ab78eb4
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:09Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-ba11f2
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:a9b04a5a818d9e8f9d2c17904571ad8d0f10493ba337babac6cec612c2ef8942
    case: TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries
    case_digest: 519c9da40455e7b15f76ac7bafd083659e6d02313e2d8cb0e4544c18738cc147
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:12Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-3cb989
    claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:e489a5089346d751b4363b896686119328530bba084b713b5da3a8b79a18fda3
    case: TC-history-an-unreadable-file-never-stops-doctor
    case_digest: 623dd39576587f705f31f4fe1d7e0752b8b2357215804119aedac7c392922313
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:24Z
    duration_seconds: 10
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-2cd41f
    claim: "Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:2af78f48000cda4ccae425f6a5eafe440692c5be298bf33bb811d53fc4f3c2f1
    case: TC-history-the-finish-waits-until-the-story-is-committed
    case_digest: 8c422734c0abfe1244f590414ca6d32cb06ec08f39d05f89e13eff559232cec9
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:30Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-a6651f
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:1cb3b526a3d2e852a7289da4dcfde454cf690dd5e3fecea5bb5d5802792386ad
    case: TC-history-a-story-edited-after-its-commit-holds-the-finish
    case_digest: 2caa4b3395afbcb07ad09017b5940a9f729d0b486ef2cf2abe9933fad7c32711
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:37Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-ignored-story-holds-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:5c7315ffb857e81fc5beae4397e483af98f01f288d61c2fc82c001c152dfbbb6
    case: TC-history-an-ignored-story-holds-the-finish
    case_digest: 86e02ab9da1d4f5559805d2d6126a8bc190ccf64d1956cc991f5101bbc274b87
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:47Z
    duration_seconds: 5
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-ignored-journal-post-holds-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_holds_the_finish for TC-history-an-ignored-journal-post-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_ignored_journal_post_holds_the_finish
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:1cb2b1398aa55239795d0b148fb3214ffbe9f4d6a7f7e3e15ce52552d023714d
    case: TC-history-an-ignored-journal-post-holds-the-finish
    case_digest: bcc54521c6af3a3d50e789eab08e36f18f5c3a2f05840ae8f81319eb81161d9a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:53Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-failed-git-read-holds-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:1186bc1bf2667cc81b1cf2280aa4f851b63584a2c280bf967e63cb82d63a6690
    case: TC-history-a-failed-git-read-holds-the-finish
    case_digest: 3f403dd3c9fe13b9d9d01caab5d6d7f1c05607d26d93e28fd6ecdbf8697cb5b8
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:49:57Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-next-names-finishing-the-archive-a7d9b2
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:aad213421c893e00f8e5cf7b6effff3605ec8eae36dee3fbd2e1d914f97c3b65
    case: TC-cli-next-names-finishing-the-archive
    case_digest: cca9f365ab26edce72dc89919cdadc945421fdf1426e40ca5fce3a75bb9eada8
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:50:03Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-914865
    claim: "Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:45d19093afe583373dee7ae47bceeddbd64dc7b49006eed7c84bf6ee3baa6dcf
    case: TC-cli-next-names-the-archive-with-a-session-open
    case_digest: cfe06814bffcaac6430899b4f9068f845bf06dbf349b5f500ac568e80d84361f
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:50:09Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-finish-command-quotes-each-path-ca5f3c
    claim: "Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:e19b922c2ed45e7de21e74d8989af8186a4143a92a33cafed7741d190363c0ec
    case: TC-cli-the-finish-command-quotes-each-path
    case_digest: d68ddc8fbac742b62d0ddb225cac20b80eb7dc0468253d538d1ea05adb2ffaf5
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:50:14Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-archive-line-runs-as-printed-744a60
    claim: "Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:39b143c74091f989a6ef6e7e1e1f8209aaca6693504f2a4a293af04b44b5f9a8
    case: TC-cli-the-archive-line-runs-as-printed
    case_digest: 413ca9241d303fc7cca2d3290c0406c7a4a723929d546d0dfef5bf5520ee6980
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:50:20Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-printed-command-commits-the-story-5bc003
    claim: "Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:a7ac975baeb1c9da52e36cfb273e1614c45051e22eb7e1160696365edd19f21d
    case: TC-cli-the-printed-command-commits-the-story
    case_digest: b75f11fceab686a56c7bb4889f42258c3e5427cf589f33f7ba10823be063e9fa
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:50:28Z
    duration_seconds: 5
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-front-matter-conflict-is-named-d7da54
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:6883bb4a4ad3e1c7853be05002e9015d8288f9e28bcdc00a1b6feaf9b86647ed
    case: TC-records-a-front-matter-conflict-is-named
    case_digest: 18be79babe813b859103021cf7d5c686461a85549f7a415e35e90cdaa6e93862
    criteria: DRIVE-17
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:50:32Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-spec-walk-follows-no-link-and-is-bounded-d43aba
    claim: "Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-records-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded
    revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
    environment: local-macos-arm64
    content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
    fingerprint: sha256:ab335b2ffe40a3c5339c14198a9dd425c6ea0aec6587740a766272a9be2d8380
    case: TC-records-the-spec-walk-follows-no-link-and-is-bounded
    case_digest: a2a6be7a89d4cea6de791695ddccb9168054bccd04d1c9b18581f35882507f55
    criteria: DRIVE-17
    requirements: REQ-records-014
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T14:50:36Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-72a5d9
    claim: "Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:94268b8f2eff1ce1297140e0d4f33d818c41c38760d67480177256d4e69e1e07
    case: TC-history-the-archive-finish-keeps-what-the-story-does-not-hold
    case_digest: 7416481d236addab0948f663b50ee255bee4899d1a4fef1357dc85ec8049d528
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:13:14Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-557452
    claim: "Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:fc08af93dfabfa06d4018a95df19a00478eaed7844f4f86f4ec6e3458dcae1d2
    case: TC-history-an-entry-that-differs-from-its-receipt-is-kept
    case_digest: 6b4fe2900d7a4a2f73e65527f91b42c9950373e5e9a1e32fd0643f55501b7d93
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:13:21Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-a0e63d
    claim: "Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:1fe355ce595d3654eeee5bacc4d953b21192fbd42c889058f90dd2d2a7379ab7
    case: TC-history-macos-metadata-never-blocks-the-archive-finish
    case_digest: 9abb5c17fb680ed8e373ae90f5e4b4e5457b4fb3d916834ccf5ab92b2cb7f26b
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:13:27Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-next-names-finishing-the-archive-c8420c
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:a48ad74eabc5530cf26c199008894d1e922146029ffb1c7ca56c56e531278d1b
    case: TC-history-next-names-finishing-the-archive
    case_digest: 070774317b2ea37170632459474d531da442a09db24605e729171fff018eeb1a
    criteria: DRIVE-17
    requirements: REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:13:33Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-front-matter-conflict-is-named-326c94
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:6086be4e6ebd736b5209b66e6c2f23388319254b8dd7d2102f2ad75c9429092a
    case: TC-history-a-front-matter-conflict-is-named
    case_digest: c28d317bab6b5af2bf356176cf7b7c972412547511c18dba7abc93ca1bafeac0
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:13:37Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-456e2b
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:e34ea630df93d38143a6fabaf911be43546c974e0ecc2227ff71b06c4c6b3c4e
    case: TC-history-a-redacted-timeline-entry-is-not-rewritten
    case_digest: ecdb628002127dd0bac95a871c821db001b94787f8905b438849e19d8ab78eb4
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:13:44Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-422bab
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:54020b71c52d14080cc2b84b1c511278746f3491396822cd0fce2f240e980d32
    case: TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries
    case_digest: 519c9da40455e7b15f76ac7bafd083659e6d02313e2d8cb0e4544c18738cc147
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:13:52Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-657ce6
    claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:cf77d6cb8bade5801de5aecb0b842bde8da5ee621d33efe89fc1f54b787c086d
    case: TC-history-an-unreadable-file-never-stops-doctor
    case_digest: 623dd39576587f705f31f4fe1d7e0752b8b2357215804119aedac7c392922313
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:12Z
    duration_seconds: 16
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-73688d
    claim: "Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:a34175570ba734df3e411015afeee5f28c039162aefc32ee4852dfd26b6b68a9
    case: TC-history-the-finish-waits-until-the-story-is-committed
    case_digest: 8c422734c0abfe1244f590414ca6d32cb06ec08f39d05f89e13eff559232cec9
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:19Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-86035d
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:41bc7839ae47f59faebe819ec48c7ce31d7618765ae4ca86dd31861cf0151098
    case: TC-history-a-story-edited-after-its-commit-holds-the-finish
    case_digest: 2caa4b3395afbcb07ad09017b5940a9f729d0b486ef2cf2abe9933fad7c32711
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:26Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-ignored-story-holds-the-finish-2f1763
    claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:e8cfbc931a0f40dab471fd228fb73644ebfc1792ac2a22795ae4efcbc2da17d4
    case: TC-history-an-ignored-story-holds-the-finish
    case_digest: 86e02ab9da1d4f5559805d2d6126a8bc190ccf64d1956cc991f5101bbc274b87
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:33Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-ignored-journal-post-holds-the-finish-52422e
    claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_holds_the_finish for TC-history-an-ignored-journal-post-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_ignored_journal_post_holds_the_finish
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:719b5267c466071dfecf7fceaffad92c21a7af06c954843188711d598d529db1
    case: TC-history-an-ignored-journal-post-holds-the-finish
    case_digest: bcc54521c6af3a3d50e789eab08e36f18f5c3a2f05840ae8f81319eb81161d9a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:40Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-failed-git-read-holds-the-finish-662b6d
    claim: "Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:736e31e09685c5025db70f10ec835b374fa81e7004527be456929569b0b906fa
    case: TC-history-a-failed-git-read-holds-the-finish
    case_digest: 3f403dd3c9fe13b9d9d01caab5d6d7f1c05607d26d93e28fd6ecdbf8697cb5b8
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:45Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-unreadable-entry-is-dropped-by-the-printed
    claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands for TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:dbe517cb475b98fc099f5fb870bc4272b04eb76da013de2a2570abb9566c3597
    case: TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands
    case_digest: 6074ca970b1b27c49b2260941825108ef0fbbefc6353aa716e128bbbde123616
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:55Z
    duration_seconds: 6
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-redacting-save-leaves-no-timeline-rewrite
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite for TC-history-a-redacting-save-leaves-no-timeline-rewrite: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:37b75fb55912133e8c2963a5f5d16c073196a44e1cc1761fdfbbdc6a013e062f
    case: TC-history-a-redacting-save-leaves-no-timeline-rewrite
    case_digest: 5ac9d84346c190942d5aaef827a471fad64fb3124d9e4fb564505b632568f91a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:14:59Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-next-names-finishing-the-archive-c58ccb
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:00f058fdc1fd125f3bd28ed8ead9b44100da6ba7411a0c631a1495978882fae9
    case: TC-cli-next-names-finishing-the-archive
    case_digest: cca9f365ab26edce72dc89919cdadc945421fdf1426e40ca5fce3a75bb9eada8
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:15:04Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-fb684d
    claim: "Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:232699f9d40fe962335ba80cb2ef6c369f359a39f1c89295eaa884ebedc429f0
    case: TC-cli-next-names-the-archive-with-a-session-open
    case_digest: cfe06814bffcaac6430899b4f9068f845bf06dbf349b5f500ac568e80d84361f
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:15:09Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-finish-command-quotes-each-path-57dcf0
    claim: "Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:1fd90fb1398e9a5106a2cbf2a34a2e94be4d9e6830380d2cd05944f96161e429
    case: TC-cli-the-finish-command-quotes-each-path
    case_digest: d68ddc8fbac742b62d0ddb225cac20b80eb7dc0468253d538d1ea05adb2ffaf5
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:15:12Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-archive-line-runs-as-printed-0eeb56
    claim: "Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:69763d4ac16a61bfdec6930e4bce4a3c2585d1cd8c518cd8285b37c516d522f2
    case: TC-cli-the-archive-line-runs-as-printed
    case_digest: 413ca9241d303fc7cca2d3290c0406c7a4a723929d546d0dfef5bf5520ee6980
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:15:16Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-printed-command-commits-the-story-e0f5f6
    claim: "Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:e02cadc16d9fff588947dffc4f241af95d6503ce4c05ab79cb1610d250639341
    case: TC-cli-the-printed-command-commits-the-story
    case_digest: b75f11fceab686a56c7bb4889f42258c3e5427cf589f33f7ba10823be063e9fa
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:15:21Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-front-matter-conflict-is-named-137168
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:823384ca74f80b0e7df98ce0000d58c53094672d1a325db262157163215996cb
    case: TC-records-a-front-matter-conflict-is-named
    case_digest: 18be79babe813b859103021cf7d5c686461a85549f7a415e35e90cdaa6e93862
    criteria: DRIVE-17
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:15:24Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-spec-walk-follows-no-link-and-is-bounded-6ce997
    claim: "Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-records-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded
    revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
    environment: local-macos-arm64
    content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
    fingerprint: sha256:ba4ed43cb1ac6c5ad63247a75497917bb77c49884be21d6ae6e81b03a5722c15
    case: TC-records-the-spec-walk-follows-no-link-and-is-bounded
    case_digest: a2a6be7a89d4cea6de791695ddccb9168054bccd04d1c9b18581f35882507f55
    criteria: DRIVE-17
    requirements: REQ-records-014
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T15:15:27Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-ab3fd3
    claim: "Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:466b45300dde2a942d99dd59a765f759642ebf0da9a7c2389f7db78205c1e6e6
    case: TC-history-the-archive-finish-keeps-what-the-story-does-not-hold
    case_digest: 7416481d236addab0948f663b50ee255bee4899d1a4fef1357dc85ec8049d528
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:09:08Z
    duration_seconds: 11
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-ea5b56
    claim: "Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:459a88b381aa289f559feff1e4a3777e4dfa8c87da00ceb6ab75188895749e1d
    case: TC-history-an-entry-that-differs-from-its-receipt-is-kept
    case_digest: 6b4fe2900d7a4a2f73e65527f91b42c9950373e5e9a1e32fd0643f55501b7d93
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:09:18Z
    duration_seconds: 6
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-a709b2
    claim: "Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:588306821cff11d5695f8b50eca28a38332a495dd5f7742fda4fd604ed25d187
    case: TC-history-macos-metadata-never-blocks-the-archive-finish
    case_digest: 9abb5c17fb680ed8e373ae90f5e4b4e5457b4fb3d916834ccf5ab92b2cb7f26b
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:09:25Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-next-names-finishing-the-archive-ae56ea
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:01d5d76f03cc5b58f2beb183dcf4350a692829956770bfb864e9661a5cf5c106
    case: TC-history-next-names-finishing-the-archive
    case_digest: 070774317b2ea37170632459474d531da442a09db24605e729171fff018eeb1a
    criteria: DRIVE-17
    requirements: REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:09:32Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-front-matter-conflict-is-named-0d34a5
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:48f7c39dde437c51b5b06a74f1cd330c9c3fa066a6a6ee387442b991be39214f
    case: TC-history-a-front-matter-conflict-is-named
    case_digest: c28d317bab6b5af2bf356176cf7b7c972412547511c18dba7abc93ca1bafeac0
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:09:37Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-1ec7fb
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:727ca0038b9148437aed7565411c178219701ebd7c9495e5118daf53dd89d0d3
    case: TC-history-a-redacted-timeline-entry-is-not-rewritten
    case_digest: ecdb628002127dd0bac95a871c821db001b94787f8905b438849e19d8ab78eb4
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:09:45Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-f81635
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:b5cfa8761970d2ca51acbcd15ca63cc68d5c6173279a02c4fc56102d8c4d2c9a
    case: TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries
    case_digest: 519c9da40455e7b15f76ac7bafd083659e6d02313e2d8cb0e4544c18738cc147
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:09:54Z
    duration_seconds: 6
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-614d92
    claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:109d562b39e456780c041caca78c61fa13a4029df768e778cf53565402bf950e
    case: TC-history-an-unreadable-file-never-stops-doctor
    case_digest: 623dd39576587f705f31f4fe1d7e0752b8b2357215804119aedac7c392922313
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:10:22Z
    duration_seconds: 25
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-c9319e
    claim: "Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:25026e1758dca3d0c3c13d3cd6832192d39c208d6c5654d77e65f3d9ab5cc08a
    case: TC-history-the-finish-waits-until-the-story-is-committed
    case_digest: 8c422734c0abfe1244f590414ca6d32cb06ec08f39d05f89e13eff559232cec9
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:10:35Z
    duration_seconds: 9
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-970e31
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:b2834aff0acb18348a8ba1e07bb8b3ceba95405c4f598c69cabc31feacdb8aae
    case: TC-history-a-story-edited-after-its-commit-holds-the-finish
    case_digest: 2caa4b3395afbcb07ad09017b5940a9f729d0b486ef2cf2abe9933fad7c32711
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:10:46Z
    duration_seconds: 8
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-ignored-story-holds-the-finish-00280e
    claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:824fb00b67f858ddabc3d5d1d5200ce9a89f0ebb566617c2010662d03c77907d
    case: TC-history-an-ignored-story-holds-the-finish
    case_digest: 86e02ab9da1d4f5559805d2d6126a8bc190ccf64d1956cc991f5101bbc274b87
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:10:58Z
    duration_seconds: 8
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-ignored-journal-post-does-not-hold-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish for TC-history-an-ignored-journal-post-does-not-hold-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:b785e90ace39bd54f2f757f74c7192d9e8241f6cc3963a8df9c33c8b46748900
    case: TC-history-an-ignored-journal-post-does-not-hold-the-finish
    case_digest: 31fbceaff5b2eb87e4aacd537d6ff532ff62df280919eabd04c48a6756d5671a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:11:06Z
    duration_seconds: 5
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-failed-git-read-holds-the-finish-6107ca
    claim: "Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:e55580cb7d0e0ea71f9549673d01e891eb01ae8abb15af68f639e5c7579cdc18
    case: TC-history-a-failed-git-read-holds-the-finish
    case_digest: 3f403dd3c9fe13b9d9d01caab5d6d7f1c05607d26d93e28fd6ecdbf8697cb5b8
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:11:14Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-an-unreadable-entry-is-dropped-by-the-printed-e874c0
    claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands for TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:4ab4d0e6a965019e714ae298a80bc58cd8eaef82a25833e67e7a30c302574cbd
    case: TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands
    case_digest: 6074ca970b1b27c49b2260941825108ef0fbbefc6353aa716e128bbbde123616
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:11:31Z
    duration_seconds: 13
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-redacting-save-leaves-no-timeline-rewrite-d78011
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite for TC-history-a-redacting-save-leaves-no-timeline-rewrite: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:de0aefbe2f6292b91d82bc03a2b34c7d3038d3c6a941de4bf020b53f42deb91a
    case: TC-history-a-redacting-save-leaves-no-timeline-rewrite
    case_digest: 5ac9d84346c190942d5aaef827a471fad64fb3124d9e4fb564505b632568f91a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:11:36Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-a-kept-entry-is-kept-by-its-file
    claim: "Ran cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id for TC-history-a-kept-entry-is-kept-by-its-file: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:8ab4d600306a4dfe5fffd3188fb4e87d7bcfa181c843bcb03f2c27824558045b
    case: TC-history-a-kept-entry-is-kept-by-its-file
    case_digest: 768ee8a528719bfdbba66e84cc11491e82351b770ade2896855af81ee75fa3fc
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:11:56Z
    duration_seconds: 16
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-next-finishes-an-ignored-story-as-printed
    claim: "Ran cargo test --locked --test sessions -- --exact next_finishes_an_ignored_story_as_printed for TC-history-next-finishes-an-ignored-story-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact next_finishes_an_ignored_story_as_printed
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:0e518b42fb584bd5342bc3873ee6198f341fe7cdc9a86c97d304f10c131536e3
    case: TC-history-next-finishes-an-ignored-story-as-printed
    case_digest: 7f40de6b91730e7a1a72c62501c17629ca1672d92fac8f4ad1cfe15e6fa9976e
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:12:06Z
    duration_seconds: 6
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-history-the-way-out-names-the-open-sessions
    claim: "Ran cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active for TC-history-the-way-out-names-the-open-sessions: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:b41070550a5b2af840a5061b8ebc72d26164cb30b606e531c94789f3ac01e84f
    case: TC-history-the-way-out-names-the-open-sessions
    case_digest: ea61f8b1731f5ac470892706e868693c5ee7e9fbc9d462a5748f39380f4be02f
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:12:17Z
    duration_seconds: 7
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-next-names-finishing-the-archive-3b567b
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:8ba1d0c86072a3562550af06b8964450305000e974989c049bb4ab4a558040a2
    case: TC-cli-next-names-finishing-the-archive
    case_digest: cca9f365ab26edce72dc89919cdadc945421fdf1426e40ca5fce3a75bb9eada8
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:12:27Z
    duration_seconds: 7
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-bf2efa
    claim: "Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:2843629d8e0bb605e3119b4139928056896d265b4e180eec47121cb9248dac0e
    case: TC-cli-next-names-the-archive-with-a-session-open
    case_digest: cfe06814bffcaac6430899b4f9068f845bf06dbf349b5f500ac568e80d84361f
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:12:41Z
    duration_seconds: 10
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-finish-command-quotes-each-path-0ab8d3
    claim: "Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:a1916486763a3f36e61913eb54245224d14f85efd29bf6d5407ad49e25b78a4f
    case: TC-cli-the-finish-command-quotes-each-path
    case_digest: d68ddc8fbac742b62d0ddb225cac20b80eb7dc0468253d538d1ea05adb2ffaf5
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:12:51Z
    duration_seconds: 5
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-archive-line-runs-as-printed-4c90e0
    claim: "Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:3eacb545f6aaeda32a1043a39528017634eb1882557cc9f75902653e0ac9b1f6
    case: TC-cli-the-archive-line-runs-as-printed
    case_digest: 413ca9241d303fc7cca2d3290c0406c7a4a723929d546d0dfef5bf5520ee6980
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:13:04Z
    duration_seconds: 10
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-the-printed-command-commits-the-story-16ebcd
    claim: "Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:402b54b6bf510eebd309d8a3fb2a20bfc10cb5a7fc6d0cfbe2529e5ee83ecc55
    case: TC-cli-the-printed-command-commits-the-story
    case_digest: b75f11fceab686a56c7bb4889f42258c3e5427cf589f33f7ba10823be063e9fa
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:13:20Z
    duration_seconds: 12
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-a-front-matter-conflict-is-named-8aa642
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:7256e110102d3af02f8bb72b18c108a9ebeff0d273b24c830b1bdbf06aa9fb14
    case: TC-records-a-front-matter-conflict-is-named
    case_digest: 18be79babe813b859103021cf7d5c686461a85549f7a415e35e90cdaa6e93862
    criteria: DRIVE-17
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:13:26Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-records-the-spec-walk-follows-no-link-and-is-bounded-20c91e
    claim: "Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-records-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded
    revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
    environment: local-macos-arm64
    content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
    fingerprint: sha256:f1652c689c70c566b27345e8a6d5ff4f8fa89ea5ccfa96169c2e0f5e9b959e44
    case: TC-records-the-spec-walk-follows-no-link-and-is-bounded
    case_digest: a2a6be7a89d4cea6de791695ddccb9168054bccd04d1c9b18581f35882507f55
    criteria: DRIVE-17
    requirements: REQ-records-014
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T16:13:33Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-19-tc-cli-next-names-finishing-the-archive-fc1037
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:f3e160828e50c4fe6d7edabadcba881009e6e1bc14c820d81da812b9f0d0b30a
    case: TC-cli-next-names-finishing-the-archive
    case_digest: cca9f365ab26edce72dc89919cdadc945421fdf1426e40ca5fce3a75bb9eada8
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:20Z
    duration_seconds: 3
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-cli-next-names-the-archive-with-a-session-open-c680de
    claim: "Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:0946f43cff619e3b5bb8db7c8d505756587875a5890ae31d6b5827beed0b8cb7
    case: TC-cli-next-names-the-archive-with-a-session-open
    case_digest: cfe06814bffcaac6430899b4f9068f845bf06dbf349b5f500ac568e80d84361f
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:25Z
    duration_seconds: 2
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-cli-the-finish-command-quotes-each-path-c735ad
    claim: "Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:2c014c080160bef69639e932d5241aa2c952df75cbda4938f0b33294a341ba4c
    case: TC-cli-the-finish-command-quotes-each-path
    case_digest: d68ddc8fbac742b62d0ddb225cac20b80eb7dc0468253d538d1ea05adb2ffaf5
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:27Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-cli-the-archive-line-runs-as-printed-5b625b
    claim: "Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:b125451cba21cd07709b86debd86197d649a4e3ea597ac5a5d1466a351d795f2
    case: TC-cli-the-archive-line-runs-as-printed
    case_digest: 413ca9241d303fc7cca2d3290c0406c7a4a723929d546d0dfef5bf5520ee6980
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:31Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-cli-the-printed-command-commits-the-story-a2638f
    claim: "Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:23521c9a82ce9ac3fec2e376ef3bd39b6c39f1b82fd26796b8a195e86df88c68
    case: TC-cli-the-printed-command-commits-the-story
    case_digest: b75f11fceab686a56c7bb4889f42258c3e5427cf589f33f7ba10823be063e9fa
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:34Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-the-archive-finish-keeps-what-the-story-does-not-cbe02e
    claim: "Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:6aa0128c030ad421f5979981ad29995eebbd9238924967150164e35747412b14
    case: TC-history-the-archive-finish-keeps-what-the-story-does-not-hold
    case_digest: 7416481d236addab0948f663b50ee255bee4899d1a4fef1357dc85ec8049d528
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:39Z
    duration_seconds: 2
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-an-entry-that-differs-from-its-receipt-is-kept-3f536d
    claim: "Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:a4afb6b4dacea0dbf6846f8c1e4bea6816e5817dd00cbd320e0d173fda8be5fe
    case: TC-history-an-entry-that-differs-from-its-receipt-is-kept
    case_digest: 6b4fe2900d7a4a2f73e65527f91b42c9950373e5e9a1e32fd0643f55501b7d93
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:43Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-macos-metadata-never-blocks-the-archive-finish-55bf2f
    claim: "Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:6d73b4a8aa9253177f9890f2bf1ff83038b97f8f3f1cab383ae4a070736f5bc5
    case: TC-history-macos-metadata-never-blocks-the-archive-finish
    case_digest: 9abb5c17fb680ed8e373ae90f5e4b4e5457b4fb3d916834ccf5ab92b2cb7f26b
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:46Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-next-names-finishing-the-archive-6264fb
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:d7199e90d5dd2c8fb0baab44079b0c92d7585d19b9db26272ecf51f2150d263f
    case: TC-history-next-names-finishing-the-archive
    case_digest: 070774317b2ea37170632459474d531da442a09db24605e729171fff018eeb1a
    criteria: DRIVE-17
    requirements: REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:49Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-a-front-matter-conflict-is-named-e19237
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:64046a05f8105165f1a8d81c96e65b6e897298aacb94dcccd1bbfdaf10992fc3
    case: TC-history-a-front-matter-conflict-is-named
    case_digest: c28d317bab6b5af2bf356176cf7b7c972412547511c18dba7abc93ca1bafeac0
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:52Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-a-redacted-timeline-entry-is-not-rewritten-a6832a
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:78bce2c8b942c76edc46b4cf7be8ae15825d00cc3ce2339cda8f18070e2a9db0
    case: TC-history-a-redacted-timeline-entry-is-not-rewritten
    case_digest: ecdb628002127dd0bac95a871c821db001b94787f8905b438849e19d8ab78eb4
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:55Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-3cffa7
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:b81dae5e22bac26c62773278a9ed1cb7af37d27546158d941f9413aac2638888
    case: TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries
    case_digest: 519c9da40455e7b15f76ac7bafd083659e6d02313e2d8cb0e4544c18738cc147
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:43:58Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-3b0644
    claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:00214c384f66173b39eb210f966c01bde4aaf9ec55dd9d3adc0bc6c34b57bf03
    case: TC-history-an-unreadable-file-never-stops-doctor
    case_digest: 623dd39576587f705f31f4fe1d7e0752b8b2357215804119aedac7c392922313
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:04Z
    duration_seconds: 3
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-the-finish-waits-until-the-story-is-committed-0c0669
    claim: "Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:a9ef8670d5b74e01f7b8c1ff75e37aa6e6d70dac34ab4ca5b98ee8f193f21a68
    case: TC-history-the-finish-waits-until-the-story-is-committed
    case_digest: 8c422734c0abfe1244f590414ca6d32cb06ec08f39d05f89e13eff559232cec9
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:07Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-a-story-edited-after-its-commit-holds-the-finish-67f44b
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:f371a08afed459439f1d167caeeea9b81f6ffea93d47f0cf8e3fc5b80743abc0
    case: TC-history-a-story-edited-after-its-commit-holds-the-finish
    case_digest: 2caa4b3395afbcb07ad09017b5940a9f729d0b486ef2cf2abe9933fad7c32711
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:11Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-an-ignored-story-holds-the-finish-201e6d
    claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:6c9f6068a655ae53c4a7c11fe4d26f86e0c07e53e096c708813806cfdcc63bbc
    case: TC-history-an-ignored-story-holds-the-finish
    case_digest: 86e02ab9da1d4f5559805d2d6126a8bc190ccf64d1956cc991f5101bbc274b87
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:14Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-an-ignored-journal-post-does-not-hold-the-finish-82b94f
    claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish for TC-history-an-ignored-journal-post-does-not-hold-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:ddc29da8a641ab2b4092c5532c07ea241aba52ba5d0382725db06dd8cd0e6b23
    case: TC-history-an-ignored-journal-post-does-not-hold-the-finish
    case_digest: 31fbceaff5b2eb87e4aacd537d6ff532ff62df280919eabd04c48a6756d5671a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:18Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-a-failed-git-read-holds-the-finish-7038fa
    claim: "Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:54d53ce094e5a2484755b1624fec5dc3bb8db8a6a4c6d005dac561dee4c95d1e
    case: TC-history-a-failed-git-read-holds-the-finish
    case_digest: 3f403dd3c9fe13b9d9d01caab5d6d7f1c05607d26d93e28fd6ecdbf8697cb5b8
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:21Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-an-unreadable-entry-is-dropped-by-the-printed-edd175
    claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands for TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:5939f5abb795b2b065e0cf522244fff3de6f60eea2ef76c5ef3b96d14f4a242b
    case: TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands
    case_digest: 6074ca970b1b27c49b2260941825108ef0fbbefc6353aa716e128bbbde123616
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:25Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-a-redacting-save-leaves-no-timeline-rewrite-46f04e
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite for TC-history-a-redacting-save-leaves-no-timeline-rewrite: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:f5dc76333788d5ed44e82a377390a66f3bb2112efa49fa25fe2f8b0cc4105c68
    case: TC-history-a-redacting-save-leaves-no-timeline-rewrite
    case_digest: 5ac9d84346c190942d5aaef827a471fad64fb3124d9e4fb564505b632568f91a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:27Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-a-kept-entry-is-kept-by-its-file-4c635b
    claim: "Ran cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id for TC-history-a-kept-entry-is-kept-by-its-file: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:c5d8d3aabded3fdfb3e35dbf28c401be33be61258ac3e5a839e0cb997309e352
    case: TC-history-a-kept-entry-is-kept-by-its-file
    case_digest: 768ee8a528719bfdbba66e84cc11491e82351b770ade2896855af81ee75fa3fc
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:33Z
    duration_seconds: 3
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-next-finishes-an-ignored-story-as-printed-4b7c95
    claim: "Ran cargo test --locked --test sessions -- --exact next_finishes_an_ignored_story_as_printed for TC-history-next-finishes-an-ignored-story-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact next_finishes_an_ignored_story_as_printed
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:824d6939c34663e431e301ffddd87a07832986fb30a86088aaa26521f8a5e1c3
    case: TC-history-next-finishes-an-ignored-story-as-printed
    case_digest: 7f40de6b91730e7a1a72c62501c17629ca1672d92fac8f4ad1cfe15e6fa9976e
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:36Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-the-way-out-names-the-open-sessions-4107d9
    claim: "Ran cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active for TC-history-the-way-out-names-the-open-sessions: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:a14da88c2b4a894a81af9e21f88d221f5f2a494521a350eea9f365f3b988b23f
    case: TC-history-the-way-out-names-the-open-sessions
    case_digest: ea61f8b1731f5ac470892706e868693c5ee7e9fbc9d462a5748f39380f4be02f
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:40Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-an-untracked-kept-entry-is-committed-first
    claim: "Ran cargo test --locked --test sessions -- --exact an_untracked_kept_entry_is_committed_first_so_its_way_out_runs for TC-history-an-untracked-kept-entry-is-committed-first: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_untracked_kept_entry_is_committed_first_so_its_way_out_runs
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:3878bc2dfbfcb43d5852245de3c3c072ad492bf3e155074176d03c2c470fc4ab
    case: TC-history-an-untracked-kept-entry-is-committed-first
    case_digest: 74983f76bcf414a85ba51e127ebc98e82f276b0bcda5fa400244b80d437759dd
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:44Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-an-inbox-entry-is-kept-whatever-its-name
    claim: "Ran cargo test --locked --test sessions -- --exact an_inbox_entry_is_kept_whatever_its_name for TC-history-an-inbox-entry-is-kept-whatever-its-name: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_inbox_entry_is_kept_whatever_its_name
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:4afd142d89b1ea728d28b1a0fc8c19a79b8cc4d718e094e1a70dceb2049bda84
    case: TC-history-an-inbox-entry-is-kept-whatever-its-name
    case_digest: 239e62b0874ea258aaed6e5e48acb7208fe7fe805f843405ce534ae27c6cf4b9
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:48Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-a-folded-inbox-entry-does-not-hold-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact a_folded_inbox_entry_does_not_hold_the_finish for TC-history-a-folded-inbox-entry-does-not-hold-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_folded_inbox_entry_does_not_hold_the_finish
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:34cf3543220064a37255361157ccfc0ebf602cb04b2603565b7623df861d1505
    case: TC-history-a-folded-inbox-entry-does-not-hold-the-finish
    case_digest: 54c0d8ad5f8d90c7aa0cf321207f18f9d244df51bc2ca4448b69c80ea673c296
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:51Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-history-an-inbox-file-inbox-add-would-not-take-is-only
    claim: "Ran cargo test --locked --test sessions -- --exact an_inbox_file_inbox_add_would_not_take_is_only_dropped for TC-history-an-inbox-file-inbox-add-would-not-take-is-only-dropped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_inbox_file_inbox_add_would_not_take_is_only_dropped
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:5ec1a8e4c1f9a2c061e83d1eef9f8f12cf36df42da3a8082e20ec08d4a991c8c
    case: TC-history-an-inbox-file-inbox-add-would-not-take-is-only-dropped
    case_digest: 1665078ecc0a685a13e6b2748ca76bed04dc4928d894c6e2f1409c67d1692c0c
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:55Z
    duration_seconds: 1
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-records-a-front-matter-conflict-is-named-610b0b
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:62fa842b199975c78c446dd9394e3ed874ac8f6945d79ff83a4beec227164034
    case: TC-records-a-front-matter-conflict-is-named
    case_digest: 18be79babe813b859103021cf7d5c686461a85549f7a415e35e90cdaa6e93862
    criteria: DRIVE-17
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:44:58Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-19-tc-records-the-spec-walk-follows-no-link-and-is-bounded-0cea68
    claim: "Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-records-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded
    revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
    environment: local-macos-arm64
    content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
    fingerprint: sha256:764ae7ea03332ab554dfa6b12c9e2b46d67ecd413f842e984ef239e83fdfc301
    case: TC-records-the-spec-walk-follows-no-link-and-is-bounded
    case_digest: a2a6be7a89d4cea6de791695ddccb9168054bccd04d1c9b18581f35882507f55
    criteria: DRIVE-17
    requirements: REQ-records-014
    limitations: inherited environment variables were not recorded
    observed: 2026-11-19T19:45:01Z
    duration_seconds: 0
    actor: claude-code (hc t4n)
  - id: 2026-11-20-tc-cli-next-names-finishing-the-archive
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-cli-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:abe6711401da83d363113e3d9f53438063895149761e0afa498139fc03b315df
    case: TC-cli-next-names-finishing-the-archive
    case_digest: cca9f365ab26edce72dc89919cdadc945421fdf1426e40ca5fce3a75bb9eada8
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:47:59Z
    duration_seconds: 4
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-next-names-the-archive-with-a-session-open
    claim: "Ran cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first for TC-cli-next-names-the-archive-with-a-session-open: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_with_a_session_open_names_an_unfinished_archive_first
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:a2d87af52e4e76b3fb833847ffed0e2cef9fd53251c2262f1825bdedca6ae558
    case: TC-cli-next-names-the-archive-with-a-session-open
    case_digest: cfe06814bffcaac6430899b4f9068f845bf06dbf349b5f500ac568e80d84361f
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:04Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-finish-command-quotes-each-path
    claim: "Ran cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out for TC-cli-the-finish-command-quotes-each-path: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_finish_command_quotes_each_path_and_leaves_metadata_out
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:e2de77eb007173ca3c85fdc9ffca0e4c3ad4894bb906df2f5e7165304a3ca9f6
    case: TC-cli-the-finish-command-quotes-each-path
    case_digest: d68ddc8fbac742b62d0ddb225cac20b80eb7dc0468253d538d1ea05adb2ffaf5
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:07Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-archive-line-runs-as-printed
    claim: "Ran cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting for TC-cli-the-archive-line-runs-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_archive_line_runs_as_printed_with_other_archives_waiting
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:2be04fa072138bed17e48fac598c0e14fb1de8659d1a745e719ab413f1ed0693
    case: TC-cli-the-archive-line-runs-as-printed
    case_digest: 413ca9241d303fc7cca2d3290c0406c7a4a723929d546d0dfef5bf5520ee6980
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:11Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-cli-the-printed-command-commits-the-story
    claim: "Ran cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post for TC-cli-the-printed-command-commits-the-story: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact the_printed_command_commits_the_story_with_or_without_a_journal_post
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:25ef800361649a5872bf736ff409e3f5c8a9a5051dd348c28eeef4bc435148ab
    case: TC-cli-the-printed-command-commits-the-story
    case_digest: b75f11fceab686a56c7bb4889f42258c3e5427cf589f33f7ba10823be063e9fa
    criteria: DRIVE-17
    requirements: REQ-cli-021
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:15Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-the-archive-finish-keeps-what-the-story-does-not
    claim: "Ran cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold for TC-history-the-archive-finish-keeps-what-the-story-does-not-hold: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_archive_finish_keeps_what_the_story_does_not_hold
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:eb0516b24dd32f7e49aca433e3bb8a6489ce49e312cd1af44950d72ea83d44d4
    case: TC-history-the-archive-finish-keeps-what-the-story-does-not-hold
    case_digest: 7416481d236addab0948f663b50ee255bee4899d1a4fef1357dc85ec8049d528
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:21Z
    duration_seconds: 2
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-an-entry-that-differs-from-its-receipt-is-kept
    claim: "Ran cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept for TC-history-an-entry-that-differs-from-its-receipt-is-kept: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_entry_that_differs_from_its_receipt_is_kept
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:2044b2453fdfa0d0f306f47bd459756a995aabe04b2fbdc5d607092b64440867
    case: TC-history-an-entry-that-differs-from-its-receipt-is-kept
    case_digest: 6b4fe2900d7a4a2f73e65527f91b42c9950373e5e9a1e32fd0643f55501b7d93
    criteria: DRIVE-17
    requirements: REQ-history-012, REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:25Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-macos-metadata-never-blocks-the-archive-finish
    claim: "Ran cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish for TC-history-macos-metadata-never-blocks-the-archive-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact macos_metadata_never_blocks_the_archive_finish
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:b6f29907e9c4eae7f4ff25a792ad6b033c00adea70162193b6f6d084da8bca91
    case: TC-history-macos-metadata-never-blocks-the-archive-finish
    case_digest: 9abb5c17fb680ed8e373ae90f5e4b4e5457b4fb3d916834ccf5ab92b2cb7f26b
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:28Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-next-names-finishing-the-archive
    claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:ceda34ac0a5ddc36433d4732e897b08b62c3c0d14ce92cb13398ed08652aea9f
    case: TC-history-next-names-finishing-the-archive
    case_digest: 070774317b2ea37170632459474d531da442a09db24605e729171fff018eeb1a
    criteria: DRIVE-17
    requirements: REQ-history-006
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:32Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-a-front-matter-conflict-is-named
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:c4e85ecdb929d78e7926b6f28001697fccf71b2e904d8ad14070ab87d1396e8a
    case: TC-history-a-front-matter-conflict-is-named
    case_digest: c28d317bab6b5af2bf356176cf7b7c972412547511c18dba7abc93ca1bafeac0
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:35Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-a-redacted-timeline-entry-is-not-rewritten
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is for TC-history-a-redacted-timeline-entry-is-not-rewritten: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacted_timeline_entry_is_not_rewritten_but_a_hand_edit_is
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:8f7aee315aaae08e3777d00df40e00b152707f5ebbef9ce6de01ab1fc10ddbaf
    case: TC-history-a-redacted-timeline-entry-is-not-rewritten
    case_digest: ecdb628002127dd0bac95a871c821db001b94787f8905b438849e19d8ab78eb4
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:39Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-a-story-an-earlier-release-wrote-still-holds-its
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:6646d2e9c392d3382d7ca3a36a63370be814081b0aeba0e9331a6380a7b6c64b
    case: TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries
    case_digest: 519c9da40455e7b15f76ac7bafd083659e6d02313e2d8cb0e4544c18738cc147
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:42Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-an-unreadable-file-never-stops-doctor
    claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:137469c4c48a93286f5d0545590caf552556e7c2aa24a587e29159a7fd55849d
    case: TC-history-an-unreadable-file-never-stops-doctor
    case_digest: 623dd39576587f705f31f4fe1d7e0752b8b2357215804119aedac7c392922313
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:49Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-the-finish-waits-until-the-story-is-committed
    claim: "Ran cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed for TC-history-the-finish-waits-until-the-story-is-committed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_finish_waits_until_the_story_and_post_are_committed
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:80e51b74fda1424d6410745e438e58a8b405a7e1e148f8a171b91ce487888528
    case: TC-history-the-finish-waits-until-the-story-is-committed
    case_digest: 8c422734c0abfe1244f590414ca6d32cb06ec08f39d05f89e13eff559232cec9
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:53Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-a-story-edited-after-its-commit-holds-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish for TC-history-a-story-edited-after-its-commit-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_edited_after_its_commit_holds_the_finish
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:2bca1aff93a0984a610d8a6f1e6e2e7f335f5d130418220d37330962e8c8fccc
    case: TC-history-a-story-edited-after-its-commit-holds-the-finish
    case_digest: 2caa4b3395afbcb07ad09017b5940a9f729d0b486ef2cf2abe9933fad7c32711
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:48:57Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-an-ignored-story-holds-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f for TC-history-an-ignored-story-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_ignored_story_holds_the_finish_and_names_git_add_f
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:8399fc34118565c76d5145a0ea0e51f398ec525b958be760b6007c0eba35387c
    case: TC-history-an-ignored-story-holds-the-finish
    case_digest: 86e02ab9da1d4f5559805d2d6126a8bc190ccf64d1956cc991f5101bbc274b87
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:01Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-an-ignored-journal-post-does-not-hold-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish for TC-history-an-ignored-journal-post-does-not-hold-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_ignored_journal_post_does_not_hold_the_finish
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:fc0874804be1aa7e144045ea4d4d9c050b80a596e16792d50adc7448b5708c47
    case: TC-history-an-ignored-journal-post-does-not-hold-the-finish
    case_digest: 31fbceaff5b2eb87e4aacd537d6ff532ff62df280919eabd04c48a6756d5671a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:05Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-a-failed-git-read-holds-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so for TC-history-a-failed-git-read-holds-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_failed_git_read_holds_the_finish_and_says_so
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:34a22531c7eef18e2f004ee6d232a1c6d4d885d0565f505af138422eaf504df2
    case: TC-history-a-failed-git-read-holds-the-finish
    case_digest: 3f403dd3c9fe13b9d9d01caab5d6d7f1c05607d26d93e28fd6ecdbf8697cb5b8
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:08Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-an-unreadable-entry-is-dropped-by-the-printed
    claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands for TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_unreadable_entry_is_dropped_by_the_printed_commands
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:089692d5c1fbc24425ad4ddaf5add4d2fcbc5caff67ff49112e9e1a3948d9991
    case: TC-history-an-unreadable-entry-is-dropped-by-the-printed-commands
    case_digest: 6074ca970b1b27c49b2260941825108ef0fbbefc6353aa716e128bbbde123616
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:13Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-a-redacting-save-leaves-no-timeline-rewrite
    claim: "Ran cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite for TC-history-a-redacting-save-leaves-no-timeline-rewrite: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_redacting_save_leaves_no_timeline_rewrite
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:de4928d60123d2c3327e483eac4131c5412fc2d13bed5b9ba2b4a73f08d7b034
    case: TC-history-a-redacting-save-leaves-no-timeline-rewrite
    case_digest: 5ac9d84346c190942d5aaef827a471fad64fb3124d9e4fb564505b632568f91a
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:17Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-a-kept-entry-is-kept-by-its-file
    claim: "Ran cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id for TC-history-a-kept-entry-is-kept-by-its-file: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:569f8657b90c8672422a924c63af5014953b2a667bebbdf164197696a0c5321a
    case: TC-history-a-kept-entry-is-kept-by-its-file
    case_digest: 768ee8a528719bfdbba66e84cc11491e82351b770ade2896855af81ee75fa3fc
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:23Z
    duration_seconds: 3
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-next-finishes-an-ignored-story-as-printed
    claim: "Ran cargo test --locked --test sessions -- --exact next_finishes_an_ignored_story_as_printed for TC-history-next-finishes-an-ignored-story-as-printed: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact next_finishes_an_ignored_story_as_printed
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:e21f89c4479d3ede73eb8c9e74b31cec1b9ff5b6eb005c7db10e892f5e5d184e
    case: TC-history-next-finishes-an-ignored-story-as-printed
    case_digest: 7f40de6b91730e7a1a72c62501c17629ca1672d92fac8f4ad1cfe15e6fa9976e
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:27Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-the-way-out-names-the-open-sessions
    claim: "Ran cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active for TC-history-the-way-out-names-the-open-sessions: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:fee6ad68e2b7df69502c49bbaa4935598934ebe926baae8deb80027e8ba314bf
    case: TC-history-the-way-out-names-the-open-sessions
    case_digest: ea61f8b1731f5ac470892706e868693c5ee7e9fbc9d462a5748f39380f4be02f
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:31Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-an-untracked-kept-entry-is-committed-first
    claim: "Ran cargo test --locked --test sessions -- --exact an_untracked_kept_entry_is_committed_first_so_its_way_out_runs for TC-history-an-untracked-kept-entry-is-committed-first: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_untracked_kept_entry_is_committed_first_so_its_way_out_runs
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:f5c568dc3363609fc4348bcc4205ee71a64e4806d0d595abb01a41777b8d9d4c
    case: TC-history-an-untracked-kept-entry-is-committed-first
    case_digest: 74983f76bcf414a85ba51e127ebc98e82f276b0bcda5fa400244b80d437759dd
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:35Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-an-inbox-entry-is-kept-whatever-its-name
    claim: "Ran cargo test --locked --test sessions -- --exact an_inbox_entry_is_kept_whatever_its_name for TC-history-an-inbox-entry-is-kept-whatever-its-name: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_inbox_entry_is_kept_whatever_its_name
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:400aa9a143d311a0bff1a60aa9258ff5f31e666f3f354178c5fa2535f08f2a1a
    case: TC-history-an-inbox-entry-is-kept-whatever-its-name
    case_digest: 239e62b0874ea258aaed6e5e48acb7208fe7fe805f843405ce534ae27c6cf4b9
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:39Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-a-folded-inbox-entry-does-not-hold-the-finish
    claim: "Ran cargo test --locked --test sessions -- --exact a_folded_inbox_entry_does_not_hold_the_finish for TC-history-a-folded-inbox-entry-does-not-hold-the-finish: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_folded_inbox_entry_does_not_hold_the_finish
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:d6c0cff8b9d51b2e3e5a76dea66405395e2093a8e4b730da300f751e72406e2e
    case: TC-history-a-folded-inbox-entry-does-not-hold-the-finish
    case_digest: 54c0d8ad5f8d90c7aa0cf321207f18f9d244df51bc2ca4448b69c80ea673c296
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:42Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-history-an-inbox-file-inbox-add-would-not-take-is-only
    claim: "Ran cargo test --locked --test sessions -- --exact an_inbox_file_inbox_add_would_not_take_is_only_dropped for TC-history-an-inbox-file-inbox-add-would-not-take-is-only-dropped: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact an_inbox_file_inbox_add_would_not_take_is_only_dropped
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:4accd8c08219c1670d65fe761f182dbced495cdd30f87912d8dbe2aaf458d224
    case: TC-history-an-inbox-file-inbox-add-would-not-take-is-only-dropped
    case_digest: 1665078ecc0a685a13e6b2748ca76bed04dc4928d894c6e2f1409c67d1692c0c
    criteria: DRIVE-17
    requirements: REQ-history-012
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:47Z
    duration_seconds: 1
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-a-front-matter-conflict-is-named
    claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:b1e9dcfc03005bf8710565458f1dcd990bd40e74b1cdead31a65f64f4bbc10cc
    case: TC-records-a-front-matter-conflict-is-named
    case_digest: 18be79babe813b859103021cf7d5c686461a85549f7a415e35e90cdaa6e93862
    criteria: DRIVE-17
    requirements: REQ-records-001
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:50Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
  - id: 2026-11-20-tc-records-the-spec-walk-follows-no-link-and-is-bounded
    claim: "Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-records-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded
    revision: 53309b87af13170939e413a6513c16dc5063683c
    environment: local-macos-arm64
    content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
    fingerprint: sha256:508800631c282b53ca958e33f3634904bf59bcb1234fe1f22b0fc22ba7a256cc
    case: TC-records-the-spec-walk-follows-no-link-and-is-bounded
    case_digest: a2a6be7a89d4cea6de791695ddccb9168054bccd04d1c9b18581f35882507f55
    criteria: DRIVE-17
    requirements: REQ-records-014
    limitations: inherited environment variables were not recorded
    observed: 2026-11-20T00:49:54Z
    duration_seconds: 0
    actor: chough-fixer (claude-opus-5-5, hc t4n)
```
