---
change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
artifact: context
---

# Context

Issue #289: every pull request regenerated `knowledge/index.md` and `feed.xml` at its move into done. GitHub ignores the `chough-generated` merge driver, so any two open PRs conflicted on those files.

Leif's choices in Claude Code chat on 2026-11-15:
- **A1, rejected:** a CI push to main needed a bypass that classic branch protection would not take.
- **A3, chosen:** rebuild the index at release.
- **Design B, chosen:** after Grok showed that a relaxed "behind but not wrong" check passed wrong indexes, run only the strict check, and only at release.
