---
id: 2026-11-12-merge-main-back-with-ours-only-when-the-trees-match
title: After a squash merge, merge main back with -s ours only when the trees match
type: lesson
tags: [lesson, git, pull-requests]
date: 2026-11-12
applies_to: [git, squash-merged pull requests split from a longer branch]
source: https://github.com/CorvidLabs/chough/pull/239
---
# After a squash merge, merge main back with -s ours only when the trees match

When part of a long branch ships as its own pull request and is squash-merged, the
default branch gains one commit whose content the long branch already holds but whose
history it lacks. The long branch's own pull request then shows the shipped work again.

Merge the default branch back with `git merge -s ours origin/main` only after `git diff
<the commit the split was cut from> origin/main` prints nothing. Equal trees mean keeping
ours drops nothing, and the pull request then shows only the work still to ship, with no
force-push. If the diff prints anything, the default branch has work the long branch
lacks, and `-s ours` would silently discard it: merge normally instead.

Seen in Chough 0.6.0: #239 squash-merged as a2e30b8c, the diff against 977bd246 was
empty, the `-s ours` merge 2852997c changed no file, and #238 went down to the 54 files
of the release preparation. The release-preparation session records the check.
