# Lesson bundle — gemini-host-cold-start-and-roadmap-refresh

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Gemini host cold start and roadmap refresh
- **Kind**: Feature
- **Specs**: cli
- **Paths**: docs/ROADMAP.md, docs/exercises/cold-start.md, skills/chough-workflow/SKILL.md, skills/chough-workflow/references/checkpoint.md, .agents/skills/chough-workflow/SKILL.md, .agents/skills/chough-workflow/references/checkpoint.md, .claude/skills/chough-workflow/SKILL.md, .claude/skills/chough-workflow/references/checkpoint.md, src/skill.rs, tests/package.rs
- **Acceptance**: Gemini host cold start: an independent Gemini host session on macOS arm64 discovers and resumes the Chough workflow from project files alone, records tool versions and friction honestly, and honors human gates (FLOW-1, FLOW-13, LIFE-4, REQ-cli-007, REQ-cli-016). Roadmap refresh: docs/ROADMAP.md records PR #8 and #9 as merged, adds merged rows for PR #15, #16 and #17, and updates More hosts with the Gemini run. Cold-start documentation: docs/exercises/cold-start.md adds a dated Gemini run section with commands, limits and assessment. Skill guidance: skills/chough-workflow/SKILL.md and references/checkpoint.md document Gemini discovery and cold resume from stage done, keeping .agents/ and .claude/ copies byte-identical without dashes. Bundle tracking: earlier digests are added to KNOWN_PREVIOUS in src/skill.rs, skill status reports copies current, and TC-cli-0105 passes.

## Evidence

- Verification commit: `601da82666809686b0c98635028d7db330edd07a`
- Base commit: `5af3f78d66b40a5af42607eafd5649c9b957d2ad`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## Origin

The Chough roadmap (docs/ROADMAP.md) includes the milestone "More hosts: Exercise Cursor, Gemini, Grok and Kimi discovery and cold starts. Proposed; only Claude Code and Codex were exercised." Additionally, docs/exercises/cold-start.md notes that only a Codex session had been recorded and that Gemini had not been exercised.

This change records the genuine cold-start exercise for the Gemini host running in Google Antigravity on macOS arm64, documents the observed discovery path and friction, refreshes the roadmap to reflect merged PRs #8, #9, #15, #16 and #17 along with this Gemini exercise, and updates skill guidance so that Gemini and Antigravity agents can discover the skill and resume cold when stage is done.

## Constraints

- Current-agent-first v1: keep the small Rust helper small without adding a model client, provider runtime, or external state.
- No em or en dashes in any added text, in the skill bundle or documentation.
- Never record or simulate human decisions. Human scope approval and implementation review remain strictly with 0xLeif.
- Keep the canonical skill bundle in skills/chough-workflow and its copies in .agents/skills/chough-workflow and .claude/skills/chough-workflow byte-identical.
- Maintain tool versions matching CI: hi 0.8.0, fledge 1.7.2, and specsync 6.1.1.

## From the change's design.md

# Design

## 1. Gemini and Antigravity Host Discovery Guidance

In skills/chough-workflow/SKILL.md (and mirrored in .agents/ and .claude/):
- Document that Google Antigravity / Gemini CLI automatically discovers skills located in .agents/skills/chough-workflow by convention, in addition to .gemini/skills/chough-workflow configured by `chough init --host gemini`.
- Clarify host usage to make discovery explicit across hosts.

## 2. Checkpoint Resumption from Stage Done

In skills/chough-workflow/references/checkpoint.md:
- Clarify that when resuming cold after a prior change has been archived and merged (stage done), next_action in the checkpoint reflects the finalized change. A new change cycle begins by resetting stage (`chough stage reset --to intent --reason "<why>"`) and defining a new native SpecSync change (`specsync change new`).

## 3. Bundle Hygiene and Digest Tracking

- When SKILL.md or references/checkpoint.md is updated, the previous digests from main at 5af3f78 (d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c and 20daaaa0cec972169abcedd508cd9ad18ff93ef7ec9d1f4e7330fcd18ca80ac0) are added to KNOWN_PREVIOUS in src/skill.rs.
- All bundle copies (skills/chough-workflow/, .agents/skills/chough-workflow/, and .claude/skills/chough-workflow/) are kept byte-identical.
- All bundle text is verified free of em (U+2014) and en (U+2013) dashes.

## 4. Cold-Start Exercise Documentation

In docs/exercises/cold-start.md:
- Add a new dated section for the Gemini host cold start observed on 2026-11-01 on macOS arm64 in Google Antigravity CLI.
- Record the exact setup, discovery path, observed commands, execution times, assessment against expectations, and limits (one agent, one run, local-macos-arm64).

## 5. Roadmap Refresh

In docs/ROADMAP.md:
- Update PR #8 row to reflect that it is merged.
- Update PR #9 row to reflect that it is merged (no release published).
- Add rows for PR #15 (timing report), PR #16 (bundle hygiene, case binding environment, colour-safe matching, actionable requirement and bug errors), and PR #17 (evidence freshness across commits and SpecSync bookkeeping).
- Update the More hosts row to record the Gemini host exercise alongside Claude Code and Codex.

## From the change's testing.md

# Testing

## Focused cases

- TC-cli-0105: agent-assisted case verifying that a fresh Gemini host discovers the workflow skill, resumes from project files alone, records honest tool versions and friction, and respects human gates.
- TC-cli-0062 to TC-cli-0065, TC-cli-0090, TC-cli-0091: package and bundle hygiene tests in tests/package.rs verifying that the bundle contains no em or en dash and that earlier bundle digests are recognized as known-outdated.
- chough skill status: verifies that all bundle files across installed hosts are current.
- chough cases check: verifies that case definitions in .chough/cases/cli.cases.toml are valid under chough-testing/1.
- fledge lanes run verify: full verification lane passing formatting, lints, tests, HI checks, and SpecSync strict checks.

## Where these lessons go

- `specs/cli/context.md`
