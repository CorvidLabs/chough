---
change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
artifact: testing
---

# Testing

`tests/changelog.rs` (13 tests) checks that the changelog names this change before release from `CHANGELOG.md` and `changelog.d/` together; that assembly appends after existing entries with no blank line, inserts a versioned heading after Unreleased, refuses a repeated version with nothing written and fragments kept, gives the same bytes twice, and that the converted fragments give back main's Unreleased bullets in main's order. Claude Code reproduced each case by hand against `tools/changelog.py`, and assembling the converted fragments gave cf2ff72's `CHANGELOG.md` byte for byte. `tests/skill_releases.rs` and `tests/package.rs` pass with the new skill bytes. Checks: `fledge run fmt`, `fledge run test`, `fledge run lint`, `fledge lanes run verify`, `chough change check`.
