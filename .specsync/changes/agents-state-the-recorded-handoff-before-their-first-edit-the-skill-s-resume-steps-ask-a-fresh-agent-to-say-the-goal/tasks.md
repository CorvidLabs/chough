---
change: agents-state-the-recorded-handoff-before-their-first-edit-the-skill-s-resume-steps-ask-a-fresh-agent-to-say-the-goal
artifact: tasks
---

# Tasks

Before implementation:

- [x] Check the SHA-256 digests of docs/exercises/handoff-cold-start/seed.sh and rubric.md against observation handoff-cold-start-frozen-pair-1.
- [x] Not run by 0xLeif's decision ("Just go without Kimi"): confirm Kimi Code CLI 0.41.0 starts through rune and has quota; record the version.
- [x] Build the binary at main 118a895, seed a fresh repository, install its skill and prepare the baseline run script for 0xLeif (done: binary sha256 27407ca7..., seeded HEAD c1b7e74b, tree 9be61b11); the run itself was not run by 0xLeif's decision ("Just go without Kimi"), so no TC-cli-0133 observation exists for it.

Human gates before implementation (not tasks): 0xLeif's scope approval of this
change; 0xLeif starts the baseline run and grades it against the frozen rubric.

Implementation:

- [x] Add a failing test in tests/package.rs: the 118a895 SKILL.md bytes (inline) read known-outdated and update; an edited copy stays edited; the canonical SKILL.md resume list contains the handoff step; record the failing run.
- [x] Add the step to skills/chough-workflow/SKILL.md as the design shows.
- [x] Add the 118a895 SKILL.md digest to KNOWN_PREVIOUS in src/skill.rs.
- [x] Refresh .agents/ and .claude/ copies with chough skill update and confirm byte identity.
- [x] Add the CHANGELOG.md Unreleased entry.
- [x] Run the closeout pass inside this change: labelled lessons into specs/cli/context.md, one completed checkpoint entry, one summary of at most 12 lines.

Human gates and steps (not tasks), in order: the scoped `specsync change check` that
materializes the delta; adding `TC-cli-skill-asks-for-the-handoff-before-the-first-edit`
as active and recording its reviewer note; the change run at the implementation tip;
writing and committing the real-repository expected facts; the real-repository run
(each run records its commit, and a change to `src/` or `skills/chough-workflow/`
after a run voids it until it is rerun); 0xLeif grading both runs; `fledge lanes run
verify`; commit with `.chough/`; `specsync change check --commit`; push and CI
(Ubuntu, macOS, Windows, MSRV); 0xLeif's implementation review; native review and
finalize, by one session only.
