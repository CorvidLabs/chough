---
kind: evidence
id: 2026-11-18-cargo-test-locked-test-git-merge-preference-writes-refuse
aliases: []
claim: "cargo test --locked --test git_merge -- preference_writes_refuse_while_chough_is_unmerged: 1 passed. With .chough/README.md unmerged in a stopped git merge, pref set and pref unset refuse as refused:unmerged naming it with every file unchanged, pref show runs, and after git merge --abort pref unset writes (#330 review P2)."
result: passed
evidence_kind: product-test
method: automated
track: functional
revision: 968172b73473b5f9149355b47bb9a59a7ea48d17
content: 845ad9bc3bc1f4da0e725592660cef43f4bc0a8388678ae77206256b820b9c22
environment: local-macos-arm64
criteria: [DRIVE-17]
requirements: [REQ-cli-069]
contracts: []
limitations: []
observed: 2026-11-18T19:36:13Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**cargo test --locked --test git_merge -- preference_writes_refuse_while_chough_is_unmerged: 1 passed. With .chough/README.md unmerged in a stopped git merge, pref set and pref unset refuse as refused:unmerged naming it with every file unchanged, pref show runs, and after git merge --abort pref unset writes (#330 review P2).**

Recorded as passed in local-macos-arm64 at revision 968172b73473, observed 2026-11-18T19:36:13Z.
