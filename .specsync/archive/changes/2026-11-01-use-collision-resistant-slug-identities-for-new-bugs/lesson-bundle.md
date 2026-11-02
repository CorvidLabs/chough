# Lesson bundle: use-collision-resistant-slug-identities-for-new-bugs

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Use collision-resistant slug identities for new bugs
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/bugs.rs, tests/bugs.rs, tests/optional.rs, specs/cli/requirements.md, specs/cli/testing.md, specs/cli/tasks.md, specs/cli/cli.spec.md, docs/FRD.md, docs/cli.md
- **Acceptance**: Manual and automatic creation of distinct bug reports in independent roots at the same legacy numeric high-water mark produces different IDs in the form BUG-<lowercase-title-slug>-<24-lowercase-hex>; the pre-fix identical-ID failure is recorded and linked before implementation and the later pass verifies the fix.
- **Acceptance**: Title normalization produces bounded lowercase ASCII slugs and preserves the complete 24-hex digest within the existing 100-byte ID limit. IDs are generated once from a versioned domain-separated SHA-256 creation payload and are never recomputed during updates.
- **Acceptance**: Every legacy BUG-NNNN record remains readable and usable without bulk migration or renumbering. Bug schema 1 is unchanged; duplicate and related links work across legacy and new forms, and the docs state that older Chough versions need an upgrade for new IDs.
- **Acceptance**: Atomic create-new refuses a same-ID different-content result without overwriting or falling back to numeric allocation. Evidence, artifact, validation, snapshot, transition, case, HI, requirement and SpecSync ID schemes are unchanged.
- **Acceptance**: TC-cli-0125 covers the collision and compatibility behavior if still free on the base. Focused Rust tests, affected cases, cargo fmt, Clippy with warnings denied, full cargo test, fledge verify and pre-push pass with CI-matching tools. No new integration, runtime, service, release or deployment is introduced.

## Evidence

- Verification commit: `4fd09eb974e0b4df70fb75c6f47b5fccab0367f2`
- Base commit: `7357d41ab92e737fea1ad1e733c38f5447226fb9`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## Observed collision

The current allocator in `src/bugs.rs` scans one root's legacy numeric IDs and chooses the maximum plus one. Independent roots with the same numeric high-water mark therefore allocate the same next ID for different reports.

This happened during the Kimi integration. The Kimi line of work recorded the stale archived-resume defect as `BUG-0020` after `BUG-0019`, while main independently recorded the older-workflow approval-matching defect as `BUG-0020` after the same high-water mark. Merge commit `b2e2f19d904aec2a5acd73569c8439f5583c01f9` had a real `.chough/bugs/BUG-0020.json` conflict. Its resolution kept main's record as `BUG-0020` and moved the Kimi record to `BUG-0024`. The archived change `align-kimi-integration-with-new-cycle-done-guidance` records that history. Manual renaming preserved both reports, but it does not make the original identities collision-resistant.

## Constraints and rejected alternatives

Max-plus-one depends on local state that independent roots and racing creators do not share. Same-second timestamps are not a safe substitute: independent creations can occur within one clock second, clocks can be skewed or move backward, and equal titles remain equal. A timestamp may contribute creation data to a digest, but it cannot be the uniqueness mechanism.

Only IDs for newly created bugs change. Existing `BUG-NNNN` records, filenames, evidence links and history must remain readable without migration or renumbering, and bug schema 1 stays unchanged. Rewriting old identities would break durable references and frozen history without fixing future allocation.

Manual `bug add` and automatic failure capture must use one generator. Creation must remain atomic and fail closed on a same-ID different-content result, with no overwrite, numeric fallback or repeated ID generation. `TC-cli-0125` is reserved for a pre-fix independent-root collision reproduction and a later passing verification.

Evidence, artifact, validation, snapshot, transition, case, HI, requirement and SpecSync identities are outside this change. No new dependency, integration, runtime, service, release or deployment is needed. The overall direction has human support, but this native definition remains draft until its own scope approval.

## From the change's design.md

# Design

## ID form and title normalization

Every newly created bug receives `BUG-<slug>-<digest>`, where `<digest>` is exactly 24 lowercase hexadecimal characters. The slug is derived once from the creation title:

1. Lowercase ASCII `A` through `Z`, retain ASCII letters and digits, replace each maximal run of every other character with one `-`, and trim leading and trailing `-`. There is no Unicode transliteration.
2. If nothing remains, use `bug`.
3. Limit the slug to 71 bytes and trim any trailing `-` left by truncation.

The 71-byte cap reserves four bytes for `BUG-`, one for the separator and 24 for the complete digest, so the whole ASCII ID is at most the existing 100-byte `valid_name` limit. Truncation never shortens or alters the digest. Validation accepts both this new shape and the exact legacy `BUG-NNNN` shape.

## Canonical creation payload

A shared generator builds the ID after the initial schema 1 record values are known and before the first save. Hash the ASCII bytes `chough/bug-id/v1`, one zero byte, then a dedicated canonical JSON creation payload. The domain separator and payload layout are version 1. Any future incompatible layout requires a new domain version rather than silently changing version 1.

The payload fields, in serialization order, are `schema_version`, `title`, `symptom`, `trigger`, `expected`, `actual`, `criteria`, `requirements`, `cases`, `environment`, `revision`, `steps`, `fixture`, `observations`, `impact`, `classification`, `scope`, the initial `status`, `regression_baseline`, `duplicate_of`, `related`, `source`, the initial `notes`, and `created_at_unix_seconds`. The payload excludes `id` and `updated_at_unix_seconds`. Each initial note retains its `at_unix_seconds`, `text` and `observations` fields in that order.

Canonical JSON is compact UTF-8 with the field order above, standard JSON string escaping, explicit `null` for absent optional values, creation order for arrays, and no unordered maps or insignificant whitespace. A dedicated serializable payload type, rather than serializing a map or the final record, fixes this contract. Hash the domain and payload with the existing `sha2` 0.10 dependency, take the first 12 digest bytes, and encode them as 24 lowercase hexadecimal characters without adding a hex dependency.

The timestamp is only one creation field; the complete payload, not the timestamp or legacy high-water mark, supplies digest variation. Manual addition and automatic capture call this same generator whenever they create a new record. Automatic capture that appends to an existing open bug does not create another ID.

The ID is generated once and stored in both the record and filename. Updates, including changes to fields represented in the original creation payload, never recompute or rename it. No canonical payload or new schema field is persisted.

## Atomic creation and collisions

Creation makes one candidate ID and one atomic create-new attempt using the existing `create_new(true)` persistence path. An existing candidate path fails closed. Same-ID different-content produces an explicit collision error and preserves the existing bytes; identical existing content follows the normal already-exists error and is also not overwritten. There is no numeric fallback, digest retry or second candidate. This handles both a digest collision and concurrent creation without silently replacing a report.

## Compatibility and limits

Loading, listing, search, show, report, status, updates and evidence references treat both accepted ID forms as bug IDs. `duplicate_of` and `related` resolve either form, including links in both directions between legacy and new records. Existing records remain at their current paths with schema 1 and their original IDs. There is no bulk migration, renumbering or rewrite of old observations and archives.

Documentation must state that older Chough versions that accept only `BUG-NNNN` need an upgrade before reading or referencing new IDs. No evidence, artifact, validation, snapshot, transition, case, HI, requirement or SpecSync ID scheme changes.

## From the change's testing.md

# Testing

This is the verification plan, not a record that these checks have run.

## Failure first and later pass

1. Before any `src/bugs.rs` edit, add TC-cli-0125 to `.chough/cases/cli.cases.toml` and add `independent_roots_at_the_same_high_water_mark_get_distinct_ids_without_breaking_legacy_links` to `tests/bugs.rs`. The case links REQ-cli-012, FLOW-8 and LIFE-5, uses the existing `test-log` artifact type, and binds exactly that test through `cargo-test`. Its path list names `src/bugs.rs`, `tests/bugs.rs` and `tests/optional.rs`; its environments are `local-macos-arm64`, `ci-linux` and `ci-windows` with portable stand-ins.
2. Seed each independent root with the same legacy numeric high-water mark. In each root, create a distinct manual report and a distinct automatic validation-capture report. At the unfixed tip, the corresponding manual IDs must collide and the corresponding automatic IDs must collide. Run the exact test directly, then record it with `cargo run --locked -- cases run TC-cli-0125 --environment local-macos-arm64 --observation-id tc-cli-0125-collision-pre-fix`. The expected observation is `failed`, never `not-run`, and its log must show the actual identical-ID assertion failure.
3. Record and link the local collision bug to that failed observation before source implementation. Preserve the observation and artifact after later edits; their becoming historical is expected and does not erase the reproduction.
4. After the fix and affected-file edits, rerun the same unchanged case definition with observation ID `tc-cli-0125-collision-post-fix`. Link the pass to the same bug and verify it only when the pass is observed strictly after the latest linked failure. Do not rewrite the pre-fix observation or claim a pass from a structural check.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-012, TC-cli-0125 | `tests/bugs.rs`: independent roots at the same seeded BUG-NNNN high-water mark produce different IDs for distinct manual reports and different IDs for distinct automatic captures. Every new ID matches `BUG-<lowercase-title-slug>-<24-lowercase-hex>`, is at most 100 bytes, and does not consume the next numeric ID. The same test reads, lists, searches, shows, reports and updates the seeded legacy bug, keeps its schema 1 ID and filename, and creates duplicate and related links in both directions between legacy and new forms. Before the fix this test fails on the identical manual and automatic IDs; after the fix it covers the compatibility behavior in the same case. |
| REQ-cli-012 | Slug unit tests cover uppercase, repeated separators, leading and trailing separators, non-ASCII text, punctuation-only text and the `bug` fallback. A 71-byte slug produces a 100-byte ID; a longer slug is cut without cutting a character or any part of the separator and 24-hex suffix. Short, empty-after-normalization and very long titles all produce valid IDs. |
| REQ-cli-012 | A known-answer test independently computes SHA-256 from the design's versioned domain and canonical creation payload, including its exact field order and JSON string escaping, then checks the first 12 digest bytes as 24 lowercase hex characters. The expected value is a fixed vector, not output from the production generator. Adjacent-field values that would be ambiguous without framing produce different payloads, equal canonical payloads reproduce the ID, and changing creation content changes the digest. |
| REQ-cli-012 | The same-ID different-content test pre-creates the selected bug path and asserts that atomic create-new refuses, preserves the original bytes and creates no numeric fallback record. A race through the same create-new path has at most one creator and never replaces the winner. Updates and automatic appends to an existing open bug preserve its ID, 24-hex suffix and filename; no update recomputes an ID from a changed title or other mutable content. |
| REQ-cli-012 | Manual and automatic creation call the same generator. Newly created-ID assertions in `tests/bugs.rs` and `tests/optional.rs` use returned IDs rather than assuming BUG-0001 or BUG-0002. Existing literal BUG-NNNN records elsewhere remain legacy fixtures, and schema 1 stays readable with no migration or renumbering. Invalid new forms, including uppercase hex, a short or long suffix, an empty slug and an ID over 100 bytes, are refused without rejecting exact legacy IDs. |
| REQ-cli-011, REQ-cli-012, REQ-cli-013 | `tests/optional.rs`: a failed repeated-validation case still captures one open bug and appends later failures to that same bug. Its first ID uses the shared new generator and its later observations do not change the ID; existing authorization, stop, resume and evidence behavior stays green. |

## Focused and full checks

Run `cargo test --locked --test bugs`, `cargo test --locked --test optional`, and `cargo run --locked -- cases check`. Run the pre-fix and post-fix TC-cli-0125 observations in the order above, then run the full `cargo test --locked` required by hosted product CI.

With HI 0.8.0, Fledge 1.7.2 and SpecSync 6.1.1, run `fledge run fmt`, `fledge run lint` (Clippy over all targets with warnings denied), `fledge run test`, `fledge lanes run verify` and `fledge lanes run pre-push`. The verify lane includes the nine configured format, lint, test, HI, SpecSync, native audit, case-definition, exercise and build steps; pre-push includes its five configured steps. Record the actual environment and revision for each observation. Do not claim hosted Ubuntu, macOS or Windows results without those hosted runs.

## Evidence boundaries

Use the existing observation schema, observation IDs and `test-log` capture for TC-cli-0125. The reserved case ID does not introduce a case-ID scheme change, and the two observation IDs above are instances of the existing evidence scheme. No evidence, artifact, validation, snapshot, transition, HI, requirement or SpecSync identity format is created or changed. Structural success, a passing case, an actor label and an agent review remain distinct from human acceptance, release and deployment.

## Where these lessons go

- `specs/cli/context.md`
