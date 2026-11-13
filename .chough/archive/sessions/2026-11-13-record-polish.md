---
kind: session
id: 2026-11-13-record-polish
aliases: [record-polish]
objective: "0.6.1: record polish: a bug's Actual line stands apart, migrated front matter names the new entries, and a fresh project's records README and the README's init paragraph are trimmed"
stage: done
stage_status: archived
iteration: 1
native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
cases:
  - TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
  - TC-records-migrated-front-matter-names-the-carried-entries
  - TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly
  - TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5
observations:
  - 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
  - 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries
  - 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories
  - 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept
  - 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-be05bb
  - 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-752301
  - 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-ed78e8
  - 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-24ba1f
  - 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-f7c2df
  - 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-305164
  - 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-fd85ba
  - 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-2098aa
  - 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-9d1662
  - 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-cf4667
  - 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-239237
  - 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-d27b46
  - 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
  - 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-7d5867
  - 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-5ac7fd
  - 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-91bb77
  - 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-bb3408
  - 2026-11-13-the-full-test-suite-passed-740-of-740-across-48-targets-cargo
  - 2026-11-13-the-full-test-suite-passed-740-of-740-across-48-targets-cargo-1fa770
  - 2026-11-13-the-full-test-suite-passed-742-of-742-across-48-targets-cargo
bugs:
  - 2026-11-12-a-bug-body-runs-its-symptom-and-its-actual-line-into-one
  - 2026-11-12-migrated-records-keep-old-observation-ids-in-their-front
  - 2026-11-12-a-fresh-project-s-records-readme-names-artifacts-as-kept-from
title: "0.6.1: record polish: a bug's Actual line, migrated names, a fresh guide"
summary:
  - A bug body gives its Actual line its own paragraph; migrated front matter names the carried entries, keeping each 0.5.x ID as their alias; a fresh project's reading guide names no 0.5.x folder, and the README's init paragraph names the host directories in one sentence.
  - "Verified: the cases red then green, three bugs verified, and the full suite 740 of 740. Next: Leif's review and merge."
closed: 2026-11-13
outcome: done
---
# 0.6.1: record polish: a bug's Actual line stands apart, migrated front matter names the new entries, and a fresh project's records README and the README's init paragraph are trimmed

## Goal

0.6.1: record polish: a bug's Actual line stands apart, migrated front matter names the new entries, and a fresh project's records README and the README's init paragraph are trimmed

## Where it stands

- PR 5: a bug's Actual paragraph, migrated front matter naming the carried entries, and a fresh project's reading guide and README init paragraph; red then green, three bugs verified

## Decisions and why

- Leif decided "Approve the intent: record polish for bugs, migrated front matter and the READMEs": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated
- Leif decided "Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s": "Yes, start (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s as 0xLeif
- Leif decided "Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s with its corrected file list": "Yes, start (Recommended) / Yes, fix and keep (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s as 0xLeif
- Leif decided "Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s with its exact file list": "Yes, start (Recommended) / Yes, fix and keep (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s as 0xLeif
- Leif decided "Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s with tests/evidence.rs added": "Yes, start (Recommended) / Yes, fix and keep (Recommended) / Yes, standing rule (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); and under his FILE-LIST RULE at 2026-11-13T04:35Z, 'Yes, standing rule (Recommended)', answering: 'Standing rule? Sometimes an approved change turns out to touch a few more files than its list says, for example old tests that pin the printed text. The work itself is unchanged. May agents send it back and re-approve it under your original words, with a note listing the added files, instead of asking you each time?' (option text: 'Only when what the change does stays the same. Anything that changes its meaning still comes to you.'); added: tests/evidence.rs (a test that looked up a migrated observation by its 0.5.x ID now looks it up by the carried entry's name, which this change's F9 fix gives it); relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s as 0xLeif
- Leif decided "Implementation review of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s": "#256 record polish (0.6.1)" (coordinator chat (Claude Code, hc t370), 2026-11-13T14:14Z, answering: 'Chough PRs passed by the reviewer, part 1: approve the implementation? Each ships after your yes, then you'll get a merge it.', he selected '#256 record polish (0.6.1)', '#258 CI flake fix (#247)', '#259 migrate preview (#246)' and '#261 relay details (#249)'; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change review 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s as 0xLeif with verdict pass

## What was verified

- 16 earlier runs of the cases below are kept in this story's receipts, not listed here.
- The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a1f4735a, PR 5 with #243 merged in; clippy --all-targets -D warnings and rustfmt --check were clean ^2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
- Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1). — 378393db8bc8 ^2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-7d5867
- Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1). — 9c0424c05f5c ^2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-5ac7fd
- Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1). — d272592c28a5 ^2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-91bb77
- Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1). — 37d378af0b47 ^2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-bb3408
- The full test suite passed 740 of 740 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 3453e6e8, PR 5 with main merged in (#244, #248); clippy --all-targets -D warnings and rustfmt --check were clean ^2026-11-13-the-full-test-suite-passed-740-of-740-across-48-targets-cargo
- The full test suite passed 740 of 740 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at c2ad25a5, PR 5's final head after records.spec.md's Public API row for names; clippy --all-targets -D warnings, rustfmt --check and specsync check --strict were clean ^2026-11-13-the-full-test-suite-passed-740-of-740-across-48-targets-cargo-1fa770
- The full test suite passed 742 of 742 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a079976d, PR 5 with main merged in (#242); clippy --all-targets -D warnings and rustfmt --check were clean ^2026-11-13-the-full-test-suite-passed-742-of-742-across-48-targets-cargo

## Bugs

- [A bug body runs its symptom and its Actual line into one paragraph](../../bugs/2026-11-12-a-bug-body-runs-its-symptom-and-its-actual-line-into-one.md): verified
- [Migrated records keep old observation IDs in their front matter](../../bugs/2026-11-12-migrated-records-keep-old-observation-ids-in-their-front.md): verified
- [A fresh project's records README names artifacts as kept from 0.5.x, and the README init paragraph is long](../../bugs/2026-11-12-a-fresh-project-s-records-readme-names-artifacts-as-kept-from.md): verified

## Open threads

## Timeline

- 2026-11-13T02:28:24Z · start · intent → intent · 0.6.1: record polish: a bug's Actual line stands apart, migrated front matter names the new entries, and a fresh project's records README and the README's init paragraph are trimmed ^t-514089
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:28:24Z · gate-opened · intent → intent · Approve the intent: record polish for bugs, migrated front matter and the READMEs ^t-1079b1
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: intent
  - drives: none
  - by: claude-code (hc t382)
- 2026-11-13T02:28:24Z · gate-closed · intent → intent · Approve the intent: record polish for bugs, migrated front matter and the READMEs ^t-c78a0a
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
  - by: claude-code (hc t382)
- 2026-11-13T02:28:24Z · advance · intent → design · Leif approved 0.6.1's PRs (02:03Z, relayed by t370) ^t-2e82a7
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:28:24Z · skip · design → plan · No visuals or interactions change ^t-6f7bf4
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:28:52Z · gate-opened · plan → plan · Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s ^t-f19d29
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T02:28:52Z · gate-closed · plan → plan · Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s ^t-0f2953
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:28:54Z · advance · plan → build · Leif approved the scope (02:03Z, relayed by t370) ^t-2dac3e
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - gate: native:approved
  - revision: 9d53604071df50b537adccad0ccf5cac9a2fc32b
  - by: claude-code (hc t382)
- 2026-11-13T02:53:56Z · direct-native-write · build → build · 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-4b06f6
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: dfaf7efcbf3ca72ef6bd5252c7c18b62df6cb9be
  - direct_write:
    - change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: claude-code (hc t382)
- 2026-11-13T02:53:56Z · back · build → plan · Leif chose to fix the file list and keep his approvals (02:53Z, relayed by t370) ^t-8a92a5
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: dfaf7efcbf3ca72ef6bd5252c7c18b62df6cb9be
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition
  - by: claude-code (hc t382)
- 2026-11-13T02:53:57Z · gate-opened · plan → plan · Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s with its corrected file list ^t-c0e776
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: dfaf7efcbf3ca72ef6bd5252c7c18b62df6cb9be
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T02:53:57Z · gate-closed · plan → plan · Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s with its corrected file list ^t-586f24
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: dfaf7efcbf3ca72ef6bd5252c7c18b62df6cb9be
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended) / Yes, fix and keep (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T02:53:59Z · advance · plan → build · Leif kept his approval with the corrected file list (02:53Z, relayed by t370) ^t-67fff5
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - gate: native:approved
  - revision: dfaf7efcbf3ca72ef6bd5252c7c18b62df6cb9be
  - by: claude-code (hc t382)
- 2026-11-13T03:13:17Z · direct-native-write · build → build · 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state approved → draft ^t-6859c8
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 115270871632389104e4590307e5d8065101c95d
  - direct_write:
    - change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: approved
    - after_state: draft
  - by: claude-code (hc t382)
- 2026-11-13T03:13:17Z · back · build → plan · The file list names exactly the changed files (Leif, 02:53Z, relayed by t370) ^t-571c21
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 115270871632389104e4590307e5d8065101c95d
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, checkpoint references: 4 case(s) and 8 observation(s) to re-check with `chough status`
  - by: claude-code (hc t382)
- 2026-11-13T03:13:18Z · gate-opened · plan → plan · Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s with its exact file list ^t-4915a2
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 115270871632389104e4590307e5d8065101c95d
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T03:13:18Z · gate-closed · plan → plan · Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s with its exact file list ^t-cf29e3
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 115270871632389104e4590307e5d8065101c95d
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended) / Yes, fix and keep (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T03:13:20Z · advance · plan → build · Leif kept his approval with the exact file list (02:53Z, relayed by t370) ^t-3433c8
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - gate: native:approved
  - revision: 115270871632389104e4590307e5d8065101c95d
  - by: claude-code (hc t382)
- 2026-11-13T03:13:20Z · advance · build → test · Leif kept his approval with the exact file list (02:53Z, relayed by t370) ^t-5118fb
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - gate: native:approved
  - revision: 115270871632389104e4590307e5d8065101c95d
  - by: claude-code (hc t382)
- 2026-11-13T03:13:20Z · advance · test → accept · Leif kept his approval with the exact file list (02:53Z, relayed by t370) ^t-f17d56
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - gate: native:approved
  - revision: 115270871632389104e4590307e5d8065101c95d
  - by: claude-code (hc t382)
- 2026-11-13T06:08:22Z · gate-opened · accept → accept · Implementation review of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s ^t-d65824
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
  - holds: accept
  - drives: review
  - by: claude-code (hc t382)
- 2026-11-13T06:10:08Z · gate-closed · accept → accept · Implementation review of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s ^t-289cda
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 2a5c63ea2a2e6c4bd14f52b6066749a8496e5a01
  - by: claude-code (hc t382)
- 2026-11-13T06:10:08Z · direct-native-write · accept → accept · 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-2faec3
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 2a5c63ea2a2e6c4bd14f52b6066749a8496e5a01
  - direct_write:
    - change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: claude-code (hc t382)
- 2026-11-13T06:10:08Z · back · accept → plan · The file list adds tests/evidence.rs under Leif's FILE-LIST RULE (04:35Z, relayed by t370); what the change does is unchanged ^t-c97de5
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 2a5c63ea2a2e6c4bd14f52b6066749a8496e5a01
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, accept: native verification and the human implementation review, checkpoint references: 4 case(s) and 22 observation(s) to re-check with `chough status`
  - by: claude-code (hc t382)
- 2026-11-13T06:10:10Z · gate-opened · plan → plan · Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s with tests/evidence.rs added ^t-12dc39
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 2a5c63ea2a2e6c4bd14f52b6066749a8496e5a01
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-13T06:10:10Z · gate-closed · plan → plan · Approve the scope of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s with tests/evidence.rs added ^t-2d85b8
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 2a5c63ea2a2e6c4bd14f52b6066749a8496e5a01
  - relayed_by:
    - decided_by: Leif
    - words: Yes, start (Recommended) / Yes, fix and keep (Recommended) / Yes, standing rule (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T02:03Z, answering: 'Start 0.6.1 as those 6 small PRs, one fix each, with you merging each?' (option text: 'The leak check and the 400 limit go first, then the rest.'); this PR's scope is the line shown to him: 'Record polish: a bug body's Actual line on its own line (F8); migrated front matter names the new entries, keeping old IDs as aliases (F9); a fresh project's .chough/README drops its 0.5.x note, and the README's init paragraph is cut (F11)'; relayed by claude-code-t370; and at 2026-11-13T02:53Z 'Yes, fix and keep (Recommended)', answering: 'Chough's paperwork listed too few files for some 0.6.1 PRs ... Fix the file lists with no code change ... Keep your existing approvals?' (option text: 'One yes covers #242's extra record (its scope and review) and the corrected lists. From now on, t382 checks the lists before every review.'); and under his FILE-LIST RULE at 2026-11-13T04:35Z, 'Yes, standing rule (Recommended)', answering: 'Standing rule? Sometimes an approved change turns out to touch a few more files than its list says, for example old tests that pin the printed text. The work itself is unchanged. May agents send it back and re-approve it under your original words, with a note listing the added files, instead of asking you each time?' (option text: 'Only when what the change does stays the same. Anything that changes its meaning still comes to you.'); added: tests/evidence.rs (a test that looked up a migrated observation by its 0.5.x ID now looks it up by the carried entry's name, which this change's F9 fix gives it); relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change approve 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T06:10:11Z · advance · plan → build · Leif's approval kept with tests/evidence.rs added (FILE-LIST RULE, 04:35Z, relayed by t370) ^t-303643
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - gate: native:approved
  - revision: 2a5c63ea2a2e6c4bd14f52b6066749a8496e5a01
  - by: claude-code (hc t382)
- 2026-11-13T06:10:11Z · advance · build → test · Leif's approval kept with tests/evidence.rs added (FILE-LIST RULE, 04:35Z, relayed by t370) ^t-e7c841
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - gate: native:approved
  - revision: 2a5c63ea2a2e6c4bd14f52b6066749a8496e5a01
  - by: claude-code (hc t382)
- 2026-11-13T06:10:11Z · advance · test → accept · Leif's approval kept with tests/evidence.rs added (FILE-LIST RULE, 04:35Z, relayed by t370) ^t-83ed9e
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - gate: native:approved
  - revision: 2a5c63ea2a2e6c4bd14f52b6066749a8496e5a01
  - by: claude-code (hc t382)
- 2026-11-13T06:10:31Z · gate-opened · accept → accept · Implementation review of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s ^t-dde431
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 5a29b6c6155cadf34aa90e4e42b30b937c43524f
  - holds: accept
  - drives: review
  - by: claude-code (hc t382)
- 2026-11-13T14:19:19Z · gate-closed · accept → accept · Implementation review of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s ^t-1a3a36
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - revision: 22a9b650eb0e6670a3342349880c1943798051a2
  - relayed_by:
    - decided_by: Leif
    - words: #256 record polish (0.6.1)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T14:14Z, answering: 'Chough PRs passed by the reviewer, part 1: approve the implementation? Each ships after your yes, then you'll get a merge it.', he selected '#256 record polish (0.6.1)', '#258 CI flake fix (#247)', '#259 migrate preview (#246)' and '#261 relay details (#249)'; relayed by claude-code-t370
    - recorded_by: claude-code-t382
    - native: specsync change review 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
    - actor: 0xLeif
  - by: claude-code (hc t382)
- 2026-11-13T14:19:21Z · advance · accept → done · native:archived ^t-0b2317
  - native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
  - gate: native:archived
  - revision: 22a9b650eb0e6670a3342349880c1943798051a2
  - by: claude-code (hc t382)

Closed 2026-11-13: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph
    revision: f4cfb57bd145d15ca613b2747d73c5372137538d
    environment: local-macos-arm64
    content: 7b5512beec4b9a2f80ee505be47bcd1cb09ac81b65b09d909d878177e022d9ed
    fingerprint: sha256:2d82781ce02d8c09b03583986568094edbe249c2ae156d8bfc6c13d333c4b3d6
    case: TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
    case_digest: 5ce9702c9e43233de2be292ee011650bc5c4eea3e9e59ccabb6ebfdcd4d9cff4
    observed: 2026-11-13T03:11:28Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries
    claim: "Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries
    revision: f4cfb57bd145d15ca613b2747d73c5372137538d
    environment: local-macos-arm64
    content: 7b5512beec4b9a2f80ee505be47bcd1cb09ac81b65b09d909d878177e022d9ed
    fingerprint: sha256:2895191ebadad78d063af842b9792e41063514c6f60b20330a94dd9a8472f71d
    case: TC-records-migrated-front-matter-names-the-carried-entries
    case_digest: 31d3e1b6c6c9b6299ba3da51213934d7bb063b3a4911db955f6741f6fe01930b
    observed: 2026-11-13T03:11:31Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories
    claim: "Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical
    revision: f4cfb57bd145d15ca613b2747d73c5372137538d
    environment: local-macos-arm64
    content: 7b5512beec4b9a2f80ee505be47bcd1cb09ac81b65b09d909d878177e022d9ed
    fingerprint: sha256:90c14ff5045120938ed975b48349568cf46a09895ebe3850bc80d7d08f4c2dba
    case: TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly
    case_digest: f5023a12e5493bcca87a2014014e9a2988bd9a4186712dae93134e87e305b229
    observed: 2026-11-13T03:11:33Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept
    claim: "Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5
    revision: f4cfb57bd145d15ca613b2747d73c5372137538d
    environment: local-macos-arm64
    content: 7b5512beec4b9a2f80ee505be47bcd1cb09ac81b65b09d909d878177e022d9ed
    fingerprint: sha256:d4c8855b0f99495ea639bf7867f9ee7dafe74b7fc27625638955efb27988571a
    case: TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5
    case_digest: fc6498e97a800fd70ade8c296a2b7daa5e7caa967d3b099e0f536fb7ef7c8f35
    observed: 2026-11-13T03:11:35Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-be05bb
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph
    revision: 6b7cd4766a52829847a23d03d0d93262d70feeee
    environment: local-macos-arm64
    content: 49a2ab3c28dbd1ec2012ee1e52591ba22d6e01e8918fc4976e1aaf668257c7a5
    fingerprint: sha256:7b4b4b55225e791aeaafbaad35705f52d0c78c738f160b0b1b93b43e4900bc4c
    case: TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
    case_digest: 5ce9702c9e43233de2be292ee011650bc5c4eea3e9e59ccabb6ebfdcd4d9cff4
    observed: 2026-11-13T03:12:20Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-752301
    claim: "Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries
    revision: 6b7cd4766a52829847a23d03d0d93262d70feeee
    environment: local-macos-arm64
    content: 49a2ab3c28dbd1ec2012ee1e52591ba22d6e01e8918fc4976e1aaf668257c7a5
    fingerprint: sha256:85d377312bc920e7a09beffac7edec3581ac69ff57d30074a8f8acf4a327e65d
    case: TC-records-migrated-front-matter-names-the-carried-entries
    case_digest: 31d3e1b6c6c9b6299ba3da51213934d7bb063b3a4911db955f6741f6fe01930b
    observed: 2026-11-13T03:12:24Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-ed78e8
    claim: "Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical
    revision: 6b7cd4766a52829847a23d03d0d93262d70feeee
    environment: local-macos-arm64
    content: 49a2ab3c28dbd1ec2012ee1e52591ba22d6e01e8918fc4976e1aaf668257c7a5
    fingerprint: sha256:095cd702b5ccee74915bdbc661176ccc9cae52529e8854f61ae225653a23b6d5
    case: TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly
    case_digest: f5023a12e5493bcca87a2014014e9a2988bd9a4186712dae93134e87e305b229
    observed: 2026-11-13T03:12:25Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-24ba1f
    claim: "Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5
    revision: 6b7cd4766a52829847a23d03d0d93262d70feeee
    environment: local-macos-arm64
    content: 49a2ab3c28dbd1ec2012ee1e52591ba22d6e01e8918fc4976e1aaf668257c7a5
    fingerprint: sha256:ad7dedf780b62e53cd3758fd03c0005d40fd5574337547620d9471082ced96c9
    case: TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5
    case_digest: fc6498e97a800fd70ade8c296a2b7daa5e7caa967d3b099e0f536fb7ef7c8f35
    observed: 2026-11-13T03:12:28Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-f7c2df
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph
    revision: f9b22f2215f9f61de538bbc17c45829385438cba
    environment: local-macos-arm64
    content: 84a021f93c1361a396a48300717a2b72419f9657749cffc84f701402a1bc2d9a
    fingerprint: sha256:b4f400ef7ad19209b51935cf759fd62c2bbf5d09b81b17cf5b5b4548b26e19f7
    case: TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
    case_digest: 5ce9702c9e43233de2be292ee011650bc5c4eea3e9e59ccabb6ebfdcd4d9cff4
    observed: 2026-11-13T03:30:18Z
    duration_seconds: 3
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-305164
    claim: "Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries
    revision: f9b22f2215f9f61de538bbc17c45829385438cba
    environment: local-macos-arm64
    content: 84a021f93c1361a396a48300717a2b72419f9657749cffc84f701402a1bc2d9a
    fingerprint: sha256:7d38e7a41900f22f49c5bda58018093f542e4604b53c1f9050b60d9d9bf979f0
    case: TC-records-migrated-front-matter-names-the-carried-entries
    case_digest: 31d3e1b6c6c9b6299ba3da51213934d7bb063b3a4911db955f6741f6fe01930b
    observed: 2026-11-13T03:30:21Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-fd85ba
    claim: "Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical
    revision: f9b22f2215f9f61de538bbc17c45829385438cba
    environment: local-macos-arm64
    content: 84a021f93c1361a396a48300717a2b72419f9657749cffc84f701402a1bc2d9a
    fingerprint: sha256:095cd702b5ccee74915bdbc661176ccc9cae52529e8854f61ae225653a23b6d5
    case: TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly
    case_digest: f5023a12e5493bcca87a2014014e9a2988bd9a4186712dae93134e87e305b229
    observed: 2026-11-13T03:30:24Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-2098aa
    claim: "Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5
    revision: f9b22f2215f9f61de538bbc17c45829385438cba
    environment: local-macos-arm64
    content: 84a021f93c1361a396a48300717a2b72419f9657749cffc84f701402a1bc2d9a
    fingerprint: sha256:681d50e5d978e708cf89dbb0a1b69bdff2f856fb999631bc9bfd74d9f1c87271
    case: TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5
    case_digest: fc6498e97a800fd70ade8c296a2b7daa5e7caa967d3b099e0f536fb7ef7c8f35
    observed: 2026-11-13T03:30:27Z
    duration_seconds: 0
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-9d1662
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph
    revision: a1f4735a820dacdb102a4ef4d9c1bda53e31f7bb
    environment: local-macos-arm64
    content: 65b5bda886fd3fbadb3834aeb5a77124da7d7b9a0d3f00f12ba7d6040b8ce43b
    fingerprint: sha256:eeccc88275c6346258883792d66382f979026c5f7e0827f147494273572eefcd
    case: TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
    case_digest: 5ce9702c9e43233de2be292ee011650bc5c4eea3e9e59ccabb6ebfdcd4d9cff4
    observed: 2026-11-13T04:25:39Z
    duration_seconds: 4
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-cf4667
    claim: "Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries
    revision: a1f4735a820dacdb102a4ef4d9c1bda53e31f7bb
    environment: local-macos-arm64
    content: 65b5bda886fd3fbadb3834aeb5a77124da7d7b9a0d3f00f12ba7d6040b8ce43b
    fingerprint: sha256:70bb3ddc40409c467b76f3fa436a6d34a30f94bb72dd3b6ac02cbb0966b285a3
    case: TC-records-migrated-front-matter-names-the-carried-entries
    case_digest: 31d3e1b6c6c9b6299ba3da51213934d7bb063b3a4911db955f6741f6fe01930b
    observed: 2026-11-13T04:25:46Z
    duration_seconds: 5
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-239237
    claim: "Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical
    revision: a1f4735a820dacdb102a4ef4d9c1bda53e31f7bb
    environment: local-macos-arm64
    content: 65b5bda886fd3fbadb3834aeb5a77124da7d7b9a0d3f00f12ba7d6040b8ce43b
    fingerprint: sha256:4e141c53866da2dfea243e1a0f17a17c09b4a5fe7c8615284543dc39c027540a
    case: TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly
    case_digest: f5023a12e5493bcca87a2014014e9a2988bd9a4186712dae93134e87e305b229
    observed: 2026-11-13T04:25:50Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-d27b46
    claim: "Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5
    revision: a1f4735a820dacdb102a4ef4d9c1bda53e31f7bb
    environment: local-macos-arm64
    content: 65b5bda886fd3fbadb3834aeb5a77124da7d7b9a0d3f00f12ba7d6040b8ce43b
    fingerprint: sha256:355e2c9b36a96e20ad140601484a72d6fe4d728c98ea69839e0e82f171df5ca0
    case: TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5
    case_digest: fc6498e97a800fd70ade8c296a2b7daa5e7caa967d3b099e0f536fb7ef7c8f35
    observed: 2026-11-13T04:25:53Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
    claim: "The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a1f4735a, PR 5 with #243 merged in; clippy --all-targets -D warnings and rustfmt --check were clean"
    result: passed
    revision: a1f4735a820dacdb102a4ef4d9c1bda53e31f7bb
    environment: local-macos-arm64
    content: 65b5bda886fd3fbadb3834aeb5a77124da7d7b9a0d3f00f12ba7d6040b8ce43b
    observed: 2026-11-13T04:26:54Z
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-7d5867
    claim: "Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph
    revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
    environment: local-macos-arm64
    content: 13e8fbe68ad3139b1c422a706d9abd7d321e4259afebf7c54f9ab96bea045d8e
    fingerprint: sha256:378393db8bc823e293f43ca9bd65b88c20b38bfa3308d8a5497d04906a9ebeff
    case: TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
    case_digest: 5ce9702c9e43233de2be292ee011650bc5c4eea3e9e59ccabb6ebfdcd4d9cff4
    observed: 2026-11-13T06:07:42Z
    duration_seconds: 2
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-5ac7fd
    claim: "Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries
    revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
    environment: local-macos-arm64
    content: 13e8fbe68ad3139b1c422a706d9abd7d321e4259afebf7c54f9ab96bea045d8e
    fingerprint: sha256:9c0424c05f5c17c52d120faa14243618564a75f8f90c15bf0a6dbbfd70397b09
    case: TC-records-migrated-front-matter-names-the-carried-entries
    case_digest: 31d3e1b6c6c9b6299ba3da51213934d7bb063b3a4911db955f6741f6fe01930b
    observed: 2026-11-13T06:07:46Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-91bb77
    claim: "Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical
    revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
    environment: local-macos-arm64
    content: 13e8fbe68ad3139b1c422a706d9abd7d321e4259afebf7c54f9ab96bea045d8e
    fingerprint: sha256:d272592c28a5f09895eb0e5a70919c38c199eaa2b49639bf8afd19559e740e89
    case: TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly
    case_digest: f5023a12e5493bcca87a2014014e9a2988bd9a4186712dae93134e87e305b229
    observed: 2026-11-13T06:07:50Z
    duration_seconds: 2
    actor: claude-code (hc t382)
  - id: 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-bb3408
    claim: "Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5
    revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
    environment: local-macos-arm64
    content: 13e8fbe68ad3139b1c422a706d9abd7d321e4259afebf7c54f9ab96bea045d8e
    fingerprint: sha256:37d378af0b471590d84bd983a81401f9bbc53fc18c267524ac6f203da4a0e40c
    case: TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5
    case_digest: fc6498e97a800fd70ade8c296a2b7daa5e7caa967d3b099e0f536fb7ef7c8f35
    observed: 2026-11-13T06:07:53Z
    duration_seconds: 1
    actor: claude-code (hc t382)
  - id: 2026-11-13-the-full-test-suite-passed-740-of-740-across-48-targets-cargo
    claim: "The full test suite passed 740 of 740 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 3453e6e8, PR 5 with main merged in (#244, #248); clippy --all-targets -D warnings and rustfmt --check were clean"
    result: passed
    revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
    environment: local-macos-arm64
    content: 13e8fbe68ad3139b1c422a706d9abd7d321e4259afebf7c54f9ab96bea045d8e
    observed: 2026-11-13T06:08:20Z
    actor: claude-code (hc t382)
  - id: 2026-11-13-the-full-test-suite-passed-740-of-740-across-48-targets-cargo-1fa770
    claim: The full test suite passed 740 of 740 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at c2ad25a5, PR 5's final head after records.spec.md's Public API row for names; clippy --all-targets -D warnings, rustfmt --check and specsync check --strict were clean
    result: passed
    revision: c2ad25a588c9a36dde78494efba1529a861e4f4b
    environment: local-macos-arm64
    content: 651559fb8d63202e04cdb50e0bb8949d59addf1fa29d67d5d0bb3570c7460033
    observed: 2026-11-13T07:30:28Z
    actor: claude-code (hc t382)
  - id: 2026-11-13-the-full-test-suite-passed-742-of-742-across-48-targets-cargo
    claim: The full test suite passed 742 of 742 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a079976d, PR 5 with main merged in (#242); clippy --all-targets -D warnings and rustfmt --check were clean
    result: passed
    revision: a079976d4314cd29337e2bcad4f1584f5ee74873
    environment: local-macos-arm64
    content: bb11ec6af4b0918640390f350b1855c36a784b98a1599cd828dc9e82777783fb
    observed: 2026-11-13T14:10:01Z
    actor: claude-code (hc t382)
```
