## ADDED

### REQUIREMENT REQ-history-008

No orphan event from a vanished checkpoint. When a command's save is refused because the checkpoint it resolved was archived, removed or adopted into sessions mode while it ran, the helper SHALL first remove the history events that command published, and any cycle directory it reserved that is then empty, when their bytes are unchanged and they were published after the command last found its checkpoint in place. A save finds its checkpoint in place just before it writes, so the helper leaves no orphan event and never removes an event a save may have written, wherever that checkpoint is moved afterwards. Any other failed save SHALL keep REQ-history-002's unselected orphan. Chough's own `session use` and `session new` SHALL wait for a running checkpoint writer rather than archive or switch its session under it. Trace: DRIVE-3, LIFE-7.

Acceptance Criteria
- A `stage advance` whose session file is moved away during its native read refuses with nothing saved and leaves no new event file.
- `session use` started during a `stage advance` waits for it, and the advance records exactly its own event.
- A session file moved away after a successful save keeps the event that save points to.
- A session file moved after a successful save to a path outside `.chough/sessions/` keeps the event that save points to.
