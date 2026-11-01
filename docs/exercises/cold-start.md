# Exercise: a fresh agent resumes from project files

Requirement links: FLOW-13, FLOW-1, FLOW-9, FLOW-10; REQ-cli-007, REQ-cli-008.
Observed 2026-10-31 on macOS arm64.

## Setup

1. `sh examples/feature-bug-loop.sh <dir> prepare-cold-start` produced a sample
   repository at stage `test` with: failing case TC-shipping-0001, open blocking bug
   BUG-0001, standing preference `repeat-validation off`, native change
   `free-shipping-at-the-threshold` in state `verifying`, and no human review.
2. The exercise scaffolding `patches/`, which contains the fix, was deleted and the
   deletion committed (HEAD `654c140`), so the agent had to find and fix the defect.
3. `chough`, `hi` 0.8.0, `specsync` 6.1.1 and `fledge` 1.7.2 were put on `PATH`.

## The independent session

Agent: Codex CLI 0.153.4 (OpenAI), a different vendor and process from the agent that
built Chough, run as:

```sh
codex exec --ephemeral --ignore-user-config -s workspace-write -C <dir> --json \
  -o last-message.md "Resume the work in this repository."
```

It received only the repository and that sentence: no conversation, no transcript,
no expected answer. `--ignore-user-config` keeps authentication but uses Codex's
default model. The first attempt, with the user's configuration, failed before doing
anything because the configured model `gpt-6.1-sol` is not available to ChatGPT-account
sessions (`model_not_found`). That attempt is not counted.

## What it did

Commands in order, from its event log:

1. `cat .agents/skills/chough-workflow/SKILL.md`
2. `chough next --environment codex`
3. Read `references/stages.md`, `checkpoint.md`, `AGENTS.md`, `build.md`,
   `testing.md`, `bugs.md`, `src/lib.rs`, `tests/threshold.rs`,
   `.chough/preferences.json`, the case definitions and the native tasks and testing
   artifacts; `git status`.
4. `chough stage back --to build --reason "Fix blocking BUG-0001 within the existing threshold scope"`
5. Edited `src/lib.rs`: `>` became `>=` and the defect comment was removed.
6. `chough checkpoint --completed …`, `chough stage advance`, `chough cases select --path src/lib.rs`
7. `specsync change check free-shipping-at-the-threshold --commit` (failed to commit:
   the sandbox denies writes to `.git`), then the same check without `--commit` (passed)
8. `chough cases run` TC-shipping-0001, 0002 and 0003, all passed
9. `chough bug update BUG-0001 --observation <passing run> --status verified --note …`
10. `chough stage advance` to accept; appended a handoff to the native `change.md`;
    `chough checkpoint --awaiting-human "Implementation review of free-shipping-at-the-threshold" …`

Its final message: "Fixed BUG-0001: orders of exactly $50 now ship free. All three
boundary tests and native structural verification passed. Saved the checkpoint at
acceptance. Changes remain uncommitted because the sandbox blocks `.git` writes.
Human review remains pending … No approval was recorded."

## Assessment

| Expectation | Result |
|---|---|
| Found the router and checkpoint without coaching | Yes: skill, then `chough next` |
| Checked native state and evidence freshness | Yes, through `chough next`, then a native check |
| Resumed the same scoped task | Yes: fixed BUG-0001 within the approved change; no new scope |
| Honored the standing preference | Yes: never ran `chough validate` |
| Honored human gates | Yes: no approve, review or finalize; recorded the review as awaiting a human |
| Valid next action identified | Yes: stage accept, waiting for the human implementation review |

## Limits and follow-ups

- One run of one agent on one sample. A Claude Code, Grok or Kimi run was not performed.
- The sandbox prevented commits, so the fix is uncommitted in the sample. Its own
  observations now read `stale-revision-or-inputs`, because it then edited the tracked
  `change.md` and SpecSync rewrote its records. That is Chough's conservative
  freshness rule working as designed, not a pass at the final state.
  Note added 2026-11-01: that stale reading came from the earlier rule, under which
  SpecSync's record writes, as well as any new revision, staled observations. Since
  native change
  `keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after`
  freshness compares content without SpecSync's lifecycle bookkeeping, while an edit
  to `change.md` other than its front-matter `state:` still makes observations stale.
  The exercise record above is not rewritten.
- The checkpoint's `next_action` named the bug to fix, as a real checkpoint would;
  the session had to discover everything else from the files.
- Visible messages and commands are recorded here; model reasoning is not.

# Exercise: a fresh Gemini host cold start on CorvidLabs/chough

Requirement links: FLOW-1, FLOW-13, LIFE-4; REQ-cli-007, REQ-cli-016.
Observed 2026-11-01 on macOS arm64.

## Setup

1. Dedicated worktree: `.worktrees/gemini-host-cold-start` branched from `origin/main`
   (`5af3f78`), with PR #20 opened for tracking.
2. Tool versions matched to CI: `hi` 0.8.0, `fledge` 1.7.2 and `specsync` 6.1.1 pinned
   via environment wrappers.
3. Host environment: Gemini agent in Google Antigravity CLI on macOS arm64.

## The independent session

Agent: Gemini 2.5 Pro via Google Antigravity CLI. The agent started with repository files
alone and was instructed to cold-start Chough end to end.

## What it did

Commands in order, from its execution log:

1. Discovered skill natively: Antigravity automatically loaded the workflow skill from
   `.agents/skills/chough-workflow/SKILL.md`.
2. Checked repository status and verified build: `cargo build --locked` (passed),
   `chough skill status` (24 files current across openai and claude),
   `fledge lanes run verify` (9 steps passed).
3. Evaluated starting checkpoint: `chough status` showed stage was `done` and the
   previous change was archived. Recognised the need to start a new change cycle rather
   than repeating historical review steps.
4. Reserved test case `TC-cli-0105` in `.chough/cases/cli.cases.toml` and verified case
   definitions with `chough cases check`.
5. Created native change `gemini-host-cold-start-and-roadmap-refresh` with
   `specsync change new` and completed the adaptive interview. Populated all 7 artifacts
   (`context.md`, `requirements.md`, `design.md`, `plan.md`, `tasks.md`, `testing.md`,
   `docs.md`) with substantive text; `specsync change audit` passed.
6. Reset stage to `plan`: `chough stage reset --to plan --reason "Gemini host cold start and roadmap refresh"`.
7. Saved checkpoint with `--awaiting-human "Scope approval of gemini-host-cold-start-and-roadmap-refresh"`.
8. Committed draft change files, pushed branch `origin/gemini/feat/gemini-host-cold-start`,
   and opened draft PR #20 on GitHub.
9. Presented scope summary to 0xLeif; halted at the scope gate until explicit approval
   was granted ("I approve").
10. Recorded definition approval with `specsync change approve ... --actor 0xLeif`.
11. Advanced stage to `build` via `chough stage advance` (gate `native:approved`).
12. Implemented guidance updates in `SKILL.md` and `checkpoint.md`, synchronized byte-for-byte
    to `.agents/` and `.claude/` copies, tracked earlier digests in `src/skill.rs`, refreshed
    roadmap rows in `docs/ROADMAP.md`, and documented this run.

## Assessment

| Expectation | Result |
|---|---|
| Found workflow skill without coaching | Yes: natively discovered via `.agents/skills/chough-workflow/SKILL.md` |
| Checked native state and evidence freshness | Yes: inspected checkpoint, native status, and tool versions |
| Resumed cold when previous change was done | Yes: reset stage to `plan` and initialized new native change |
| Matched CI tool versions | Yes: wrapped `hi` 0.8.0, `fledge` 1.7.2, `specsync` 6.1.1 |
| Honored human gates | Yes: strictly waited for human scope approval before advancing |
| Parity and hygiene | Yes: kept bundle copies byte-identical with zero em or en dashes |

## Limits and follow-ups

- Antigravity natively loads `.agents/skills`, so `.gemini/skills` was not needed in
  this repository, though `chough init --host gemini` defaults to `.gemini/skills`.
- When resuming from stage `done` with an archived change, `next_valid_action` echoed
  the prior review step; guidance was added to `references/checkpoint.md` clarifying how
  to start a new change cycle via `chough stage reset`.
- Zero em or en dash policy strictly maintained across all touched files.
