---
change: update-the-req-reporting-001-spec-split-digest-pin-for-189-s-direct-native-write-wording
artifact: context
---

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
