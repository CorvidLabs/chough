---
change: use-collision-resistant-slug-identities-for-new-bugs
artifact: plan
---

# Plan

## Approach

Prove the independent-root collision before implementation, then add one shared generator and dual-form validation in `src/bugs.rs`. Keep automatic capture in its existing path, retain legacy records, and verify mixed-form links and fail-closed creation before the broader lanes.

Ownership is by file group: the implementation owner takes `src/bugs.rs`; the regression owner takes `tests/bugs.rs` and `tests/optional.rs`; the contract and documentation owner takes `specs/cli/requirements.md`, `specs/cli/cli.spec.md`, `specs/cli/tasks.md`, `specs/cli/testing.md`, `docs/FRD.md` and `docs/cli.md`. One records integrator owns `.chough/cases/cli.cases.toml` and the new bug, observation and artifact records, with no concurrent `.chough/` writers.

The relative estimate is medium overall. The algorithm is small; fixture construction is the main uncertainty, especially valid automatic-capture inputs in independent roots and a deterministic same-ID different-content test that does not require a real SHA-256 collision.

## Out of scope

Migrating or renumbering legacy bugs, changing bug schema 1, changing any non-bug ID scheme, adding dependencies, and changing the validation command's capture policy. No new integration, runtime, hosted service, release or deployment. No rewrite of observations, frozen archives or existing bug history.

## Steps

1. Record the allocator defect locally. Add the independent-root regression and reserved `TC-cli-0125`, then run it at the unfixed base before source changes. Use separate root pairs seeded to legacy high-water `BUG-0019` for manual addition and automatic capture of distinct reports. Record the observed identical `BUG-0020` allocation as a failed case observation and link it to the allocator bug. Implementation does not start until this failure is retained and linked.
2. In `src/bugs.rs`, implement the specified slugger, canonical payload type, domain-separated SHA-256 truncation and dual-form validator. Route both creation paths through the single generator, retain atomic create-new, remove max-plus-one allocation from new creation, and update comments that describe only numeric IDs. Add fixed payload and digest vectors, slug boundaries, generator immutability and same-ID different-content preservation tests. A test-only candidate-ID seam may force the collision; production must not gain an override.
3. Complete `tests/bugs.rs` and `tests/optional.rs` coverage for distinct manual and automatic IDs at equal legacy high-water marks, the 100-byte limit with all 24 digest characters, legacy loading and updates, cross-form duplicate and related links, and no overwrite or numeric fallback. Keep existing legacy fixtures unchanged where possible.
4. Update the listed requirement, contract, task, testing and documentation files to describe only the approved behavior, including the old-version upgrade requirement and the unchanged schema and non-bug ID schemes. Materialize the scoped requirement delta with the native tool at the documented point, not by hand.
5. Rerun the same regression and `TC-cli-0125` after the fix, record a passing observation strictly later than the linked failure, and link it to the same allocator bug. Then run focused Rust tests, affected cases, `cargo fmt`, Clippy with warnings denied, full `cargo test`, Fledge verify and pre-push with the CI-matching tool versions. Report actual results only.

Post-verification native review, any requested commit or push, hosted checks and human implementation review remain plain-text lifecycle steps, not task checkboxes or approval recorded by this plan.

## Risks

- Accidental payload drift would allocate different IDs for logically identical input across versions. Mitigate with a dedicated payload type, fixed canonical vectors and the versioned domain separator.
- Identical creation payloads in the same second intentionally produce the same candidate. The existing record stays untouched and creation reports already exists; no retry manufactures another identity.
- A 96-bit digest truncation is collision-resistant rather than mathematically collision-free. Atomic different-content refusal is mandatory and must be tested through a forced candidate, not an attempted natural collision.
- Automatic capture deduplicates an open bug for the same case. Fixtures must use distinct cases or otherwise ensure that the test reaches new-record creation rather than an append.
- Independent-root fixtures need valid revisions, observations and case data. Keep setup helpers shared and make the pre-fix failure distinguish an allocator collision from fixture or environment failure.
- Older binaries reject new IDs. Documentation must make the upgrade requirement visible rather than implying bidirectional version compatibility.

## Constraints consulted

`AGENTS.md`, `INTENT.md`, FLOW-7, FLOW-8, LIFE-2 and LIFE-5, current REQ-cli-012, `specs/cli/context.md`, the 100-byte `valid_name` bound, existing `sha2` 0.10, atomic `write_new`, and the archived Kimi integration record for merge `b2e2f19`. The change keeps `.chough/` out of native affected paths, preserves stable case and evidence identities, and requires failure-first evidence followed by a strictly later pass.
