---
change: focused-chough-next-a-verification-guard-before-build-and-done-stage-resume
artifact: docs
---

# Docs

- docs/cli.md: `next` as the compact action view (fields, text default, `--json`,
  16384-byte cap, list and string bounds, one native read), marked as a change from
  earlier builds that printed the status report; `status` as the full view that always
  prints JSON; the done-stage new-cycle action; reset replacing next_action; the
  `refused:native-delta-changed` and `refused:native-delta-unverifiable` labels with
  their next actions.
- docs/architecture.md: the guard reads the native approval ledger and deltas
  read-only under a 4 MiB bound, recomputes SpecSync 6.1.1's digests and copies
  nothing; why `change check` is not used by gates.
- docs/integrations.md: a Google Antigravity row (unverified, narrative only, closed
  PR #20) and the Gemini row renamed Gemini CLI, not exercised.
- docs/exercises/cold-start.md: the dated Antigravity section; the Codex record is
  unchanged.
- README.md: the quick start's `chough next` shows the text view; the host list says
  `.agents/skills` is shared.
- skills/chough-workflow/SKILL.md (host paragraph) and references/checkpoint.md
  (done-stage resume); installed copies under .agents/ and .claude/ refreshed.
- examples/feature-bug-loop.sh and examples/visual-design.sh: `chough next --json`.
- Pull request description: the `chough next` output change and before and after
  sizes.
