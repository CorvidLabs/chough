---
change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
artifact: requirements
---

# Requirements

The deltas MODIFY five requirements:

- **REQ-cli-043:** the move into done writes the journal post but no longer regenerates `index.md` or `feed.xml`; it reports `librarian_index` as `deferred`.
- **REQ-cli-047:** `library index --check` stays strict but runs only in the release check (`fledge run release-check`, which rebuilds first), not in the verify, CI or pull request lanes.
- **REQ-cli-052:** the deferred knowledge step makes no native read.
- **REQ-records-011:** after a move into done, the feed lists the new post only once `chough library index` has run.
- **REQ-history-006:** the session archive follows the deferred knowledge step.
