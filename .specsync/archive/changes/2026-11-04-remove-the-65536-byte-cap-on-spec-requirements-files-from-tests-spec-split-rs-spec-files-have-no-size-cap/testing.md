---
change: remove-the-65536-byte-cap-on-spec-requirements-files-from-tests-spec-split-rs-spec-files-have-no-size-cap
artifact: testing
---

# Testing

`cargo test --locked --test spec_split` runs 4 tests and passes with
specs/cli/requirements.md at 66,330 bytes. Before the change the same command fails
`every_requirements_companion_fits_the_record_bound`, as hosted CI did on #91.
TC-history-split-keeps-requirement-wording and
TC-reporting-split-keeps-requirement-wording run on local-linux-aarch64, and
`fledge lanes run verify` passes on Linux.
