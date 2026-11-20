---
change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
artifact: requirements
---

# Requirements

REQ-cli-067 gains two sentences in its "Paths outside the declaration" item and three
criteria (the delta holds the text): a committed file whose content at HEAD is the
default branch's is not the change's own work, and a base commit the repository lacks
is named plainly and counted from the merge base. No sentence is removed.
