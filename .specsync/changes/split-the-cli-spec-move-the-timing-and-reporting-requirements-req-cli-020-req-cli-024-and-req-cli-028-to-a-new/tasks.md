---
change: split-the-cli-spec-move-the-timing-and-reporting-requirements-req-cli-020-req-cli-024-and-req-cli-028-to-a-new
artifact: tasks
---

# Tasks

- [ ] Record pre-split digests at a33c747 and 70009d6
- [ ] Add specs/reporting and move file ownership
- [ ] Write the cli and reporting deltas
- [ ] Move the 9 cases and twin the 6 co-linked cases with the documented ID mapping
- [ ] Document the mappings and update cited IDs
- [ ] Extend tests/spec_split.rs
- [ ] Confirm the reporting delta bodies match the a33c747 digests before materializing (materialize, the moved cases, verify and the before/after comparison are in testing.md)
