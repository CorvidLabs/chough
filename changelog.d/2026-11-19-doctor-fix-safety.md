- `chough doctor --fix` no longer destroys work when it finishes an archive: an
  evidence entry the story's receipts don't hold, with its screenshot, and an unfolded
  inbox entry are kept, and the finding names how to move or drop each (#381). macOS
  metadata such as `.DS_Store` and `._*` files no longer blocks the finish (#377).
- `chough next` names finishing an archive while the session folder still holds files:
  commit the folder, story and journal post, then `chough doctor --fix` (#371).
- `chough doctor` finds Git conflict markers in a record's front matter and says to
  resolve the conflict, not to shelve the session. A Timeline entry that
  `doctor --fix --redact` redacted is no longer reported as rewritten (#374).
- The spec walk behind the whole-repository advice follows no link and stops at 4,096
  entries or 32 folders deep, saying so, so a link loop in `specs/` no longer hangs
  `chough doctor` or `chough init` (#402).
- `chough doctor --fix` no longer removes an archived session's folder while its story is
  not committed; it names it to commit first, `git add -f` for an ignored story (#389). A story an earlier
  release wrote still holds its entries, and an unreadable entry is kept and named.
