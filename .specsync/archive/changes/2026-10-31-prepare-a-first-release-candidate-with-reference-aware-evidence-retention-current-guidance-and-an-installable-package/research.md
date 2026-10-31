---
change: prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package
artifact: research
---

# Research

- References to observations: checkpoint `observations`; bug `observations` and
  `regression_baseline`; validation run `observations`; evidence `artifacts` (paths
  with digests). Native: SpecSync acceptance manifests list `.chough/` files for
  changes whose paths include them (100 entries in the BUG-0004 archive).
- Native archives authenticate from Git history, not working files (tested in a fresh
  clone of `f19823d`: prune one observation, commit, status stays `authenticated-history`).
- Active changes listing `.chough/` stale their verification on any `.chough` edit
  (observed on PR #7); the policy must not prune files an active manifest lists.
- `cargo package --list` on main: 300 files (100 `.chough`, 42 `.specsync`, 24 skill
  files ×3 copies, docs, examples). The binary embeds only `skills/chough-workflow`.
- `src/skill.rs` `KNOWN_PREVIOUS` lists digests of earlier bundle files; files changed
  since main builds were installed (e.g. `references/stages.md`, `lifecycle.md` on
  PR #7) need their previous digests listed so `skill update` replaces them.
