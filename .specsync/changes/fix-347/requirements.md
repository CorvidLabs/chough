---
change: fix-347
artifact: requirements
---

# Requirements

- REQ-cli-012 (`deltas/cli.md`): a Log entry names each piece of linked evidence as a
  Markdown link relative to the bug's file, to the open session's entry or to the
  archived story that holds it, and by name alone when only the bug holds it; the
  front matter keeps the stored form.
- REQ-history-012 (`deltas/history.md`): a new finding, `stale_bug_links`, with its
  fix, which is not a decision and appends one Log entry.
