---
change: keep-the-five-most-recent-archived-sessions-and-roll-older-ones-into-the-knowledge-vault-know-29
artifact: design
---

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
