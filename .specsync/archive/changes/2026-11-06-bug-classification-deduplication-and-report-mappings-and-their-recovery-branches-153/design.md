---
change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
artifact: design
---

# Design

All runtime changes are in `src/bugs.rs`. No record schema changes; schema 1 bug records stay as they are.

- `update`: after the status, classification, scope and duplicate handling, a bug whose status is not `duplicate` loses its `duplicate_of`. The change "no longer a duplicate of <id>" goes into the same note as the status change.
- `update --related <id>` refuses the bug's own ID before any write: "a bug cannot relate to itself; name another bug with --related <id> (run `chough bug list --all` to see the bug IDs)".
- The self-duplicate refusal gains the same recovery. `load` of an unknown ID says "unknown bug: <id>; run `chough bug list --all` to see the bug IDs". That covers update, report, show and links.
- The two verified refusals append "link one with `chough bug update <id> --observation <id>`" and "link a later pass with `chough bug update <id> --observation <id>`".
- Search, scoring, thresholds, value lists and the report format are unchanged; the new tests pin them.
