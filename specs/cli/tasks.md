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
  exposed BUG-0024 and missing mapping guidance BUG-0021. Any later TC-cli-0106
  pass is a retrospective documentation review, not another Kimi run.
- [x] Align the integrated archived-done action with REQ-cli-021. TC-cli-0105
  reproduced BUG-0025 before the override was removed and passed afterward;
  TC-cli-0121, TC-cli-0122, focused Rust tests and the local verify and pre-push
  lanes passed. BUG-0025 is verified from the later pass. Original observations
  remain retained, and no fresh Kimi run or hosted check is claimed.
- [x] Complete `use-collision-resistant-slug-identities-for-new-bugs`: implement the
  shared creation generator and immutable new-form IDs, retain schema 1 legacy
  records and cross-form links, refuse same-ID different-content creation without
  overwrite or numeric fallback, and finish the public documentation. TC-cli-0125
  retains the failed `tc-cli-0125-collision-pre-fix` and strictly later passed
  `tc-cli-0125-collision-post-fix` on `local-macos-arm64`; BUG-0026 is verified.

Implemented approved work (human acceptance pending):

- [x] Implement `use-slug-identities-for-all-new-chough-records`: curated slug
  cases, random OBS, VAL and SNAP allocation, atomic refusal, legacy reads and
  resume, inherited artifact paths, documentation and package-update coverage.
  The four identity bugs retain failed observations and strictly later passes.
- [x] Apply the separately approved compact-next fixture follow-up: use automatic
  allocation and reference the returned ID, preserving every original assertion.
  The focused next suite passed all 3 tests and the unfiltered Rust 1.88.0 suite
  passed all 182 tests. Keep the original failed TC-cli-0120 observation and a later
  passing case record. Full verification and pre-push results live in the Chough
  evidence records; human implementation review and finalization remain gates.

Completed approved work:

- [x] Complete `make-checkpoint-merges-safe-across-parallel-changes`: bounded schema 2
  checkpoint merges, real Git driver coverage, documentation and safe skill updates
  are implemented. `checkpoint-merge-real-git-pre-fix` is retained before
  implementation, and the strictly later `checkpoint-merge-real-git-post-fix` passed
  on `local-macos-arm64` and verified the linked local bug. The full verify lane
  passed 9 steps in 221.890 seconds, the full Rust suite passed with Rust 1.88.0,
  and pre-push passed 5 steps in 5.422 seconds.

Open follow-ups, not part of the approved change:

- [ ] Exercise discovery and cold starts on Cursor, Gemini and Grok.
- [ ] Use the workflow on a real CorvidLabs project feature and bug.
