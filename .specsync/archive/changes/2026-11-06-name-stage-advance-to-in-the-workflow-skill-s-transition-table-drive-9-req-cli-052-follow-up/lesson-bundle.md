# Lesson bundle: name-stage-advance-to-in-the-workflow-skill-s-transition-table-drive-9-req-cli-052-follow-up

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Name stage advance --to in the workflow skill's transition table (DRIVE-9, REQ-cli-052 follow-up)
- **Kind**: Documentation
- **Specs**: cli
- **Paths**: skills/chough-workflow/references/stages.md, .agents/skills/chough-workflow/references/stages.md, src/skill.rs, tests/package.rs, CHANGELOG.md
- **Acceptance**: The bundled and installed chough-workflow references/stages.md transition tables have a byte-identical row for 'chough stage advance --to <stage>' saying every step is checked first on one native read, a refused step records nothing, and into done it reads native status once. The stages.md that main shipped from 0.1.0 to 01f1d023 (sha256 658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f) is reported known-outdated by 'chough skill status' and becomes current with 'chough skill update', while an edited copy stays edited. The new test fails first, and fledge lanes run verify passes.

## Evidence

- Verification commit: `7868b0a64ee43687534718678e5e901a4c7ec9c9`
- Base commit: `01f1d023072da58e11feffd7408c98b9e341c3a9`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

#147 (REQ-cli-052, DRIVE-9) added `chough stage advance --to <stage>`, but the chough-workflow skill's transition table in `references/stages.md` does not name it. Agents learn their stage commands from that table, so they keep running `stage advance` once per step. The orc assigned this follow-up after #147 merged (t18 relay, 2026-11-05 late evening MST). It is a separate small change and PR. kimi-perf32 (#32) is not touching the skill. The pgrep flake is #144.

`stages.md` has been byte-identical since #22, so every installed copy from 0.1.0 to 0.4.0 has this one digest, 658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f at main 01f1d023. It must join `KNOWN_PREVIOUS` in `src/skill.rs`. Otherwise `chough skill status` would call those copies `edited` rather than `known-outdated`, and `skill update` would leave them stale.

## From the change's design.md

# Design

- **The row:** one new row in the "Transitions shared by every stage" table, after "Next stage":
  - Intent: "Several stages at once".
  - Command: `chough stage advance --to <stage>`.
  - Notes: every step is checked first, on one native read; a refused step records nothing; into done it reads native status once.
- **Both copies:** the same bytes go in `skills/chough-workflow/references/stages.md` and in this repository's installed copy, `.agents/skills/chough-workflow/references/stages.md`.
- **`src/skill.rs`:** `KNOWN_PREVIOUS` gains `references/stages.md` at 658ce798…, the copy shipped from 0.1.0 (#22) to main 01f1d023. Installed copies of it then report `known-outdated` and update. An edited copy still reports `edited` and is never overwritten.
- **Test:** `tests/package.rs` gets one test.
  - It writes main 01f1d023's `stages.md` into an installed copy and asserts its digest.
  - `skill status` reports it `known-outdated`.
  - `skill update` makes it `current`, and the result names `chough stage advance --to <stage>`.

## Risks

- **Byte drift between copies:** none expected. Both copies get the same bytes, and the existing package tests compare them.
- **Prose rules:** the no-em-or-en-dash check still applies.

## Constraints consulted

- REQ-cli-052 and DRIVE-9.
- The skill-copy rules in `src/skill.rs` (#54, 0.4.0).
- tests/prose.rs.

## Where these lessons go

- `specs/cli/context.md`
