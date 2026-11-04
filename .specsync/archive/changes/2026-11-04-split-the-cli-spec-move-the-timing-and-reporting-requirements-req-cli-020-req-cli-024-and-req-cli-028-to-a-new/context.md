---
change: split-the-cli-spec-move-the-timing-and-reporting-requirements-req-cli-020-req-cli-024-and-req-cli-028-to-a-new
artifact: context
---

# Context

PR #71 (refuse an unknown native change, issue 63) cannot land after merging main
a33c747 (#79): its cli requirements companion grows to 65,698 bytes, over the
65,536-byte bound that tests/spec_split.rs (`every_requirements_companion_fits_the_record_bound`)
enforces. Main alone is 64,152 bytes. The next PRs in the train add more to cli
(#72 about 248 bytes, #78 about 988, #77 about 1,658), so the companion would reach
about 68,600 bytes before #69.

The parent agent chose, on the evening of 2026-11-03 (MST) under Leif's (0xLeif)
standing Chough authority (2026-11-03 7:49 AM MST) and his t820u and t822u asks to
finish and merge the Chough PRs, to split again rather than raise the bound, as the
history split did (#59, 4daf861): keep every requirement's wording, document the ID
mapping, and weaken nothing. Leif's own words on 2026-11-03 at 10:05 AM MT favour
this: "we should break stuff apart and decouple it if possible but if things need
to be coupled, that's fine."

The timing and reporting group (REQ-cli-020, REQ-cli-024, REQ-cli-028, 7,604 bytes)
was chosen because all three trace LIFE-10, they have natural owned files
(src/timing.rs, tests/timing.rs), and no open train PR before #69 modifies any of
them (#72 touches 009, #71 and #77 touch 007, #78 touches 021). #69 modifies
REQ-cli-024 and REQ-cli-028 and adds REQ-cli-035, so its growth lands in the new
module once its delta is pointed at the new IDs.

Unlike the history group, REQ-cli-020 and REQ-cli-024 share six cases with
requirements that stay in cli (TC-cli-0080, 0082, 0088, 0089, 0100, 0104). Chough
validates a case's requirement links only against its own module's companion, so
such a case cannot link both. Each keeps its cli ID and its links to the cli
requirements and gains a TC-reporting twin with the identical binding for the moved
requirement; no requirement loses a case binding.

Already ruled out: raising the limit (Leif's no-weakening rule and the parent's
choice); shortening requirement text (weakens requirements); keeping `REQ-cli-` IDs
in another module (SpecSync refuses it); the cases and bugs group (REQ-cli-004 to
017, about 16,300 bytes and 54 cases, collides with #72's REQ-cli-009 delta).
