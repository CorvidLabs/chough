---
change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
artifact: context
---

# Context

G2: #389 (s9-F4, F5, F14) and #390 (s9-F6), from the 0.6.2 candidate swarm's fresh-eyes
walk. Leif's decision on #390, 2026-11-19: "B: Warn".

Measured on main 7bf110dc (t4b, 2026-11-20):
- F4: a passing `evidence add --criterion WC-1 --kind product-test --outcome passed`,
  then a review gate: `gate list` says `Tests: 0 passed, 0 failed, 0 not run`. Cause:
  `gates::tests` counts only observations bound to a case.
- F5: done with the librarian on, then `chough doctor`: its fix names session.md and the
  story, not the journal post. After that commit, `doctor --fix` exits 0, reports no
  finding, and `knowledge/journal/<post>.md` is still untracked. D1 (#414) fixed the
  rest of F5: the folder no longer goes while the story is uncommitted.
- F14: the diff line is `git diff <base>..<HEAD>`; with the work uncommitted HEAD is the
  base, so it is empty. A session started before the first commit has no base and the
  summary says only "unknown (no base commit recorded)" (code read, gates.rs 690-760).
- F6 (#390): without SpecSync, `session new; stage advance --to done` prints
  `Move: intent -> done` and archives, with no warning and no human decision.

REQ-cli-061 pins the summary's wording (cases; `git diff <base>..<HEAD>`), so F4 and
F14 change approved sentences, and #390 adds a refusal: this scope needs Leif's own
approval, not the standing OK.
