---
change: fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag
artifact: testing
---

# Testing

- `tests/change.rs`: `ship_passes_the_relay_to_the_native_review`, `ship_passes_a_relay_that_starts_with_a_hyphen_as_one_argument_per_field`.
- `tests/humans.rs`: scope, review and gate-record calls carry the relay; `a_relay_that_starts_with_a_hyphen_reaches_the_native_review_joined_to_its_flag`.
- The hyphen tests fail on the previous `Step::arguments` and pass with this change.
- Cases in `.chough/cases/cli.ship-relay.cases.toml`, including `TC-cli-ship-relay-hyphen-values-join-their-flags`.
