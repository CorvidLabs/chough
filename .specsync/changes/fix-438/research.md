---
change: fix-438
artifact: research
---

# Research

`gate record` currently validates the listed gates, then starts one `checkpoint --close-gate` process per gate. Each child can save the same session independently. An otherwise valid group can therefore close a prefix before a later child rejects its save. The documented one-session restriction is already enforced by `check_listed`. A single checkpoint close renders its candidate before its native step.

Observed on `0d5988f8` through the public CLI. At 4,095 entries and 568,056 bytes, the grouped command exited 1 on the second gate. The first was closed at 4,096 entries and 568,559 bytes. The remaining gate's single-close retry refused and kept the resulting session hash. At the independent byte boundary, the grouped command started at 1,047,831 bytes, closed the first gate at 1,048,334 bytes and refused the second because its candidate would be 1,048,819 bytes. The two-gate control below both bounds exited 0 and closed both gates.

The Chough record already uses an atomic replacement. Sharing one render and replacement prevents this prefix-save failure without a second lifecycle ledger. Compensating for an earlier close after a child refusal would create additional Timeline events or rewrite history, so the draft uses bounded preparation before publication. Mixed-native behavior remains a planned test area. The non-native reproductions supply no evidence about native invocation or rollback.
