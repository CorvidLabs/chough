# CLI tasks

Requirements are durable; completing a task does not mark a human criterion
achieved. The bootstrap tasks are history; the evolution's task list lives in the
native change package until it is archived.

- [x] Bootstrap: helper, regression tests, Fledge verify, initial private push and
  hosted CI (observed at run 41218954725 for d38aa19).
- [x] Evolution (evolve-chough-into-a-resumable-stage-workflow): checkpoint schema 2,
  stages and gates, test definitions and execution, local bugs, preferences and
  validation, release routing, librarian, host skills, docs, diagrams and exercises.
- [x] Record one Kimi Code CLI 0.41.0 macOS arm64 run on 2026-11-01 in the
  [cold-start exercise](../../docs/exercises/cold-start.md) and
  [concise log](../../.chough/artifacts/kimi-cold-start.log), with
  [TC-cli-0106](testing.md) review. Discovery passed, while safe cold resume
  exposed BUG-0020 and missing mapping guidance BUG-0021. Any later TC-cli-0106
  pass is a retrospective documentation review, not another Kimi run.

Open follow-ups, not part of the approved change:

- [ ] Exercise discovery and cold starts on Cursor, Gemini and Grok.
- [ ] Use the workflow on a real CorvidLabs project feature and bug.
