---
change: assert-positive-track-selection-and-every-delivery-observation-kind-and-that-a-dry-run-never-stands-in-for-one-152-ma
artifact: tasks
---

# Tasks

- [ ] tests/cases.rs `requesting_the_copy_localization_track_selects_its_linked_cases`:
  - with no track, copy-localization is omitted with its own reason;
  - with `--track copy-localization`, its linked case is selected and design-QA stays omitted;
  - with both tracks, all three linked cases are selected.
- [ ] tests/cases.rs `every_delivery_kind_is_recorded_as_itself_and_a_dry_run_as_a_dry_run`: publication, installation, deployment, live-health and the legacy delivery kind are each accepted and reported as themselves, and the dry run as a dry run.
- [ ] tests/reduce.rs `delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not`, for each of the four kinds:
  - `evidence prune` protects the delivery observation with its kind as the reason, and never protects the dry run for its kind;
  - the done trim moves the delivery observation into the vault (ledgered) and removes the superseded dry run with no vault copy.
- [ ] Add the REQ-cli-014 criterion, and a CHANGELOG line.

After `specsync change check` materializes the delta, not a task box: record the cases and the mutation logs, and run `fledge lanes run verify` on Linux.
