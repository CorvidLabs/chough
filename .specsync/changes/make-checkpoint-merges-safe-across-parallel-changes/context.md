---
change: make-checkpoint-merges-safe-across-parallel-changes
artifact: context
---

# Context

## Observed problem

Parallel Chough branches repeatedly conflicted on `.chough/session.json`. Merge commit `b2e2f19d904aec2a5acd73569c8439f5583c01f9` names that file among its conflicts and its resolution manually combined checkpoint history while selecting current-cycle values. Merge commit `e31fa7c768625b8cb2c2b1b12ce77c195deac99d` records another hand resolution and explicitly says that it kept the branch checkpoint. Merge commit `14b4e7f508b9701a7fbf19fb752c8176a66293b4` again names `.chough/session.json` as conflicted. These are observed integration costs, not a hypothetical formatting issue.

The file mixes two kinds of data. Decisions, completed work, questions, observations, bugs and references behave like exact-string collections that can often combine by membership. Objective, stage, native change, consents, blockers, awaiting-human gates and related scope fields describe one current cycle. Selecting different fields from different branches can create a cycle that neither branch represented, so those fields need atomic conflict handling.

## Constraints and prior decisions

REQ-cli-007 keeps one canonical schema 2 checkpoint at `.chough/session.json`. Schema 1 remains readable for context and status and migrates only through the explicit backup-preserving command. This change must not add a journal, shard directory, second writable store, runtime, service or duplicate lifecycle state. Native SpecSync remains the lifecycle authority, and checkpoint merges must not infer transitions, approvals, reviews, consents or human gates.

The selected design is a bounded `chough checkpoint --merge <base> <ours> <theirs>` command plus a `.gitattributes` entry naming `chough-session`. Each clone must configure its own Git driver. Initialization must not mutate Git configuration, and the explicit command remains available when no driver is configured.

Implementation must wait for fresh failure-first evidence from a real Git merge at the unfixed revision, linked to a local bug. A later pass must use the same kind of fixture. Evidence uses readable IDs without allocating a case number. No existing checkpoint, transition, case, bug, observation, HI, requirement or SpecSync ID scheme changes.

The main accepted limitation is that independent current-cycle edits can remain unresolved. The command must refuse those edits rather than guess. Driver setup is intentionally per clone, so a clone without setup retains ordinary Git behavior until the user configures it or invokes the fallback.
