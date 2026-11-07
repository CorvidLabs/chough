---
change: return-the-observation-id-from-cases-record-and-evidence-add-issue-60
artifact: testing
---

# Testing

Add `TC-cli-manual-record-returns-observation-id` and
`TC-cli-evidence-add-returns-observation-id` in a change-specific cases file.
Both bind focused `cargo-test` regressions in `tests/cases.rs` and run on local
macOS arm64 and the hosted Linux and macOS environments.

The manual-record regression invokes `cases record` and asserts `case`, the new
`observation`, the pre-existing `recorded`, `outcome` and `provenance` fields.
The evidence regression invokes `evidence add` and asserts its new `observation`
beside the existing fields. In both, the returned observation must equal the
recorded path stem and the stored record's `id`; parsing an ID from a path alone
is not sufficient.

After scope approval, add the tests and definitions before changing either
response. Run both against the unfixed implementation, retain the failed output,
and link it to one deduplicated local bug. After implementation, rerun the same
cases and verify the bug only from a strictly later pass. Run the complete Rust
suite through Fledge, the full verify lane, `specsync change audit --strict`, and
pre-push. Record what actually ran on the final content; do not claim hosted CI,
human review, archive, merge or deployment before it is observed.

Existing suites cover the unchanged stored schemas, ID allocation, legacy reads,
exit codes and failure outcomes. No visual, localization, release or live-health
check applies.
