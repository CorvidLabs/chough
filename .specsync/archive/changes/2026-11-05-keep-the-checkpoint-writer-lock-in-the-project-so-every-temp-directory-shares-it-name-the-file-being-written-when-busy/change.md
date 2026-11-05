---
id: keep-the-checkpoint-writer-lock-in-the-project-so-every-temp-directory-shares-it-name-the-file-being-written-when-busy
state: archived
type: bug_fix
base_commit: d9269bec6ea0bc6c9e6c9f37bb9a6f6a05cd5024
---

# Keep the checkpoint writer lock in the project so every temp directory shares it, name the file being written when busy, and specify serialized writers and the local state directory (issue 83)

## Intent

Keep the checkpoint writer lock in the project so every temp directory shares it, name the file being written when busy, and specify serialized writers and the local state directory (issue 83)

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- A writer with a different TMPDIR waits for the project lock and then saves. A busy refusal in sessions mode names the session file and saves nothing. Legacy writers racing session adopt either save in the remaining checkpoint or refuse naming the adoption with nothing saved. The lock lives in .chough/local which Git ignores and which in a project without sessions holds only the lock and its gitignore. Each new test fails where stated and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
