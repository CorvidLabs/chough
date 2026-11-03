---
change: refuse-over-bound-checkpoint-candidates-before-publishing-gate-history
artifact: design
---

# Design

Validate the complete candidate checkpoint, including the serialized final history cursor, before publishing gate history. Reuse existing validation and serialization semantics; preserve legacy cursor-free behavior and immutable history format. Handle both observation count and byte capacity and both opening and clearing gates. A dry preparation phase may calculate cursor/event identities in memory if needed, then publish only a valid candidate. Do not replace the lifecycle owner, migrate old history, delete orphan events or broaden bounds. Independent review checks publication order and exact byte-bound edges.
