# Lesson bundle: make-native-workflow-stage-references-and-installed-copies-consistent

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Make native workflow stage references and installed copies consistent
- **Kind**: Documentation
- **Paths**: skills/chough-workflow/references/plan.md, skills/chough-workflow/references/accept.md, .agents/skills/chough-workflow/, .claude/skills/chough-workflow/
- **Acceptance**: Plan applies existing explicit authority before asking any missing decision, native writer examples use the acting agent account with actual human authority as source, accept preserves the actual implementation acceptance boundary, managed repository host copies match canonical guidance without overwriting edits, and cross-bundle actor/copy consistency plus safe predecessor upgrades are tested.

## Evidence

- Verification commit: `e2a3e5876d35d8ea7eeefb5787ced352190abd58`
- Base commit: `7cd739219ff667dcf6d153a6f439d6c3661b1fce`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

PR44 implements REQ-cli-029 in its primary native change. Peer review found that
plan/accept references and tracked installed copies still contradict the new
acting-agent/human-authority rule. This follow-up completes that existing contract
without changing its canonical meaning or introducing a helper command or runtime.

Actual authority is 0xLeif's standing chat permission of 2026-11-02 for needed
repository changes delivered through PRs, relayed in issue11comment6581702299.
The acting project account is corvid-agent; the human instruction is cited as the
separate source, scope and limits. Existing approval history and all proof remain.

Declared ownership is plan.md, accept.md and the installed openai/claude bundles.
The primary change retains src/skill.rs and tests/workflow_authority.rs, where
additive predecessor registration and package consistency support are implemented.
The two native scopes have disjoint declared paths and one shared PR/checkpoint.
Grok41's case-order paragraphs must remain intact when the changes integrate.

Only ordinary native operations already authorized by the human are automated.
Actual implementation acceptance and merge/release/deployment stay separate. No
fresh-agent cold-start run or new human decision is asserted by this refinement.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
