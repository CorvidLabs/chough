---
change: update-compact-next-fixture-for-slug-observations
artifact: testing
---

# Testing

Retain the observed pre-update failure of both_forms_stay_within_the_cap_with_totals_for_large_records: the obsolete "current-one" ID is refused before report assertions.

After the fixture update, run fledge run test -- --test next, then the full Rust suite and configured verify/pre-push lanes. The generated current observation must be referenced and counted as current. Keep every existing bound, total, legacy label and exactly-one-native-read assertion unchanged. No command success records human acceptance or archive.

## Observed focused verification

After native scope approval and the prepared fixture-only update, `fledge run test -- --locked --test next` passed all 3 tests with Rust 1.99.1 on local-macos-arm64. No assertion was changed. The original failed TC-cli-0120 observation remains retained; the later case record and full verification logs belong in `.chough/evidence/` and `.chough/artifacts/`. Full-suite and configured-lane results must be read from those actual observations, not inferred from this focused pass.

The subsequent unfiltered `RUSTUP_TOOLCHAIN=1.88.0 fledge run test -- --locked` passed all 182 tests, with no ignored or filtered tests, on local-macos-arm64.
