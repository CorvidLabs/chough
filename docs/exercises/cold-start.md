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

- One run of one agent on one sample. No Claude Code, Cursor, Gemini, Grok or Kimi
  run was performed on that sample.
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

## Kimi cold start: 2026-11-01

Host and environment: Kimi Code CLI 0.41.0 on macOS arm64, environment
`local-macos-arm64`, worktree `.worktrees/kimi-host-cold-start`, branch
`kimi/feat/kimi-host-cold-start`, base `5af3f78`.

Input and discovery: the supplied task named the repository, objective, `AGENTS.md`
and canonical skill. Kimi independently listed and loaded
`.agents/skills/chough-workflow/SKILL.md` as a Project skill. Discovery passed in
this run, with that explicit task guidance; this does not prove discovery in every
Kimi configuration.

Commands and results:

1. `cargo build --locked` passed in 10 seconds.
2. `chough next --environment local-macos-arm64` emitted 11,567 JSON lines in 16
   seconds. It reported stage `done` and native state `archived`, but its valid action
   repeated stale review and ship guidance. Native SpecSync status confirmed the
   archived state and advised merging the PR if it was still open.
3. `chough status --json` emitted about 11,567 JSON lines in 36 seconds and repeated
   the same contradiction.
4. `chough skill status` reported every installed bundle file current, and
   `chough cases check` reported valid definitions.
5. The local baseline verify lane passed 9 steps in 114 seconds.

Findings and scope: BUG-0020 records the stale archived action, and BUG-0021 records
the absent Kimi mapping guidance; both are in this change. BUG-0022 records initial
tool drift, and BUG-0023 records oversized `next` and `status` output; both remain
open and out of scope. The baseline used HI 0.5.0 and Fledge 1.8.0 with SpecSync
6.1.1, so it was neither final CI-toolchain verification nor hosted CI. Later
release-verified HI 0.8.0 and Fledge 1.7.2 tools were placed under
`target/tmp/ci-tools`; SpecSync 6.1.1 is required with them for final checks.

Evidence and limits: see the [concise Kimi log](../../.chough/artifacts/kimi-cold-start.log)
and [TC-cli-0106](../../specs/cli/testing.md). The original observation
`tc-cli-0106-kimi-original` failed because the mapping guidance and consistent
corrected documents were absent at base. Any later TC-cli-0106 pass is a
retrospective documentation review, not another Kimi execution. No fresh post-fix
Kimi run, agent-recorded human decision, release or deployment is claimed.
