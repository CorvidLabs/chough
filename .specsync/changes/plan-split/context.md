---
change: plan-split
artifact: context
---

# Context

Leif agreed in chat on 2026-11-21 how the plan stage may split execution across
workers and how the test stage runs its fix-and-rerun loop. This change writes that
agreement into the workflow skill (the source bundle `skills/chough-workflow/` and
its installed copy `.agents/skills/chough-workflow/`, which must stay equal), with
tests that pin the wording. Leif's words are the intent being written down, not a
recorded approval; the scope gate still needs his decision.

Constraints already agreed:

- The plan stage keeps its single scope approval; the split is optional guidance
  inside it, not a second gate.
- Fan out only when the cuts share no files, each cut fits its named context
  budget, and history has at least 3 similar samples showing a wall-clock win.
  Fewer than 3 samples is too thin; say so. Do not invent a speed.
- Chough does not launch agents and adds no model client. Workers report with
  `chough inbox add`; the session owner folds the inbox.
- After the slices the session records worker count, wall-clock, conflicts, inbox
  questions and stage-backs; tokens only when the host reports a number, otherwise
  unknown. The next plan reads that the way it reads `chough timing`. No second
  timing store.
- A later slice may be replanned inside the approved paths; a change to intent,
  contract or paths goes back through scope approval.
- The test stage is a bounded loop on the focused set: run the change's checks and
  selected cases, fix, rerun; the project lane still runs once before test ends.
  Group failures by cause before filing; search existing bugs first; ten
  assertions with one trigger are one group. A fixed group becomes one lesson or
  one bug; a group still open at the end is one bug with every failed observation
  linked. Stop after a few passes or when the focused set is green; name what
  remains. The validate side stage stays the long repeated run and still needs
  consent.
- No new Rust command, no token database, no model client. If an agreed record
  cannot be written with existing commands, stop and name the missing command.

<!-- What a session picking this up mid-flight needs to know: constraints,
     prior attempts, anything already ruled out. -->

Already ruled out: adding Chough commands or flags for any of this (Leif: if the
records cannot be written with the commands that already exist, stop and say which
command is missing; do not invent a flag). Editing `specs/*/requirements.md` by
hand is forbidden; SpecSync applies an approved delta, and none is needed here
because no canonical spec sentence changes.