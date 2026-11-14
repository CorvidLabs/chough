---
change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
artifact: testing
---

# Testing

- `tests/package.rs` `the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change`: SKILL.md routes to the fast path; the section precedes the transitions, is at most 32 lines, and names its steps in order.
- Existing `the_skill_asks_a_resuming_agent_to_state_the_handoff_before_its_first_edit` keeps SKILL.md under 6144 bytes.
- `tests/skill_releases.rs` pins the new current bundle digests; skill status tests cover the known earlier digests.
- Case `TC-cli-skill-routine-fast-path` in `.chough/cases/cli.routine-fast-path.cases.toml`.
- Field check after merge: rerun a local model on a fresh routine issue and count tool calls to its first edit.
