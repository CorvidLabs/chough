# CI archive execution performance

## Human authority and limits

Acting agent: corvid-agent. Human source: 0xLeif in chat on 2026-11-02 authorized ticket changes through PRs ("I already gave it some approval and told it can do any changes. It just needs a PR."), then "Can you finish up all these?" and "Merge approved, green PRs and resynchronize", followed by "get the 3 prs merged?". This concrete scope fixes the #46 archive executor within unchanged validation and budgets. It does not authorize lowering gates, rewriting recorded proof, a longer timeout, release or deployment. Implementation closeout and merge require actual tests, independent review and green exact-head checks.

## Problem

PR46 at 9af1921 twice authenticated 40 archives but exceeded the unchanged 10-minute job limit. Native 6.1.2 starts a whole-archive authentication snapshot for each requested archive. Historical checkout replay must stay serial.

## Approach

Use exactly two workers for independent read-only status requests against the fixed current checkout. Validate every obligation with the existing predicates. Join every submitted request before leaving the executor and before the final fingerprint. Report results in deterministic ID order. Keep native commands, trust inputs, history replay, version, output bounds and all deadlines unchanged.

## Out of scope

Native cache/validator changes, persistent caches, archive or evidence rewrites, gate or timeout changes, product runtime changes, release and deployment.

## Ownership and estimate

Root owns native/checkpoint/qualification/closeout. Worker owns only the two declared Python files. Small code change; uncertainty is hosted wall time under two CPUs. No duration promise is made.

## Constraints consulted

AGENTS.md, INTENT.md, LIFE-1/2/6/7, FRD native closeout boundary, cli contract/companions, workflow lifecycle and context.md lesson about Chough writes preceding final native verification. Existing readonly validation conditions remain authoritative.
