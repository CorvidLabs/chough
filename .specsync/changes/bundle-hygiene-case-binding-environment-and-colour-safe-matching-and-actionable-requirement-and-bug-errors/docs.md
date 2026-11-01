---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: docs
---

# Docs

- docs/cli.md: the `env` binding field and its bounds (committed, not for secrets,
  names recorded as a limitation, the refused cargo-test names, a declared PATH can
  change what a command argv resolves to); the standing limitation that inherited
  variables are not recorded; escape removal before matching and the raw artifact;
  requirement and bug error hints; and, marked as a change from earlier releases, the
  refusal of expect_output fragments containing ESC.
- docs/architecture.md: artifacts stay inside the project; keep screenshots and other
  binaries small and, if needed, ignore them with Git (.gitignore or
  .git/info/exclude); a clone without the file reads stale-artifact, never passed. Git
  LFS is not recommended while SpecSync `change check` refuses tracked LFS files
  (CorvidLabs/spec-sync#789).
- skills/chough-workflow/references/testing.md: the same artifact guidance, `env`
  (with NO_COLOR as an opt-in example) and escape removal. No accept-order text: that
  is the evidence-freshness change's.
- Installed copies under .agents/ and .claude/ refreshed from the bundle.
- Pull request description: names the ESC fragment refusal as the one change that can
  break an existing case file.
