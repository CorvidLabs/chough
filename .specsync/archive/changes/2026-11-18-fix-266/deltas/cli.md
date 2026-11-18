# cli delta

## ADDED

### REQUIREMENT REQ-cli-069

Git merge or rebase in progress. When Git reports a merge in progress (`MERGE_HEAD`), a rebase in progress (a `rebase-merge` or `rebase-apply` folder in the Git directory) or unmerged paths, `chough next` and `chough status` SHALL say so in a line of their own, naming the operation and the unmerged paths under `.chough/` (at most 10, with their total) and saying to finish or abort it first, and SHALL give the same as `git_operation` in their JSON; `chough doctor` SHALL report it as a `git_operation` finding with no fix, naming the operation and those paths, beside the findings of REQ-history-012. While any path under `.chough/` is unmerged, every command that writes a Chough record (REQ-cli-040) SHALL refuse as `refused:unmerged`, naming those paths and saying to finish or abort the merge or rebase first, with every file unchanged; a command that only reads SHALL still run, and Chough's merge drivers, which write only the files Git hands them, SHALL still run. Outside a Git work tree, or with no merge or rebase in progress, nothing changes. Trace: DRIVE-17, DRIVE-11, DRIVE-3.

Acceptance Criteria
- During a merge that leaves a `.chough/` record conflicted, `chough next` and `chough status` exit 0 and name the merge and the path, `chough doctor` reports one `git_operation` finding naming them, and `chough checkpoint --decision`, `chough evidence add` and `chough bug add` refuse as `refused:unmerged` with every file unchanged.
- During a merge whose only conflict is outside `.chough/`, `next`, `status` and `doctor` still name the merge, and a write succeeds.
- Once the merge is committed or aborted, none of this appears.

## MODIFIED

(none)

## REMOVED

(none)
