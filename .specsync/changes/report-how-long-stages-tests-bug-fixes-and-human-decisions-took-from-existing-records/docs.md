---
change: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
artifact: docs
---

# Docs

- docs/cli.md: `chough timing` options, sections and labels; the `gate-opened` and
  `gate-closed` transition kinds; bounds and unknown reasons.
- docs/architecture.md: records table notes that transitions now include gate records
  and that timing is derived, not stored.
- README.md: command list.
- docs/FRD.md: LIFE-10 trace.
- skills/chough-workflow/references/checkpoint.md: gate records are written by
  `--awaiting-human` and `--clear-awaiting-human`.
- skills/chough-workflow/references/plan.md: estimate from `chough timing` ranges,
  say when history is too thin, never present a range as a target; installed copies
  under .claude/skills and .agents/skills are refreshed.
