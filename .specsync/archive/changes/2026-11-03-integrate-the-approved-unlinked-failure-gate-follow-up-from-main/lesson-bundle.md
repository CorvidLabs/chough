# Lesson bundle: integrate-the-approved-unlinked-failure-gate-follow-up-from-main

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Integrate the approved unlinked-failure gate follow-up from main
- **Kind**: Documentation
- **Paths**: src/stage.rs, tests/stages.rs, docs/cli.md
- **Acceptance**: Preserve the exact approved unlinked-failure and later-not-run gate behavior and its tests from pinned main3d00e44 while retaining PR44 user-doc actor/source guidance, with no proof/archive rewrite or native gate bypass; qualify the combined main result under Rust1.95/1.88 and required Fledge/cases before ordinary branch push, with native review/ship reserved for root.

## Evidence

- Verification commit: `7b63a415088a9bbc1263854cfa4d4e3d3f7eb9b2`
- Base commit: `e4eadd817a38127a1ebf49815b599cd89cfbefd0`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

While preparing the approved PR44 integration of main8fb0ea7, shared origin/main
advanced to3d00e44e184e83aa8f92dc1283fefe482745135d after PR43 merged. The first
merge attempt was automatically conflicted and was aborted before manual source
resolution. The first integration scope remains preserved and owns common bundle,
canonical metadata, case definitions and records integration.

This additional disjoint scope owns only src/stage.rs, tests/stages.rs and
docs/cli.md, which the earlier scope did not declare. It imports the already
approved and archived43 gate: referenced failed case observations remain refused
until linked to a local bug, and a later not-run does not hide the latest executed
failure. Retain that exact behavior and its tests, plus PR44's agent actor/source
examples in the same CLI reference. No new product or canonical meaning is added.

Actual human authority remains0xLeif's standing2026-11-02 through-PR instruction
relayed at https://github.com/CorvidLabs/chough/issues/11#issuecomment-6581702299
and his later "Can you finish up all these?" request after existing PR status and
closeout gates were presented. The acting project account is corvid-agent. Covered
scope is this bounded follow-up integration and qualification. Root remains sole
native review/ship writer; PR merge, release, deployment and live cleanup remain
separate. No new as-is human acceptance is asserted.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
