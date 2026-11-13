---
kind: evidence
id: 2026-11-13-the-check-read-live-ci-history-for-corvidlabs-chough-in-27-6-s
aliases: []
claim: "The check read live CI history for CorvidLabs/chough in 27.6 s: 808 runs over 45 days, 135 main runs, 134 job lists, 0 flake issues, 205 bug records (3 name DRIVE-19). Verdict running: latest restart the rerun of run 43357893936 on leif/0.6.1-observation-cap at 2026-11-13T03:33:05Z, clean from 2026-11-27T03:33:05Z, 15 restarts in the window. Recomputing from the saved input gave a byte-identical receipt (input sha256 40fe8591bf6c78c0). This shows the check runs on real data, not that the window is clean"
result: passed
evidence_kind: structural
method: python3 .github/scripts/flake-window.py --save-input <scratch file> against CorvidLabs/chough, then --input <that file> --now 2026-11-13T03:4xZ
revision: 1d711eddf2ac335963f51cd802604e839b212a7e
content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
environment: local-macos-arm64
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: []
limitations: []
observed: 2026-11-13T03:50:48Z
actor:
  - agent: claude-code
    terminal: t38r
---
**The check read live CI history for CorvidLabs/chough in 27.6 s: 808 runs over 45 days, 135 main runs, 134 job lists, 0 flake issues, 205 bug records (3 name DRIVE-19). Verdict running: latest restart the rerun of run 43357893936 on leif/0.6.1-observation-cap at 2026-11-13T03:33:05Z, clean from 2026-11-27T03:33:05Z, 15 restarts in the window. Recomputing from the saved input gave a byte-identical receipt (input sha256 40fe8591bf6c78c0). This shows the check runs on real data, not that the window is clean.**

Recorded as passed in local-macos-arm64 at revision 1d711eddf2ac, observed 2026-11-13T03:50:48Z.
