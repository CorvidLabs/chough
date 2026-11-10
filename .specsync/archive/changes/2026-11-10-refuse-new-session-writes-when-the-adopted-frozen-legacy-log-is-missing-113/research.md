---
change: refuse-new-session-writes-when-the-adopted-frozen-legacy-log-is-missing-113
artifact: research
---

# Research

Live issue113 names the original merged111/ded0e9ef missing-log gap. Current main4bbd still has anchor returning None for missing file and only (None,Some(current)) consulting remembered proof. Main tests cover appended newline plus vaulted/shelved holders, not log deletion. REQ-history-002 acceptance and history.spec.md error table already require mismatches to refuse; no new HI or canonical requirement is proposed. REQ-history-006 and DRIVE-3 require independent sessions, so importing the old anchor into each new cursor is rejected.

Reconciled existing refs: anchor-after-rollup4e892be5 and merged90e5de6f (#142) address readable holder locations, not missing-file detection; their existing controls remain relevant and are preserved. Main also contains anchor-holder retention c6a57d5f (#161), not the absent-file verification repair. No dedicated113 implementation was found. No dirty187/132 or60 plan-only source was adopted; no196/evaluation records were changed.

Existing INT01 executable ef328 source8a37 is explicitly distinct from main4bbd; selected history/sessions/Cargo equality does not establish full binary equivalence because cases/checkpoint/evidence/main/stage differ. Root confirmed no qualified exact-main executable available. No build is run without lease. Private CLI controls and unchanged contract oracle discriminate the defect in that older artifact. The archived-holder probe's unused always-true assertion is preserved and uncredited; actual holder-cursor check is separate. Full pre-rename byte comparison was not captured and is not claimed. Proposed current-source Rust regression includes an actual exact holder-byte assertion.

Alternative of simply treating None/None as always invalid is rejected because a project that never adopted any legacy log must still work. Searching only open sessions is rejected because archived/shelved/vaulted valid holders are already supported. Rewriting old records, weakening freshness/retention or broadening filesystem paths is outside scope. Actual tools/commands/status/fences and raw failures live externally under evidence/chough113-preparation, not duplicate native lifecycle state.
