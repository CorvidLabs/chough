# Lesson bundle: update-compact-next-fixture-for-slug-observations

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Update compact-next fixture for slug observations
- **Kind**: Refactor
- **Specs**: cli
- **Paths**: tests/next.rs
- **Acceptance**: The compact-next large-record fixture creates its current observation through automatic allocation and references the returned ID; all existing 16384-byte bounds, one-native-read counts, totals and historical-label assertions remain unchanged; tests/next.rs and the full suite pass with the approved identity behavior.

## Evidence

- Verification commit: `9e422fc1629d3b70b443054d17e0bb4c669bf86e`
- Base commit: `6800b2dadebec358ab1a6fdb18b5d78136eb6631`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The approved identity implementation rejects old-style IDs on new observation writes. The compact-next large-record fixture in tests/next.rs still supplies "current-one", so it fails before exercising the output bounds. The failure is retained in the takeover's initial-tests.log.

The file is absent from the identity approval's frozen affected_paths. Preserve that approval and delta. This native dependent change covers tests/next.rs only; no production behavior or canonical contract changes.

## From the change's testing.md

# Testing

Retain the observed pre-update failure of both_forms_stay_within_the_cap_with_totals_for_large_records: the obsolete "current-one" ID is refused before report assertions.

After the fixture update, run fledge run test -- --test next, then the full Rust suite and configured verify/pre-push lanes. The generated current observation must be referenced and counted as current. Keep every existing bound, total, legacy label and exactly-one-native-read assertion unchanged. No command success records human acceptance or archive.

## Observed focused verification

After native scope approval and the prepared fixture-only update, `fledge run test -- --locked --test next` passed all 3 tests with Rust 1.99.1 on local-macos-arm64. No assertion was changed. The original failed TC-cli-0120 observation remains retained; the later case record and full verification logs belong in `.chough/evidence/` and `.chough/artifacts/`. Full-suite and configured-lane results must be read from those actual observations, not inferred from this focused pass.

The subsequent unfiltered `RUSTUP_TOOLCHAIN=1.88.0 fledge run test -- --locked` passed all 182 tests, with no ignored or filtered tests, on local-macos-arm64.

## Observed native ordering blocker

Both unfiltered Rust suites passed all 182 tests. Full verify nevertheless failed at audit, step 6: tests/next.rs is not counted as covered while this follow-up depends on the identity change being accepted first. `specsync change check update-compact-next-fixture-for-slug-observations` returned exit 1: the parent is verifying and must be accepted before this change can start. The accepted-first edge was an unsuitable planning choice for a co-reviewed fixture. The current CLI has no dependency removal/amend command; reopen applies to stale accepted evidence, correct supports only public_contract/architecture_risk, and supersede requires an accepted predecessor. Preserve the valid approval and failed command evidence; do not rewrite native state, fabricate parent acceptance or weaken the audit. Native closeout remains blocked pending a supported ordering resolution.

## Where these lessons go

- `specs/cli/context.md`
