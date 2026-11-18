---
change: fix-347
artifact: testing
---

# Testing

Two new tests in `tests/bugs.rs`, both failing on main:

- `a_new_bugs_log_links_its_evidence_relative_to_the_bug`: `bug add --observation`
  writes the Reported line with a link to the entry, the link opens a file,
  `bug update --observation` links what it adds, and the front matter keeps the
  stored form with no link in it.
- `doctor_relinks_a_bugs_evidence_after_its_session_is_archived`: after the
  session is closed the entry file is gone; doctor reports one `stale_bug_links`
  finding for the bug, naming the entry and the story, not a decision; `--fix`
  appends one Log entry linking the story, every earlier Log line is unchanged
  and in order, the story file exists, and the bug is not reported again.

Two existing tests that pinned the bare `<session>/<id>` text in a Log line now
expect the link. The link text is built from evidence names the bug already
holds, so no new free-text input is added.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-012 | the first test and its case |
| REQ-history-012 | the second test and its case |
