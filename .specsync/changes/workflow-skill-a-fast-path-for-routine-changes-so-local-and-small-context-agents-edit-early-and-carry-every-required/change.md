---
id: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
state: draft
type: feature
base_commit: ddc56e96ff77cb0313d7591bee61d5a4a2950f0e
---

# Workflow skill: a fast path for routine changes, so local and small-context agents edit early and carry every required delta (LIFE-22)

## Intent

Workflow skill: a fast path for routine changes, so local and small-context agents edit early and carry every required delta (LIFE-22)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The workflow skill's stage contract opens with a routine-change fast path: a bounded first read, the session and SpecSync commands in order, a delta for every affected spec, a stop at each human gate, and an early first edit; SKILL.md points to it; installed copies at the previous bundle update to it; the release corpus pins the new bytes.

## No-spec Rationale

Guidance text in the shipped skill bundle and the digests that pin it; no requirement's wording or behavior changes.
