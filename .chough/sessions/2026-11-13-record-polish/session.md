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
blockers: []
waiting_on_human:
  - gate: Implementation review of 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
    holds: accept
    drives: review
    opened: 2026-11-13T06:10:31Z
run_consents: []
next: Continue accept
title: "0.6.1: record polish: a bug's Actual line, migrated names, a fresh guide"
summary:
  - A bug body gives its Actual line its own paragraph; migrated front matter names the carried entries, keeping each 0.5.x ID as their alias; a fresh project's reading guide names no 0.5.x folder, and the README's init paragraph names the host directories in one sentence.
  - "Verified: the cases red then green, three bugs verified, and the full suite 740 of 740. Next: Leif's review and merge."
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
- [Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-9d1662.md)
- [Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-cf4667.md)
- [Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-239237.md)
- [Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-d27b46.md)
- [The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a1f4735a, PR 5 with #243 merged in; clippy --all-targets -D warnings and rustfmt --check were clean](evidence/2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo.md)
- [Ran cargo test --locked --test bugs -- --exact a_bug_body_gives_its_actual_line_its_own_paragraph for TC-cli-a-bug-body-gives-its-actual-line-its-own-paragraph: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-a-bug-body-gives-its-actual-line-its-own-paragraph-7d5867.md)
- [Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries-5ac7fd.md)
- [Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories-91bb77.md)
- [Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1).](evidence/2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-bb3408.md)
- [The full test suite passed 740 of 740 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 3453e6e8, PR 5 with main merged in (#244, #248); clippy --all-targets -D warnings and rustfmt --check were clean](evidence/2026-11-13-the-full-test-suite-passed-740-of-740-across-48-targets-cargo.md)
- [The full test suite passed 740 of 740 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at c2ad25a5, PR 5's final head after records.spec.md's Public API row for names; clippy --all-targets -D warnings, rustfmt --check and specsync check --strict were clean](evidence/2026-11-13-the-full-test-suite-passed-740-of-740-across-48-targets-cargo-1fa770.md)
- [The full test suite passed 742 of 742 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a079976d, PR 5 with main merged in (#242); clippy --all-targets -D warnings and rustfmt --check were clean](evidence/2026-11-13-the-full-test-suite-passed-742-of-742-across-48-targets-cargo.md)

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
