---
kind: bug
id: 2026-11-06-readme-helper-table-has-no-doctor-row-after-168
aliases: [BUG-readme-helper-table-has-no-doctor-row-after-168-8c6b8da4b3fb7088779321b3]
title: "README helper table has no doctor row after #168"
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: []
requirements: []
cases: [TC-cli-readme-helper-table-covers-help]
evidence:
  - OBS-tc-cli-readme-helper-table-covers-help-d6d3f90f55066f1df7271880
  - OBS-tc-cli-readme-helper-table-covers-help-e3ced10f0b4c241f3494efeb
receipts:
  - id: 2026-11-06-tc-cli-readme-helper-table-covers-help-b2b64d
    aliases: OBS-tc-cli-readme-helper-table-covers-help-d6d3f90f55066f1df7271880
    claim: "Ran cargo test --locked --test docs -- --exact readme_helper_table_names_every_command_the_helper_lists for TC-cli-readme-helper-table-covers-help: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test docs -- --exact readme_helper_table_names_every_command_the_helper_lists
    revision: e017ebe5fee73e81e8aa092ba362f9cb1b893017
    environment: local-macos-arm64
    content: d754b38a741179395e027db9b8fe38dbf1f1496e0cdb57dea13418319447392c
    fingerprint: sha256:1d903daef10231a4e48642c4fda47ebcc81225e6c9b7375db7717fbcf6c53fc0
    case: TC-cli-readme-helper-table-covers-help
    case_digest: ceb5661bfa42aba6427816c6570519bcee4122e58423ab49691845f07f6726e0
    observed: 2026-11-06T20:23:26Z
    duration_seconds: 1
  - id: 2026-11-06-tc-cli-readme-helper-table-covers-help-c94fea
    aliases: OBS-tc-cli-readme-helper-table-covers-help-e3ced10f0b4c241f3494efeb
    claim: "Ran cargo test --locked --test docs -- --exact readme_helper_table_names_every_command_the_helper_lists for TC-cli-readme-helper-table-covers-help: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test docs -- --exact readme_helper_table_names_every_command_the_helper_lists
    revision: e017ebe5fee73e81e8aa092ba362f9cb1b893017
    environment: local-macos-arm64
    content: b7cfc908710c9ece287eaa7bd7cd98eec515e247cfb5f4bc9fc0abb1b2fe76b5
    fingerprint: sha256:a1bf00c50e47afae0f2fbfc597f354a22307a055e6569826b7c64e52221cae5a
    case: TC-cli-readme-helper-table-covers-help
    case_digest: ceb5661bfa42aba6427816c6570519bcee4122e58423ab49691845f07f6726e0
    observed: 2026-11-06T20:25:02Z
    duration_seconds: 3
environment: local-macos-arm64
revision: e017ebe5fee73e81e8aa092ba362f9cb1b893017
found_by: manual report
related: []
created: 2026-11-06T20:23:57Z
updated: 2026-11-06T20:25:02Z
---
# README helper table has no doctor row after #168

**What happened:** tests/docs.rs readme_helper_table_names_every_command_the_helper_lists fails on #164 merged with main: 'README.md What the helper does: no row names `doctor`'
Actual: No README helper row names doctor

**Steps:** Merging main at 16779048 (#168 adds the top-level chough doctor) into the FLOW-18 branch

**Expected:** README.md's helper table has a row naming every top-level chough command, including doctor

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-06T20:23:57Z · Reported (manual report); evidence: OBS-tc-cli-readme-helper-table-covers-help-d6d3f90f55066f1df7271880
- 2026-11-06T20:23:57Z · Added despite similar reports: A different command from a different PR: #168's new top-level doctor has no row, whereas the verified bug was #166's session close missing from the session row
- 2026-11-06T20:25:02Z · status: open -> verified; evidence: OBS-tc-cli-readme-helper-table-covers-help-e3ced10f0b4c241f3494efeb
