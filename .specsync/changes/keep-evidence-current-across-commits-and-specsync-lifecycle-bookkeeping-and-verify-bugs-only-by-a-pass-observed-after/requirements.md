---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: requirements
---

# Requirements

REQ-cli-004 is modified (see `deltas/cli.md`). It still records the Git revision with each observation, but freshness compares a working content identity that leaves out Chough records, build output and SpecSync lifecycle bookkeeping. The recorded revision alone no longer stales an observation. Records without the new identity keep the earlier revision-bound comparison. LIFE-6 is added to its trace, because the fix lets Chough evidence and SpecSync lifecycle records coexist.

The delta item is `### REQUIREMENT REQ-cli-004`, which is the form SpecSync accepts in a delta. SpecSync applies a MODIFIED item only when the canonical heading is exactly `### REQ-cli-004`. The current heading is `### REQ-cli-004 — Bind observations to content and environment`. Before `change check`, the implementation normalizes it to `### REQ-cli-004` and moves the title text into the body. This is the same shape REQ-cli-003 has had since its modification.

REQ-cli-012 is not modified. Its text does not say how verification is ordered, so the BUG-0014 fix narrows behavior within the existing wording. See the open question in plan.md.

Linked existing HI criteria: LIFE-2, LIFE-5, LIFE-6, LIFE-7 and FLOW-13. No HI criterion is added or reworded.
