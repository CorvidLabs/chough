---
change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
artifact: tasks
---

# Tasks

- [ ] Add the failing test `measure_report_pairs_two_binaries_on_one_root_and_keeps_native_wait_apart` (tests/timing.rs), then write examples/measure-report.sh and name it in docs/cli.md's profiling procedure.
- [ ] REQ-cli-002 (tests/cli.rs): the canonical and installed accept.md state the three certainty labels, source and limits, and that a project decision always outranks a lesson. A unique lesson sentence in a module's `## Lessons` and in an archive's lesson bundle appears in no file under `.chough/` or `knowledge/` after init, checkpoint, next, status, library index and a done move. The module note links the source with a count, and `next` names the source without printing the sentence.
- [ ] REQ-cli-007 (tests/stages.rs): with a `bin/aps` stand-in that records any call, init, checkpoint, stage moves, next, status and session new and use all succeed and never call it. Only the canonical checkpoint (or each session file in sessions mode) is a schema-2 checkpoint, and a `checkpoint` call changes only the active one.
- [ ] REQ-cli-012 (tests/bugs.rs): docs/cli.md tells users of older Chough versions that accept only `BUG-NNNN` to upgrade for new-form IDs.
- [ ] REQ-cli-029 authority (tests/workflow_authority.rs): plan.md records source, scope and limits before its `specsync change approve` example, asks when authority is missing or exceeded, and says an actor label alone never supplies authority. lifecycle.md says an agent label, bot review or passing check creates no human approval. With the native change draft, a checkpoint decision and an inbox decision claiming approval leave `stage advance` to build refused on the native gate, and the stub receives no `approve` or `review`.
- [ ] REQ-cli-029 cold start (tests/cases.rs, tests/workflow_authority.rs): `cases record` with a verdict missing for an expected outcome is refused and writes nothing. A cold-start case observed only as `not-run` reads `not-run` in status, never `passed`. The ROADMAP rows for #35/#36 and #44 claim no passing cold-start result.
- [ ] REQ-cli-031 (tests/cli.rs): docs/walkthrough.md's first section is "New project", and before the next heading it covers a minimal HI file, `specsync init`, `specsync scaffold`, `specsync change adopt` and `ungated:sdd-disabled`.
- [ ] REQ-cli-045 (tests/reduce.rs): with the librarian off by a recorded preference, the done move reports `checkpoint_references` `skipped`, and the observation, file and design references are unchanged. With no reset recorded, references to an observation from 65 days ago and one from 10 days ago both stay, and with no file or design reference the step is `skipped` with the no-reset reason.
- [ ] Add a CHANGELOG line.

After the tests pass, not a task box: apply and record one targeted mutation per new assertion, record the cases, and run `fledge lanes run verify` on Linux.
