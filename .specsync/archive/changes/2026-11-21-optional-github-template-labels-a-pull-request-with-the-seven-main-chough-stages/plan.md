---
change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
artifact: plan
---

# Plan

## Approach

Add REQ-cli-070 and ship the template under `integrations/github/`. Copy the same workflow and script into `.github/workflows/` and `tools/` so this repository uses what it ships. A test reads `const ORDER` from `src/stage.rs` and fails when the template's stage names drift from it. Another test runs the script against a temporary git repository and a stand-in `gh`.

## Out of scope

`chough init` does not grow a flag that installs the workflow. `src/` does not call `gh`. Release and the side stages are not progress labels. Jackdaw's copy is updated in Jackdaw, after this template exists. No change to stage gates or to SpecSync status.

## Steps

1. Write the delta, the template, the dogfood copies, the docs, the changelog fragment and the case.
2. Approve under the user's 2026-11-21 instruction to add this shape.
3. Run the new test, then `chough change check`.

## Risks

The label workflow's token can create labels and cannot merge or push. A project whose default branch is not `main` must edit the `push` trigger or push events will not relabel. Until this is on `main`, this pull request is the first run.

## Constraints consulted

Pin `actions/checkout` to the commit `.github/workflows/ci.yml` already uses. `tasks.md` has no unchecked boxes. The delta stays as approved.
