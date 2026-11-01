---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: docs
---

# Docs

- docs/cli.md: the `env` binding field and its bounds (committed, not for secrets,
  names recorded as a limitation); escape removal before matching and the raw
  artifact; the ESC fragment refusal; requirement and bug error hints.
- docs/architecture.md: artifacts stay inside the project; keep screenshots and other
  binaries small, ignore them with Git or track them with Git LFS; a clone without
  the file reads stale-artifact, never passed.
- skills/chough-workflow/references/testing.md: the same artifact guidance, `env`
  (with NO_COLOR as an opt-in example), escape removal, and running cases for new
  requirement IDs only after `specsync change check <id> --commit`.
- skills/chough-workflow/references/build.md and plan.md: tasks.md lists
  implementation work only; case recording for new IDs, push, CI and review are
  plain-text post-verification steps.
- Installed copies under .agents/ and .claude/ refreshed from the bundle.
