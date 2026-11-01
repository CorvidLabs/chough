---
change: fresh-agent-handoff-from-existing-records-chough-next-shows-recent-decisions-the-last-failure-and-a-lessons-pointer
artifact: testing
---

# Testing

## Cases

Case IDs TC-cli-0126 to TC-cli-0133 are reserved for this change (TC-cli-0125 is
claimed by PR #26). The case profile allows only `active` or `retired`, so they are not
added to `.chough/cases/cli.cases.toml` as proposed; they are added as `active` after
the scoped `change check` materializes the delta, each with an observation at the
implementation tip. Test names are the planned names.

| Case | Requirement | Criteria | Method | What it shows |
|---|---|---|---|---|
| TC-cli-0126 | REQ-cli-021 | FLOW-13, LIFE-3 | automated, `tests/next.rs::an_archived_change_before_done_names_a_new_cycle_not_continue` | At plan, test and accept with an archived stub change whose terminal evidence authenticates, `next` text, `next --json` and `status` give the new-cycle action (accept names `chough stage advance` first), never `Continue <stage>`; the suggested reset with placeholders filled succeeds; paused, awaiting-human, blocking-bug and unauthenticated-archive fixtures keep today's action |
| TC-cli-0127 | REQ-cli-021 | LIFE-3, FLOW-13 | automated, `tests/next.rs::next_shows_the_five_newest_decisions_with_kind_and_reason` | 7 decisions (`decided:`, `rejected:`, `failed:`, unprefixed, dated, 4 KiB, last with two ` because `) give exactly the last 5 newest first, the stated kinds, the reason after the last ` because `, null kind and reason for unprefixed entries, the 512-byte cut marker, and `total` 7 |
| TC-cli-0128 | REQ-cli-021 | LIFE-3, FLOW-13 | automated, `tests/next.rs::next_names_the_latest_failed_observation_by_recorded_time_with_its_bugs` | Failed observations whose `observed_at_unix_seconds` disagree with their file names and modification times: the latest recorded wins, a tie goes to the greater ID, passed and not-run records are ignored, each linking bug shows ID and status (at most 3 with total), an unlinked failure shows no bugs, no failures shows none |
| TC-cli-0129 | REQ-cli-021, REQ-cli-002 | LIFE-1, FLOW-12 | automated, `tests/next.rs::next_points_to_the_lesson_source_and_counts_archived_bundles` | context.md with `## Lessons` gives its path and line; without the section says so; a missing file says missing; a present `<module>.3md` is preferred; 7 archived bundles give total 7 and the 5 newest paths by directory name; no lesson text appears in either form |
| TC-cli-0130 | REQ-cli-021 | LIFE-3 | automated, `tests/next.rs::handoff_blocks_stay_within_the_cap_and_are_byte_identical` | 50 decisions of 4 KiB, 200 observations (100 failed), 40 bugs linking the latest failure, 40 bundles, 16 modules (10 sources listed, total 16), plus the existing 500 stale observations, 40 gates and 40 blockers of 4 KiB: both forms at most 16384 bytes with correct totals and `truncated` true; two runs and a run after changing every record's modification time are byte-identical; a counting stub shows one native read |
| TC-cli-0131 | REQ-cli-016, REQ-cli-019 | LIFE-1 | automated, `tests/package.rs::a_skill_copy_from_main_before_the_learning_handoff_is_known_outdated_and_updated` | fb6e487 bytes of each edited bundle file read known-outdated and update to current; an edited copy stays edited; the existing no-dash test passes |
| TC-cli-0132 | REQ-cli-002, REQ-cli-015 | FLOW-12, LIFE-1 | agent-assisted, reviewer-note | At the implementation tip each skill statement in the acceptance criteria is present: certainty labels with source and limits; decision outranks lesson; same-PR failure gets a bug or lesson; closeout inside the open change before `check --commit`, at most 12 summary lines, never edits specs after archive, works with the librarian off; the `verification_ignored_paths` wording in lifecycle.md and accept.md; `chough skill status` reports both host copies current |
| TC-cli-0133 | REQ-cli-021 | FLOW-13, LIFE-3, LIFE-1 | manual, transcript | The cold-start protocol below, graded by 0xLeif |

TC-cli-0131 and the REQ-cli-015 part of TC-cli-0132 are regression cases on
requirements this delta does not change (REQ-cli-016, REQ-cli-019, REQ-cli-015). They
are added with the delta cases, after the scoped check, for traceability only.

## Cold-start protocol (TC-cli-0133)

Frozen before implementation: `docs/exercises/handoff-cold-start/seed.sh` and
`rubric.md`, bound by SHA-256 in one observation. A change to either voids every
earlier run and needs a new frozen pair and observation. After the observation and
before the baseline, 0xLeif reads the frozen rubric; his reading is recorded as his
statement, not as an approval.

Setup the seed builds: a goal; two human decisions in the checkpoint, one
`rejected: <approach> because <reason>`; one failed experiment mixing a real defect
with a case binding error (failed observation, linked bug); a verified fix with a
`verified` lesson in `specs/<module>/context.md` Lessons; an unfinished next task.

Runs, each a fresh agent process on a host other than the author's (Kimi Code CLI
0.41.0, else Codex CLI 0.153.4), given only the repository, the installed skill and
the prompt `Resume this project.`:

1. Baseline (mandatory, before implementation): binary and skill from main at
   fb6e487.
2. Change: the same host, model and prompt; binary and skill from the implementation
   tip.
3. Real repository: the same host and prompt once on this repository at the
   implementation tip.

Each run's transcript and command log are observation artifacts with host, model and
version, and the observation records the exact commit SHA of the binary and skill the
run used. If `src/` or `skills/chough-workflow/` change after the change run or the
real-repository run, that run is void and is rerun at the new tip before grading. Pass, per the rubric: before its first edit the transcript states the goal,
cites both decisions and the reason for the rejection, names the failure, its bug
status and the lesson, and starts the correct next task; the command log shows no
repeat of the failed experiment and no proposal of the rejected approach.

Does not count: the same session or a fresh command process of the authoring agent;
any hint beyond the fixed prompt; recovery that needed `git log`, `gh` or
docs/ROADMAP.md; simulated actors; a grader feeding answers.

0xLeif grades every run against the frozen rubric; his grade is recorded as his
statement, not as an approval. The criterion holds when he grades the change run a
pass; the baseline and real-repository results are reported beside it whatever they
are, and a baseline pass is reported as no demonstrated difference from main. Each run
is a sample of one.

## Regression

Existing `tests/next.rs` (TC-cli-0120, TC-cli-0121, TC-cli-0122), `tests/stages.rs`
and `tests/package.rs` cases keep passing; the done-stage resume wording is unchanged.
Full `cargo test --locked`, `fledge lanes run verify` with hi 0.8.0, fledge 1.8.0 and
specsync 6.1.2 locally, and CI on Ubuntu, macOS, Windows and MSRV. The pull request
reports `chough next` and `chough next --json` byte counts on this repository.
