# Lesson bundle: keep-observations-current-when-the-generated-knowledge-index-is-rebuilt-136

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep observations current when the generated knowledge index is rebuilt (#136)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, tests/evidence.rs, .chough/cases/cli.knowledge-rebuild-136.cases.toml, docs/cli.md, docs/architecture.md, docs/FRD.md, specs/cli/cli.spec.md, skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/lifecycle.md, tests/fixtures/skill-releases/manifest.json, tests/skill_releases.rs, CHANGELOG.md, knowledge/
- **Acceptance**: Rebuilding the knowledge index (chough library index) after recording an observation leaves that observation current: the regression records a passed case observation, confirms current, rebuilds knowledge/INDEX.md (which lists the observation) and confirms it is still current. The configured library-dir name is honored; the legacy fingerprint byte stream is unchanged for older records; .chough/, target/ and SpecSync bookkeeping exclusions behave as before.

## Evidence

- Verification commit: `14487b9c7ba7e6ad71035f17d63b53fe2bd04703`
- Base commit: `996dadc2d26b200a7f745215ef9c151304107e05`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue #136 (filed at Leif's request, 2026-11-05): `knowledge/INDEX.md` lists the most recent observations and sits inside REQ-cli-004's working content identity, so rebuilding the index after recording an observation makes that observation stale. Reproduced by the regression test in tests/evidence.rs: without the fix the observation reads `historical` after `chough library index`.

Constraint: the legacy fingerprint keeps its exact byte stream for older records, so the exclusion applies only to the content identity and the dirty flag. The library dir name is the configured `library-dir` pref (default `knowledge`). KNOW-3: the knowledge directory points at records, it is not one.

## From the change's testing.md

# Testing

| Requirement | Coverage |
|---|---|
| REQ-cli-004 | `TC-cli-knowledge-rebuild-keeps-observation-current` binds the regression `rebuilding_the_knowledge_index_keeps_the_observation_current` in tests/evidence.rs: record a passed case observation, confirm it reads current, rebuild knowledge/INDEX.md listing it, confirm it still reads current |

Failing-first: the regression is committed before the fix and run there through the case (failed observation linked to the #136 bug); the same case passes after the fix. The full `fledge run test` suite and the existing freshness and retention suites run unchanged.

## Where these lessons go

- `specs/cli/cli.3md`
