# Lesson bundle: update-the-req-reporting-001-spec-split-digest-pin-for-189-s-direct-native-write-wording

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Update the REQ-reporting-001 spec-split digest pin for 189's direct-native-write wording
- **Kind**: BugFix
- **Paths**: tests/spec_split.rs
- **Acceptance**: cargo test --locked --test spec_split passes with the pinned digest matching the materialized REQ-reporting-001 wording, and PR #192's full CI is green

## Evidence

- Verification commit: `582ce2ae655e5a6acc9127f1845974d6e854ad64`
- Base commit: `a3c7a590ad6fcbe3c6950fe8eb7297353f821716`
- Verified by: `specsync check --spec history`

## From the change's context.md

# Context

The 189 change (PR #192, DRIVE-21) reworded REQ-reporting-001: direct-native-write
events (REQ-history-013) start or end no segment and are not counted as an
unrecognised kind. The 2026-11-04 reporting split pins each moved requirement's
wording digest in tests/spec_split.rs, and a wording change must update its pin
with a #77-style comment (see the REQ-cli-025/#77 and REQ-cli-024/#169 rows). The
189 lane missed the pin for REQ-reporting-001.

The failure was hidden until the change shipped: the CI Archive gate was red while
the change was still open, which short-circuited the workflow, so the MSRV, Product
and Fledge jobs showed skipping. Once PR #192's head carried the archived change,
the full suite ran and four jobs failed on
reporting_requirements_keep_their_wording_under_their_new_ids.

Constraints: the 189 change is archived, so this small covering change carries the
test-only fix. Canonical specs are unchanged — the materialized REQ-reporting-001
wording is already reviewed on PR #192 — and the pin keeps the file's #77-style
comment convention.

## From the change's testing.md

# Testing

- `cargo test --locked --test spec_split` at the fix: 4 passed, 0 failed —
  reporting_requirements_keep_their_wording_under_their_new_ids now passes with the
  pinned digest matching the materialized REQ-reporting-001 wording.
- Full suite on PR #192 CI: the four jobs that failed on the stale pin must be
  green before merge; no other behavior changes, so no new cases are added.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
