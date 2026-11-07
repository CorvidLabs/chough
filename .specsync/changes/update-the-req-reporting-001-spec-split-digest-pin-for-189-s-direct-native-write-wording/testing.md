---
change: update-the-req-reporting-001-spec-split-digest-pin-for-189-s-direct-native-write-wording
artifact: testing
---

# Testing

- `cargo test --locked --test spec_split` at the fix: 4 passed, 0 failed —
  reporting_requirements_keep_their_wording_under_their_new_ids now passes with the
  pinned digest matching the materialized REQ-reporting-001 wording.
- Full suite on PR #192 CI: the four jobs that failed on the stale pin must be
  green before merge; no other behavior changes, so no new cases are added.
