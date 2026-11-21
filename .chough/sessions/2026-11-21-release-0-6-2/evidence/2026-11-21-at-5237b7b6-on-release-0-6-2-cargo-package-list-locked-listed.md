---
kind: evidence
id: 2026-11-21-at-5237b7b6-on-release-0-6-2-cargo-package-list-locked-listed
aliases: []
claim: "At 5237b7b6 on release/0.6.2: cargo package --list --locked listed only src (34), skills/chough-workflow (24), Cargo.toml, Cargo.lock, README.md and LICENSE plus cargo's generated files; cargo publish --dry-run --locked built chough v0.6.2 and stopped at 'aborting upload due to dry run'; crates.io's owners API listed 0xLeif as the only owner of chough, at 0.6.1. At 55002ee7, after the change log was assembled and the knowledge index rebuilt, fledge run release-check passed. A rehearsal: nothing was published."
result: passed
evidence_kind: structural
revision: 55002ee7e5f5e9315fdc5ed6b7c90fd22fe700d2
content: 90633e84886e441c29c84b513c8e8c80e027de7ed7853aaba02dad547c4387be
environment: local-macos-arm64
criteria: [LIFE-25]
requirements: []
contracts: []
limitations: []
observed: 2026-11-21T00:06:38Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**At 5237b7b6 on release/0.6.2: cargo package --list --locked listed only src (34), skills/chough-workflow (24), Cargo.toml, Cargo.lock, README.md and LICENSE plus cargo's generated files; cargo publish --dry-run --locked built chough v0.6.2 and stopped at 'aborting upload due to dry run'; crates.io's owners API listed 0xLeif as the only owner of chough, at 0.6.1. At 55002ee7, after the change log was assembled and the knowledge index rebuilt, fledge run release-check passed. A rehearsal: nothing was published.**

Recorded as passed in local-macos-arm64 at revision 55002ee7e5f5, observed 2026-11-21T00:06:38Z.
