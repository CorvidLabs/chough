# Lesson bundle: plan-split

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Plan split
- **Kind**: Documentation
- **Specs**: cli
- **Paths**: skills/chough-workflow, .agents/skills/chough-workflow, tests, changelog.d, src/skill.rs
- **Acceptance**: The workflow skill's plan reference teaches an optional fork inside the one scope approval: one question, whether the slice can be cut into a left half and a right half with no shared file; one worker when it cannot; one manager and two workers when it can, the manager merging both reports and writing neither half; one fork as the batch, with another fork only when a half still has its own seam and the total stays at about four agents. After the merge the session records whether the halves joined cleanly, the wall-clock, and any conflict. The testing reference teaches the bounded fix-and-rerun loop on the focused set with grouping by cause before filing and one bug or one lesson per group, and the stages and build references agree; tests pin those sentences and the installed skill copy matches the source.

## Evidence

- Verification commit: `d070e731ad78b92457c3617597a391f2ff8fab83`
- Base commit: `51b76c626599ce28494a23b1edd740d1dff21854`
- Verified by: `specsync check --spec cli --spec history --spec records --spec reporting`

## From the change's context.md

# Context

Leif agreed in chat on 2026-11-21 how the plan stage may split execution across
workers and how the test stage runs its fix-and-rerun loop. He later replaced the
split with a fork: "Yes, replace it with the fork." This change writes that
agreement into the workflow skill (the source bundle `skills/chough-workflow/` and
its installed copy `.agents/skills/chough-workflow/`, which must stay equal), with
tests that pin the wording.

Constraints already agreed:

- The plan stage keeps its single scope approval. A fork is optional guidance
  inside it, not a second gate.
- One question: can this slice be cut into a left half and a right half with no
  shared file? If it cannot, one worker does the whole slice. If it can, one
  manager spawns two workers. Each worker has its own clone and its own session.
  Do not use a worktree for a fork. The two file lists share no file. A worker that
  finishes stops and does not edit the other half. The manager does not write either
  half. Before the merge, a path outside that worker's list is left out, and the
  other worker keeps going. The manager merges both reports into one answer. A shared
  file stays with the manager. One fork is the batch. A half is forked again only when
  that half still has its own seam, and the total stays at about four agents.
- Chough does not launch the agents and adds no model client. The host spawns them.
  Workers report with `chough inbox add`, and the manager folds the inbox.
- After the merge the session records whether the two halves joined cleanly, the
  wall-clock, and any conflict. That is the sample the next fork reads. Tokens only
  when the host reports a number, otherwise unknown. No second timing store.
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

## Where these lessons go

- `specs/cli/cli.3md`
