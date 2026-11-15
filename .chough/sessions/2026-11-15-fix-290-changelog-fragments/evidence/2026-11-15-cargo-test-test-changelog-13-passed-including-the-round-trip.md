---
kind: evidence
id: 2026-11-15-cargo-test-test-changelog-13-passed-including-the-round-trip
aliases: []
claim: "cargo test --test changelog: 13 passed, including the round-trip that rebuilds cf2ff72's CHANGELOG.md byte for byte and the refusal of a repeated version heading; fledge run fmt and lint clean"
result: passed
evidence_kind: product-test
method: cargo test --locked --test changelog; fledge run fmt; fledge run lint
revision: 3a95c197d1b8a5229134dbd91d5e8bf23f83a320
content: c68b423c4f71081b2b6285e61ebccbe42cfc93161157a41c05cb8f1c14cc8f9d
environment: local-macos-arm64
fingerprint: sha256:bb85aa0975bd3eb377c4c0b3c5ad2840147b82d3af68c7e6bab5f214cb8d7458
external: ext-c4ce10
criteria: [LIFE-16]
requirements: []
contracts: []
limitations: []
observed: 2026-11-15T15:12:33Z
actor:
  - agent: claude-code
    terminal: t28
---
**cargo test --test changelog: 13 passed, including the round-trip that rebuilds cf2ff72's CHANGELOG.md byte for byte and the refusal of a repeated version heading; fledge run fmt and lint clean.**

Recorded as passed in local-macos-arm64 at revision 3a95c197d1b8, observed 2026-11-15T15:12:33Z.
