---
change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
artifact: plan
---

# Plan

## Approach

One product change in two places, behind two failing tests, plus docs.

## Out of scope

KNOW-22 to KNOW-28.

## Steps

1. Leif approves the definition; then move to build.
2. Commit the failing tests: `done_drops_every_file_and_design_reference` and `an_archived_session_file_reference_protects_nothing`, in tests/reduce.rs. Move the #109 folder-reference test to a trim before done.
3. Implement the drop and the retention rule.
4. Update docs/cli.md, docs/FRD.md and the CHANGELOG. Add the per-change case file.
5. Run the dry run on this repository, verify on Linux, and close out.
