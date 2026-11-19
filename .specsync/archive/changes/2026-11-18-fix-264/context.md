---
change: fix-264
artifact: context
---

# Context

GitHub #264, found by a swarm stress test. A project was migrated to 0.6.0; a
0.5.x binary then wrote one more record into the migrated tree. The second
`chough migrate --apply` on the same day wrote the new session folder and then
failed: the receipt name `<date>-from-0-5.md` was already taken by the first
migration's receipt, the error named no recovery, and running it again failed
the same way. The only way out was to rename the first receipt by hand.

Leif's decision, in his comment on #264 on 2026-11-18 from his planning
interview: "suffix and check first. A second same-day migration plan names its
receipt with a deterministic suffix (`…-from-0-5-2.md`), part of the plan and
its digest, and `--apply` checks every target before it writes anything, so a
refusal never leaves a partial write."

REQ-records-009 fixes the receipt's path as one name per day, so this is a
contract change to that requirement. It builds on #312 (#263), which made the
pre-write check cover a case-colliding target.
