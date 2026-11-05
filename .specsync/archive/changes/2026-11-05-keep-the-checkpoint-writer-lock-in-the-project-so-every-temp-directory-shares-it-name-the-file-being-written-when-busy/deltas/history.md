## ADDED

### REQUIREMENT REQ-history-007

Local state directory. `.chough/local/` SHALL hold only Git-ignored local state of one worktree, with a `.gitignore` of `*`: the active-session pointer in sessions mode and the checkpoint writer lock (REQ-cli-040). A project without `.chough/sessions/` SHALL gain no tracked file and no session file from sessions support; its first checkpoint writer creates only `.chough/local/` with that `.gitignore` and the lock. Trace: DRIVE-3, DRIVE-11.

Acceptance Criteria
- In a project without sessions, `init` and `stage advance` create no `.chough/sessions/`, and `.chough/local/` holds only `.gitignore` and `checkpoint.lock`, both ignored by Git.
- In sessions mode, `.chough/local/` holds only `.gitignore`, `checkpoint.lock` and `active-session`.
