---
change: fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag
artifact: design
---

# Design

`Step::arguments` (`src/humans.rs`) builds the native approve or review call and appends the relay. Every value is emitted joined to its flag (`--words=<value>`), so clap in SpecSync never reads a hyphen-leading value as a flag. `checkpoint` and `change ship` pass the relay through. No visual or interaction change.
