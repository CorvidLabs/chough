---
change: integrate-merged-case-ordering-guidance-with-bounded-history
artifact: design
---

# Integration design

Merge origin/main normally into the existing branch, never rebase or force-push.
Retain immutable archives and evidence as exact parent blobs. Resolve the canonical
companion to a monotonic version while retaining every requirement and change row.
The primary history scope owns helper/test integration. For checkpoint divergence,
select the full PR37 current-cycle group as its owner and use the supported merge
rules for all additive collections. Retain both original inputs under temporary
proof; preserve every transition occurrence and record the chosen source.

No additional contract requirement, runtime, persistent cache or lifecycle ledger
is introduced. Real project adoption and cleanup remain not run.
