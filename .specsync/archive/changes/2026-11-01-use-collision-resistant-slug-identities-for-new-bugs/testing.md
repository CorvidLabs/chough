---
change: use-collision-resistant-slug-identities-for-new-bugs
artifact: testing
---

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
