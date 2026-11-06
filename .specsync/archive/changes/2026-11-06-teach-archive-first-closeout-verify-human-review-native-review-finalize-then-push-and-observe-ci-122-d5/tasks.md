---
change: teach-archive-first-closeout-verify-human-review-native-review-finalize-then-push-and-observe-ci-122-d5
artifact: tasks
---

# Tasks

- [x] Add the failing test `closeout_guidance_pushes_for_ci_only_after_finalize` in tests/workflow_authority.rs.
- [x] Rewrite the closeout order in the lifecycle and accept references (bundled and installed), docs/lifecycle.md, the walkthrough and the lifecycle diagram. Keep the no-intervening-commit rule.
- [x] Re-render `docs/diagrams/lifecycle.svg` with `docs/diagrams/render.mjs`.
- [x] Add the shipped 0.4.0 digests of `references/lifecycle.md` and `references/accept.md` to `KNOWN_PREVIOUS` in src/skill.rs, so installed copies update.
- [x] Extend `examples/feature-bug-loop.sh` to run the corrected closeout against the archive-first gate. The gate must refuse while the change is open and pass once the change is archived, before any CI. The human actor is simulated and labeled synthetic.
- [x] Add a CHANGELOG line, and run verify and the exercise on Linux.
