---
change: context-knowledge-directory-knowledge-by-default-with-the-librarian-on-and-an-opt-out-one-regenerated-summary-per
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-015, KNOW-1, KNOW-5, KNOW-6 | `tests/optional.rs::the_librarian_is_on_by_default_in_knowledge_and_can_be_turned_off` |
| REQ-cli-015, KNOW-2, KNOW-3, KNOW-4, KNOW-12 | `tests/evidence.rs::knowledge_entries_summarize_each_archive_deterministically` |
| REQ-cli-033, KNOW-7, KNOW-8, KNOW-11 | `tests/evidence.rs::archive_summary_previews_cold_archive_proof_and_keeps_protections` |
| REQ-cli-033, KNOW-7, KNOW-9, KNOW-10, KNOW-11 | `tests/evidence.rs::archive_summary_apply_is_authorized_ledgered_and_restores_on_validity_change` |
| REQ-cli-033, KNOW-7, KNOW-9 | `tests/evidence.rs::archive_summary_apply_fails_closed_when_a_file_changes_during_the_native_status_check` |
| REQ-cli-033, KNOW-7 | `tests/evidence.rs::archive_summary_lists_a_crlf_file_differing_from_head_as_uncommitted_under_autocrlf` |

All four fail at 0363d4a. The last two were added after review 5756934010 and fail at d5fabf5 (the apply removes bytes written during the validity check; the CRLF file is listed as eligible).

## Cases

- TC-cli-librarian-default-on-knowledge
- TC-cli-knowledge-archive-entries
- TC-cli-archive-summary-preview
- TC-cli-archive-summary-apply
- TC-cli-archive-summary-changed-during-check
- TC-cli-archive-summary-unfiltered-head-compare

## Focused and full checks

`cargo test`, the cases through `chough cases run`, and `fledge lanes run verify` with hi 0.8.0, fledge 1.8.0 and specsync 6.1.2.

## Dry run

`chough library archive` (preview only) on this repository. The result is reported in the PR and not applied.
