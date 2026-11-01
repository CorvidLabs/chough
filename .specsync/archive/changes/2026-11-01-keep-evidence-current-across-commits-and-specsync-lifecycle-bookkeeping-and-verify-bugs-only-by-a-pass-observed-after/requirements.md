---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: requirements
---

# Requirements

Three requirements are modified (see `deltas/cli.md`):

- REQ-cli-004 still records the Git revision with each observation, but freshness compares a working content identity that leaves out Chough records, build output and SpecSync lifecycle bookkeeping. The recorded revision alone no longer stales an observation. Records without the new identity keep the earlier revision-bound comparison. LIFE-6 is added to its trace, because the fix lets Chough evidence and SpecSync lifecycle records coexist.
- REQ-cli-012 gains one sentence: a bug reads as reproduced and later passed, and accepts `verified`, only when a present linked pass was observed strictly after its most recent present linked failure. One acceptance line covers an older or same-second pass and a bug with no linked failure. The rest of its text is unchanged. This is the BUG-0014 fix; without it the behavior change would have no requirement behind it.
- REQ-cli-017 lists the optional working content identity among the fields schema 2 may add, requires it to be a SHA-256 digest when present, and says schema 2 records without it keep their freshness semantics. Its current text lists the specific fields schema 2 may add and has no general allowance, so the new field needs this item.

All three delta items use the form `### REQUIREMENT REQ-cli-NNN`, which is the form SpecSync accepts in a delta and the form every archived delta uses. SpecSync applies a MODIFIED item only when the canonical heading is exactly `### REQ-cli-NNN`. REQ-cli-012 and REQ-cli-017 already have that heading. REQ-cli-004 carries its title in the heading, after a dash. After approval and before `change check`, the implementation replaces exactly these lines of specs/cli/requirements.md, and nothing else:

```text
### REQ-cli-004

Bind observations to content and environment. The helper SHALL bind each observation
to Git revision, working content identity, and an explicit environment, and report
freshness limitations. Trace: LIFE-2, LIFE-7.
```

The title becomes the first sentence of the body, and the delta body starts with the same sentence, so the title text survives materialization; the rest of the canonical text is then replaced by the delta at `change check`. This is the shape REQ-cli-003 has had since its modification (#8). The edit cannot be made in this draft: a draft change does not cover paths, and `specsync change audit` (part of the verify and pre-push lanes) fails with "meaningful changed paths are not covered by an active change: specs/cli/requirements.md" when the canonical file is edited before approval. Showing the exact text here puts it under review with the scope instead.

Linked existing HI criteria: LIFE-2, LIFE-5, LIFE-6, LIFE-7 and FLOW-13. No HI criterion is added or reworded.
