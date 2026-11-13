---
id: 217-bar-2-seed-the-flake-tracker-the-github-flake-label-on-the-known-flake-issues-and-a-note-naming-drive-19-and-each
state: implementing
type: documentation
base_commit: 38fe24c063e3afde49a5be5093cd69791bd06126
---

# #217 bar 2: seed the flake tracker: the GitHub flake label on the known flake issues, and a note naming DRIVE-19 and each GitHub issue on the flake bug records

## Intent

#217 bar 2: seed the flake tracker: the GitHub flake label on the known flake issues, and a note naming DRIVE-19 and each GitHub issue on the flake bug records

## Affected Canonical Specs

- None

## Acceptance Criteria

- The flake label exists on CorvidLabs/chough and is on #187, #132, #169 and #247; the bug records for #187, #132 and #169 carry a Log note naming DRIVE-19 and their issue as GitHub #<n>, and the three local-only flake records a note naming DRIVE-19 and that no issue was filed; docs/test-plan-1.0.md bar 2 names the seeded issues instead of 'Known today: #187 and #132'

## No-spec Rationale

Seeding only: a GitHub label, bug record notes (outside verification) and one docs line naming the seeded issues; the tracker's rules are already in docs/test-plan-1.0.md bar 2 and the window's contract is REQ-cli-062
