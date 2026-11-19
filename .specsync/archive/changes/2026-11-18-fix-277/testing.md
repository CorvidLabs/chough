---
change: fix-277
artifact: testing
---

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
