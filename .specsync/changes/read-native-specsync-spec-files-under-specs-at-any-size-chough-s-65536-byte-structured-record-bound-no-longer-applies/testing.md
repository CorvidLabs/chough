---
change: read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies
artifact: testing
---

# Testing

- tests/cli.rs `spec_requirements_companions_are_read_at_any_size`: two
  requirements companions over 4 MiB; `evidence add` accepts an ID in its module and
  refuses one owned by another module with the existing message, never a size
  refusal. It failed first with "structured record exceeds 65536 bytes".
- tests/cli.rs `records_other_than_the_checkpoint_keep_the_65536_byte_bound` and the
  checkpoint capacity tests still pass unchanged.
- tests/cli.rs `build_reference_from_main_4daf861_is_known_outdated`: main's build
  reference reads known-outdated, and `skill update` makes it current with the new
  section. It failed first with `edited`.
- Cases TC-cli-spec-files-read-at-any-size and
  TC-cli-build-reference-from-main-known-outdated on local-linux-x86_64.
- fmt, lint, the full tests and `fledge lanes run verify`.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-034, TC-cli-spec-files-read-at-any-size | `tests/cli.rs::spec_requirements_companions_are_read_at_any_size`: cli and checkout-ui requirements companions padded past 4 MiB; `evidence add` accepts REQ-cli-001 and refuses REQ-checkout-ui-004 with the "belongs to module checkout-ui" hint and no size refusal. Failed first with "structured record exceeds 65536 bytes". |
| REQ-cli-034, REQ-cli-032 | `tests/cli.rs::records_other_than_the_checkpoint_keep_the_65536_byte_bound` and the checkpoint capacity tests, unchanged: Chough's own records keep their bounds. |
| REQ-cli-034, TC-cli-build-reference-from-main-known-outdated | `tests/cli.rs::build_reference_from_main_4daf861_is_known_outdated`: main's build reference reads known-outdated and `skill update` makes it current with "## Keep it composable". Failed first as `edited`. |
