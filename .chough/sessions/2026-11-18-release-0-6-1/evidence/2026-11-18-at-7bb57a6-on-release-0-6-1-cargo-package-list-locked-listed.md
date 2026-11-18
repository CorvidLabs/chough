---
kind: evidence
id: 2026-11-18-at-7bb57a6-on-release-0-6-1-cargo-package-list-locked-listed
aliases: []
claim: "At 7bb57a6 on release/0.6.1: cargo package --list --locked listed only src (33), skills/chough-workflow (24), Cargo.toml, Cargo.lock, README.md and LICENSE plus cargo's generated files; cargo publish --dry-run --locked built chough v0.6.1 and stopped at 'aborting upload due to dry run'; crates.io's owners API listed 0xLeif as the only owner of chough, at 0.6.0; fledge run release-check passed. A rehearsal: nothing was published."
result: passed
evidence_kind: dry-run
revision: 7bb57a609060fb2b53e672466bb689e431b75f5d
content: 563235bb22691510e88467a7e7779d3fe64f63da178e70c3136a44177b461ec2
environment: local-macos-arm64
criteria: [LIFE-25]
requirements: []
contracts: []
limitations: []
observed: 2026-11-18T17:30:59Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**At 7bb57a6 on release/0.6.1: cargo package --list --locked listed only src (33), skills/chough-workflow (24), Cargo.toml, Cargo.lock, README.md and LICENSE plus cargo's generated files; cargo publish --dry-run --locked built chough v0.6.1 and stopped at 'aborting upload due to dry run'; crates.io's owners API listed 0xLeif as the only owner of chough, at 0.6.0; fledge run release-check passed. A rehearsal: nothing was published.**

Recorded as passed in local-macos-arm64 at revision 7bb57a609060, observed 2026-11-18T17:30:59Z.
