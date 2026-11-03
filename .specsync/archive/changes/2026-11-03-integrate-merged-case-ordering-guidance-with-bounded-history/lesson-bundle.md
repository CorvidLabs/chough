# Lesson bundle: integrate-merged-case-ordering-guidance-with-bounded-history

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Integrate merged case ordering guidance with bounded history
- **Kind**: Documentation
- **Paths**: skills/chough-workflow/references/plan.md, skills/chough-workflow/references/testing.md, skills/chough-workflow/references/accept.md, .agents/skills/chough-workflow/, .claude/skills/chough-workflow/
- **Acceptance**: Preserve already approved case-order guidance and bounded-history guidance together, keep canonical and installed copies consistent without overwriting edits, preserve all archived proof and transition occurrences with one coherent PR37 checkpoint cycle, and pass fresh native audit plus the mandatory Fledge lane, Rust1.88 suite and affected history cases after integration.

## Evidence

- Verification commit: `b9e464d4daabd699a84f2c9a815437ba86f73ca8`
- Base commit: `d80066472bc52eb0968d39e3db8b4e4a274eb19f`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

Main now contains PR41 case-order guidance and PR43 failed-case gating. This branch
retains the active bounded-history change and its independently repaired retention
behavior. Integrate the accepted inputs without changing product policy, inventing
criterion IDs or rewriting existing approvals, archives or recorded proof.

Authority is 0xLeif's actual standing chat instruction allowing ticket changes
through PRs and his 2026-11-02 instruction, "Can you finish up all these?" after the
reviewed closeout status. The project agent account is corvid-agent; that actor is
separate from the human source. Merge, release, live adoption and cleanup remain
with their existing human boundaries. The checkpoint owner selects the complete
PR37 current-cycle group and retains every additive collection and transition.

Integration update: approved main d839fef (#47) arrived after the9d02 qualification. Its unchanged setup/hint inputs are imported under the archived upstream change. Earlier exact-source results remain retained; final qualification must cover this new combined source before closeout. No new product policy or human decision is inferred.

## From the change's design.md

# Integration design

Merge origin/main normally into the existing branch, never rebase or force-push.
Retain immutable archives and evidence as exact parent blobs. Resolve the canonical
companion to a monotonic version while retaining every requirement and change row.
The primary history scope owns helper/test integration. For checkpoint divergence,
select the full PR37 current-cycle group as its owner and use the supported merge
rules for all additive collections. Retain both original inputs under temporary
proof; preserve every transition occurrence and record the chosen source.

No additional contract requirement, runtime, persistent cache or lifecycle ledger
is introduced. Real project adoption and cleanup remain not run.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
