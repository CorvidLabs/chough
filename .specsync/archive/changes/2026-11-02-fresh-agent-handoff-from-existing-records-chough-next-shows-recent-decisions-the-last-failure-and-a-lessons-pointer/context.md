---
change: fresh-agent-handoff-from-existing-records-chough-next-shows-recent-decisions-the-last-failure-and-a-lessons-pointer
artifact: context
---

# Context

0xLeif, 2026-11-01, directed a new cycle: project-local learning and reliable
handoffs. This change is the first slice. Asked the open design questions, he settled
them (answers to design questions, not scope approval of this definition):

- Cold-start proof: freeze a seeded fixture repository and a written pass/fail rubric
  before implementation and hash them as an observation; run a fresh agent on a
  different host (Kimi Code CLI 0.41.0 or Codex CLI 0.153.4) on main as a mandatory
  baseline, then on the change; 0xLeif grades; plus one run on this real repository.
- Lessons live in the existing `specs/<module>/context.md` Lessons section, which
  SpecSync already briefs. Chough only points at them. No `.3md`, no `library/` store.
- Assumption to state in the design: decisions stay `Vec<String>` in the checkpoint; a
  prefix convention is read best-effort, with a latest-N fallback; no checkpoint
  schema change.

What is true at fb6e487 (origin/main), checked in this worktree:

- The checkpoint was still at stage `test` for the archived 0.1.0 change, and
  `chough next` said `Continue test: specsync change check --commit, push, CI after
  archive, then 0xLeif's implementation review` while native status read archived with
  authenticated terminal evidence and named `merge the PR on GitHub if it is still
  open`. The Action chain in `src/stage.rs` (around lines 783 to 797) reaches
  `done_action` only at stage done; at test or accept the archived change opens the
  gate, so the final `Continue <stage>` branch wins. The reset that opened this cycle
  (`chough stage reset --to plan`, transition at fb6e487) moved this repository off
  that state; the defect remains for any checkpoint left behind an archive.
- `chough next` (REQ-cli-021, `src/main.rs` `next`) shows the objective, stage, native
  summary, next action, gate, human gates, blockers, blocking bugs and stale
  observations, within 16384 bytes. It shows no decisions, no failure summary and no
  lesson pointer; those are only in `chough context` (69,721 bytes here) and `chough
  status` (515,382 bytes here).
- Decisions are free text (`SessionV1.decisions: Vec<String>`, `deny_unknown_fields`,
  `src/checkpoint.rs`); REQ-cli-007 requires an explicit migration for any schema
  change. Observations carry `outcome` (`Outcome` in `src/evidence.rs`) and
  `observed_at_unix_seconds`; bugs link observations by their observations, note
  observations and regression baseline (`bugs::observation_references`).
- SpecSync 6.1.2 briefs lessons from `specs/<module>/context.md` (or a parsed
  `<module>.3md`) on `change new`, `show` and `status`. It writes a `lesson-bundle.md`
  into each archive but never surfaces those bundles and keeps no fold marker; this
  repository has 14 of them. `chough context` already prints `<module>.3md` when present
  and `context.md` otherwise (`src/context.rs`).
- Skill guidance: `references/accept.md` Compounding already allows lessons at any
  stage in the native source, with no certainty label and no rule that a decision
  outranks a lesson. The librarian (`references/librarian.md`) is off by default
  (`src/prefs.rs`), and REQ-cli-015 requires checkpoints and resume to work without
  it. A CRLF test failure fixed inside PR #22 left no bug or lesson in `.chough/`.
  `references/lifecycle.md` and `accept.md` still call `verification_ignored_paths`
  "proposed upstream", though SpecSync 6.1.2 ships it.
- The canonical REQ-cli-002 heading carries a title after an em dash (`Focus context
  for resumption`); SpecSync applies a MODIFIED item only to an exact `### REQ-cli-002`
  heading (the REQ-cli-004 precedent in PR #17).

Coordination: draft PR #26 (Kimi, collision-resistant bug IDs) owns REQ-cli-012,
TC-cli-0125 and BUG-0026 and edits `.chough/session.json`. This change does not touch
any of those, starts cases at TC-cli-0126, and keeps checkpoint commits apart from code
commits so the conflict surface stays at `.chough/session.json`.

Ruled out for this slice (from the recon critique): a structured decision schema
(needs a migration and collides with #26); folding or tracking unfolded bundles
(needs a ledger SpecSync does not have, which would be a second lesson authority);
editing `context.md` after archive (a contract-companion edit with no native change
behind it); a new requirement (the blocks fit REQ-cli-021 and the lesson rule fits
REQ-cli-002); repository housekeeping (moving the checkpoint to done, the ROADMAP Kimi
row), which goes to a separate pull request.
