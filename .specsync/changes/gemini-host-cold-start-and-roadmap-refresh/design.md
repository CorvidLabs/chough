---
change: gemini-host-cold-start-and-roadmap-refresh
artifact: design
---

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
