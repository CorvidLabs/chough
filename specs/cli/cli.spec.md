---
module: cli
version: 1
status: active
files:
  - src/main.rs
  - src/bugs.rs
  - src/cases.rs
  - src/checkpoint.rs
  - src/context.rs
  - src/evidence.rs
  - src/library.rs
  - src/prefs.rs
  - src/records.rs
  - src/skill.rs
  - src/stage.rs
  - src/validate.rs
  - tests/cli.rs
  - tests/bugs.rs
  - tests/cases.rs
  - tests/optional.rs
  - tests/stages.rs
  - tests/common/mod.rs

db_tables: []
depends_on: []
---

# Chough CLI

## Purpose

Provide the optional local helper for Chough's current-agent workflow. It installs
skills without replacing user files, assembles bounded focused context, reports
native lifecycle guidance, and validates observations tied to repository content
and environment. See requirements.md and docs/FRD.md.

## Public API

The binary exposes CLI commands; it does not export a Rust library API. The
observable command contract is `chough --help` and README.md. Private Rust
implementation details are not a public library promise.

There are no public Rust exports. Command behavior is specified under Behavioral
Examples and in the README command reference, rather than presented as Rust API
symbols to the structural checker.

## Invariants

1. Initialization never overwrites an existing user file, including an edited skill.
2. The helper never creates or authenticates human approval, review, or deployment.
3. HI owns criterion IDs; SpecSync owns its lifecycle and module companions.
4. Observations bind revision, working content, and environment; current evidence
   is not equivalent to proven criterion achievement.
5. Session/module/artifact paths stay inside the selected project. Context is
   bounded and deterministic for the same files and native tool responses.
6. Missing tools, missing sources, unsupported schemas, and stale evidence remain
   visible; they do not silently become successful states.

## Behavioral Examples

```gherkin
Given an edited session and installed skill
When init is repeated
Then existing files remain byte-for-byte unchanged

Given product-test observations on one revision/environment
When relevant working files change or a different environment is selected
Then status reports those observations stale or mismatched

Given a selected module and scoped HI family
When a new agent requests context
Then it receives instructions, intent, module companions, decisions and next action
without loading the whole repository or conversation history
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Invalid path/schema | A record escapes the root or uses an unsupported version | Fail with an actionable error |
| Missing native tool/source | Context or lifecycle information is unavailable | Report unavailable/unknown without inventing state |
| Existing observation ID | An observation record already exists | Refuse replacement and retain the original |
| Invalid traceability | A criterion or requirement cannot be established | Refuse to record the observation |

## Dependencies

Rust dependencies are declared in Cargo.toml. Git supplies revision and working
content identity; HI supplies scoped intent; SpecSync supplies native lifecycle
output. The helper invokes no model provider and runs no product tests itself.

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-10-31 | Authorized first-slice bootstrap contract; no native approval claimed |
