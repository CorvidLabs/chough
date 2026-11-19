# Lesson bundle: fix-295-the-init-case-runs-the-real-preservation-test

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix #295: the init case runs the real preservation test
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/cli.rs, .chough/cases/cli.walk-ready.cases.toml
- **Acceptance**: TC-cli-walk-ready-init-preserves runs init_preserves_user_files_and_installs_a_complete_skill. That test keeps edited session and skill bytes and adds a missing bundle file without rewriting the edited neighbor. specsync change audit reports no uncovered meaningful path.

## Evidence

- Verification commit: `24a068b9f81132fb55f4fc0ae692d1369704115d`
- Base commit: `82a52e2dd5cb9f131b5c1a33133002fbda83b9f6`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Pull request 295 clarified REQ-cli-001, then its case ran `echo preserves`. Review asked to bind that case to `init_preserves_user_files_and_installs_a_complete_skill` and to add the missing-bundle assertion, keeping the original receipts and correcting the story. The requirement text already says this. The archived clarification change cannot cover the new test file, so this change does.

## From the change's testing.md

# Testing

`chough cases run TC-cli-walk-ready-init-preserves --environment local-macos-arm64` runs the cargo test `init_preserves_user_files_and_installs_a_complete_skill`. That test keeps edited session and skill bytes, then deletes one bundle file and checks a later init adds it without rewriting the edited neighbor.

## Where these lessons go

- `specs/cli/cli.3md`
