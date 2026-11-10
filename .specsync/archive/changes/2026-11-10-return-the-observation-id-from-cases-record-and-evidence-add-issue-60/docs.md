---
change: return-the-observation-id-from-cases-record-and-evidence-add-issue-60
artifact: docs
---

# Docs

Update `docs/cli.md` so the `cases check|list|select|run|record` and `evidence
add` entries name their returned IDs: `case` and `observation` for manual case
recording, and `observation` for direct evidence addition. Keep the documented
OBS allocation and exit-code behavior unchanged.

Add one bullet under `CHANGELOG.md`'s Unreleased heading describing the additive
keys and issue #60. Do not move or rewrite existing union-merged changelog
entries, and do not add release or deployment claims.

No HI wording, workflow skill, diagram or visual design copy changes. Canonical
edits are limited to the approved REQ-cli-011 and REQ-cli-017 delta and their
materialized requirement text and cli change-log row.
