---
kind: evidence
id: 2026-11-13-before-the-notes-the-flake-window-is-blocked-132-169-187-and
aliases: []
claim: "Before the notes, the flake window is blocked: #132, #169, #187 and #247 are labelled flake but no bug record names them as GitHub #<n>, and #247 is open (4 flake issues, 205 bug records, 3 naming DRIVE-19; input sha256 a4634e4dea37a09b)"
result: failed
evidence_kind: structural
method: "python3 .github/scripts/flake-window.py (#217 branch, REQ-cli-062) --root <this worktree> --save-input <scratch file>, after the flake label was put on #187, #132, #169 and #247 and before any bug record note"
revision: 38fe24c063e3afde49a5be5093cd69791bd06126
content: 6224d9769799531c0255d85af9ab565325a1a6fbbc6ead110c92886d7076f51f
environment: local-macos-arm64
criteria: [DRIVE-19, LIFE-21]
requirements: []
contracts: []
limitations: []
observed: 2026-11-13T04:33:06Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Before the notes, the flake window is blocked: #132, #169, #187 and #247 are labelled flake but no bug record names them as GitHub #<n>, and #247 is open (4 flake issues, 205 bug records, 3 naming DRIVE-19; input sha256 a4634e4dea37a09b).**

Recorded as failed in local-macos-arm64 at revision 38fe24c063e3, observed 2026-11-13T04:33:06Z.

## Key failing lines

- flake issue #132 has no bug record naming GitHub #132
- flake issue #169 has no bug record naming GitHub #169
- flake issue #187 has no bug record naming GitHub #187
- flake issue #247 is open
- flake issue #247 has no bug record naming GitHub #247
