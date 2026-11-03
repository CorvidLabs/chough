---
change: raise-the-checkpoint-session-byte-limit-to-131072-and-the-observation-pointer-limit-to-400
artifact: context
---

# Context

PR #50 (issue #49 cleanups) could not take main 747a12c: a correct three-way `chough checkpoint --merge` produced a 68,737-byte checkpoint, over the 65,536-byte bound, with the observation pointer list already at its 200-item cap. Leif's agent posted a compaction plan on PR #50 (issuecomment-6595491835) and said its safe-only candidate leaves 110 bytes and no slots.

Human decision (Leif, 0xLeif, chat, 2026-11-02 9:40 PM MT): "Do your agent's cleanup and raise the progress file and test-result limits." Captured as HI LIFE-11. No history pruning. Approvals are recorded as corvid-agent under Leif's 2026-11-02 9:14 AM MT instruction "Use corvid-agent for all approvals for all things!! You are corvid-agent".
