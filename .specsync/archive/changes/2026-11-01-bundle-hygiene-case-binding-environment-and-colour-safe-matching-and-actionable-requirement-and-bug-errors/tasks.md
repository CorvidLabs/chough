---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: tasks
---

# Tasks

- [x] Before any fix, add the regression tests and the case definitions for BUG-0017 (error half), BUG-0018 and BUG-0019, run each case at the unfixed tip, confirm it records failed, and link that failed observation to its bug.
- [x] Replace the em dashes in templates/frd.md and templates/cases.toml with colons.
- [x] Add the 2756dc2 digests of templates/frd.md, templates/cases.toml and references/testing.md to KNOWN_PREVIOUS.
- [x] Add the tests/package.rs test that the shipped bundle holds no em or en dash.
- [x] Name the searched companions and the likely module or cause in requirement refusals, for cases and evidence add.
- [x] Add the scope and classification hints and the --help value lists for bug add and bug update.
- [x] Add the bounded binding `env` (name length, case-insensitive duplicates, refused cargo-test names), pass it to the executed command, and record the standing inherited-environment limitation and the declared names.
- [x] Match expect_output and cargo counts against text without complete terminal escape sequences, never removing a line break, keep the raw artifact, and refuse fragments containing ESC.
- [x] Add regression tests for each rule, including unchanged digests of existing cases and the `test result:` line-count property.
- [x] Update skills/chough-workflow/references/testing.md, docs/cli.md and docs/architecture.md.
- [x] Refresh the .agents/ and .claude/ skill copies with `chough skill update`.

Human gates and post-verification steps (not tasks): the human's scope approval of
this change; `specsync change check` to materialize the delta; rerunning the unchanged
cases for BUG-0017 to BUG-0019 so each records a pass observed after its linked
failure, linking each pass, setting BUG-0018 and BUG-0019 verified, and adding a note
that keeps BUG-0017 open for the evidence-freshness change; `fledge lanes run
pre-push`; push and CI on Ubuntu, macOS and Windows; the human implementation review;
native review and ship.
