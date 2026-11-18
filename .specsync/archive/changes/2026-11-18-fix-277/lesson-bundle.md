# Lesson bundle: fix-277

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 277
- **Kind**: BugFix
- **Specs**: cli, records
- **Paths**: src/bugs.rs, src/migrate.rs, tests/migrate_evidence.rs, docs/cli.md, .chough/cases/records.migrated-bug-links.cases.toml, changelog.d/2026-11-18-fix-277.md
- **Acceptance**: A migrated bug's Log names each carried observation as a Markdown link, relative to the bug's file, to the open session's evidence entry or to the archived story that holds its receipt; every such link opens a file; a receipt only the bug holds is named as the bug's own; a removed observation keeps its 0.5.x ID. The bug's front matter is unchanged.

## Evidence

- Verification commit: `a34ad65fc53db0ae563f76054e5096662dca9cce`
- Base commit: `9dd28de28015ce6f873e237044cc9650e74a3712`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

GitHub #277, found in the review of #256. Since #256 a migrated bug's front
matter names each carried observation by the name of the entry or receipt it
became. The bug's Markdown body, in its Log lines, still named them by bare
0.5.x ID, so a reader of the file could not follow them to the session or story
that holds the entry without searching.

Leif's decision, in his comment on #277 on 2026-11-18 from his planning
interview: "relative links. A migrated bug's body links each carried
observation to the file of the entry or story that holds it, as a new bug's
body does."

A new bug's Log names its evidence as `<session-id>/<evidence-id>`, which is not
a Markdown link either; this change follows the decision's first sentence and
writes relative Markdown links in a migrated bug's Log. A new bug's Log is not
changed.

No requirement text changes: REQ-records-009 already says each bug becomes a
Markdown bug and each carried observation an entry or a receipt.

## From the change's design.md

# Design

Nothing visual changes. A migrated bug's Log line reads, for example,
`Reported (manual report); evidence: [<entry name>](../sessions/<session>/evidence/<entry name>.md), [<receipt name>](../archive/sessions/<story>.md), <receipt name> (a receipt of this bug)`.

## From the change's testing.md

# Testing

- `tests/migrate_evidence.rs::a_migrated_bugs_body_links_its_carried_observations`
  migrates a 0.5.x bug that links an open session's observation, an archived
  session's observation and one only the bug holds. The Reported line links the
  entry and the story, names the bug's own receipt as such, holds no bare
  0.5.x ID, and every link in the body opens a file from the bug's folder. It
  fails on main, where the line holds the three bare IDs.
- The unit test `bugs::tests::migrated_bugs_carry_every_field_alias_link_and_fixture`
  expects the linked Log line and a removed observation's bare ID.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-009 | the two tests above and the case in `.chough/cases/records.migrated-bug-links.cases.toml` |

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/records/context.md`
