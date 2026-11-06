---
change: a-session-archives-itself-at-its-own-move-into-done-155
artifact: design
---

# Design

## Approach

`sessions::archive_at_done(root)` runs last among the done steps in `stage::run`, after the ledger rotation and before the report:
- In sessions mode, it takes the key this command resolved (the pinned checkpoint path).
- If the inbox has unfolded entries, it returns `kept_open`.
- Otherwise it moves the file with the same no-replace rename as `archive_done`, removes the pointer, and re-pins this command's checkpoint to the archived path. The report's history read then follows the file instead of refusing with "archived or removed while this command ran".

`archive_done` on `session new` and `use` is unchanged. It still archives done sessions left open by older versions; that is how the sweep happens.

## Risks

- **Running last** means REQ-history-011 counts this session only at the next move into done. Between dones the archive holds six sessions, as it does today after a switch, and REQ-history-011's text and tests stay as they are.
- **A failure in the step** (for example an existing archive file) is reported as `refused` with its reason. The move into done stands.
- **Concurrent writers** are covered: the done move holds the checkpoint lock, and other commands still refuse a vanished session.

## Constraints consulted

DRIVE-13, DRIVE-3, REQ-history-006, REQ-history-008, REQ-history-011, #155.
