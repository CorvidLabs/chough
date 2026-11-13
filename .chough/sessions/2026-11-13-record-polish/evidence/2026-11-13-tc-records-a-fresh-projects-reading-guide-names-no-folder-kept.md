---
kind: evidence
id: 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept
aliases: []
claim: "Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5]
case_digest: [fc6498e97a800fd70ade8c296a2b7daa5e7caa967d3b099e0f536fb7ef7c8f35]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - records
  - "--"
  - "--exact"
  - a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: f4cfb57bd145d15ca613b2747d73c5372137538d
content: 7b5512beec4b9a2f80ee505be47bcd1cb09ac81b65b09d909d878177e022d9ed
environment: local-macos-arm64
fingerprint: sha256:d4c8855b0f99495ea639bf7867f9ee7dafe74b7fc27625638955efb27988571a
criteria: [DRIVE-24, KNOW-32]
requirements: [REQ-records-002]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:11:35Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision f4cfb57bd145, observed 2026-11-13T03:11:35Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 stdout ----
- thread 'a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5' (932808210) panicked at tests/records.rs:1333:5:
- # Chough records
- This folder holds Chough's records for this project. Each record is one Markdown file:
- people read its body, agents parse its front matter.
- - `sessions/<id>/session.md`: an open piece of work. Its front matter says where the
-   work stands; its Timeline lists every move, appended and never rewritten.
-   `evidence/` holds what was checked and `agent/` an agent's notes and inbox. The
-   folder leaves once the session's story is written and committed.
- - `archive/sessions/<id>.md`: the story of a finished session, kept for good, with its
-   key screenshots beside it in `archive/sessions/<id>/`.
- - `archive/shelved/<id>/`: a session set aside unread.
- - `archive/migrations/<id>.md`: the receipt of a migration from Chough 0.5.x.
- - `bugs/<id>.md`: one bug each, kept for good; its Log only grows.
- - `cases/*.cases.toml`: test-case definitions, which a person edits.
- - `preferences.json`: the project's standing preferences, which a person edits.
- - `artifacts/`: kept from Chough 0.5.x only for files an open bug names as its fixture.
- - `local/`: this machine's state (the active session, the writer lock, gate lists,
-   logs). It is never committed.
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.57s
