---
change: windows-safe-git-paths-in-the-issue-30-same-second-test
artifact: context
---

# Context

On hosted CI for PR #48, `Product / windows-latest` failed in `tests::same_second_runs_in_two_git_roots_keep_distinct_identities_after_merge`. The error was `git clone … \\?\C:\…\Temp\.tmp…: hostname contains invalid characters`.

On Windows, `canonicalize` returns verbatim `\\?\` paths, which git cannot read as a clone or pull source. The ubuntu and macOS jobs were cancelled by fail-fast, not failed. The test now gives git the plain temporary paths and keeps the canonical roots for Chough's own calls.
