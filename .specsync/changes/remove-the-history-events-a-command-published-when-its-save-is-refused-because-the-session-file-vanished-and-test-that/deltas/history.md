## ADDED

### REQUIREMENT REQ-history-008

No orphan event from a vanished checkpoint. When a command's save is refused because the checkpoint it resolved was archived, removed or adopted into sessions mode while it ran, the helper SHALL first remove the history events that command published, and any cycle directory it reserved that is then empty, when their bytes are unchanged, so it leaves no orphan event. Any other failed save SHALL keep REQ-history-002's unselected orphan. Chough's own `session use` and `session new` SHALL wait for a running checkpoint writer rather than archive or switch its session under it. Trace: DRIVE-3, LIFE-7.

Acceptance Criteria
- A `stage advance` whose session file is moved away during its native read refuses with nothing saved and leaves no new event file.
- `session use` started during a `stage advance` waits for it, and the advance records exactly its own event.
