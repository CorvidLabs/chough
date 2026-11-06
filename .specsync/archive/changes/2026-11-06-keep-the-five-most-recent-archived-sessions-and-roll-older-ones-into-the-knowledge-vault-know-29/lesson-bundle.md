# Lesson bundle: keep-the-five-most-recent-archived-sessions-and-roll-older-ones-into-the-knowledge-vault-know-29

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep the five most recent archived sessions and roll older ones into the knowledge vault (KNOW-29)
- **Kind**: Feature
- **Specs**: history, cli
- **Paths**: src/sessions.rs, src/vault.rs, src/library.rs, src/stage.rs, tests/reduce.rs, tests/vault.rs, .gitattributes, docs/cli.md, CHANGELOG.md
- **Acceptance**: At done with the librarian on, only the five most recent archived sessions (by their native change finalization, then key; none counts oldest) stay in .chough/sessions/archive; older ones move byte for byte, never replacing a file, into knowledge/attachments/<folder>/sessions/<key>.json, reported as applied.session_rollup; library index writes knowledge/sessions/<key>.md linking each, union-merged; with the librarian off nothing moves. The new tests fail first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `3be6c5e46362f610e5716de0036c66ed5f8b9f14`
- Base commit: `73a7da5653312e907621c0165d0b5f903c22ff53`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

## KNOW-29

"Only the five most recent done sessions stay in .chough/sessions/archive; older ones roll into the knowledge vault as a summary note with attachments, and nothing is lost from Git history." This is Leif's wording, relayed by the orc on 2026-11-05 at 19:57 MST, and captured with "Capture as written" at 20:08 MST. His words: "once that's all that change is done ... we can like archive like after, you know, five of them. Like we only keep the most recent five."

## On main today

- 19 archived sessions in `.chough/sessions/archive/`, about one more per change.
- Each is about 1–2 KB, except the adopted legacy session (73 KB).

## Out of scope

KNOW-24, KNOW-25 to KNOW-28.

## From the change's design.md

# Design

## Approach

- **Roll-up:** `sessions::roll_up_at_done(root)` runs from the done move after the trim and before the knowledge refresh, so the refresh writes the new notes.
  - It orders archived sessions by `library::finalized_at(native_change)`, newest first, then by key. A session without a change or a finalization counts as the oldest.
  - It keeps the first five. For each of the others, it hard-links the file into `<library-dir>/attachments/<folder>/sessions/<key>.json` (which never replaces a file) and then removes the source. If the attachment already holds the same bytes, only the source is removed; if it holds different bytes, the session stays and is reported.
- **Notes:** `vault::session_notes` writes a note for each `attachments/*/sessions/*.json`. `library index` includes them, and `--check` covers them.
- **Merging:** `.gitattributes` gains `knowledge/sessions/*.md merge=union`.

## Risks

- A rolled session no longer protects proof. Its cycle is closed; KNOW-21 already ignores the file references of archived sessions, and KNOW-16 dropped its closed observations.
- The order follows the vault's hot and cold archive order.

## Constraints consulted

- KNOW-29 and DRIVE-13.
- REQ-cli-041, REQ-cli-043, REQ-cli-046, REQ-history-006.
- The no-replace rule.

## From the change's testing.md

# Testing

## tests/reduce.rs

- `done_keeps_the_five_newest_archived_sessions_and_rolls_the_rest_into_the_vault`:
  - seven archived sessions;
  - s1 to s5 stay;
  - s0 (a tie, broken by key) and s6 (no change) move byte for byte into `knowledge/attachments/<key>/sessions/<key>.json`;
  - their notes name the objective and link the attachment;
  - `library index --check` and `session list` pass.
- `with_the_librarian_off_archived_sessions_stay`: no session moves, and the step reports `skipped`.

## tests/vault.rs

`only_generated_knowledge_notes_merge_by_union` checks that a session note is `union`. All three tests fail first.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-011 | tests/reduce.rs `done_keeps_the_five_newest_archived_sessions_and_rolls_the_rest_into_the_vault`, `with_the_librarian_off_archived_sessions_stay` |
| REQ-cli-051 | tests/reduce.rs `done_keeps_the_five_newest_archived_sessions_and_rolls_the_rest_into_the_vault`; tests/vault.rs `only_generated_knowledge_notes_merge_by_union` |

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/history/context.md`
- `specs/cli/context.md`
