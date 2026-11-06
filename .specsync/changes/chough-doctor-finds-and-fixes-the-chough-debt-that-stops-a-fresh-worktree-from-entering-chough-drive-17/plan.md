---
change: chough-doctor-finds-and-fixes-the-chough-debt-that-stops-a-fresh-worktree-from-entering-chough-drive-17
artifact: plan
---

# Plan

## Approach

Add `chough doctor` (REQ-cli-055) with five findings (REQ-history-012): legacy checkpoint, stale pointer, finished session, stale gates and named unfinished sessions. It reuses #166's finished rule and `session close`, and makes no native read. With no flag or `--dry-run` it only reports. `--fix` applies the safe fixes. Decision fixes need `--reason`; without it they are skipped (DRIVE-17).

## Out of scope

- Several unfinished open sessions with no pointer: that is live work, not debt, and `session use` stays the answer.
- Schema 1 migration, oversized or unreadable sessions (`session shelve`), evidence retention, the roll-up and the knowledge index.
- Native SpecSync state: no reads, no approvals, no reviews.

## Steps

1. Capture DRIVE-17 in hi/drive.md. Write five failing tests in tests/doctor.rs and run them on Linux.
2. Get scope approval through the orc.
3. Add src/doctor.rs and the `Doctor` command in src/main.rs. In src/sessions.rs, expose `finished`, the archive move, `close` and adoption as functions.
4. Add src/doctor.rs and tests/doctor.rs to cli.spec.md. Update docs/cli.md, the skill's checkpoint reference (both copies) and CHANGELOG.md.
5. After #166 lands, merge main, add the cases, run `fledge lanes run verify` on Linux, then the native check last, and open the pull request.

## Risks

See design.md.

## Constraints consulted

DRIVE-17, DRIVE-16, DRIVE-13, DRIVE-3, REQ-history-006, REQ-cli-037, spec-sync #941.
