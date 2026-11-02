# Lesson bundle: use-slug-identities-for-all-new-chough-records

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Use slug identities for all new Chough records
- **Kind**: Feature
- **Specs**: cli
- **Paths**: Cargo.toml, Cargo.lock, src/identity.rs, src/main.rs, src/cases.rs, src/evidence.rs, src/validate.rs, src/library.rs, src/bugs.rs, src/skill.rs, tests/common/mod.rs, tests/cases.rs, tests/evidence.rs, tests/optional.rs, tests/bugs.rs, tests/cli.rs, tests/timing.rs, tests/package.rs, tests/stages.rs, specs/cli/requirements.md, specs/cli/testing.md, specs/cli/tasks.md, specs/cli/cli.spec.md, docs/FRD.md, docs/cli.md, docs/architecture.md, README.md, skills/chough-workflow/templates/cases.toml, skills/chough-workflow/references/testing.md, skills/chough-workflow/references/validate.md, skills/chough-workflow/references/librarian.md, .agents/skills/chough-workflow/templates/cases.toml, .agents/skills/chough-workflow/references/testing.md, .agents/skills/chough-workflow/references/validate.md, .agents/skills/chough-workflow/references/librarian.md, .claude/skills/chough-workflow/templates/cases.toml, .claude/skills/chough-workflow/references/testing.md, .claude/skills/chough-workflow/references/validate.md, .claude/skills/chough-workflow/references/librarian.md
- **Acceptance**: New case definitions accept curated `TC-<module>-<title-slug>` IDs with lowercase ASCII normalization, module-aware length bounds and global duplicate refusal. Exact legacy `TC-<module>-NNNN` cases remain valid with unchanged definition digests and no renumbering.
- **Acceptance**: Automatic observations use `OBS-<slug>-<24-lowercase-hex>`, validation runs use `VAL-<slug>-<24-lowercase-hex>`, and librarian snapshots use `SNAP-<slug>-<24-lowercase-hex>`. Random 96-bit suffixes allow identical repeated runs to remain distinct history; timestamps remain in their existing fields and determine chronology.
- **Acceptance**: Automated artifact names remain exactly `.chough/artifacts/<observation-id>.log`. Observation, validation, snapshot and artifact creation is atomic and refuses same-ID different-content creation without overwrite, timestamp or numeric fallback, or digest retry. Random-source and write failures preserve existing records.
- **Acceptance**: Explicit observation IDs are optional for automatic allocation. New writes supplied an old-style ID fail with an omit-ID remedy, while every legacy case, observation, validation and snapshot ID remains readable, linkable and resumable with no bulk migration or schema rewrite.
- **Acceptance**: Mixed legacy and new IDs work through bug links, checkpoint references, status and next, timing, retention and pruning, native manifests, validation resume and librarian indexes. Older Chough versions require an upgrade for slug case IDs. Transitions, preference keys, consent keys, stop markers and pruning-ledger entry shapes do not gain IDs.
- **Acceptance**: Failure-first independent-root and repeated-run evidence is retained and linked to local bugs before implementation, followed by later passes for the new generators and compatibility. Slug case definitions themselves use the new form. Focused and full Rust tests, real command exercises, formatting, Clippy with warnings denied, fledge verify and pre-push pass with CI-matching tools. Bug known-answer vectors remain unchanged.
- **Acceptance**: The existing getrandom package is used directly without a runtime or service. No change is made to bug identity generation, HI criteria, requirement IDs, SpecSync IDs, release or deployment.

## Evidence

- Verification commit: `caf97f4971a1307b96c58954e7a2767d885e4ebe`
- Base commit: `66bef80464cc8f6b7ba5e7e8023dfd6352f9d9ec`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## Problem

Chough still allocates several durable identities from local counters or second-resolution timestamps. `src/cases.rs` accepts only `TC-<module>-NNNN` and derives automatic observation names from a lowercase case ID, the current second and, after a collision, a numeric suffix. `src/validate.rs` uses the same timestamp and counter pattern for repeated runs. `src/library.rs` scans dated snapshot directories and selects the next numeric suffix. Independent roots can choose the same identity for different history, and identical work repeated within one second has no intrinsic distinction before local counter allocation.

The recently archived bug-identity change fixed the same independent-root problem for bugs with readable slugs and deterministic creation digests. It deliberately left cases, observations, validation runs, artifacts and snapshots unchanged. This change completes that identity work without reopening bug generation or introducing another store.

## Collision sources and consequences

Bare slugs collide whenever titles, case names or reasons repeat. Timestamps collide at their clock resolution and can move backward or differ across machines. Local numeric counters depend on state that independent roots and racing creators do not share. A deterministic hash of the complete observation payload also collides intentionally when repeated runs have identical inputs and timestamps, so it cannot preserve each repeated attempt as separate history. Random generation can still collide, so atomic no-replace creation remains necessary rather than optional.

A case ID is different: it is a curated durable definition identity, not an allocation for each execution. Numeric legacy cases must remain stable, while newly authored cases need readable slugs and exact global duplicate refusal. Case definition digests and execution observations remain separate.

## Constraints

All current record names pass through the existing 100-byte `valid_name` ceiling. New forms must fit that ceiling without truncating their random suffixes. Legacy case, observation, validation and snapshot names remain readable, linkable and resumable at their existing paths. There is no migration, renumbering, schema rewrite or edit to frozen archives.

Automatic observations, validation runs and snapshots need independent random 96-bit suffixes so identical repeated runs remain distinct. Their existing timestamp fields remain the chronology source. Automated logs keep the exact path `.chough/artifacts/<observation-id>.log` and inherit the observation ID rather than generating another identity.

The lockfile already contains getrandom 0.4.3. Chough will use that package directly, with no runtime, service, central allocator or persistent random state. Creation makes one candidate and one no-replace attempt. Existing content is never overwritten, and no timestamp, numeric, digest or random fallback follows a collision or write failure.

Explicit observation IDs remain available in the new form but become optional. A newly written old-style observation ID receives an omit-ID remedy; old IDs on stored records remain valid for reads and resume. Bug generation, bug known-answer vectors, transitions, preference and consent keys, stop markers and pruning-ledger entry shapes stay unchanged. HI, requirement and SpecSync identities remain with their native owners.

This native change is draft. This assignment covers only `context.md`, `design.md`, `plan.md` and `research.md`; no approval, implementation, canonical spec edit or Chough record write is part of it.

## From the change's design.md

# Design

## Shared normalization and grammars

Add one crate-internal identity module for slug normalization, new-form validation, random suffix encoding and candidate construction. Reuse the normalizer from the existing bug generator without changing its output, domain separator, canonical payload, known answers or update behavior.

Normalization iterates over input bytes, lowercases ASCII `A` through `Z`, retains ASCII `a` through `z` and `0` through `9`, replaces each maximal run of every other byte with one hyphen, and trims leading and trailing hyphens. There is no Unicode transliteration. The caller supplies a nonempty ASCII fallback for an empty result. Truncate the normalized slug to the caller's byte budget and remove trailing hyphens left by truncation. Output is ASCII, so byte truncation cannot split UTF-8. A valid slug is nonempty, contains only lowercase ASCII letters, digits and single interior hyphens, and neither starts nor ends with a hyphen.

All new forms remain at most 100 bytes, including every separator and all 24 suffix digits:

| Record | Exact new grammar | Slug byte budget | Total overhead |
|---|---|---:|---:|
| Case | `TC-<module>-<title-slug>` | `96 - module.len()` | 4 plus module length |
| Observation | `OBS-<slug>-<24-lowercase-hex>` | 71 | 29 |
| Validation run | `VAL-<slug>-<24-lowercase-hex>` | 71 | 29 |
| Snapshot | `SNAP-<slug>-<24-lowercase-hex>` | 70 | 30 |

For random-suffix forms, the suffix is exactly 24 characters from `[0-9a-f]`, obtained from 12 bytes. The slug is the part before the final separator and must satisfy the shared slug grammar and prefix-specific budget. These forms are valid under the existing path and record-name rules.

Case validation accepts either the exact legacy form `TC-<module>-NNNN`, where `N` is four ASCII digits, or the new curated form. `<module>` is the exact already validated case-file module stem, and the title slug uses the shared grammar and module-aware budget. Normalization helps an author derive a new ID from the title, but validation does not recompute an ID from the current title. Later title edits therefore do not rename a case. The existing loader-wide set refuses an exact duplicate across all case files, not merely within one file. Existing numeric IDs and their definition digests remain unchanged; every case definition added for this change uses a slug ID.

## Random allocation and slug sources

Use getrandom 0.4.3 directly as a declared dependency and call its fill operation once with a 12-byte array. Encode each byte as two lowercase hexadecimal digits without adding a formatting dependency. One allocation consumes one random result and constructs one candidate. Do not include a timestamp or content digest in the suffix and do not read the destination to decide whether to generate again.

Slug sources are stable and readable rather than uniqueness mechanisms:

- `cases run` and `cases record` normalize the full case ID for an observation.
- Direct `evidence add` without an explicit ID normalizes the first linked case ID when supplied, otherwise the observation kind.
- A validation run normalizes its case ID.
- A librarian snapshot normalizes its reason.

Fallbacks are `observation`, `validation` and `snapshot` for their respective record types, and `case` when initially deriving a case title slug. Random suffixes distinguish identical repeated runs even when slug, content and timestamp are equal. Existing `observed_at_unix_seconds`, `started_at_unix_seconds` and snapshot `created_at_unix_seconds` fields remain unchanged and determine chronology. ID sorting never replaces timestamp sorting.

Bug IDs remain deterministic for a different reason: a bug is one creation record keyed by its canonical initial content, so identical creation payloads intentionally produce the same candidate and refuse rather than manufacture another report. An observation, validation attempt or snapshot is history of an event, so identical repeated events still need separate identities.

## Explicit observations and legacy reads

`cases run --observation-id`, `cases record --observation-id` and the positional ID on `evidence add` accept an optional exact new-form observation ID. `evidence add` can omit its positional ID and allocate automatically, matching the case commands when their option is absent. An explicitly supplied new-form ID is used once as supplied; it is not regenerated.

When a new write supplies an otherwise legacy-valid observation ID that does not match the new form, refuse before creating an observation or automated artifact. The remedy says to omit the ID for automatic allocation, including removing `--observation-id` where applicable. Malformed path or record names retain the ordinary invalid-ID error. No validation or snapshot explicit-ID option is added.

Read and reference validation remains dual-form where a defined legacy grammar exists and permissive under the existing bounded `valid_name` rule where old observation and validation IDs were arbitrary readable names. Existing timestamp observations, numeric-suffixed validation runs and dated snapshot directories continue to load. `validate resume` resolves both old and new run IDs. This is write-time allocation policy, not a rewrite of stored records.

## Atomic no-replace creation

For observation JSON, validation JSON and automated log files, retain the existing `OpenOptions::create_new(true)` persistence path as the atomic allocation mechanism. Existence prechecks may improve diagnostics but are not the uniqueness mechanism. Map an already-exists result to a refusal, read the existing item within its applicable bound, and distinguish same-ID different-content from identical already-existing content. Both refuse and preserve existing bytes. A different-content refusal names the identity collision. There is no overwrite, replacement rename, timestamp suffix, numeric suffix, digest retry or second random candidate.

An automated run allocates its observation ID once and uses exactly `.chough/artifacts/<observation-id>.log`. The log and observation are separate no-replace writes; there is no claim of a multi-file transaction. A failure after one write preserves every pre-existing record and reports the incomplete attempt rather than recording success or retrying under another ID.

For snapshots, replace date-counter selection with `<library-dir>/snapshots/SNAP-<slug>-<24-lowercase-hex>`. Create the parent if needed, then use `create_dir`, not `create_dir_all`, on the final snapshot directory so an existing final component refuses atomically. Populate copied files only through create-new writes and write `manifest.json` last as the completion record. An existing directory is never removed or reused. If its stored manifest and referenced content can be compared and differ from the candidate, report a same-ID different-content collision; an unreadable or incomplete existing directory also remains preserved and refused. The manifest schema, entry shape, source bytes and source digests remain unchanged.

Random-source failures occur before any destination write. Create-new and later write failures never modify pre-existing records and never trigger another candidate. Snapshot population is not presented as a crash-proof multi-file transaction: manifest-last publication makes an interrupted new snapshot incomplete rather than a valid completed snapshot, and later readers must not describe an unreadable manifest as valid history.

## Compatibility and unchanged ownership

Mixed old and new IDs continue through observation loading, bug evidence links, checkpoint references, status and next, timing, retention and pruning, native manifests, validation status and resume, and librarian indexes. Retention continues to key observation sets by exact IDs and artifact paths. Pruning ledger entries gain no ID. Snapshot indexes discover legacy dated directories and new slug directories by their stored paths and timestamps.

No bulk migration or schema version change occurs. Observation schemas 1 and 2, validation schema 1, snapshot manifest schema 1 and bug schema 1 stay as they are. Bug known-answer vectors remain byte-for-byte unchanged. Transitions, preferences, consents, stop markers and pruning entries keep their shapes. No second lifecycle or checkpoint store, runtime, service or external coordination is introduced. Older Chough versions require an upgrade for slug case IDs and the other new forms; legacy records do not require a downgrade or migration.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
