---
change: new-project-setup-path-and-sdd-off-hint-document-hi-specsync-init-scaffold-and-specsync-change-adopt-for-a-new-project
artifact: context
---

# Context

Issue #40 came from a Linux x86_64 dogfood run of a new sample project. Versions were chough from main 825a380, hi 0.8.0, specsync 6.1.2 and fledge 1.8.0. `specsync init` writes `.specsync/sdd.json` with `enabled: false` and prints "SDD change workflow is off. Enable with `specsync change adopt`". After that, `chough next` said only "Approved deltas: not checked: no native lifecycle", and plan -> build passed as `ungated:sdd-disabled`. Nothing in Chough's output or docs pointed at `specsync change adopt`. Once adopted, the same view said "not checked: no native change recorded". The reproduction is in `.chough/artifacts/bug40-repro.log`.

The docs also lag. The walkthrough shows the legacy `BUG-0001` form, and the exercise notes record specsync 6.1.1 while the README pins 6.1.2.

Constraints: PR #44 (0xLeif) owns `SKILL.md`, `references/lifecycle.md` and the README rewrite, so README gets one pointer line only. Chough reports native state and never runs `adopt` for the user.
