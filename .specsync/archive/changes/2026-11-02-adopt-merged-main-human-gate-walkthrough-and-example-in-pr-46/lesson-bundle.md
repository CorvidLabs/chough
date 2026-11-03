# Lesson bundle: adopt-merged-main-human-gate-walkthrough-and-example-in-pr-46

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Adopt merged main human-gate walkthrough and example in PR 46
- **Kind**: Operations
- **Specs**: cli
- **Paths**: docs/walkthrough.md, examples/feature-bug-loop.sh
- **Acceptance**: The main 9d02 walkthrough and feature-bug example remain byte-identical after integration; the full mandatory lane and Rust 1.88 tests pass with both preserved main gates and report reads

## Evidence

- Verification commit: `0df1cd971acf4588491d9a3c97fe701b9c4b5f59`
- Base commit: `9dd30dd5d27720c33fab3fcf97dbce345fe3d482`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Main advanced to 9d02 after the first integration was approved. Its docs/walkthrough.md and examples/feature-bug-loop.sh were outside that change's exact file list. This narrow change owns adopting those two already approved files unchanged; the first integration owns the accompanying stage, tests, canonical metadata and report reads.

Authority is 0xLeif's standing permission for ticket changes through PRs and his 2026-11-02 instruction "Can you finish up all these?" The acting project account is corvid-agent. No new as-is acceptance is claimed. Root alone records native review and ship; Leif merges. Existing archived proof and Kimi not-run boundaries remain intact.

## From the change's design.md

# Design

No new architecture or contract is introduced. This change adopts two already approved main files unchanged. Native ownership, product qualification and root closeout remain separate records.

## From the change's testing.md

# Testing

Compare both files exactly with origin/main at 9d02. Run the mandatory nine-step Fledge lane including the real disposable feature-bug loop; run all Rust tests on 1.95 and 1.88 and the three existing report cases. Preserve any failure. Refresh native verification and audit for both integration changes. Hosted archive checks remain pending root closeout.

## Where these lessons go

- `specs/cli/context.md`
