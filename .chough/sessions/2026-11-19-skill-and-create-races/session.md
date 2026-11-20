---
kind: session
id: 2026-11-19-skill-and-create-races
aliases: [skill-and-create-races]
objective: "Skill and create-new batch: skill update writes CRLF copies and names a file it cannot write (#373), parallel skill update and init runs all succeed (#386), a host folder that cannot be inspected is skipped and reported while the rest update (#403), and doctor reads an uncommitted session that fails validation (#408)"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: skill-and-create-races
pr: ""
criteria: [LIFE-4, LIFE-12, DRIVE-17]
requirements: [REQ-cli-016, REQ-cli-001]
cases: []
design: []
files: []
observations:
  - 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state
  - 2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated
  - 2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly
  - 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected
  - 2026-11-20-tc-cli-parallel-skill-updates-each-succeed
  - 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly
  - 2026-11-20-tc-history-doctor-reads-uncommitted-records
  - 2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it
  - 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Continue build
---
# Skill and create-new batch: skill update writes CRLF copies and names a file it cannot write (#373), parallel skill update and init runs all succeed (#386), a host folder that cannot be inspected is skipped and reported while the rest update (#403), and doctor reads an uncommitted session that fails validation (#408)

## Goal

Skill and create-new batch: skill update writes CRLF copies and names a file it cannot write (#373), parallel skill update and init runs all succeed (#386), a host folder that cannot be inspected is skipped and reported while the rest update (#403), and doctor reads an uncommitted session that fails validation (#408)

## Where it stands

## Decisions and why

- 0xLeif decided "Approve the scope of skill-and-create-races": "Approve" (Claude Code chat answer, relayed by his planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-19 about 14:12Z, on D2 skill-and-create-races at 2bf6318 with the four recommended choices; recorded at the batch start on that text, the draft carried onto main 056cbb80 by a merge with no delta word changed); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve skill-and-create-races as 0xLeif

## What was verified

- [Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state.md)
- [Ran cargo test --locked --test cli -- --exact a_symlinked_host_is_skipped_and_the_rest_updated for TC-cli-a-symlinked-host-is-skipped-and-the-rest-updated: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-symlinked-host-is-skipped-and-the-rest-updated.md)
- [Ran cargo test --locked --test cli -- --exact a_skill_file_it_cannot_write_is_refused_plainly for TC-cli-a-skill-file-it-cannot-write-is-refused-plainly: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-a-skill-file-it-cannot-write-is-refused-plainly.md)
- [Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected.md)
- [Ran cargo test --locked --test parallel -- --exact parallel_skill_updates_each_succeed for TC-cli-parallel-skill-updates-each-succeed: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-parallel-skill-updates-each-succeed.md)
- [Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly.md)
- [Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-history-doctor-reads-uncommitted-records.md)
- [Ran cargo test --locked --test doctor -- --exact doctor_names_a_special_file_without_opening_it for TC-history-doctor-names-a-special-file-without-opening-it: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-history-doctor-names-a-special-file-without-opening-it.md)
- [Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1).](evidence/2026-11-20-tc-history-doctor-skips-ignored-and-reports-once.md)

## Open threads

## Timeline

- 2026-11-19T07:43:53Z · start · intent → intent · Skill and create-new batch: skill update writes CRLF copies and names a file it cannot write (#373), parallel skill update and init runs all succeed (#386), a host folder that cannot be inspected is skipped and reported while the rest update (#403), and doctor reads an uncommitted session that fails validation (#408) ^t-2aa121
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T07:43:53Z · advance · intent → design ·  ^t-3ffc10
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-19T07:43:53Z · advance · design → plan ·  ^t-d15454
  - revision: 9d260310bca8553a0203e5ef90bc8372b4e15aae
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T09:09:15Z · gate-opened · plan → plan · Approve the scope of skill-and-create-races ^t-7d30e5
  - native_change: skill-and-create-races
  - revision: ff3c80cae8022b05bb8ac41b13e53d4feab91a2c
  - holds: plan
  - drives: approve
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T09:09:15Z · gate-closed · plan → plan · Approve the scope of skill-and-create-races ^t-afa4bb
  - native_change: skill-and-create-races
  - revision: ff3c80cae8022b05bb8ac41b13e53d4feab91a2c
  - relayed_by:
    - decided_by: 0xLeif
    - words: Approve
    - where: Claude Code chat answer, relayed by his planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-19 about 14:12Z, on D2 skill-and-create-races at 2bf6318 with the four recommended choices; recorded at the batch start on that text, the draft carried onto main 056cbb80 by a merge with no delta word changed
    - recorded_by: chough-fixer-t4g
    - native: specsync change approve skill-and-create-races
    - actor: 0xLeif
  - by: chough-fixer (claude-opus-5-5, hc t4n)
- 2026-11-20T09:10:10Z · advance · plan → build · native:approved ^t-bca690
  - native_change: skill-and-create-races
  - gate: native:approved
  - revision: f37fc73fb8f76e7c872a50ce6cdda4f69b3a1aa8
  - by: chough-fixer (claude-opus-5-5, hc t4n)
