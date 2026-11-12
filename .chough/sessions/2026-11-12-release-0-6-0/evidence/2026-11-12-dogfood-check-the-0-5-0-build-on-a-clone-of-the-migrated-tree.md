---
kind: evidence
id: 2026-11-12-dogfood-check-the-0-5-0-build-on-a-clone-of-the-migrated-tree
aliases: []
claim: "Dogfood check: the 0.5.0 build on a clone of the migrated tree reports no open session, refuses bug list at the first Markdown bug and writes nothing for those reads; its session new writes a 0.5.x session file, after which the 0.6.0 build refuses the tree as refused:unmigrated naming that file"
result: passed
evidence_kind: product-test
revision: a4358e31d7bbedce8ca5ea1d43a23e5103f9a824
content: fb8e7010230dde4e3a9852ba896a428fbdfe2ff283e76a77d8072062b094ee44
environment: local-macos-arm64
criteria: [DRIVE-27]
requirements: [REQ-records-010]
contracts: [specs/records/records.spec.md]
limitations: ["Run by hand in a throwaway clone, deleted afterwards"]
observed: 2026-11-12T15:03:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**Dogfood check: the 0.5.0 build on a clone of the migrated tree reports no open session, refuses bug list at the first Markdown bug and writes nothing for those reads; its session new writes a 0.5.x session file, after which the 0.6.0 build refuses the tree as refused:unmigrated naming that file.**

Recorded as passed in local-macos-arm64 at revision a4358e31d7bb, observed 2026-11-12T15:03:36Z.
