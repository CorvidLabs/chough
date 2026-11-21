---
change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
artifact: docs
---

# Docs

Each fix makes a page say what 0.6.2 does; no code changes behavior.

- **Closeout:** `docs/lifecycle.md`, `docs/cli.md` and the skill's lifecycle, stages and
  accept references commit the session folder with the story and the journal post, since
  `doctor --fix` removes nothing while `session.md` is uncommitted.
- **Diagrams:** `lifecycle.mmd` shows authorized release keeping the session open at done,
  and the first commit holding the session folder; `ownership.mmd` adds Chough running
  SpecSync's change steps and the human's approval or review. Both SVGs re-rendered.
- **Gating:** `docs/lifecycle.md` and the skill name `refused:no-human-decision` and
  `--reason`, `--no-native-change`, the no-`sdd.json` case, the archived-change order,
  and moving to accept before the review gate.
- **`docs/architecture.md`:** what closes a session, the story's layout (receipts last),
  the checkpoint pointer with `--session`, `session close <key>`, Chough running SpecSync's
  writers.
- **`docs/cli.md`:** temp files in `.chough/local/`, the leak scan's `.specsync/changes/`,
  no-hard-link scope, `pref unset --source`, the change steps' local baseline, the story's
  bugs, `name` versus `--decided-by`, SpecSync's recorded refusals, version labels.
- **Other pages:** README (0.6.2 table, no-commit snapshot), `ci.md` (pin test name),
  `FRD.md` (two rows), `roadmap-1.0.md` (closed flakes, 0.6.0 released, the 0.6.2 row),
  `releasing.md`, `test-plan-1.0.md`, `DECISIONS.md` (D-021's pin now), two exercises.
- **Skill:** also the native status fallback, LFS (spec-sync#789 closed) and the fledge
  plugin. `src/skill.rs` lists the six replaced v0.6.2 files as known previous, with the
  0.6.2 release table its test asks for, and the skill-release fixture follows, so an
  installed 0.6.2 skill updates.
