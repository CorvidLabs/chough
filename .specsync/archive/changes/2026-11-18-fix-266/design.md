---
change: fix-266
artifact: design
---

# Design

`doctor::git_operation(root)` reads Git's state once per process:
- the operation, found through `git rev-parse --git-dir` (so worktrees work): `MERGE_HEAD`, or a `rebase-merge` or `rebase-apply` folder, as `git status` reads a rebase, since Git can leave `REBASE_HEAD` behind after a rebase finishes (#330 review); `CHERRY_PICK_HEAD` and `REVERT_HEAD` are named too;
- the unmerged paths under `.chough/`, from `git ls-files -u -z -- .chough`.

It returns nothing outside a Git work tree or when there's nothing to report.

- `chough next` and `chough status` add `git_operation` to their view (null when there's nothing to report) and print one `Git:` line near the top.
- `chough doctor` adds a `git_operation` finding with no fix.
- The write refusal is in `checkpoint::lock_checkpoint`, which every record writer takes before it writes (REQ-cli-040), and in `observation_room`, which `evidence add`, `cases run` and `cases record` call before they write their entry. A writer is therefore refused before it writes anything.
- The merge drivers take no lock and are unaffected.
