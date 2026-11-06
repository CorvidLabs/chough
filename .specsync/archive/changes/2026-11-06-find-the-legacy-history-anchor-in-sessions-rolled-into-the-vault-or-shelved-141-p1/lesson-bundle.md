# Lesson bundle: find-the-legacy-history-anchor-in-sessions-rolled-into-the-vault-or-shelved-141-p1

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Find the legacy history anchor in sessions rolled into the vault or shelved (141, P1)
- **Kind**: BugFix
- **Specs**: history
- **Paths**: src/history.rs, tests/sessions.rs, CHANGELOG.md
- **Acceptance**: A session without its own legacy anchor keeps writing gate and stage events when the session holding the anchor has been rolled into the vault or shelved, finding the anchor there with the same exact match; a changed legacy log is still refused. The new test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `14f6b289df3fd6f4723f94caf8fda65067b47942`
- Base commit: `260ecc949ea01214f4bec43b30ea4226ce8fae59`
- Verified by: `specsync check --spec history`

## From the change's context.md

# Context

Issue #141 (P1). ch-d7 found it on pristine main `260ecc94`. It is my regression from KNOW-29 (#135).

`adopted_anchor` (src/history.rs) found the session that recorded the legacy history anchor by scanning only `.chough/sessions/` and `.chough/sessions/archive/`. KNOW-29's roll-up moved that session, `adopt-event-history-...`, into `knowledge/attachments/adopt-event-c467f940/sessions/`. Since then, every session without its own anchor refuses gate and stage writes with "legacy history anchor changed; restore the recorded legacy bytes", although `.chough/transitions.jsonl` is byte-identical. That blocks every open Chough session in this repository.

The new test reproduces the refusal exactly, for a session rolled into the vault and for a shelved one.

Shelved sessions (#133) have the same gap.

## From the change's design.md

# Design

## Approach

`adopted_anchor` scans, in order:
1. `.chough/sessions/`;
2. `archive/`;
3. `shelved/`;
4. each `<library-dir>/attachments/*/sessions/`.

Each file is read within the checkpoint cap. The match rule stays exact: the recorded anchor must equal the current one. The library directory comes from the `library-dir` preference; if that can't be read, the vault scan is skipped.

## Risks

- The anchor now also depends on the knowledge directory. A follow-up could keep the anchor holder out of the roll-up, which would also need the KNOW-29 fixture to model a single anchor holder. It is offered to Leif, not done here.

## Constraints consulted

- REQ-history-002: unexpected anchor changes still refuse.
- KNOW-29.
- REQ-history-006: shelve.

## From the change's testing.md

# Testing

- tests/sessions.rs `sessions_still_write_after_the_anchor_holder_rolls_into_the_vault_or_is_shelved`:
  - the adopting session moves to `knowledge/attachments/legacy/sessions/legacy.json`, and in a second run to `.chough/sessions/shelved/legacy.json`;
  - another session's `stage advance` succeeds, and the legacy log is unchanged;
  - a changed legacy log is still refused.

  It fails first, with the P1 refusal.
- `a_session_started_after_legacy_adoption_writes_events_and_keeps_the_log_guarded` still passes.
- `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/history/context.md`
