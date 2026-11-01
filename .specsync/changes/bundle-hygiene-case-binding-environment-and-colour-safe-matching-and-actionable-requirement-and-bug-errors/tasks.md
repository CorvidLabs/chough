---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: tasks
---

# Tasks

- [ ] Replace the em dashes in templates/frd.md and templates/cases.toml with colons.
- [ ] Add the earlier digests of every edited bundle file to KNOWN_PREVIOUS.
- [ ] Add the tests/package.rs test that the shipped bundle holds no em or en dash.
- [ ] Name the searched companions and the likely module or cause in requirement refusals, for cases and evidence add.
- [ ] Add the scope and classification hints and the --help value lists for bug add and bug update.
- [ ] Add the bounded binding `env`, pass it to the executed command, and record the declared names as a limitation.
- [ ] Match expect_output and cargo counts against text without terminal escape sequences, keep the raw artifact, and refuse fragments containing ESC.
- [ ] Add regression tests for each rule, including unchanged digests of existing cases.
- [ ] Update skills/chough-workflow/references/build.md, plan.md and testing.md, docs/cli.md and docs/architecture.md.
- [ ] Refresh the .agents/ and .claude/ skill copies with `chough skill update`.
- [ ] Add case definitions for the new behavior linked to BUG-0017, BUG-0018 and BUG-0019.

Human gates and post-verification steps (not tasks): the human's scope approval of
this change; `specsync change check` to materialize the delta; running and recording
the new cases and verifying BUG-0017 to BUG-0019; push and CI on Ubuntu, macOS and
Windows; the human implementation review; native review and ship.
