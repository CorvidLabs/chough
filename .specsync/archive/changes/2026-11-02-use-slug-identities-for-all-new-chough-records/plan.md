---
change: use-slug-identities-for-all-new-chough-records
artifact: plan
---

# Plan

## Approach

Prove the current independent-root and same-second repeated-run collisions before implementation, then introduce one shared identity module and route every new case, observation, validation and snapshot through the specified grammars. Keep legacy readers and bug generation unchanged. Follow the failing evidence with later passing runs of the same scenarios, then exercise mixed-history consumers and the full project lanes.

The relative estimate is medium to large overall: implementation is medium, while verification and fixture breadth make the complete change larger than the recent single-generator bug change. The main uncertainty is fixture construction, not slug or random encoding complexity. Independent roots need equivalent valid Git and case inputs, repeated runs need a controlled same-second scenario, collision tests need injected candidates without a production override, and random-source and partial-write failures need fault seams that prove preservation.

## Ordered ownership

Work proceeds in this order to avoid overlapping writes:

1. The evidence and regression owner takes `tests/common/mod.rs` and the affected integration test files, initially `tests/cases.rs`, `tests/evidence.rs`, `tests/optional.rs`, `tests/cli.rs`, `tests/timing.rs`, `tests/stages.rs`, `tests/bugs.rs` and `tests/package.rs`. This owner adds failure-first coverage and later compatibility passes. One records integrator separately owns every `.chough/` case, bug, observation and artifact write, with no concurrent `.chough/` writers.
2. The identity implementation owner takes `Cargo.toml`, `Cargo.lock`, new `src/identity.rs`, `src/main.rs` and the narrowly shared normalization call in `src/bugs.rs`. Bug payload and suffix algorithms remain fixed. This owner lands before record-generator work.
3. The record implementation owner takes `src/cases.rs`, `src/evidence.rs`, `src/validate.rs` and `src/library.rs`, connecting optional observation IDs, random allocation, atomic refusal and legacy reads without changing record schemas.
4. The guidance owner takes `src/skill.rs`, `docs/FRD.md`, `docs/cli.md`, `docs/architecture.md`, `README.md`, the canonical skill files and their installed host copies. Canonical bundle changes precede digest registration and byte-identical installed-copy updates.
5. The native contract owner completes the remaining selected artifacts and `deltas/cli.md`, then uses the native materialization path at the authorized point. Canonical specs are never hand-edited, `.chough/` stays out of native affected paths, and frozen archives stay untouched.

Only clearly disjoint file groups may proceed concurrently after failure-first evidence is retained. The records integrator remains serial throughout.

## Out of scope

No migration, renumbering, schema rewrite, bug-generator change, new bug known answer, central ID reservation, deterministic run-content hash, timestamp or numeric fallback, random retry, runtime, service or second lifecycle store. No transition, preference, consent, stop-marker or pruning-ledger identity is added. No HI criterion, requirement ID, SpecSync ID, frozen archive, release or deployment changes. No production option forces a random suffix or bypasses collision refusal.

## Steps

1. Add curated slug regression definitions such as `TC-cli-independent-root-random-identities`, `TC-cli-identical-repeated-runs-remain-distinct`, `TC-cli-legacy-identity-compatibility` and `TC-cli-atomic-identity-collision-refusal`. Add direct integration tests for slug case acceptance and the new generators. At the unfixed base, run the focused tests and real commands in independent roots with equal local state, plus identical same-second observations, validation attempts and snapshots. Retain the observed collisions or refusals as failed observations and link them to local bugs before source implementation. Do not convert a fixture or environment failure into product evidence.
2. Add getrandom 0.4.3 as a direct dependency without changing the locked version. Implement the shared normalizer, exact validators, 12-byte random fill and lowercase hex encoding in `src/identity.rs`. Move only the bug slugger's common normalization onto that helper; leave its domain separator, payload, SHA-256 truncation and known-answer tests unchanged. Add normalizer and boundary vectors for punctuation, repeated separators, ASCII case, non-ASCII input, empty results, trailing-hyphen truncation and every per-prefix byte limit.
3. Extend case validation with the curated slug alternative while retaining exact legacy numeric forms and global duplicate refusal. Verify that legacy definition bytes and digests are unchanged, new regression case IDs use slugs, title edits do not rename cases, over-budget module and slug combinations refuse, and duplicates fail across the complete loaded set.
4. Implement observation, validation and snapshot allocation with the specified slug sources and one random suffix per creation. Make direct evidence IDs optional, accept only exact new-form explicit observation IDs for new writes, and give old-style supplied IDs the omit-ID remedy before any artifact write. Preserve automated artifact naming by inheriting the observation ID exactly.
5. Connect every new record to one atomic create-new file or final-directory attempt. Add forced-candidate tests for same-ID different-content refusal without a natural random collision. Prove byte-for-byte preservation, no replacement, no numeric or timestamp fallback, and no second random or digest attempt. Cover artifact collision, validation creation, snapshot final-directory collision, manifest-last behavior, random-source failure and later write failure. Test-only seams may supply fixed bytes or failures; production has no override.
6. Complete mixed-history coverage. Read and link legacy observations, resume legacy and new validation runs, index legacy dated and new slug snapshots, and pass mixed IDs through bug links, checkpoint references, status and next, timing, retention previews and application, pruning labels and native manifests. Keep old fixtures where they establish compatibility. Confirm transition, preference, consent, stop and pruning shapes and all bug known answers remain unchanged.
7. Update canonical requirements and companions only through the approved native flow, then align CLI help, user documentation, architecture and skill guidance. State the old-version upgrade requirement, optional explicit observations, exact artifact inheritance and lack of migration. Keep canonical and installed skill copies byte-identical and register previous digests as required by the existing update tests.
8. Rerun the original direct tests and real independent-root and repeated-run scenarios after implementation. Record strictly later passing observations with newly allocated IDs and link them to the same local bugs. Run focused Rust tests and affected cases, formatting, Clippy with warnings denied, full `cargo test`, real command exercises, `fledge lanes run verify` and pre-push using the CI-matching tool versions. Report only observed results.

Post-verification native review, any requested commit or push, hosted checks and human implementation review remain plain-text lifecycle steps, not task checkboxes or approval recorded by this plan.

## Risks

- A deterministic run hash would collapse identical repeated history. Use random 96-bit suffixes and test identical content and timestamps, not only different inputs.
- Random IDs are collision-resistant, not collision-free. Force a candidate collision and require different-content refusal with no retry or overwrite.
- Timestamp fields must remain chronological even though IDs are unordered. Test status, timing and librarian ordering with IDs whose lexical order differs from timestamp order.
- A 100-byte ID that loses suffix digits is invalid. Test exact boundary lengths for OBS, VAL, SNAP and modules of several lengths.
- Refactoring the bug slugger could alter known answers. Keep bug payload and digest code isolated and run the existing vectors unchanged.
- Optional direct evidence IDs can break scripts that intentionally supply legacy names. The required omit-ID remedy must be explicit, while stored legacy IDs still load and resume.
- A snapshot is a directory tree rather than one file. Final-directory create-new allocation and manifest-last publication must be tested without claiming crash-proof multi-file atomicity.
- Fault injection can accidentally become a production ID override or silently delete prior content. Keep seams test-only and assert preservation of every pre-existing path after each failure.
- Broad fixture edits can erase legacy coverage or make failures ambiguous. Retain old fixtures, isolate each generator and validate fixture prerequisites before interpreting an outcome.

## Constraints consulted

`AGENTS.md`, `INTENT.md`, FLOW-7, FLOW-8, FLOW-9, FLOW-12, FLOW-13, LIFE-2, LIFE-3, LIFE-5, LIFE-7 and LIFE-8; current REQ-cli-005, REQ-cli-007, REQ-cli-009, REQ-cli-011, REQ-cli-012, REQ-cli-015, REQ-cli-017 and REQ-cli-018; `specs/cli/cli.spec.md`, `specs/cli/context.md`, current generator source, getrandom 0.4.3 in `Cargo.lock`, and the recent bug-identity and checkpoint-merge native deltas. The design preserves one checkpoint, native ownership, stable legacy references, honest evidence and failure-first proof followed by later passes.
