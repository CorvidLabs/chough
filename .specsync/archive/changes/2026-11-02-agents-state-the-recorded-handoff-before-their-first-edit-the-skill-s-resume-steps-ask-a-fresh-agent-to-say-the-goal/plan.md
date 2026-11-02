---
change: agents-state-the-recorded-handoff-before-their-first-edit-the-skill-s-resume-steps-ask-a-fresh-agent-to-say-the-goal
artifact: plan
---

# Plan

## Approach

Add one resume step to SKILL.md, refresh the host copies, record the earlier digest
with a test, modify REQ-cli-002, then rerun the frozen cold start: a baseline at main
118a895, a change run at the implementation tip and the real-repository run, all
graded by 0xLeif.

## Out of scope

Any change to `chough next`, `status` or other command output; any reference file
other than SKILL.md; the seed and rubric; other hosts or models; issues #11, #31 and
#32.

## Steps

1. Before implementation: check the frozen pair's digests, confirm Kimi starts and
   has quota, and run the baseline at main 118a895.
2. Write the failing package test for the 118a895 SKILL.md copy and the step's
   presence, then edit SKILL.md, refresh host copies and add the digest.
3. Scoped `specsync change check`, then add the case and record observations.
4. Change run at the implementation tip; write the real-repository expected facts and
   commit them; real-repository run. 0xLeif grades all three.
5. Closeout inside the change, verify lane, `check --commit`, push, CI, 0xLeif's
   review, native review and finalize by one session only.

## Risks

- Kimi quota or trust dialog stops a run: confirm before each run and keep a stopped
  attempt as not-run.
- The step teaches to the rubric: addressed by reading P4 to P6 and the
  real-repository run beside P1 to P3, and saying what a pass cannot show.
- Parallel sessions closing out the same change: one session owns closeout.

## Constraints consulted

specs/cli/context.md lessons: cold-start change run (the hypothesis), record bounds
in criteria, closeout order, `.chough/` stays out of affected paths, confirm the
external agent can start. AGENTS.md: no recorded or inferred human approval, review
or grade.
