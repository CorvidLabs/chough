---
change: release-prep-0-6-4
artifact: requirements
---

# Requirements

1. Cargo.toml `version` "0.6.4"; Cargo.lock updated.
2. CHANGELOG.md `## [0.6.4] - 2026-11-26` assembling every changelog.d fragment
   (six-fix records, approve passthrough REQ-cli-071, chough-first routing REQ-cli-072),
   newest first, with the `[0.6.4]` tag link; fragments removed from changelog.d.
3. Generated knowledge rebuilt: `chough library index`; `--check` passes.
4. `fledge run release-check` exits 0.
5. `cargo build --locked` and `cargo test --locked` pass at 0.6.4.

The human steps (tag, cargo publish, GitHub release, clean-install observation, roadmap
row) are out of scope per docs/RELEASING.md and LIFE-25; they follow after merge.
