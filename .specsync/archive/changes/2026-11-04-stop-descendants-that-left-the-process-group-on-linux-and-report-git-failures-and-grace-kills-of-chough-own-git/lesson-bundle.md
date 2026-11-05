# Lesson bundle: stop-descendants-that-left-the-process-group-on-linux-and-report-git-failures-and-grace-kills-of-chough-own-git

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Stop descendants that left the process group on Linux and report Git failures and grace kills of Chough own Git commands (issue 81)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/records.rs, src/main.rs, tests/stop.rs, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: On Linux a stop during cases run also stops a descendant that called setsid and an orphan of the tree. A grace kill after Chough own Git is reported on stderr. A failed git rev-parse HEAD other than no repository or no commit is reported on stderr. Each new test fails on main 91a1fdd and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `379889dae305174186f89ca79640a4fce787d78a`
- Base commit: `91a1fdd82a4e314e5037c6256f17199ef0d604f0`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue https://github.com/CorvidLabs/chough/issues/81 collects follow-ups from the Chough
bot reviews of #79 (FLOW-14):

1. A process that calls `setsid` leaves the process group Chough stops, so it outlives a
   stop. Reproduced on 2026-11-04 (Claude Code session): a case binding whose child ran
   `POSIX::setsid(); sleep 90` kept running after SIGINT and SIGTERM ended Chough in
   about 50 ms, for its full 90 s.
2. `head_revision` turns a failed `git rev-parse HEAD` into `None` without saying so.
3. When the grace kill after a Git command fires (a descendant held Git's output open),
   nothing says so.
4. A background process left by SpecSync's own Git held a native read's pipe about
   120 s in a review harness; the issue asks for a repro first.

FLOW-14 ("everything it started stops too") covers item 1; LIFE-7 covers reporting
items 2 and 3. Item 4 is investigated in this change and reported on the PR; it changes
code only if the repro shows a Chough defect.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes and his
brief to fix issues 81 to 88 one PR each. Linux only, as his brief says: other Unix
systems keep the process-group stop.

Item 4, reproduced on 2026-11-04 (Linux, SpecSync 6.1.2, this repository): with a `git`
on PATH that leaves `sleep 300 &` holding its output, `chough status`'s native read ends
as `timed-out` at the 120 s deadline. SpecSync waits for its own Git child's output to
close, so the command Chough runs never exits and the 2 s grace (which starts when that
command exits) does not apply; Chough's deadline bounds it and a stop signal still ends
everything at once. No Chough code changes for item 4.

## From the change's design.md

# Design

- src/records.rs `stop_tree` (Linux): before signalling the group, collect the child's
  descendants by walking `/proc/<pid>/stat` parent links, so processes that left the
  group with `setsid` are found; then SIGKILL the group and every collected process.
  When the binary has made Chough a child subreaper (below), orphans of the tree are
  re-parented to Chough, so Chough's own other children are collected too.
- src/main.rs (Linux): at startup the binary calls `prctl(PR_SET_CHILD_SUBREAPER, 1)`,
  so a double-forked descendant whose parent exits is re-parented to Chough instead of
  init and is found by the walk. Unit tests in the crate do not set it, so a test never
  stops another test's processes.
- `run_owned` prints `chough: stopped a process that kept Git's output open <n> s after
  Git exited` on stderr when its grace kill fires.
- `head_revision` prints `chough: could not read the Git revision (<reason>); this
  transition records no revision` on stderr when `git rev-parse HEAD` fails for a reason
  other than "not a git repository" or a HEAD with no commit, and still returns None.

## From the change's testing.md

# Testing

- tests/stop.rs (Linux):
  - `stopping_a_case_run_stops_a_descendant_that_left_the_group` (REQ-cli-011): a binding
    whose child runs `setsid sleep <n>` while the binding waits; after SIGTERM, no
    `sleep <n>` survives.
  - `stopping_a_case_run_stops_an_orphan_that_left_the_group` (REQ-cli-011): the binding
    double-forks a `setsid sleep <n>` whose parent exits at once; after SIGTERM it is gone.
  - `a_grace_kill_after_git_is_reported` (REQ-cli-011): a Git stand-in leaves a
    descendant holding its output; `stage advance` succeeds and stderr names the stop.
  - `a_failed_head_revision_is_reported` (REQ-cli-011): a Git stand-in fails
    `rev-parse HEAD` with exit 1; `stage advance` succeeds and stderr names the failure.
  - `a_stop_test_leaves_another_runs_processes_alone` (REQ-cli-011, second #95 review): a
    decoy `sleep 691` stands in for this suite running in another worktree; the setsid
    scenario must neither take it for a survivor nor kill it. Fails at ffb1c24, where the
    tests matched the fixed `sleep 691`. The setsid, orphan and grace-kill tests now use
    a `sleep` duration unique to each fixture, and `Reap` kills only that.
- Each of the first four fails on main 91a1fdd; `fledge lanes run verify` passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-011 | tests/stop.rs `stopping_a_case_run_stops_a_descendant_that_left_the_group`, `stopping_a_case_run_stops_an_orphan_that_left_the_group`, `a_grace_kill_after_git_is_reported`, `a_failed_head_revision_is_reported`, `a_stop_test_leaves_another_runs_processes_alone` |

## Where these lessons go

- `specs/cli/context.md`
