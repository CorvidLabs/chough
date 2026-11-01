---
change: prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill
artifact: docs
---

# Docs

- README.md: `Install` (Rust 1.88 or later; `cargo install chough --locked` for
  released versions, the Git install for unreleased main, the clone route) and a new
  `Install the toolchain` section with the one command and its Rust 1.89 need; the
  status paragraph points to CHANGELOG.md instead of saying no release exists.
- AGENTS.md: tool version line hi 0.8.0, fledge 1.8.0, specsync 6.1.2.
- CHANGELOG.md (new): `[0.1.0] - Unreleased`, first release, delivered work by PR.
- docs/releasing.md (new): the human release steps after merge; nothing in this change
  runs them.
- docs/ci.md, docs/integrations.md: Fledge 1.8.0, installed with cargo in CI.
- docs/DECISIONS.md: D-019, pinned toolchain, Fledge Action at v1.8.0 in CI and cargo locally, one README command.
- specs/cli/context.md: history of PRs #3 to #18 and current working files.
- skills/chough-workflow: the nine corrections; host copies refreshed.
- Pull request description: lists the open questions and that tag, publish and GitHub
  release are not part of this change.
