# Lesson bundle: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: SpecSync 6.4.0: change ready asks the approval dry run, and spec history is recorded in fragments
- **Kind**: Migration
- **Specs**: cli
- **Paths**: .github/workflows/ci.yml, .github/scripts/check-specsync-archive.py, .github/scripts/test-specsync-archive.py, .specsync/config.toml, .specsync/sdd.json, .gitattributes, AGENTS.md, README.md, docs/ci.md, docs/cli.md, docs/integrations.md, docs/walkthrough.md, src/change.rs, src/ready.rs, tests/ready.rs, tests/package.rs, tests/change.rs, specs/cli/requirements.md, specs/cli/history/, changelog.d/, .chough/cases/cli.specsync-6-4.cases.toml, .chough/cases/cli.specsync-merge-driver.cases.toml, .chough/cases/cli.change-ready.cases.toml, skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/lifecycle.md, docs/exercises/feature-bug-loop.md, tests/fixtures/skill-releases/manifest.json, tests/skill_releases.rs
- **Acceptance**: The criteria in deltas/cli.md (REQ-cli-067): ready names whatever SpecSync's approval dry run refuses, drops its delta-set rule, leaves SpecSync's own untracked files and another active change's paths out, and touches no .specsync/hashes.json; the pins name 6.4.0 and the spec merge line is gone.

## Evidence

- Verification commit: `7eb2db10031f5d2b254b117103f2a2986e40eddc`
- Base commit: `de49485fc5fb93f05bb15db7cd471023beebd8a8`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Three issues, one move to SpecSync 6.4.0, drafted on its first release candidate,
v6.4.0-rc.1, so it is ready when 6.4.0 publishes.

- #404: adopt the next SpecSync. Move the pin; drop the `specs/**/*.spec.md
  merge=specsync` line, its docs and the advice to rebuild a spec's Change Log by hand,
  since 6.4.0 records each change in its own history fragment
  (`specs/<module>/history/<change-id>.md`) instead of a Change Log row and a `version:`
  bump. Its item 2, accepting a delta SpecSync rebased (spec-sync #1089), waits:
  #1089 is still an open pull request and not in the candidate.
- #406, the rest of it: `chough change ready` says "Nothing is missing." for drafts that
  `specsync change approve` then refuses (25 delta shapes, a delta with no SHALL, a
  declared spec with no spec file, an accepted change, an approval refusal ready does not
  know). Leif's decision, "Ask SpecSync itself": ready calls SpecSync's read-only dry run
  of the approval (spec-sync #1091, in 6.4.0) and does not copy its rules. Also from
  #406: SpecSync's own untracked files, and the work of another open change on the same
  branch, are named as outside the declared paths.
- #307, folded into #406 by Leif's decision of 2026-11-19: an affected path no declared
  module owns, which the dry run's `ownership` check names.

Leif also chose to drop `change ready`'s handling of `.specsync/hashes.json` (making the
file before `specsync check` runs and restoring it afterwards).

Constraints: every machine that runs change commands in this repository uses the same
SpecSync, because 6.3.0's `change check` on a change 6.4.0 recorded adds a Change Log row
that 6.4.0 then refuses as a double record (6.4.0's own release notes).

## From the change's design.md

# Design

1. **The pin, 6.3.0 to 6.4.0, everywhere Chough names it**: the five SpecSync Action
   steps in `.github/workflows/ci.yml` (`version:` and the Action commit), the version the
   archive check and its test expect (`.github/scripts/`), `.specsync/config.toml`'s
   header, `AGENTS.md`, the README's install line and its Rust note, `docs/ci.md`,
   `docs/cli.md`, `docs/integrations.md`, `docs/walkthrough.md`, and the install hint in
   `src/change.rs` and `src/ready.rs`. The Action commit is the candidate's until 6.4.0
   publishes, then the published tag's: a swap of one commit hash, before review.
2. **Ready asks SpecSync's dry run** (`src/ready.rs`). It runs
   `specsync --json change approve <id> --dry-run`, with no `--actor`. Each refused check
   is a finding, SpecSync's message shown as given with control characters escaped; its
   fix is SpecSync's `fix` when it gives one, shown and never run, otherwise
   `chough change ready <id>` once the message's problem is fixed. A skipped check is not
   a finding: it waits on a refused one, which is. A `specsync` that cannot start, or
   prints no dry-run document, gives one finding naming
   `cargo install --locked specsync@6.4.0`, never "Nothing is missing.".
3. **The delta-set rule goes** (Q1, A): the dry run's `deltas` refusal names the same
   missing and extra deltas. The stale-text, exports, cases, paths, interview and
   artifact rules stay: they are Chough's own checks, or list every item where the dry
   run gives each check's first problem.
4. **Paths outside the declaration** leave out SpecSync's own untracked
   `.specsync/change.lock` and `.specsync/workflow-v2-baseline.json`, and a path an
   affected path of another active change covers (Q2, A).
5. **`.specsync/hashes.json`**: ready no longer makes or restores it. `Cache`,
   `make_cache`, `restore_cache` and `open_cache` go, with their stderr line; the exports
   check just runs `specsync check --spec <module> --format json`.
6. **History fragments**: `.gitattributes` loses `specs/**/*.spec.md merge=specsync`;
   `specs/**/requirements.md` keeps SpecSync's driver. `docs/cli.md` and `AGENTS.md` drop
   the advice to rebuild a spec's Change Log by hand. `.specsync/sdd.json` loses the
   inert `specs/` entry.

## Open questions

Q1. Ready's own delta-set rule: **(A) drop it (recommended)**, since the dry run names
the same missing and extra deltas, one rule fewer, though in one SpecSync line with
`chough change ready <id>` as the fix rather than one `write` or `remove` command per
file; or **(B) keep it beside the dry run**, with per-file fixes, each such delta then
named twice.

Q2. Another active change's work on the same branch (#406's run on 66 real changes found
hundreds of such lines per change): **(A) leave out a path that another active change's
affected path covers (recommended)**, one exception in the paths rule; or **(B) leave it
for a later batch**.

## From the change's testing.md

# Testing

Red first, with a `specsync` stand-in that prints a dry-run document:

- `tests/ready.rs`: a refused `ownership` check with a `fix` is named in SpecSync's words
  with that fix; a refused `deltas` check without one names `chough change ready <id>`; an
  accepting dry run adds nothing; a stand-in with no dry-run document names the approval
  unchecked with `specsync@6.4.0`; a missing delta is no longer a Chough finding of its
  own; untracked `.specsync/change.lock` and `.specsync/workflow-v2-baseline.json` are
  not named while another untracked `.specsync/` file is; another active change's path
  is not named; no command the stand-in runs finds `.specsync/hashes.json`.
- `tests/package.rs`: `.gitattributes` routes `specs/**/requirements.md`, not
  `specs/**/*.spec.md`, to SpecSync's driver; `AGENTS.md` gives no Change Log
  renumbering advice; every pin names 6.4.0.
- With the real candidate: the drafts of #406's 25 shapes, as the tester ran them on
  #311, each named by ready.
- Cases in `.chough/cases/cli.specsync-6-4.cases.toml`; the full lane; the tester's run
  on the 66 real changes.

## Where these lessons go

- `specs/cli/cli.3md`
