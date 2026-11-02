---
change: fresh-agent-handoff-from-existing-records-chough-next-shows-recent-decisions-the-last-failure-and-a-lessons-pointer
artifact: tasks
---

# Tasks

Before implementation:

- [x] Write docs/exercises/handoff-cold-start/seed.sh (static files, local Git identity) and rubric.md (pass items and disqualifiers from the acceptance criteria).
- [x] Run the seed once into a scratch directory and confirm the repository it builds matches the rubric's setup.
- [x] Record one observation with seed.sh and rubric.md as artifacts so their SHA-256 digests are bound; commit before any implementation commit.
- [x] Confirm the external host starts (Kimi Code CLI 0.41.0, else Codex CLI 0.153.4) and record its version.
- [x] Build the main binary at fb6e487, seed a fresh repository, install its skill, run the fixed prompt `Resume this project.` once, and record the transcript and command log as observation artifacts (the mandatory baseline).

Human gates before implementation (not tasks), in order: after the hash observation
and before the baseline run, 0xLeif reads the frozen rubric (recorded as his
statement, not as an approval); after the baseline run, 0xLeif grades it against the
frozen rubric.

Implementation:

- [x] Add failing tests in tests/next.rs for archived-not-done at plan, test and accept; the decisions block; the last failure block; the lessons pointer; the cap fixture; byte-identical output across runs and after modification-time changes; record the failing run.
- [x] Factor done_action's new-cycle steps into shared constants and add the archived-not-done rule in src/stage.rs.
- [x] Add the decisions, last_failure and lessons fields and text lines to the next view in src/main.rs, and add the new lists to LISTS.
- [x] Add any bounded reader needed in src/records.rs, src/evidence.rs or src/bugs.rs as pub(super).
- [x] Update references/accept.md, checkpoint.md, bugs.md, librarian.md and lifecycle.md (and SKILL.md only if needed) as the design says.
- [x] Add the fb6e487 digests of every edited bundle file to KNOWN_PREVIOUS, add tests/fixtures/skill-fb6e487/ and the known-outdated test in tests/package.rs.
- [x] Refresh .agents/ and .claude/ copies with chough skill update and confirm byte identity.
- [x] Normalize the canonical REQ-cli-002 heading to exactly `### REQ-cli-002` after approval, as requirements.md shows.
- [x] Update docs/cli.md, CHANGELOG.md (Unreleased) and docs/ROADMAP.md (only to list this slice).
- [x] Run the closeout pass inside this change: labelled lessons into specs/cli/context.md, one completed checkpoint entry, one summary of at most 12 lines.

Human gates and steps (not tasks), in order: 0xLeif's scope approval of this change,
before any implementation commit; normalizing the canonical REQ-cli-002 heading (the
task above), which must land before the next step because the MODIFIED item applies
only to an exact `### REQ-cli-002` heading and neither status nor audit reports the
mismatch; the scoped `specsync change check` that materializes the delta; adding
TC-cli-0126 to TC-cli-0133 to `.chough/cases/cli.cases.toml` as active and running the
automated ones (a step, not a checkbox, because `change check` refuses while a box is
unchecked); the change run on the same host and model with the
implementation binary; one run on this repository at the implementation tip (each run
records its commit SHA, and a change to `src/` or `skills/chough-workflow/` after a run
voids it until it is rerun at the new tip); 0xLeif grading both runs against the frozen rubric; `fledge lanes run verify` locally; commit
with `.chough/`; `specsync change check --commit`; push and CI (Ubuntu, macOS,
Windows, MSRV); 0xLeif's implementation review; native review and finalize.
