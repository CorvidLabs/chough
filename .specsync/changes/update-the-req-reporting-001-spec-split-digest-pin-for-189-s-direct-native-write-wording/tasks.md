---
change: update-the-req-reporting-001-spec-split-digest-pin-for-189-s-direct-native-write-wording
artifact: tasks
---

# Tasks

- [x] Update the REQ-cli-020 → REQ-reporting-001 pinned digest in tests/spec_split.rs
      to the materialized wording (f10aaac5…) with its #77-style comment.
- [ ] Full CI green on PR #192: the four jobs that failed on the stale pin
      (MSRV / 1.88.0, Product / ubuntu-latest, Product / macos-latest,
      Fledge / HI / SpecSync) must pass before merge.
