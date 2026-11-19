---
change: fix-250
artifact: design
---

# Design

Nothing visual changes. A withdrawn gate's Timeline entry gains one field line:

```text
- 2026-11-18T10:00:00Z · gate-closed · accept → accept · Native approval of ch-alpha ^t-6b37ce
  - native_change: ch-alpha
  - withdrawn: Superseded; ch-alpha is archived
```

Doctor's JSON gives `"decision_reason": "<reason>"` on each decision finding it fixed.
