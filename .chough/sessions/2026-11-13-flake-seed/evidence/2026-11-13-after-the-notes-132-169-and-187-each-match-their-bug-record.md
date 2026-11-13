---
kind: evidence
id: 2026-11-13-after-the-notes-132-169-and-187-each-match-their-bug-record
aliases: []
claim: "After the notes, #132, #169 and #187 each match their bug record and 6 records name DRIVE-19. The only blockers left are #247: open, and its record (on t37f's fix branch leif/fix-247-example-race) is not on main yet. Same GitHub input as the red run (sha256 a4634e4dea37a09b), so only the records changed"
result: passed
evidence_kind: structural
method: python3 .github/scripts/flake-window.py (#217 branch, REQ-cli-062) --root <this worktree> --save-input <scratch file>, after the six bug record notes, on the same GitHub input as the red run
revision: 38fe24c063e3afde49a5be5093cd69791bd06126
content: df3781c657ff395e955399be8c8722b832b6d3cb234f717ce59a9d3459790314
environment: local-macos-arm64
criteria: [DRIVE-19, LIFE-21]
requirements: []
contracts: []
limitations: []
observed: 2026-11-13T04:34:08Z
actor:
  - agent: claude-code
    terminal: t38r
---
**After the notes, #132, #169 and #187 each match their bug record and 6 records name DRIVE-19. The only blockers left are #247: open, and its record (on t37f's fix branch leif/fix-247-example-race) is not on main yet. Same GitHub input as the red run (sha256 a4634e4dea37a09b), so only the records changed.**

Recorded as passed in local-macos-arm64 at revision 38fe24c063e3, observed 2026-11-13T04:34:08Z.
