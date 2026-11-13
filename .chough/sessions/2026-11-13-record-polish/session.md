---
kind: session
id: 2026-11-13-record-polish
aliases: [record-polish]
objective: "0.6.1: record polish: a bug's Actual line stands apart, migrated front matter names the new entries, and a fresh project's records README and the README's init paragraph are trimmed"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
pr: ""
criteria: []
requirements: []
cases:
  - TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph
  - TC-records-migrated-front-matter-names-the-carried-entries
  - TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly
  - TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5
design: []
files: []
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
bugs:
  - 2026-11-12-a-bug-body-runs-its-symptom-and-its-actual-line-into-one
  - 2026-11-12-migrated-records-keep-old-observation-ids-in-their-front
  - 2026-11-12-a-fresh-project-s-records-readme-names-artifacts-as-kept-from
blockers: []
waiting_on_human: []
run_consents: []
next: Continue accept
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

## What was verified

- [Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph.md)
- [Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries.md)
- [Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories.md)
- [Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: failed (exit 101, tests run 1).](evidence/2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-be05bb.md)
- [Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-752301.md)
- [Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-ed78e8.md)
- [Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-24ba1f.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-f7c2df.md)
- [Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-305164.md)
- [Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-fd85ba.md)
- [Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-2098aa.md)

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
