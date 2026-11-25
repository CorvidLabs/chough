# Docs — fix-439

- `docs/cli.md`: the refused-check row in the `next`/`status` notes
  table changes from "the reason is not shown yet" to naming
  `verification-attempts.json` as the source the CLI reads the refusal
  from; the `run specsync change check <id>` pointer stays.
- No new subcommands, no CLI surface change.
- No changelog entry needed beyond changelog.d note if the repo's
  convention requires one for user-visible text (add one line: refused
  checks now show the recorded reason).