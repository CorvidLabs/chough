---
id: fix-266
state: archived
type: bug_fix
base_commit: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
---

# Fix 266

## Intent

fix 266

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- During a git merge or rebase (MERGE_HEAD, or a rebase-merge or rebase-apply folder in the Git directory), chough next and chough status print a line naming it and saying to finish or abort it first, and chough doctor reports a git_operation finding with no fix that names it and any unmerged paths. While a path under .chough/ is unmerged, a command that writes Chough's records refuses as refused:unmerged, naming the paths and saying to finish or abort the merge or rebase first, with every file unchanged; reads still work. With no merge or rebase in progress nothing changes.

## No-spec Rationale

Not applicable
