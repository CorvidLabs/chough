---
change: gemini-host-cold-start-and-roadmap-refresh
artifact: plan
---

# Plan

## Approach

Implement the Gemini host discovery guidance, document the cold-start exercise, refresh the roadmap, add prior bundle digests to KNOWN_PREVIOUS, and add case TC-cli-0105 to verify the Gemini cold-start evidence.

## Out of scope

- No changes to Chough CLI architecture, native SpecSync state machines, or schema versions.
- No model provider API client or hosted runtime integration.
- No fabrication of human approvals or reviews.

## Steps

1. Reserve case TC-cli-0105 in .chough/cases/cli.cases.toml for the Gemini host cold start.
2. Update skills/chough-workflow/SKILL.md and references/checkpoint.md with Gemini host discovery guidance and stage-done cold resume instructions.
3. Mirror edits byte-for-byte to .agents/skills/ and .claude/skills/.
4. Add the previous digests of the edited bundle files to KNOWN_PREVIOUS in src/skill.rs.
5. Update docs/exercises/cold-start.md with the dated Gemini run.
6. Refresh docs/ROADMAP.md with merged PRs #8, #9, #15, #16, #17 and the Gemini host entry.
7. Record the Gemini cold-start scratch log as .chough/artifacts/gemini-cold-start.log and record TC-cli-0105 observation.
8. Run cargo fmt, clippy, test, chough skill status, chough cases check, and fledge lanes run verify.
9. Stop at the scope gate, checkpoint with awaiting-human, push branch, open draft PR, and request human scope approval.

## Risks

- Divergence across bundle copies: mitigated by checking chough skill status and diffing copies.
- Em or en dash insertion: mitigated by package tests in tests/package.rs.

## Constraints consulted

- Current-agent-first v1; small Rust helper; no dashes; byte-identical bundle copies; tool versions matching CI.
