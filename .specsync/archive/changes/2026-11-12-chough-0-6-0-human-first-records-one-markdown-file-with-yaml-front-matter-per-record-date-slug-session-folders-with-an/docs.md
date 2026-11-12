---
change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
artifact: docs
---

# Docs

Every document that names a 0.5.x record path, command or rule is updated in build:

- `docs/cli.md`: the record tree, every changed command and flag (`--holds`,
  `--drives`, `--redact`, `--screenshot`, `--failing-lines`, `correct`,
  `journal add`, `migrate`), the removed commands, `refused:unmigrated`,
  `refused:leak` and `refused:human-identity-unknown`, and the identity file.
- `docs/architecture.md`, `docs/FRD.md` (requirement map, retired IDs),
  `docs/lifecycle.md`, `docs/walkthrough.md`, `docs/DECISIONS.md` (the format,
  gate and approval decisions with their sources), `docs/exercises/`.
- `README.md`: what a project's `.chough/` looks like now and the one migration step.
- `AGENTS.md` and `CLAUDE.md`: sessions are folders, timelines replace event files,
  the merge-driver setup shrinks to `chough bug merge`, the after-merge
  `library index` step stays.
- `CHANGELOG.md`: a 0.6.0 section. Breaking: the record format; run
  `chough migrate`; Chough 0.5.x cannot read 0.6.0 records, so upgrade every agent
  before a project migrates. Lists removed and new commands.
- The skill bundle (`skills/chough-workflow/`) and its installed copy
  (`.agents/skills/chough-workflow/`): SKILL.md resume steps, checkpoint (the session
  file replaces session.json and transitions), plan (a human's gate decision drives
  approval as that human; agents acting alone approve as themselves), lifecycle,
  accept (the two-commit finish), bugs, librarian (link, never copy; journal),
  testing (logs local and in CI; screenshots: check for personal information first),
  stages, and the cases template. The release manifest pin follows.
- `.chough/README.md` and `knowledge/index.md` are new generated guides
  (REQ-records-002, REQ-cli-043).
- `specs/records/` is created; `specs/cli`, `specs/history` and `specs/reporting`
  companions and change logs are updated by materialization, and the history and
  reporting context pages note the retired IDs.
