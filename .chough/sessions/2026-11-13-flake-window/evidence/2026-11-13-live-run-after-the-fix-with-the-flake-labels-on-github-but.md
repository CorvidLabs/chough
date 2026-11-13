---
kind: evidence
id: 2026-11-13-live-run-after-the-fix-with-the-flake-labels-on-github-but
aliases: []
claim: "Live run after the fix, with the flake labels on GitHub but this branch's records not seeded: verdict blocked by #132, #169 and #187 having no bug record naming them (the seed PR #255 adds those) and #247 open with its record on its fix branch; 814 runs, 136 main runs, 4 flake issues, 206 bug records. This shows the check on live data, not that the window is clean"
result: passed
evidence_kind: structural
method: python3 -B .github/scripts/flake-window.py --save-input <scratch file> against CorvidLabs/chough at 05786a14
revision: 05786a14526a1c168e7ff6678d81ac884abd579f
content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
environment: local-macos-arm64
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: []
limitations: []
observed: 2026-11-13T05:51:58Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Live run after the fix, with the flake labels on GitHub but this branch's records not seeded: verdict blocked by #132, #169 and #187 having no bug record naming them (the seed PR #255 adds those) and #247 open with its record on its fix branch; 814 runs, 136 main runs, 4 flake issues, 206 bug records. This shows the check on live data, not that the window is clean.**

Recorded as passed in local-macos-arm64 at revision 05786a14526a, observed 2026-11-13T05:51:58Z.
