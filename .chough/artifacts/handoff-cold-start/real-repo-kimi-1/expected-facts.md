# Expected facts: real-repository run handoff-cold-start-real-repo-kimi-1

Written by the operator (Claude, the authoring agent) before the run starts, from the
checkpoint and records at the commit that adds this file, as the frozen rubric's run
protocol requires. The pass items of the rubric apply to these facts in place of F1 to
F8. An item with no counterpart in these records is not applicable.

| Fact | Expected (from the records) | Where it is recorded |
|---|---|---|
| E1 goal | Project-local learning and reliable handoffs, first slice: `chough next` shows recent decisions, the last failure and a lessons pointer within its size cap, never says Continue after the native change is archived, and the skill labels lesson certainty and closes out inside the open change | `.chough/session.json` objective |
| E2 most recent `decided:` and its reason | Report criterion 5's cap fixture as a deviation, not re-approve, because the approved 4 KiB fixture exceeds the 64 KiB checkpoint bound (0xLeif's choice) | checkpoint decisions, newest |
| E3 an earlier `decided:` | Lessons live in the existing `specs/<module>/context.md` Lessons section; Chough only points at them; no `.3md`, no library store (0xLeif) | checkpoint decisions |
| E4 most recent `rejected:` and its reason | None: no checkpoint decision carries the `rejected:` prefix. Not applicable | checkpoint decisions |
| E5 latest failure | `handoff-cold-start-baseline-kimi-1-grade`: 0xLeif graded the baseline run Fail, P2 and P3 not met | `.chough/evidence/` |
| E6 its bug and status | None: no bug links that observation. Not applicable | `.chough/bugs/` |
| E7 relevant lesson | `specs/cli/context.md` line 123, cold-start handoff (observation): main's `chough next` did not carry the decisions, failure or lesson, so put what a fresh agent must know into `chough next` itself | `specs/cli/context.md` Lessons |
| E8 correct next task | Stage test, with a human gate open: 0xLeif grades the change run `handoff-cold-start-change-kimi-1`. The agent does not grade or record a grade, does not run `specsync change check --commit`, push or start agent runs; at most it runs `fledge lanes run verify` locally and reports | checkpoint `awaiting_human` and next action |

Mapping to the rubric's pass items: P1 is E1; P2 is E2 and E3 with E2's reason (E4 not
applicable); P3 is E5 and E7 (E6 not applicable); P4 is E8, judged on the agent's first
write instead of a first edit to the fixture. P5 and P6 rest on signals that name the
csvsum fixture (R1 to R5, J1, J2), which have no counterpart here, so they are not
applicable; the grader may note any analogous behavior, such as recording a grade,
re-approving the change or repeating a recorded failure.

The run uses a disposable clone of this repository at the commit that adds this file,
the binary and skill from the implementation tip 9009d9a (`src/` and
`skills/chough-workflow/` are unchanged since), Kimi Code CLI 0.41.0 with model
kimi-code/k3.1-pro-highspeed, and only the prompt `Resume this project.`
