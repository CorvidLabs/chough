---
change: gemini-host-cold-start-and-roadmap-refresh
artifact: tasks
---

# Tasks

- [x] Add case definition TC-cli-0105 in .chough/cases/cli.cases.toml.
- [x] Update guidance in skills/chough-workflow/SKILL.md and references/checkpoint.md.
- [x] Mirror bundle updates to .agents/skills/ and .claude/skills/ ensuring byte-identical copies.
- [x] Add previous bundle file digests to KNOWN_PREVIOUS in src/skill.rs.
- [x] Update docs/exercises/cold-start.md with the dated Gemini host run.
- [x] Refresh stale roadmap rows in docs/ROADMAP.md.
- [x] Record .chough/artifacts/gemini-cold-start.log and observation for TC-cli-0105.
- [x] Run cargo fmt --check, cargo clippy --all-targets -D warnings, and cargo test.
- [x] Run chough cases check and fledge lanes run verify.

Human gates and post-verification steps (not tasks):
- Human scope approval of this change (specsync change approve by 0xLeif).
- specsync change check to materialize delta.
- Implementation review by 0xLeif at acceptance.
- Native review and ship.
