---
change: correct-archived-resume-guidance-and-record-the-kimi-cold-start
artifact: requirements
---

# Requirements

No requirement is added, removed, or reworded. There is no canonical contract delta and `no_spec_change` remains true. The change corrects behavior and documentation that did not meet existing requirements, then records observations against those requirements.

## Primary traceability

### REQ-cli-003: actual native status and guidance

The existing requirement says the helper reports actual native SpecSync status and guidance, with failed, unavailable, undecodable, oversized, and timed-out responses explicitly unknown. BUG-0020 violated this by reporting a saved review-and-ship action as `next_valid_action` while the parsed native summary was archived and supplied a different next action.

The fix reads the already parsed `native_summary` only at the `done` branch, after the existing pause, human-gate, blocking-bug, and closed-gate checks. Archived native guidance is returned after the existing `Continue done:` prefix, with its text preserved exactly. Missing native guidance and every unusable native response produce a safe instruction to inspect native status, never the stale saved action. `native_specsync` error details and `native_summary` remain visible.

Trace: FLOW-13 and LIFE-3 for cold resumption, with the requirement's existing LIFE-1, LIFE-6, and LIFE-7 obligations preserved. Verification: TC-cli-0105, with a failed pre-fix observation and a later passed post-fix observation linked to BUG-0020.

### REQ-cli-007: one canonical checkpoint

The existing requirement makes `.chough/session.json` the single checkpoint and requires a fresh process to resume from its next action under native authority. The fix changes only the derived `next_valid_action` report. It does not add a checkpoint field, migrate a schema, rewrite the saved next action, or create a second lifecycle store. `session_next_action` continues to expose the unchanged saved string so the difference between saved context and valid native guidance remains inspectable.

Trace: FLOW-13 and LIFE-3. Verification: TC-cli-0105 checks both `next` and `status`, including the unchanged `session_next_action`, paused and awaiting-human precedence, and safe missing-native behavior.

### REQ-cli-016: host skill installation and safe updates

The existing requirement already maps supported host directories and permits replacement only of missing files or files byte-identical to a known earlier bundle. The `openai` mapping already installs `.agents/skills`, which the observed Kimi Code CLI 0.41.0 discovered. No new host value or integration is required.

The documentation explains that mapping. The base `SKILL.md` digest joins `KNOWN_PREVIOUS`, allowing unedited base copies to update while edited copies remain protected. The canonical, `.agents`, and `.claude` files must be byte-identical after the update and report current.

Trace: FLOW-1 and LIFE-4. Verification: the expanded automated TC-cli-0037 covers the base digest, safe replacement, current status, and preservation of edits. TC-cli-0106 records the original guidance failure and a later retrospective documentation pass, both linked to BUG-0021, without claiming a new Kimi run.

## Unchanged requirements that constrain the records

- REQ-cli-004 and REQ-cli-017 keep observations bound to revision, working content identity, environment, artifacts, and case definition digests. This change uses the existing observation schema and does not rewrite old records or claim that an observation is human acceptance.
- REQ-cli-009 and REQ-cli-011 keep test definitions separate from executions and require explicit expected outcomes and honest manual or agent-assisted verdicts. TC-cli-0105 therefore has pre-fix and post-fix executions, while TC-cli-0106 has explicit retrospective review verdicts rather than an invented automated binding.
- REQ-cli-012 requires a present pass observed strictly after the latest present linked failure before a bug reads reproduced and later passed or accepts `verified`. BUG-0020 and BUG-0021 follow that order. BUG-0022 and BUG-0023 stay open and out of scope even though the original observation is appended as evidence.
- REQ-cli-008 and contract Invariants 1, 2, 4, 6, and 8 remain unchanged: native gates still decide lifecycle moves, safe updates preserve edits, agents do not record human decisions, evidence keeps its freshness semantics, unknown native states stay visible, and there is one canonical checkpoint.

BUG-0022 is an environment issue under LIFE-6, not a request to change tool-version requirements. BUG-0023 is an output-size finding linked to LIFE-3 and REQ-cli-007, but output redesign is outside this scope. Neither bug is marked fixed by this change. BUG-0001 needs no work because its reported configuration-comment issue is already fixed on the base.

All linked HI criteria and requirement IDs already exist. No HI wording or ID changes are part of this change.
