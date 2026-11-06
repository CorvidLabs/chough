---
change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
artifact: context
---

# Context

Issue #138, from the audit's Chough assertion review (main `700dad96`). It mapped 233 acceptance criteria to passing assertions; nine had none. The audit's `REQ-…/AC-N` labels are ordinals, not product IDs. Two of the nine criteria changed after the audit: KNOW-21 (#120) rewrote REQ-cli-045, so its no-reset criterion now reads "no observation reference is dropped".

| Audit label | What is not asserted today |
|---|---|
| REQ-cli-002/AC-3 | The skill's three certainty labels, source and limits, and that a project decision outranks a lesson. Also that Chough keeps no lesson store, index or ledger: the knowledge notes only point to lessons. |
| REQ-cli-007/AC-4 | No second writable checkpoint store, and no adapter such as APS required. |
| REQ-cli-012/AC-13 | The CLI documentation's upgrade note for new-form bug IDs. |
| REQ-cli-029/AC-2 | Authority recorded with source, scope and limits before approval; asking when it is missing or exceeded; agent labels and bot approvals never becoming human decisions. |
| REQ-cli-029/AC-5 | Source/package verification kept apart from a fresh-agent cold start; no skipped cold-start proof promoted to a pass. |
| REQ-cli-031/AC-3 | The walkthrough begins with the new-project setup path. |
| REQ-cli-045 (audit AC-2) | With the librarian off, the references are unchanged and reported `skipped`. |
| REQ-cli-045 (audit AC-3) | With no reset time, no observation reference is dropped. |
| REQ-reporting-003 (audit AC-4) | Before/after measurement: one input root, explicit binary identities, local work apart from native wait, incomplete reads kept partial or unknown, and native slowness never reported fixed by a local change. |

## Leif's decisions

- The 95% plan, item C6: these nine get real assertions, in one issue, one change and one PR.
- 2026-11-05 21:52 MST, through the orc, "Both recommended":
  - REQ-cli-029/AC-5 is asserted in Chough's evidence model plus ROADMAP text.
  - REQ-reporting-003's measurement criterion is asserted through a new script, `examples/measure-report.sh`, plus a test.

No runtime behavior changes, and no new REQ or HI IDs are taken.
