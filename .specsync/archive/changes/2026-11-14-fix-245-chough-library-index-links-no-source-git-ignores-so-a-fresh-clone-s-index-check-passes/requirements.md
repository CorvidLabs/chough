---
change: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
artifact: requirements
---

# Requirements

REQ-cli-043 (modified): the index links each rule, skill, decision and policy file that
exists and that Git doesn't ignore, and none that is missing or ignored. A new criterion
covers a gitignored `.grok` skill copy, checked in the working tree and in a fresh
clone.
