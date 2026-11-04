---
change: prepare-the-0-2-0-release-set-the-package-version-to-0-2-0-and-date-the-change-log-section-for-0-2-0
artifact: context
---

# Context

Leif (0xLeif) asked on 2026-11-03 at 4:52 PM MST: "Tag v0.2.0 after #66 lands". PR #66
merged as 8298e5a. Chough 0.1.0 was tagged at c03d419 after a release preparation PR
(#22) and a change log dating PR (#27), then published to crates.io and as a GitHub
release (docs/ROADMAP.md, "0.1.0 release" row). Main still declares version 0.1.0 and
its change log holds everything since 0.1.0 under Unreleased, so a tag of v0.2.0 on main
as it stands would build a binary that reports 0.1.0.

This change only sets the version and dates the change log. The tag, any crates.io
publish and the GitHub release are separate steps after merge; each is its own
decision (docs/releasing.md, skills/chough-workflow/references/release.md).
