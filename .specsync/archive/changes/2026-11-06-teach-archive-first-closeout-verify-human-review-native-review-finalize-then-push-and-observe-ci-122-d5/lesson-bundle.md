# Lesson bundle: teach-archive-first-closeout-verify-human-review-native-review-finalize-then-push-and-observe-ci-122-d5

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Teach archive-first closeout: verify, human review, native review, finalize, then push and observe CI (122, D5)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: skills/chough-workflow/references/lifecycle.md, skills/chough-workflow/references/accept.md, .agents/skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/accept.md, docs/lifecycle.md, docs/walkthrough.md, docs/diagrams/lifecycle.mmd, docs/diagrams/lifecycle.svg, src/skill.rs, tests/workflow_authority.rs, examples/feature-bug-loop.sh, CHANGELOG.md
- **Acceptance**: Every place that teaches the closeout (bundled and installed lifecycle and accept references, docs/lifecycle.md, the walkthrough and the diagram) puts pushing and observing CI after local verification, the human implementation review, the native review and finalize, keeping the no-intervening-commit rule; installed 0.4.0 copies update; the exercise runs the corrected closeout against the archive-first inspector, which refuses the open change and passes once archived. The new test fails first, and fledge lanes run verify and the exercise pass on Linux.

## Evidence

- Verification commit: `fde7074045b684a43b8292db1c390d41cae70488`
- Base commit: `d3b301e35fcf78430da3ca49b172ae00babaa657`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## The finding

Issue #122, audit finding D5. This is the Chough side; spec-sync#881 is its counterpart. The audit was of main `700dad96`.

Chough's guidance tells an agent to push and observe required CI before the human implementation review and finalize. The hosted Archive gate is archive-first: it refuses while any native change is open. So following the guidance meets a red gate and either waits or works around it.

These places teach the wrong order:
- the shape in `skills/chough-workflow/references/lifecycle.md` (lines 16 to 19 and 82);
- step 6 of `references/accept.md`;
- the closeout diagram and text of `docs/lifecycle.md`;
- `docs/walkthrough.md`.

The installed `.agents/` copies carry the same text.

## Expected

Per the issue: local verification, then the real human implementation review, then the scoped native review, then finalize and archive, and only then push and observe CI before merge. The existing no-intervening-commit rule and every human gate stay as they are.

## Priority

Leif, 2026-11-05 18:22 MST via the orc: C1 is the first Chough item of the 95% plan, after KNOW-23.

## From the change's design.md

# Design

## Approach

Every place that teaches the closeout gets one order:
1. local checks;
2. the awaiting-human gate;
3. commit with `.chough/`;
4. `check --commit`;
5. present the work;
6. the human's implementation review;
7. `specsync change review`;
8. `ship` or `finalize`, on the same branch with no commit in between;
9. clear the gate and run `stage advance` to done;
10. commit, then push;
11. observe CI at the real tip;
12. merge.

The Archive gate is red only until step 8, so CI is observed once, on an archived tip. The no-intervening-commit rule and FLOW-10 are unchanged. The installed `.agents` copies are synced. `KNOWN_PREVIOUS` gains the 0.4.0 digests of the two references. The diagram is re-rendered with the existing renderer.

## Risks

- **An earlier push is still allowed.** A team may still want an early CI run on a draft PR. The guidance says it may push earlier for visibility but must expect the archive gate to be red and must not wait on it.
- **A synthetic actor.** The exercise's simulated reviewer is labeled synthetic, exactly as the archive fixtures label theirs.

## Constraints consulted

- FLOW-10 and REQ-cli-031 (no commit between review and finalize).
- docs/ci.md (the archive gate).
- Issue #122 and the audit's expected recovery.

## From the change's testing.md

# Testing

## tests/workflow_authority.rs

`closeout_guidance_pushes_for_ci_only_after_finalize` checks the bundled and installed lifecycle and accept references, docs/lifecycle.md, the walkthrough and the diagram source. In each, every instruction to push and observe CI must come after the first mention of finalize.

It fails first: the lifecycle shape puts "push, CI" before the human implementation review.

## Exercise

`fledge run exercise` (case TC-cli-0101) follows the documented local closeout:
1. Before the review, the archive inspector `.github/scripts/check-specsync-archive.py` refuses the open change.
2. A simulated reviewer runs the native review, then finalize.
3. The inspector then passes, with no CI involved.

The simulated actor is a fixture claim, never Leif's identity.

## Installed copies

An installed copy from 0.4.0 updates, because its digests are added to `KNOWN_PREVIOUS`. The existing upgrade tests still pass.

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
