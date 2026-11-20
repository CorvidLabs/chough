---
change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
artifact: design
---

# Design

Nothing visual changes beyond the lines named.

- F4: `gates::tests` also counts each observation the session names that links no case,
  by its own entry name, in the same Tests line.
- F14: the diff line is `git diff <base>` while `git diff --quiet HEAD` reports changes,
  with "(work not committed yet)"; with no recorded base and exactly one root commit,
  what changed and the diff count from that root and say "no base commit was recorded:
  the session started before the first commit".
- F5: `unfinished_archive`'s fix names the session's journal post (the story links it;
  else `knowledge/journal/<date>-<slug>.md` by the session's title) with the folder's
  files and the story, and `doctor --fix` skips while it is untracked or uncommitted.
  Narrowed by Leif's "A: narrow it, keep B" (2026-11-20): the fix names the post, and
  an untracked or ignored post never holds the finish; the history delta holds the text.
- #390 (B: Warn): in `stage advance`, for each stage passed that is plan or accept,
  ungated (`ungated:not-configured` or `ungated:sdd-disabled`) and holding no closed
  human gate: print "no human decision is recorded for <stage>", and in the done
  summary. `--to` across such a stage refuses as `refused:no-human-decision` without
  `--reason <text>`, naming the flag; `--reason` is a new `stage advance` flag, leak-
  checked like every recorded text, and written into that move's Timeline entry.

Options for #390 were Leif's (A require a gate, B warn, C document); he chose B. Option A
can follow after 0.6.2 as its own change.
