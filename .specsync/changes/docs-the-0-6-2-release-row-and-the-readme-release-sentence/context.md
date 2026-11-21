---
change: docs-the-0-6-2-release-row-and-the-readme-release-sentence
artifact: context
---

# Context

Chough 0.6.2 is released: #432 merged as 1bac91a4, tag `v0.6.2` points at it, crates.io
serves `chough` 0.6.2 (2026-11-21T00:29Z) and the GitHub release is published. Leif's
"Yes, release 0.6.2 (Recommended)" (2026-11-20 about 23:59Z, relayed by his planning
session, hc t4c) covers the release steps; step 9 of `docs/releasing.md` is the
roadmap's delivered row.

Leif then asked for the README and all documentation, the Mermaid diagrams included, to
be checked and made accurate for 0.6.2 (relayed by t4c): every command, flag, output,
diagram stage, gate and arrow against what 0.6.2 does. Documentation only; anything
that is a code bug rather than a doc bug is filed, not fixed (#433 to #440).

The audit covered README.md, every page under `docs/`, the four diagrams, the workflow
skill (bundle and installed copy) and the exercises, by running the 0.6.2 binary and
reading `src/` and the specs. The clean-install observation of release step 8 is
recorded in this session.
