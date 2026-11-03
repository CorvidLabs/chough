# Lesson bundle: integrate-merged-bounded-history-into-the-agent-operated-native-workflow

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Integrate merged bounded history into the agent-operated native workflow
- **Kind**: Refactor
- **Specs**: cli
- **Paths**: src/skill.rs, CHANGELOG.md, docs/ROADMAP.md, docs/FRD.md, docs/cli.md, specs/cli/cli.spec.md, specs/cli/context.md, specs/cli/requirements.md, specs/cli/testing.md, .chough/cases/cli.cases.toml, skills/chough-workflow/references/checkpoint.md, .agents/skills/chough-workflow/references/checkpoint.md, .claude/skills/chough-workflow/references/checkpoint.md, tests/package.rs, tests/workflow_authority.rs
- **Acceptance**: Integrate already-merged main a788869 into PR44 with ordinary merge and canonical version23, preserving approved history/retention and agent-operated native workflow meanings, all149 case definitions, all45 predecessor identities, one complete selected PR44 current-cycle group, every additive collection and legacy transition occurrence, and byte-identical existing archives/cold-start/evidence. Pass all required Fledge steps, complete Rust1.95/1.88 tests with measured counts, affected cases and native verification before guarded normal push; stop at accept with current committed verification for root-only review/ship/done. No force/rebase, worker merge, live adoption/cleanup, release or deployment.

## Evidence

- Verification commit: `f59bc0a7fe736268479f1a21104e68c97b4a4ef7`
- Base commit: `3e25fb494c15cb64d4e9f3ef209729e96fc968a5`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Acting project agent: corvid-agent. Human source: 0xLeif explicitly said "Merge approved, green PRs and resynchronize" in chat on 2026-11-02, supplementing the standing through-PR permission relayed at https://github.com/CorvidLabs/chough/issues/11#issuecomment-6581702299 and "Can you finish up all these?". Scope: resynchronize PR44 with already squash-merged PR37 at main a788869b32b8442cbc8d7be2bf19c19a922954a8, preserve both accepted implementations and immutable proof, qualify and push normally. Root alone records native review/ship and done; this worker never merges. No new as-is human acceptance, release, deployment, live history adoption or cleanup is inferred.

Qualified PR44 source is 2caacd5, final archived tip3e25fb4. Main a788869 imports PR37 source qualified at6c1999e plus checkpoint-only authority2ec0d20. Both contracts are22; integrate to23 without changing approved meanings. The shared legacy base is d839fef. Preserve all1,909 protected parent paths, all case definitions and transition occurrences. Current-cycle selection keeps one complete PR44 cycle, with additive histories retained. No live adoption or retention apply.

The predecessor failure recorded by BUG-integrated-current-main-workflow-references-are-misclassified-as-edits-8cbff17ede6b1bad3926bce3 is verified. The module lessons require exact predecessor coverage and honest source/package versus cold-start limits.

## From the change's design.md

# Design

This is integration of two accepted implementations, with no new product behavior. Preserve the complete history implementation and tests from merged main a788869, retain the current agent-operated workflow and both host copies, and combine canonical ownership/requirement metadata at version23. src/skill.rs keeps the union of45 known predecessor identities. The project checkpoint remains schema2 without a history cursor; no live adoption runs. One coherent PR44 current-cycle group is selected; all collections and every parent transition occurrence are retained. Existing evidence, archives, cold-start fixtures and frozen predecessors stay byte-identical.

No new runtime, lifecycle state, transport, storage service or user-facing approval question is introduced. Root retains native closeout; merge follows the explicit human merge authorization and current-head gates.

## From the change's testing.md

# Testing

Run the existing package checkpoint safe-update regression and both workflow_authority regressions, plus all affected history/retention cases and the source ordering case. Keep case execution outside tasks.md checkboxes. Complete implementation tasks and run the scoped check before validating or recording cases. Required gates are all9 Fledge verify steps and every Rust test on1.95 and1.88, with actual counts measured and no exclusions. Verify149 case IDs, all protected parent blob identities, exact requirements and all45 predecessor registrations. Retain new raw logs and source identities; never modify prior proof.

Limits: local macOS execution is distinct from hosted CI; source/package checks are not fresh-agent behavior, Kimi is not run, existing timing flake remains open unless reproduced and genuinely fixed, and no live project adoption or cleanup occurs. Root alone records native review/ship/done.

## Where these lessons go

- `specs/cli/context.md`
