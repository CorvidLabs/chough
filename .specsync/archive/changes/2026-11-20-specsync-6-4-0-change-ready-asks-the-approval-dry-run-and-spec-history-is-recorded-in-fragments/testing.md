---
change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
artifact: testing
---

# Testing

Red first, with a `specsync` stand-in that prints a dry-run document:

- `tests/ready.rs`: a refused `ownership` check with a `fix` is named in SpecSync's words
  with that fix; a refused `deltas` check without one names `chough change ready <id>`; an
  accepting dry run adds nothing; a stand-in with no dry-run document names the approval
  unchecked with `specsync@6.4.0`; a missing delta is no longer a Chough finding of its
  own; untracked `.specsync/change.lock` and `.specsync/workflow-v2-baseline.json` are
  not named while another untracked `.specsync/` file is; another active change's path
  is not named; no command the stand-in runs finds `.specsync/hashes.json`.
- `tests/package.rs`: `.gitattributes` routes `specs/**/requirements.md`, not
  `specs/**/*.spec.md`, to SpecSync's driver; `AGENTS.md` gives no Change Log
  renumbering advice; every pin names 6.4.0.
- With the real candidate: the drafts of #406's 25 shapes, as the tester ran them on
  #311, each named by ready.
- Cases in `.chough/cases/cli.specsync-6-4.cases.toml`; the full lane; the tester's run
  on the 66 real changes.
