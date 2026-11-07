---
change: return-the-observation-id-from-cases-record-and-evidence-add-issue-60
artifact: context
---

# Context

Issue #60 reports that automation cannot directly consume the observation ID it
just created: `cases run` returns `case` and `observation`, while `cases record`
and `evidence add` return only the evidence path in `recorded`. Kimi reproduced
this on main, and the issue's bounded fix is to expose the IDs already present in
memory and in the stored record.

The earlier issue comment describes a prepared local branch, but no such branch,
pull request or native approval exists in the current repository. This change is
therefore planned from main `5ff54dd7`; it inherits no approval or verification.

Existing HI criteria FLOW-7 and LIFE-4 and requirements REQ-cli-011, REQ-cli-017
and REQ-cli-023 cover this small CLI correction. No new HI or requirement ID is
needed. The related bug-output and stage-refusal envelope suggestions in issue
#60 are explicitly out of scope; so is adding unreferenced observations to
`next`.

Constraints: failing tests first; one theme per PR; preserve existing JSON keys,
exit codes, record files and schemas; use a change-specific cases file; strict
audit before every push; archive before waiting for hosted CI; no release work.
