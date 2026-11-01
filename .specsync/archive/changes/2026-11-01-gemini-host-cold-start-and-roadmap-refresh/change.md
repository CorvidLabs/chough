---
id: gemini-host-cold-start-and-roadmap-refresh
state: archived
type: feature
base_commit: 5af3f78d66b40a5af42607eafd5649c9b957d2ad
---

# Gemini host cold start and roadmap refresh

## Intent

Gemini host cold start and roadmap refresh

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Gemini host cold start: an independent Gemini host session on macOS arm64 discovers and resumes the Chough workflow from project files alone, records tool versions and friction honestly, and honors human gates (FLOW-1, FLOW-13, LIFE-4, REQ-cli-007, REQ-cli-016). Roadmap refresh: docs/ROADMAP.md records PR #8 and #9 as merged, adds merged rows for PR #15, #16 and #17, and updates More hosts with the Gemini run. Cold-start documentation: docs/exercises/cold-start.md adds a dated Gemini run section with commands, limits and assessment. Skill guidance: skills/chough-workflow/SKILL.md and references/checkpoint.md document Gemini discovery and cold resume from stage done, keeping .agents/ and .claude/ copies byte-identical without dashes. Bundle tracking: earlier digests are added to KNOWN_PREVIOUS in src/skill.rs, skill status reports copies current, and TC-cli-0105 passes.

## No-spec Rationale

Guidance, evidence, and roadmap currency only; canonical contract and requirements in specs/cli/ are unchanged.
