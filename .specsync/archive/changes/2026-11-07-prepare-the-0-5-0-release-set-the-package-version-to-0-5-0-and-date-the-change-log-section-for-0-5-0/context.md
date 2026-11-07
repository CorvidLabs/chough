---
change: prepare-the-0-5-0-release-set-the-package-version-to-0-5-0-and-date-the-change-log-section-for-0-5-0
artifact: context
---

# Context

Leif asked for the 0.5.0 release preparation ("Prepare 0.5.0 now", relayed by the orc on 2026-11-06), per `docs/releasing.md` and `_audits/chough-release-proposal.md`. The cut is main at c58b4be7, after #178. Open #180 (performance) and the parked #69 are not in it.

Why 0.5.0, not 0.4.1: a history event that carries `relayed_by` (#140) cannot be read by 0.4.0, so under Cargo's 0.x rules this is a breaking release. Native Windows is also dropped (#139, FLOW-17), and there are new commands: `stage advance --to`, `session shelve`, `session close`, `checkpoint --close-gate`, `chough doctor` and the fledge Linux tasks.

This follows #116, the 0.4.0 preparation: version, a dated change log section and its link, with no tag, publish or GitHub release.
