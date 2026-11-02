---
change: agents-state-the-recorded-handoff-before-their-first-edit-the-skill-s-resume-steps-ask-a-fresh-agent-to-say-the-goal
artifact: testing
---

# Testing

## Cases

| Case | Requirement | Criteria | Method | What it shows |
|---|---|---|---|---|
| TC-cli-skill-asks-for-the-handoff-before-the-first-edit | REQ-cli-002 | LIFE-3, FLOW-13, LIFE-1, FLOW-12 | agent-assisted, reviewer-note | At the implementation tip SKILL.md's resume list has the step between "Check what is still true" and the stage contract, naming the goal, binding decisions with reasons and rejected approaches, the last failure with its bug status, the lessons with their source, the next task, what counts as an edit and saying what the records lack; at most 8 lines, no dash, SKILL.md under 6144 bytes; `chough skill status` reports both host copies current |
| TC-cli-0133 (existing) | REQ-cli-021 | FLOW-13, LIFE-3, LIFE-1 | manual, transcript | New observations for the baseline at 118a895 and the change run at the tip, graded by 0xLeif against the frozen rubric |

The package test (automated, in the full suite) covers the known-outdated update of
the 118a895 copy and the step's presence. The new case is added after the scoped
check materializes the delta.

## Cold-start protocol

Unchanged from the learning change: frozen seed and rubric, Kimi Code CLI 0.41.0,
model kimi-code/k3.1-pro-highspeed, prompt `Resume this project.`, driven through
rune, trust dialog accepted as recorded host setup. The real-repository run uses
expected facts for this repository written and committed before it starts, in the
style of the earlier real-repo-kimi-1/expected-facts.md. Grades are 0xLeif's
statements, recorded as such.

## Regression

The full suite, `chough cases check` and the verify lane; `chough next` output on this
repository is compared before and after to show it did not change.
