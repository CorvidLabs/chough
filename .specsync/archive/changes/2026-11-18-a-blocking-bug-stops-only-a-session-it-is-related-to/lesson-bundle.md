# Lesson bundle: a-blocking-bug-stops-only-a-session-it-is-related-to

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: A blocking bug stops only a session it is related to
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/bugs.rs, src/stage.rs, src/main.rs, src/skill.rs, tests/bugs.rs, tests/advance_to.rs, tests/stages.rs, tests/next.rs, tests/skill_releases.rs, tests/fixtures/skill-releases/manifest.json, specs/cli/requirements.md, skills/chough-workflow/references/stages.md, skills/chough-workflow/references/bugs.md, .agents/skills/chough-workflow/references/stages.md, .agents/skills/chough-workflow/references/bugs.md, docs/FRD.md, docs/cli.md, docs/lifecycle.md, docs/diagrams/stages.mmd, docs/diagrams/stages.svg, changelog.d/2026-11-18-blocking-bug-session.md
- **Acceptance**: An open blocking bug stops leaving test only for a session that lists it, has its evidence prefixed with that session id, or shares a criterion, requirement, or case. Another session advances, and chough next names the bug and says it does not block this session. A blocking bug that names nothing stops no session. A related blocking bug still refuses with refused:blocking-bugs. Closing it releases that session.

## Evidence

- Verification commit: `c08a9e3360abd993ba10932f35d0ef8ae78025d3`
- Base commit: `8abb5bb89b5244e72327e1ea6abfc7e66634ebe1`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Jackdaw PR 184 could not move from test to accept. The refusal was refused:blocking-bugs for two Web-performance bugs filed by other people. Those bugs were not on the session's bugs list and shared no criterion, requirement, or case with it. One of them names no criterion, requirement, case, or evidence.

On main 8abb5bb, open_blocking returns every open bug whose scope is blocking. forward_step and assess both use that list. REQ-cli-054 said an open blocking bug stops leaving test and did not say which session.

Ruled out: keeping the repo-wide stop, stopping every session until a nameless bug is linked, matching on module or native change (a bug has neither field), and adding a fourth scope.

## From the change's design.md

# Design

Both call sites read one split of the open blocking bugs. A bug stops the session when the session lists it, its evidence contains a link starting with that session's id, or it shares a criterion, requirement, or case with the session. Every other open blocking bug is named on chough next and does not stop the session. A bug that names nothing is in that second set.

The refusal text for a related bug stays refused:blocking-bugs. Skip and advance --to use the same forward step as advance.

## From the change's testing.md

# Testing

cargo test --locked --test bugs, --test advance_to, --test skill_releases, the stages test a_linking_bug_lifts_the_unlinked_failure_refusal_and_other_moves_are_unchanged, and the next test both_forms_stay_within_the_cap_with_totals_for_large_records. All of those passed.

The new test a_blocking_bug_stops_only_a_session_it_is_related_to failed on the previous behavior, then passed. It covers a bug that names nothing, a bug listed on one session, and a bug that shares a requirement with one session.

## Where these lessons go

- `specs/cli/cli.3md`
