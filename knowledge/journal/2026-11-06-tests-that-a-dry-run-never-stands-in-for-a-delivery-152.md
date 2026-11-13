---
id: 2026-11-06-tests-that-a-dry-run-never-stands-in-for-a-delivery-152
title: Tests that a dry run never stands in for a delivery (#152)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T14:11:25Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-assert-positive-track-selection-and-every-delivery-observation.md
  - .specsync/archive/changes/2026-11-06-assert-positive-track-selection-and-every-delivery-observation-kind-and-that-a-dry-run-never-stands-in-for-one-152-ma/
record: .chough/archive/sessions/2026-11-06-assert-positive-track-selection-and-every-delivery-observation.md
actor:
  - agent: claude-code
    terminal: t382
---
# Tests that a dry run never stands in for a delivery (#152)

Following audit findings MA-01 and MA-02, this change added tests and contract wording only: requesting the copy-localization track selects its linked cases, every delivery-observation kind is recorded as itself, and a dry run never stands in for one. The dry-run rule is asserted where Chough tells them apart, in reporting and in retention, where delivery kinds are kept as proof and a superseded dry run is not. The REQ-cli-014 criterion and a changelog line were added.

The three new cases passed on macOS, and each new test failed under a targeted mutation the existing tests let through: 4 of 4 mutations caught, rerun on the merged tree.

The maintainer approved the scope and the implementation review on 6 November 2026, both relayed through the coordinating agent, and the story closed as done that day.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-assert-positive-track-selection-and-every-delivery-observation.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-assert-positive-track-selection-and-every-delivery-observation-kind-and-that-a-dry-run-never-stands-in-for-one-152-ma/)
