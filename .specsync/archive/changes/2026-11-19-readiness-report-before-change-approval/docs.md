---
change: readiness-report-before-change-approval
artifact: docs
---

# Docs

- `docs/cli.md` Commands table: the `change` row gains `ready`. The passthrough
  table and the sentence that those steps add nothing stay as they are. A new
  section after that table describes `chough change ready <id>`: read-only, exit
  0 or 1, and the six finding classes with the fix each one names.
- `README.md` "What the helper does": the `change` row gains `ready` and says
  the report lists what is still missing before approval.
- `CHANGELOG.md` Unreleased: one bullet for `chough change ready` (REQ-cli-067,
  DRIVE-29).
- `skills/chough-workflow/references/stages.md` and the same file under
  `.agents/skills/chough-workflow/references/`: the fast path runs
  `chough change ready <id>` after the artifacts are filled and before the
  human's scope approval.
- `skills/chough-workflow/references/plan.md` and its `.agents` copy: run
  `chough change ready <id>` before the scope approval. The delta section keeps
  `## ADDED`, `## MODIFIED` and `### REQUIREMENT`.
