# Lesson bundle: windows-safe-git-paths-in-the-issue-30-same-second-test

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Windows-safe git paths in the issue 30 same-second test
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/main.rs, .chough/cases/
- **Acceptance**: The same-second two-root unit test passes git plain temporary paths, never Windows verbatim canonical paths, and still asserts the same identities, times, bytes and freshness
- **Acceptance**: fledge lanes run verify passes locally; hosted Windows Product CI is reported as observed

## Evidence

- Verification commit: `7dedd3fb718b507bbd174b75b6f765fc031eec21`
- Base commit: `46ca7a4407ebad831ee05f8fc580822190b36b74`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

On hosted CI for PR #48, `Product / windows-latest` failed in `tests::same_second_runs_in_two_git_roots_keep_distinct_identities_after_merge`. The error was `git clone … \\?\C:\…\Temp\.tmp…: hostname contains invalid characters`.

On Windows, `canonicalize` returns verbatim `\\?\` paths, which git cannot read as a clone or pull source. The ubuntu and macOS jobs were cancelled by fail-fast, not failed. The test now gives git the plain temporary paths and keeps the canonical roots for Chough's own calls.

## From the change's testing.md

# Testing

- Rerun case `TC-cli-same-second-runs-in-two-git-roots-stay-distinct` and `fledge lanes run verify` on local-linux-x86_64.
- The hosted `Product / windows-latest` job is the cross-platform proof; it is reported only as observed.

## Where these lessons go

- `specs/cli/context.md`
