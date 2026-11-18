---
change: fix-260
artifact: context
---

# Context

GitHub #260: when the disk is full, `chough bug add` fails but leaves a 0-byte `.chough/bugs/<name>.md`, and from then on `bug list`, `status` and `next` all fail on that file until someone removes it by hand. Past a file size limit it leaves a cut-off record whose front matter is complete, which the readers accept as a valid bug although the command reported failure. `chough journal add` leaves the same empty post. `checkpoint` and `inbox add` leave nothing in the same conditions.

REQ-records-001 already says every record Chough writes is one Markdown file that starts with front matter, written by create-new for a new record. A file left empty or cut off by a failed write is not that. Bugs and journal posts are both created through one writer, `write_new_bytes`, so this change fixes that one place: when the write fails after the file was created, the writer removes the file it created and reports the failure. It changes no requirement text.

Out of scope: `chough doctor` does not report an uncommitted empty or cut-off record that something else left. A new doctor finding needs a sentence in REQ-history-012 first. Writing through a temporary file and a link, as `inbox add` does, is a larger change than this defect needs.
