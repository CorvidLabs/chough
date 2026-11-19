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

`doctor_relinks_only_what_the_archived_story_holds_as_a_receipt` (added after the
tester's findings on #360): a note that links an entry never recorded, and a
hand-edited link whose entry name holds a credential, are not relinked; only the
entry the story holds as a receipt is, and the credential is never written again.
It fails without the receipt check.

`a_bug_too_near_its_bound_gets_advice_and_no_relink_fix` (added after the swarm's
finding 62-F9 on #360): a bug 60 bytes below its bound gets a finding whose fix
starts `none:` and names the story, and `--fix` leaves it byte for byte. It fails
without the fit check, where the finding offers `chough doctor --fix` for ever.

`a_qualified_id_of_archived_evidence_links_the_story` (added after the reviewer's
finding on #360): after a session is archived, its entry given by the qualified
`<session>/<entry>` ID to `bug add` and `bug update` links the story, as the bare ID
does. It fails with the earlier code, which linked the removed session folder.

Two existing tests that pinned the bare `<session>/<id>` text in a Log line now
expect the link. The link text is built from evidence names the bug already
holds, so no new free-text input is added.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-012 | the first test and its case |
| REQ-history-012 | the second test and its case |
