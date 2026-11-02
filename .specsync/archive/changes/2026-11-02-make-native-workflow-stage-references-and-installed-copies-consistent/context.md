---
change: make-native-workflow-stage-references-and-installed-copies-consistent
artifact: context
---

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
