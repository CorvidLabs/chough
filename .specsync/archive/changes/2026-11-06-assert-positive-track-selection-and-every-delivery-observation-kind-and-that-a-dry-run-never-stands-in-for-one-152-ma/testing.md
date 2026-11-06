---
change: assert-positive-track-selection-and-every-delivery-observation-kind-and-that-a-dry-run-never-stands-in-for-one-152-ma
artifact: testing
---

# Testing

The new tests assert behavior that holds today. Each is shown real by a targeted mutation that makes it fail while the existing tests named in the issue still pass:

| Mutation (src) | Fails | Existing test that lets it pass |
|---|---|---|
| `select` never selects copy-localization, even when requested | `requesting_the_copy_localization_track_selects_its_linked_cases` | `selection_is_focused_on_the_affected_scope_and_tracks` |
| `--kind live-health-observation` no longer accepted (`#[value(skip)]`) | `every_delivery_kind_is_recorded_as_itself_and_a_dry_run_as_a_dry_run` | `schema_2_evidence_accepts_delivery_kinds_and_keeps_dry_runs_distinct` |
| Deployment observations counted as routine | `delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not` | `schema_2_evidence_accepts_delivery_kinds_and_keeps_dry_runs_distinct` |
| Dry runs counted as non-routine key items | `delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not` | `schema_2_evidence_accepts_delivery_kinds_and_keeps_dry_runs_distinct` |

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-010 | tests/cases.rs `requesting_the_copy_localization_track_selects_its_linked_cases`, `selection_is_focused_on_the_affected_scope_and_tracks` |
| REQ-cli-014 | tests/cases.rs `every_delivery_kind_is_recorded_as_itself_and_a_dry_run_as_a_dry_run`; tests/reduce.rs `delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not` |
| REQ-cli-017 | tests/cases.rs `every_delivery_kind_is_recorded_as_itself_and_a_dry_run_as_a_dry_run` |

## Lanes

`fledge lanes run verify` passes on Linux.
