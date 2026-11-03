---
id: windows-safe-git-paths-in-the-issue-30-same-second-test
state: archived
type: bug_fix
base_commit: 46ca7a4407ebad831ee05f8fc580822190b36b74
---

# Windows-safe git paths in the issue 30 same-second test

## Intent

Windows-safe git paths in the issue 30 same-second test

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The same-second two-root unit test passes git plain temporary paths, never Windows verbatim canonical paths, and still asserts the same identities, times, bytes and freshness
- fledge lanes run verify passes locally; hosted Windows Product CI is reported as observed

## No-spec Rationale

Test-only fix: the REQ-cli-023 acceptance text is unchanged; only the test's git invocation paths change
