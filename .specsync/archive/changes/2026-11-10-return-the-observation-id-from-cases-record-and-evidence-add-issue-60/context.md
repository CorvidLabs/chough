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

Existing HI criteria FLOW-7 and LIFE-4 trace this small CLI correction. The
canonical amendment modifies REQ-cli-011 for `cases record` and REQ-cli-017 for
`evidence add`; REQ-cli-023's allocation behavior stays unchanged. No new HI or
requirement ID is needed. The related bug-output and stage-refusal envelope
suggestions in issue #60 are explicitly out of scope; so is adding unreferenced
observations to `next`.

Leif said "yes to all" to the original scope at 91345d26, but native approval
was refused because its no-spec flag conflicted with the declared public
contract change. No approval was recorded. This draft-only amendment preserves
the accepted behavior and acceptance criteria while supplying the required
canonical delta for a fresh decision.

Constraints: failing tests first; one theme per PR; preserve existing JSON keys,
exit codes, record files and schemas; use a change-specific cases file; strict
audit before every push; archive before waiting for hosted CI; no release work.
