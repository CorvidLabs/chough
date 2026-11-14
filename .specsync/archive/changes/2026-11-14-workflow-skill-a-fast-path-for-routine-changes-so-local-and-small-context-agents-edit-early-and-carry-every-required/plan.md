---
change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
artifact: plan
---

# Plan

## Approach

Add a "Fast path for a routine change" section at the top of `references/stages.md`, before the shared transitions: a bounded first read, the session and native-change commands in order (including a delta per affected spec), a stop at scope approval, an early first edit, the test and case, the checks and change check, a stop for review, and "work only in your own checkout". SKILL.md's step 5 routes to it; one clause ("default `openai`") is dropped from SKILL.md to stay under its byte bound.

## Out of scope

The stage-by-stage detail, chough commands, SpecSync behavior, and per-host install behavior.

## Steps

1. Write the fast path in `references/stages.md` and the pointer in `SKILL.md`.
2. Add the previous digests to `KNOWN_PREVIOUS`; update the release corpus manifest and `MANIFEST_SHA256`.
3. Update the repository's `.agents` copy with `chough skill update`.
4. Add the package test and the case; add the changelog entry.
5. Run fmt, test, lint, `chough change check`.

## Risks

Guidance can drift from commands; the fast path names only existing commands, and the test pins its order.

## Constraints consulted

SKILL.md byte bound (package test); the release corpus pins (skill_releases); KNOWN_PREVIOUS for updatable copies (REQ-cli-016); the cli lesson on frozen deltas.
