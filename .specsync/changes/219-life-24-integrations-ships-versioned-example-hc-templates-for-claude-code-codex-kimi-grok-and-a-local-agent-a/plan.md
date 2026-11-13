---
change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
artifact: plan
---

# Plan

## Approach

Add files under `integrations/` and one test. Add the test to the cli module's
files and link the folder from docs/integrations.md. Chough's binary and skill are
unchanged.

## Out of scope

- The coordination guide (`integrations/coordination.md`): the next PR.
- The harness adapters (`integrations/<host>/adapter.sh`): #218.
- Picking and qualifying the local agent: #220.

## Steps

1. Commit the failing `tests/integrations.rs` (owned by cli) and record the red
   run.
2. Add the README, the five templates and the manifest.
3. docs/integrations.md links the folder; add a CHANGELOG line.
4. Record green, run the full local suite, run the audit, then review.

## Risks

- hc template format drift: the test pins only the three keys hc documents.

## Constraints consulted

LIFE-24 (no runtime dependency), LIFE-26 (no local paths or names in committed
files), the observed host mappings in docs/integrations.md, and hc's `--no-daemon`
rule for Codex.
