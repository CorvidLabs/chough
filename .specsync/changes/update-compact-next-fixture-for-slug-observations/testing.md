---
change: update-compact-next-fixture-for-slug-observations
artifact: testing
---

# Testing

Retain the observed pre-update failure of both_forms_stay_within_the_cap_with_totals_for_large_records: the obsolete "current-one" ID is refused before report assertions.

After the fixture update, run fledge run test -- --test next, then the full Rust suite and configured verify/pre-push lanes. The generated current observation must be referenced and counted as current. Keep every existing bound, total, legacy label and exactly-one-native-read assertion unchanged. No command success records human acceptance or archive.
