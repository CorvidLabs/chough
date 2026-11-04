---
change: remove-the-65536-byte-cap-on-spec-requirements-files-from-tests-spec-split-rs-spec-files-have-no-size-cap
artifact: tasks
---

# Tasks

- [x] Remove `every_requirements_companion_fits_the_record_bound` and its constant from tests/spec_split.rs.
- [x] Update TC-history-split-keeps-requirement-wording and TC-reporting-split-keeps-requirement-wording to expect 4 passing tests, with no size clause.
- [x] Run both cases and `fledge lanes run verify` on Linux with specs/cli/requirements.md over 65,536 bytes.
