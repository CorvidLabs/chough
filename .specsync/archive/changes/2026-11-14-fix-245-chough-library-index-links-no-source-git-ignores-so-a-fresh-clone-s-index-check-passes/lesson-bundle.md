# Lesson bundle: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/vault.rs, tests/vault.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: chough library index links no rule, skill, decision or policy file that Git ignores, so after chough init --host grok in a repository that ignores .grok, the index links nothing under .grok/ and library index --check passes in the working tree and in a fresh clone (red then green)

## Evidence

- Verification commit: `d8cbdb39c52643cfa4863a721de2ac85221dce26`
- Base commit: `85304969e053e7477dff11d2ee53b6dddcb6a070`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

corvid-agent, adopting Chough 0.6.0 in CorvidLabs/peck, found that `chough init --host
grok` writes the skill to `.grok/skills/chough-workflow/`, which that project's
`.gitignore` ignores. `knowledge/index.md` then links that file, which is never
committed, so in a fresh clone or in CI `chough library index --check` fails. That
breaks REQ-cli-044's rule that links resolve. Filed as GitHub #245. t37f reproduced it
(red at aa652cac): `src/vault.rs`'s `rule_files()` links every host's
`<dir>/chough-workflow/SKILL.md` that exists on disk, without asking Git.

On 2026-11-13T14:12Z Leif approved the scope ("Approve (Recommended)", relayed by t370):
the index stops linking skill copies Git ignores. The issue's second ask, a warning
from `chough init` when the host's skill directory is ignored, will come later as its
own change.

## From the change's design.md

# Design

`rule_files()` in `src/vault.rs` drops the paths Git ignores, using one batched
`git check-ignore --stdin -z`. Outside a Git repository nothing is dropped. The index
then lists the same sources in a working tree and in a fresh clone. REQ-cli-043 says
the index links the rule, skill, decision and policy files that exist and that Git
doesn't ignore. `docs/cli.md`'s index row and the CHANGELOG say so.

## From the change's testing.md

# Testing

`tests/vault.rs`: `git init`, a `.gitignore` that ignores `.grok`, a commit,
`chough init --host grok`, another commit, `git clone`, then `chough library index
--check` in the clone. The test also expects the working tree's index to link nothing
under `.grok/`, and `--check` to pass there with the ignored copy present and with it
removed. Red at the test commit, with the issue's exact error. Green at the fix. The
full local suite runs at the final head.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-043 | `tests/vault.rs`: the index links no source Git ignores, and a fresh clone's check passes |

## Where these lessons go

- `specs/cli/cli.3md`
