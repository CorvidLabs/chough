---
change: make-checkpoint-merges-safe-across-parallel-changes
artifact: requirements
---

# Requirements

One requirement is modified (see `deltas/cli.md`):

- REQ-cli-007 retains the single canonical `.chough/session.json`, schema 2 checkpoint contents, schema 1 read behavior, explicit byte-preserving migration and every existing acceptance criterion. Its trace remains FLOW-13 and LIFE-3.
- The new `chough checkpoint --merge <base> <ours> <theirs>` command reconciles schema 2 checkpoints only. History and reference collections use exact-string ordered three-way membership. Current-cycle fields merge as one group so independently valid edits from different cycles cannot be combined silently.
- Compatible merges write only the ours result. Conflicting current-cycle groups, schema 1 or mixed-version inputs, malformed records, unknown fields and bound overflow fail closed without changing any input or creating another record.
- `.gitattributes` associates the canonical checkpoint with the `chough-session` driver. Documentation supplies the per-clone driver configuration, while init leaves Git configuration unchanged and the explicit command remains the fallback.
- Real Git evidence must first retain the pre-implementation conflicting merge failure linked to a local bug, then demonstrate a later pass. No second checkpoint store, new case number, changed ID scheme, runtime, service, release or deployment is introduced.

The delta contains the full replacement text for REQ-cli-007. No HI criterion is added or reworded.
