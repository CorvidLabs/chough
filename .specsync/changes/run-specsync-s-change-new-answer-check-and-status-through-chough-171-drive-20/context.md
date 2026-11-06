---
change: run-specsync-s-change-new-answer-check-and-status-through-chough-171-drive-20
artifact: context
---

# Context

Issue #171, slice 1 of "Chough drives SpecSync". Leif's interview answer 1 ("Also records my gates", 2026-11-06, orc tab): agents only call chough, and Chough runs the SpecSync agent steps itself. Captured as DRIVE-20. The interim re-audit (2026-11-06) counted 18 direct `specsync` calls in each walkthrough: init, scaffold, check, adopt, change new, two approve attempts, four answers, status, four change checks, review and finalize. This slice moves 10 of them behind `chough`: new, the four answers, the four checks and status.

## Decisions that bind this change

- Leif, relayed by the orc (t18), 2026-11-06: "A: amend, behind a summary" and "Capture + approve slice 1". DRIVE-20, DRIVE-21, FLOW-19 and FLOW-20 are captured as drafted, with "amended by FLOW-19" notes on FLOW-16 and the Drive intent. The slice order is approved, and this slice's definition is approved as drafted in the plan.
- Slice order: (1) this one; (2) `chough change ship`; (3) direct lifecycle writes flagged (DRIVE-21); (4) gate summaries and one-reply lists; (5) native gate records with `relayed_by`, which needs spec-sync #900 released; (6) project setup, if Leif wants it.
- IDs agreed with t27, who drives #169/#170: they take DRIVE-18, DRIVE-19 and REQ-cli-056; this change takes DRIVE-20, DRIVE-21, FLOW-19, FLOW-20 and REQ-cli-057. They touch `src/evidence.rs`; this change does not.
- Ruled out for this slice:
  - Approve, review, ship and finalize.
  - History records of Chough-run steps (slice 3).
  - Rewriting SpecSync's recovery text to name `chough` commands.
  - A `--no-commit` form of check: the plain `specsync change check <id>` used to materialize a delta stays a direct call for now.
