# Lesson bundle: docs-387-no-git-while-agents-write

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Docs 387 no git while agents write
- **Kind**: Documentation
- **Specs**: cli
- **Paths**: docs/cli.md, skills/chough-workflow/references/checkpoint.md, .agents/skills/chough-workflow/references/checkpoint.md, src/skill.rs, tests/fixtures/skill-releases/manifest.json, tests/skill_releases.rs, tests/docs.rs, changelog.d/, specs/cli/requirements.md, specs/cli/cli.spec.md, .chough/cases/cli.no-git-while-agents-write.cases.toml
- **Acceptance**: The workflow skill's checkpoint reference, in the bundle and in an installed copy, and `docs/cli.md` each say not to run `git pull`, `git merge`, `git rebase`, `git checkout` or `git stash` in a worktree while another agent works there, why, and that a lone agent may run one between its own commands.

## Evidence

- Verification commit: `1b1ab5c124c9f3a26ae244dba1351505474d44b7`
- Base commit: `9d260310bca8553a0203e5ef90bc8372b4e15aae`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

#387 (0.6.2 candidate swarm s3, 2026-11-18): a `git merge` started while Chough
writers ran dropped writes that had already exited 0. In 12 of 20 rounds, six
parallel `checkpoint --decision` plus `git merge side` lost one to five decisions with
nothing on standard error, even when Git refused ("would be overwritten"). A plain
flock-and-rename appender loses them too, and the guard of #330 cannot see a merge
before `MERGE_HEAD` exists. This is a Git race, not Chough's lock: the writer lock
orders Chough's own writers, and Git does not take it.

So this change is guidance only: the skill and the docs say it. Doctor noting it was
raised in the issue and is left out (it is code, and a Git command that has not
started yet leaves nothing for doctor to see).

Changing the skill's checkpoint reference needs its 0.6.1 digest in
`KNOWN_PREVIOUS` (`src/skill.rs`), so a 0.6.1 install updates; that is why the cli
module and `src/skill.rs` are declared.

## From the change's design.md

# Design

Nothing visual changes. Two short paragraphs: one in
`skills/chough-workflow/references/checkpoint.md` (and its installed copy under
`.agents/`), beside "With several agents, one agent owns live checkpoint writes", and
one in `docs/cli.md` after the writer-lock section.

## Where these lessons go

- `specs/cli/cli.3md`
