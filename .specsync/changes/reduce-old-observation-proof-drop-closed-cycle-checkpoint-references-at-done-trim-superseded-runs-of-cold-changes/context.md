---
change: reduce-old-observation-proof-drop-closed-cycle-checkpoint-references-at-done-trim-superseded-runs-of-cold-changes
artifact: context
---

# Context

Leif asked on 2026-11-04 to reduce the artifacts and evidence in `.chough/` and keep the
knowledge in the vault. He chose:
- "Old checkpoint refs" (KNOW-16);
- "Old newest-per-case" (KNOW-17);
- "Loose artifact folders" (KNOW-18);
- "Key items move" (KNOW-19);
- "By cycle time" for proof that no archive lists (KNOW-20).

When asked to confirm KNOW-16 to KNOW-19 explicitly, he answered "Yes, build it", and
then "go ahead and build the reduction PR".

This change covers observation proof: KNOW-16, KNOW-17 and KNOW-20, plus parallel
validity reads so a wide trim stays fast enough. Loose artifacts and attachments
(KNOW-18, KNOW-19) need a new ledger entry kind and file moves, so they follow in the
next change. The safeguards are unchanged: ledger first, the KNOW-10 restore check, no
removal while a native change is open, and every byte recoverable from Git.
