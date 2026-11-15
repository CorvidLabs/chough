# Lesson bundle: clarify-req-cli-001-init-preserves-bytes-acceptance

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Clarify REQ-cli-001 init-preserves-bytes acceptance
- **Kind**: Documentation
- **Specs**: cli
- **Paths**: specs/cli/requirements.md, .chough/cases/cli.walk-ready.cases.toml, CHANGELOG.md, docs/cli.md
- **Acceptance**: Repeated init preserves edited session and skill bytes; missing bundle files can be added without overwriting neighbors; a session with edited files stays byte-identical on later init.

## Evidence

- Verification commit: `ba30b7d713b9b331bf7e2e2362a531e1aa59c56a`
- Base commit: `cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

REQ-cli-001 ("Preserve user files") already states that init must not replace
existing files, and its acceptance line covers repeated init and adding missing
bundle files. The walk-through of `chough change ready` (DRIVE-29) showed the
readiness command reports a missing delta whenever a change touches a spec, so
this documentation change needs its own `deltas/cli.md` before approval. No code
or behavior changes; only the acceptance line is clarified to name both
preservation behaviors explicitly.

## Where these lessons go

- `specs/cli/cli.3md`
