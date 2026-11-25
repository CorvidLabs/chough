# Lesson bundle: cover-req-cli-072-test-and-fixture-updates

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Cover REQ-cli-072 test and fixture updates
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/docs.rs, tests/fixtures/skill-releases/manifest.json, tests/skill_releases.rs
- **Acceptance**: The archive audit covers tests/docs.rs, tests/fixtures/skill-releases/manifest.json and tests/skill_releases.rs; hosted Archive and verify lanes pass on the branch head; the skill-chough-first step runs in hosted CI.

## Evidence

- Verification commit: `7fc2dc223601661e15b0aff2247e54e4e645b699`
- Base commit: `9d826c7c660b579ad7e83faf6fac6e31fb29062d`
- Verified by: `specsync check --spec cli`

## From the change's context.md

## Context

The REQ-cli-072 change (archived) edited tests/docs.rs (README lane-list wording),
tests/skill_releases.rs (manifest pin) and tests/fixtures/skill-releases/manifest.json
(fixture digests) without declaring them in affected_paths; the archive audit refused
hosted CI. corvid-agent's review named the gap (Blocking 1) and DRIVE-20 routes the
fix through chough.

## From the change's testing.md

## Testing

- `cargo test --locked` full suite green on the branch head (already run after the
  review fixes).
- Hosted Archive + verify lanes green after push.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| (no-spec-change) | cargo test --locked: 1044 passed, 0 failed at the review-fix commit |

## Where these lessons go

- `specs/cli/cli.3md`
