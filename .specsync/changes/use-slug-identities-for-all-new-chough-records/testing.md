---
change: use-slug-identities-for-all-new-chough-records
artifact: testing
---

# Testing

This is the verification plan, not a record that these checks have run. Planned IDs below are reserved for the failure-first evidence; later automatic IDs must be read from actual command output.

## Failure first and later passes

1. Before any `src/` edit, add these definitions to `.chough/cases/cli.cases.toml` and their exact `cargo-test` bindings. Every case is functional and automated, uses `test-log`, names its implementation and test paths, and lists `local-macos-arm64`, `ci-linux` and `ci-windows` only where its stand-ins are portable.

   | Case | Direct regression | Case links |
   |---|---|---|
   | `TC-cli-slug-observation-allocation` | `tests/cases.rs`: `independent_roots_and_identical_repeated_runs_allocate_distinct_observations` | FLOW-7, LIFE-2, REQ-cli-011, REQ-cli-017 and REQ-cli-023 |
   | `TC-cli-slug-validation-allocation` | `tests/optional.rs`: `independent_roots_and_identical_repeated_runs_allocate_distinct_validation_runs` | FLOW-9, LIFE-7, REQ-cli-011, REQ-cli-013 and REQ-cli-023 |
   | `TC-cli-slug-snapshot-allocation` | `tests/optional.rs`: `independent_roots_and_identical_repeated_runs_allocate_distinct_snapshots` | FLOW-12, REQ-cli-015 and REQ-cli-023 |
   | `TC-cli-slug-case-compatibility` | `tests/cases.rs`: `slug_case_ids_validate_without_changing_legacy_ids_digests_or_global_uniqueness` | FLOW-7, LIFE-2 and REQ-cli-009 |

2. Run each exact regression directly with `cargo test --locked --test <target> -- --exact <test>` at the unfixed tip. The observation, validation and snapshot tests must each run the same operation repeatedly in one root and in two independent roots with identical initial records, then assert distinct new-form IDs. The case test must assert lowercase slug syntax and bounds, legacy acceptance and digest stability, and global duplicate refusal. The expected pre-fix result is a failed assertion that shows old-form allocation or slug rejection, not a compile error and not a vacuous filter.
3. Save each direct command, exit status, assertion output and generated IDs in the matching artifact below. Record a failed schema 2 product-test observation with `evidence add`, the already-materialized requirement and criterion links, `local-macos-arm64`, and no `--case` link because the old validator cannot yet load the slug definition. Do not use unmaterialized REQ-cli-023 in these pre-fix observations. Never substitute `not-run` for that validator limitation.

   | Case | Failed observation and artifact basename |
   |---|---|
   | `TC-cli-slug-observation-allocation` | `OBS-slug-observation-allocation-6f0a2f94c38b11d27e45a901` |
   | `TC-cli-slug-validation-allocation` | `OBS-slug-validation-allocation-18b3c7409ad62ef510b47c92` |
   | `TC-cli-slug-snapshot-allocation` | `OBS-slug-snapshot-allocation-b731a60c294f8de0175c34a2` |
   | `TC-cli-slug-case-compatibility` | `OBS-slug-case-compatibility-47d2a9106cb83ef519a0c674` |

   Each artifact path is exactly `.chough/artifacts/<observation-id>.log`. These are explicit new-form observation IDs for retained reproductions, not expected outputs from the automatic generator.
4. Run `chough bug search` for each symptom and affected IDs. Reuse an existing record only for the same defect; otherwise add a local product-defect bug with the slug case link and failed observation before implementation. Preserve the failed records and logs after later edits. Do not edit bug identity generation or allocate a bug ID by hand.
5. After implementation and scope approval, run the scoped SpecSync check to materialize REQ-cli-023 and the other approved requirement changes before validating the new case links. Rerun the same direct tests and `cargo run --locked -- cases check`. Then run each of the four cases with `cargo run --locked -- cases run <slug-case-id> --environment local-macos-arm64` and no `--observation-id`. Each returned observation must have the new OBS form and an inherited artifact path. Link the actual passed observation to the same bug and mark it verified only when that pass is observed strictly after its latest linked failure. Do not rewrite a failure, reuse its observation ID, or infer a pass from `cases check`.

## Allocation and compatibility evidence

| Scope | Required checks |
|---|---|
| Curated cases, REQ-cli-009 | Accept curated lowercase ASCII alphanumeric runs separated by single hyphens. Refuse empty slugs, uppercase or mixed-case input, repeated separators, leading or trailing separators, punctuation, non-ASCII input, traversal-like text and an over-limit ID. Check the exact module-aware use of the existing 100-byte bound immediately below, at and above the limit. Refuse duplicate IDs in one file and across different module files, including an exact legacy ID duplicated by a new definition. Exact `TC-<module>-NNNN` definitions retain their IDs, file positions, serialized fields and known digests with no renumbering. Reordering and title edits remain digest-neutral; assertion and binding edits remain digest-changing. |
| Common identity slugs, REQ-cli-023 | Assert OBS and VAL slugs come from the normalized case ID and SNAP slugs come from the snapshot reason. Cover repeated punctuation, non-ASCII characters, a source with no ASCII alphanumeric character and its nonempty fallback, and long sources at the 100-byte complete-ID boundary. Truncation must preserve the final hyphen and all 24 suffix digits. Invalid explicit new forms, uppercase hex, short or long suffixes and empty slugs are refused without weakening legacy reads. |
| Observation allocation, REQ-cli-011, REQ-cli-017 and REQ-cli-023 | In two independent roots with identical cases and starting records, and for identical repeated runs in one root, assert every automatic ID matches `OBS-<slug>-<24-lowercase-hex>`, every suffix encodes a distinct 96-bit random value, and no ID contains a timestamp or numeric fallback. Assert all IDs are distinct without treating that probabilistic property as a deterministic randomness proof. Timestamps and `duration_seconds` keep their existing fields and meanings. Each automated record has exactly `.chough/artifacts/<observation-id>.log`, and that log contains the bound command's captured bytes. |
| Explicit observation IDs, REQ-cli-011 and REQ-cli-023 | Omit the ID for automatic case runs, manual case records and every other path whose CLI gains automatic allocation. Supply a valid explicit new-form ID and assert it is used once. Supply representative legacy timestamp, numeric-suffix and free-form old IDs to every new-write path and assert refusal with an omit-ID remedy, no artifact or observation write, and no fallback. Existing records with those same old IDs remain valid on reads. |
| Validation allocation and resume, REQ-cli-011, REQ-cli-013 and REQ-cli-023 | Independent roots and identical repeated runs receive distinct `VAL-<slug>-<24-lowercase-hex>` IDs. Every attempt still allocates its own OBS ID and artifact. A stopped or paused new run resumes under its original VAL ID with stored times, interval, environment, authority, counts and elapsed time. A legacy timestamp-and-numeric run also loads, appears in `validate status` and `timing`, and resumes without changing its ID or schema; its later attempts use new OBS IDs. Stop-marker bytes, consent fields and preference keys remain unchanged. |
| Snapshot allocation and indexes, REQ-cli-015 and REQ-cli-023 | Independent roots and repeated snapshots of identical source bytes and reason receive distinct `SNAP-<slug>-<24-lowercase-hex>` directories. Source bytes and SHA-256 values stay equal, manifests retain schema 1 and `created_at_unix_seconds`, and chronology uses that timestamp rather than parsing the ID. Regenerated indexes list new and legacy `<date>-<n>` snapshots together without renaming either form. Escaping, symlinked and oversized sources keep their existing refusals. |
| Legacy observations and references, REQ-cli-017, REQ-cli-018 and REQ-cli-023 | Load schema 1 and schema 2 observations with legacy free-form IDs alongside new OBS IDs. Resolve both forms through checkpoint references, bug observations and notes, status and next, validation references, active and latest-archive native manifests, and librarian indexes. No record is migrated or rewritten merely by reading it. Older Chough versions are documented as requiring an upgrade for new-form IDs. |
| Retention and pruning, REQ-cli-018 | Preview and apply fixtures mixing legacy and new observations and artifacts. Assert the same newest, last-failure, validation-boundary, checkpoint, bug and native-manifest protection reasons, exact artifact inheritance and byte accounting. A pruned new observation is labeled pruned, a missing unlisted one unavailable, and neither counts as current or passed. Compare pruning-ledger keys and values with the existing schema; the ledger gains no snapshot, validation, artifact or other ID field. |
| Timing, REQ-cli-020 | Derive test groups, run times and validation elapsed time from mixed legacy and new IDs with fixed timestamps. Ordering and durations come from timestamps and stored duration fields, never lexical IDs or random suffixes. Resume accumulates total run wall-clock as before. The timing command remains read-only, with every Chough and SpecSync file byte-identical afterward. |

## Random failures and atomic refusal

- Put the random fill behind a narrow internal test seam, not a runtime configuration or service. Unit tests in `src/identity.rs` independently check that exactly 12 random bytes become exactly 24 lowercase hexadecimal characters, including all-zero, all-`ff` and mixed-byte vectors. A failing source returns its error, performs no create attempt, leaves a tree digest of existing records unchanged and is not retried.
- Force the same 96-bit suffix for different-content observation, validation and snapshot creations. Each path makes one candidate and one atomic create-new attempt, reports the colliding ID, preserves the original bytes and creates no timestamp, numeric or digest alternative. Instrument the test seam to prove there was no second random draw. Equivalent artifact collision coverage pre-creates `.chough/artifacts/<observation-id>.log` with different bytes and asserts byte preservation and no replacement record.
- Exercise create-new races for each record kind and the artifact path. At most one creator wins, a loser cannot overwrite the winner, and no fallback ID appears. Inject write failures after allocation and, where applicable, between artifact and observation creation. Existing records and artifacts stay byte-identical, no partial new record is accepted, and temporary files are cleaned up. Keep existing bounds, symlink refusal and canonical path checks intact.
- Keep the bug creation payload, `chough/bug-id/v1` domain, SHA-256 truncation, slug limit and update immutability unchanged. Run every existing known-answer and independent-root bug test without changing its expected ID. Any shared create-new helper must also preserve the bug collision diagnostic and behavior.

## Focused, full and real command checks

Run the four direct regressions first, then the focused suites:

```sh
cargo test --locked --test cases
cargo test --locked --test optional
cargo test --locked --test evidence
cargo test --locked --test timing
cargo test --locked --test bugs
cargo test --locked --test cli
cargo test --locked --test package
cargo run --locked -- cases check
```

After the retained pre-fix failures, run the four slug cases and link their later passes as described above. Exercise real `evidence add`, `cases run`, `cases record`, `validate run`, `validate stop`, `validate resume`, `library snapshot`, `library index`, `evidence prune`, `status`, `next` and `timing` commands in isolated Git fixtures, inspecting actual IDs, paths and preserved bytes rather than only parsing library output.

Run `cargo test --locked` and `cargo build --locked --release`. Where installed, also run `cargo +1.88.0 test --locked`; otherwise leave the MSRV result to actual hosted CI. With HI 0.8.0, Fledge 1.8.0 and SpecSync 6.1.2, run:

```sh
fledge run fmt
fledge run lint
fledge run test
fledge lanes run verify
fledge lanes run pre-push
```

`lint` is Clippy over all targets with warnings denied. Verify includes the nine configured format, lint, test, HI, strict SpecSync, native audit, case-definition, real feature-and-bug exercise and build steps. Pre-push includes its five configured steps. Package tests must retain the four pre-change bundle files, independently verify their registered LF digests, report known-outdated, update unedited copies to current, preserve edited copies, and reject em and en dashes. Report hosted Ubuntu, macOS, Windows and MSRV results only from actual hosted runs.

## Evidence boundaries

The failed observations may become historical after implementation and remain valid reproduction evidence. Later passes must be separate observations with distinct IDs. Structural checks, random-looking output, a passing case, a local bug status and an actor label are not human acceptance, native finalization, merge, release or deployment. No migration, hosted service, model client, custom runtime or change to HI, requirement, SpecSync or bug identity is part of this verification.
