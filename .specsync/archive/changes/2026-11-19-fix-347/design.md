---
change: fix-347
artifact: design
---

# Design

Nothing visual changes. A new bug's Log reads:

```text
- 2026-11-18T10:00:00Z · Reported (manual report); evidence: [2026-11-18-probe-failure](../sessions/2026-11-18-probe/evidence/2026-11-18-probe-failure.md)
```

After the session is archived and `chough doctor --fix` runs, one entry is added:

```text
- 2026-11-19T09:00:00Z · Evidence now in its archived session: [2026-11-18-probe-failure](../archive/sessions/2026-11-18-probe.md)
```
