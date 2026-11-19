# Lesson bundle: fix-266

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 266
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/doctor.rs, src/main.rs, src/checkpoint.rs, tests/git_merge.rs, docs/cli.md, .chough/cases/cli.git-merge-in-progress.cases.toml, changelog.d/2026-11-18-fix-266.md
- **Acceptance**: During a git merge or rebase (MERGE_HEAD, or a rebase-merge or rebase-apply folder in the Git directory), chough next and chough status print a line naming it and saying to finish or abort it first, and chough doctor reports a git_operation finding with no fix that names it and any unmerged paths. While a path under .chough/ is unmerged, a command that writes Chough's records refuses as refused:unmerged, naming the paths and saying to finish or abort the merge or rebase first, with every file unchanged; reads still work. With no merge or rebase in progress nothing changes.

## Evidence

- Verification commit: `6a7cf86a46ee055382edc0172222bf3d5497e40d`
- Base commit: `cef86e2622a2a908856b0c2cf11b29d3c1d3e472`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

GitHub #266, from the 0.6.0 swarm report (#273, s4). During a `git merge` that conflicts on `.chough/` records, `chough next` and `chough status` exit 0 with normal guidance and no warning, and `chough doctor` reports nothing about the merge or the conflict markers. With a session-archive-versus-session-edit conflict, `next` even fails with a misleading "no open session". Nothing tells the user their records are mid-conflict, and Chough writes into the conflicted tree.

Leif's decision (2026-11-18, planning interview, recorded on #266 and relayed by hc t4c as the scope approval): warn, and refuse `.chough` writes. `next`, `status` and `doctor` report an in-progress git merge or rebase (`MERGE_HEAD`, `REBASE_HEAD`, unmerged paths) with "finish or abort it first". Write commands refuse only while paths under `.chough/` are unmerged.

The rule is a new requirement, REQ-cli-069. The `doctor` finding is defined there, beside the findings of REQ-history-012, rather than by editing that requirement, which #250 and #320 also change.

Sent back once, 2026-11-18, on Leif's word "Let the fixer send it back once" (relayed by hc t4c): the #330 review found that a finished rebase can leave `REBASE_HEAD` behind, so the code reads a rebase only from its `rebase-merge` or `rebase-apply` folder, and REQ-cli-069 and the acceptance criteria drop `REBASE_HEAD` to match. Nothing else in the scope changed.

## From the change's design.md

# Design

`doctor::git_operation(root)` reads Git's state once per process:
- the operation, found through `git rev-parse --git-dir` (so worktrees work): `MERGE_HEAD`, or a `rebase-merge` or `rebase-apply` folder, as `git status` reads a rebase, since Git can leave `REBASE_HEAD` behind after a rebase finishes (#330 review); `CHERRY_PICK_HEAD` and `REVERT_HEAD` are named too;
- the unmerged paths under `.chough/`, from `git ls-files -u -z -- .chough`.

It returns nothing outside a Git work tree or when there's nothing to report.

- `chough next` and `chough status` add `git_operation` to their view (null when there's nothing to report) and print one `Git:` line near the top.
- `chough doctor` adds a `git_operation` finding with no fix.
- The write refusal is in `checkpoint::lock_checkpoint`, which every record writer takes before it writes (REQ-cli-040), and in `observation_room`, which `evidence add`, `cases run` and `cases record` call before they write their entry. A writer is therefore refused before it writes anything.
- The merge drivers take no lock and are unaffected.

## From the change's testing.md

# Testing

`tests/git_merge.rs` builds a real merge conflict in a fixture: two branches change the same session record, then `git merge` stops with it unmerged.
- `a_merge_conflict_in_chough_warns_and_refuses_writes`: `next` and `status` exit 0 and name the merge and the path; `doctor` reports one `git_operation` finding; `checkpoint --decision`, `evidence add` and `bug add` refuse as `refused:unmerged` with every file unchanged. After `git merge --abort`, no warning or finding remains and a write succeeds.
- `a_merge_conflict_outside_chough_warns_without_refusing`: the conflict is in a product file; the reads warn and a write succeeds.

Both fail on main. Case `TC-cli-git-merge-in-progress-warns-and-refuses-writes` binds the first.

## Where these lessons go

- `specs/cli/cli.3md`
