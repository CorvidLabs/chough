# Lesson bundle: fix-286

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 286
- **Kind**: BugFix
- **Specs**: history, cli, records
- **Paths**: src/evidence.rs, src/evidence_migrate.rs, tests/evidence.rs, tests/migrate_evidence.rs, docs/cli.md, specs/history/requirements.md, specs/history/history.spec.md, .chough/cases/history.receipts-keep-links.cases.toml, changelog.d/2026-11-18-fix-286.md
- **Acceptance**: A receipt in an archived story keeps its evidence entry's criteria and requirements, each one scalar, comma-and-space separated, and its limitations, one scalar, semicolon-and-space separated, each only when the entry has any, placed after case_digest; a migrated receipt keeps them the same way. The What was verified lines are unchanged.

## Evidence

- Verification commit: `d0b21c134088d30ebdea9229f66f342733c34b6a`
- Base commit: `9dd28de28015ce6f873e237044cc9650e74a3712`
- Verified by: `specsync check --spec cli --spec history --spec records`

## From the change's context.md

# Context

GitHub #286, found in corvid-agent's review of #285. When a session is archived,
each evidence entry becomes a receipt in the story. The receipt kept the claim,
result, command, revision, environment, digests, case and times, and dropped the
entry's `criteria`, `requirements` and `limitations`. The full entry survived
only in Git history. So an archived record read stronger than the evidence was,
and a reader could not go from a story to the requirement or criterion it
proved.

Leif's decision, in his comment on #286 on 2026-11-18 from his planning
interview: "keep all three. Archived receipts keep the entry's `criteria`,
`requirements` and `limitations`, so the story says what was verified against
what and its honest limits stay visible. This adds the three to REQ-history-006's
receipt field list."

The story's What was verified line is not changed. The issue also suggested
naming a limitation on that line; the decision is about the receipt fields.

A receipt's values are scalars, so the two ID lists are written as `aliases`
already is, comma-and-space separated, and limitations, which are free text and
may hold commas, are separated by a semicolon and a space.

## From the change's design.md

# Design

Nothing visual changes. A receipt in a story's `## Receipts` block gains up to
three lines, after `case_digest`:

```yaml
criteria: LIFE-22
requirements: REQ-cli-016
limitations: inherited environment variables were not recorded
```

## From the change's testing.md

# Testing

- `tests/evidence.rs::a_story_receipt_carries_each_field_its_entry_knows` now adds an
  entry with two limitations and expects, on the archived story's receipts:
  `criteria` and `requirements` on every receipt, `limitations` joined by `; ` on
  the limited entry and on the case run (whose entry records that inherited
  environment variables were not recorded), no `limitations` key on an entry
  without one, and the three keys in their place in the field order. It fails on
  main, where a receipt has no `criteria`.
- `tests/migrate_evidence.rs::observations_become_entries_receipts_or_removals_and_every_old_id_resolves`
  allows the three keys on a migrated receipt and expects its `criteria` and
  `requirements` to equal the 0.5.x observation's.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-006 | the two tests above and the case in `.chough/cases/history.receipts-keep-links.cases.toml` |

## Where these lessons go

- `specs/history/context.md`
- `specs/cli/cli.3md`
- `specs/records/context.md`
