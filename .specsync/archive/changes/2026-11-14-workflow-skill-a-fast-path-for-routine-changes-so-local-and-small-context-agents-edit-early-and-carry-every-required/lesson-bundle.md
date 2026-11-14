# Lesson bundle: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Workflow skill: a fast path for routine changes, so local and small-context agents edit early and carry every required delta (LIFE-22)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: skills/chough-workflow/SKILL.md, skills/chough-workflow/references/stages.md, .agents/skills/chough-workflow/SKILL.md, .agents/skills/chough-workflow/references/stages.md, src/skill.rs, tests/skill_releases.rs, tests/fixtures/skill-releases/manifest.json, tests/package.rs, CHANGELOG.md, .chough/cases/cli.routine-fast-path.cases.toml
- **Acceptance**: The workflow skill's stage contract opens with a routine-change fast path: a bounded first read, the session and SpecSync commands in order, a delta for every affected spec, a stop at each human gate, and an early first edit; SKILL.md points to it; installed copies at the previous bundle update to it; the release corpus pins the new bytes.

## Evidence

- Verification commit: `a5e71c300699664c788869db071579d62e984777`
- Base commit: `ddc56e96ff77cb0313d7591bee61d5a4a2950f0e`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

On 2026-11-13/14 Leif ran a bake-off on #276 with three local models (gpt-oss-120b, Flint 40B, Ripple 65B) through Pi in Herdr. Every model spent 30 to 300 tool calls reading AGENTS.md, docs and specs before its first edit; gpt-oss never edited. The two that finished took 153 and 509 minutes, neither opened a Chough session, and Ripple's native change carried no delta, so `specsync change approve` refused with "semantic delta modules must exactly match affected specs". Leif asked to fix this "for all local agents" in chat on 2026-11-14 and, shown this fast path as a chough PR, answered "ok yes please do all of this".

LIFE-22 already holds the intent: any agent that can read Markdown and run commands, local agents included, should work.

Constraints: SKILL.md must stay under 6144 bytes (package test), so the fast path lives in the stage contract every agent already reads, with one pointer in SKILL.md. Changed bundle bytes need their previous digests in `KNOWN_PREVIOUS` and the release corpus manifest's current digests and manifest digest updated.

## From the change's design.md

# Design

No visual or interaction change; design was skipped in the session. The structural choice is placement: the stage contract, which every agent reads at step 5, rather than a new bundle file, so the bundle inventory and SKILL.md byte bound stay as they are.

## From the change's testing.md

# Testing

- `tests/package.rs` `the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change`: SKILL.md routes to the fast path; the section precedes the transitions, is at most 32 lines, and names its steps in order.
- Existing `the_skill_asks_a_resuming_agent_to_state_the_handoff_before_its_first_edit` keeps SKILL.md under 6144 bytes.
- `tests/skill_releases.rs` pins the new current bundle digests; skill status tests cover the known earlier digests.
- Case `TC-cli-skill-routine-fast-path` in `.chough/cases/cli.routine-fast-path.cases.toml`.
- Field check after merge: rerun a local model on a fresh routine issue and count tool calls to its first edit.

## Where these lessons go

- `specs/cli/cli.3md`
