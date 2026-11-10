---
change: capture-the-2026-11-10-mvp-interview-as-hi-criteria-and-add-the-1-0-roadmap-test-plan-and-draft-issue-mapping
artifact: context
---

# Context

Leif's MVP / 1.0 interview of 2026-11-10 (audit file
`chough-mvp-interview-2026-11-10.md`, held outside this repo) decided what 1.0
means, the four-part bar, the host scope, the session slug-folder design,
digest-committed evidence, KNOW fully in scope, Herdr/Rune out of core, and the
weekly 0.x release cadence. The same day he decided #14 (fix all four case
bindings), #165 (roll-up copies, never moves), #167 (append-only `chough
correct`) and #201 (record agent, model and hc terminal).

This change captures those decisions as hi criteria and turns them into a
roadmap, a test plan and a draft issue list. No code changes.

Constraints: hi IDs are never reused or renumbered; existing criteria are
never reworded or removed, so the FLOW-17/FLOW-21 and KNOW-29/KNOW-30 tensions
are flagged in the PR body rather than edited. DRIVE-23 is reserved by the
in-flight slice-6 branch, so the new DRIVE criterion is DRIVE-24. Docs must
match current main behavior (FLOW-18); the docs drift suite in tests/docs.rs
enforces links and command tables.
