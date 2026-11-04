---
change: lock-inbox-add-against-a-concurrent-archive-and-protect-observations-named-by-unfolded-inbox-entries-from-retention
artifact: docs
---

# Docs

- docs/cli.md: the inbox section says `inbox add` waits for a running writer and that
  retention protects observations unfolded entries name.
- docs/cli.md: an entry is published whole, so a stopped add leaves no partial entry
  (#99 review).
- CHANGELOG.md: one Unreleased entry citing #98.
