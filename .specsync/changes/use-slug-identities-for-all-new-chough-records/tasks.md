---
change: use-slug-identities-for-all-new-chough-records
artifact: tasks
---

# Tasks

- [x] Add the four slug-named regression definitions and their direct Rust tests before implementation: `TC-cli-slug-observation-allocation`, `TC-cli-slug-validation-allocation`, `TC-cli-slug-snapshot-allocation` and `TC-cli-slug-case-compatibility`. Reproduce independent-root and repeated-identical-run collisions or old-form allocation, retain failed direct-test observations, and link each failure to a deduplicated local bug before any `src/` edit. Do not invoke the old case validator as a substitute for these failures or record `not-run` merely because it rejects the new case IDs.
- [x] Add `src/identity.rs` and declare the existing `getrandom` 0.4.3 package as a direct dependency without changing its locked version. Centralize ASCII slug normalization, bounded new-form parsing, one 96-bit random draw and lowercase hexadecimal encoding, with no runtime, service, timestamp fallback, numeric fallback or digest retry.
- [x] Extend case validation in `src/cases.rs` for curated lowercase `TC-<module>-<title-slug>` IDs, module-aware use of the existing 100-byte bound and global duplicate refusal across all modules. Preserve exact legacy `TC-<module>-NNNN` IDs, existing definition-digest known answers, position-free digests, selection and retirement behavior. Keep the new regression definitions in slug form.
- [x] Allocate automatic observations as `OBS-<slug>-<24-lowercase-hex>` in `src/cases.rs` and `src/evidence.rs`. Make explicit observation IDs optional where automatic allocation applies, accept explicit new-form IDs, and refuse old-style IDs on new writes with an omit-ID remedy. Keep automated artifacts at exactly `.chough/artifacts/<observation-id>.log` and make observation and artifact create-new failures preserve existing bytes without fallback or retry.
- [x] Allocate validation runs as `VAL-<slug>-<24-lowercase-hex>` in `src/validate.rs`, preserving schema 1 run fields, counts, elapsed time, authority, pause, stop and resume. Legacy run IDs and their old observation references must still load, report and resume without migration.
- [x] Allocate librarian snapshots as `SNAP-<slug>-<24-lowercase-hex>` in `src/library.rs`, preserving manifest schema, source bytes, digests, timestamp-based chronology and deterministic indexes. Legacy dated snapshot directories remain readable without renaming or migration.
- [x] Complete direct and integration coverage for random-source failures, forced collisions, same-ID different-content refusal, atomic preservation, legacy reads, validation resume, artifact inheritance, native manifests, pruning and timing. Prove that transitions, preference keys, consent keys, stop markers and pruning-ledger entry shapes gain no IDs. Leave bug identity generation and every existing bug known-answer vector unchanged.
- [x] Apply the exact CLI, FRD, architecture, README, canonical skill and installed skill updates named in `docs.md`. Register pre-change LF digests for all four changed bundle files, keep canonical and installed copies byte-identical, and add known-outdated, update and edited-copy-preservation coverage.

Human gates: native scope approval, human implementation review, native finalization and archive, merge, and any optional release remain separate human decisions. No checked task, local observation, structural pass or agent label approves, archives, merges, releases or deploys this change.

## Required verification after native materialization

The checkmarks above record implemented code and test definitions. They do not claim that the steps below have passed. These remain mandatory before implementation acceptance; they are plain text because the scoped check itself materializes the requirements needed by the case validator.

1. After implementation, materialize only the approved semantic delta, including REQ-cli-023, through the scoped SpecSync flow so the new case links resolve. Run the unchanged direct regressions and all four slug cases to passed observations without explicit observation IDs. Link each strictly later pass to its original local bug and retain both failure and pass artifacts. Update `specs/cli/testing.md` and `specs/cli/tasks.md` with actual evidence rather than planned outcomes.

2. Pass focused and full Rust tests, real CLI exercises, formatting, warnings-denied Clippy, package and bundle checks, `fledge lanes run verify` and `fledge lanes run pre-push` using HI 0.8.0, Fledge 1.8.0 and SpecSync 6.1.2. Run the Rust 1.88.0 MSRV suite where that toolchain is available and leave hosted results unclaimed unless actually observed.

Takeover observations: all four slug cases passed with automatic IDs and their original failures and hashes were retained. The initial scoped suites passed 181 tests with one fixture excluded. After 0xLeif approved the separate tests/next.rs definition, the fixture-only patch was applied without assertion changes; the focused next suite passed all 3 tests and the full Rust 1.88.0 suite passed all 182 tests without exclusions. The required full Rust 1.99.1 verify, CLI exercise, pre-push and later case results are recorded in Chough observations and artifacts. Native evidence must be refreshed after the final record writes. No human acceptance, archive or merge is claimed.
