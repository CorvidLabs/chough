---
id: update-the-req-reporting-001-spec-split-digest-pin-for-189-s-direct-native-write-wording
state: implementing
type: bug_fix
base_commit: a3c7a590ad6fcbe3c6950fe8eb7297353f821716
---

# Update the REQ-reporting-001 spec-split digest pin for 189's direct-native-write wording

## Intent

Update the REQ-reporting-001 spec-split digest pin for 189's direct-native-write wording

## Affected Canonical Specs

- None

## Acceptance Criteria

- cargo test --locked --test spec_split passes with the pinned digest matching the materialized REQ-reporting-001 wording, and PR #192's full CI is green

## No-spec Rationale

The 189 delta (PR #192) reworded REQ-reporting-001 — direct-native-write events (REQ-history-013) start or end no segment and are not an unrecognised kind — but missed updating the pinned digest in tests/spec_split.rs, so CI failed in the MSRV, Product and Fledge jobs; test-only fix, canonical specs unchanged.
