---
change: claude-host-installs-the-shared-agents-skill-copy-legacy-claude-copies-are-reported-and-removed-only-on-an-explicit
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-016, LIFE-12 | `tests/optional.rs::claude_host_installs_the_shared_agents_copy_once`: `init --host claude --host openai` creates only `.agents/skills/chough-workflow/` and no `.claude/`; `skill update --host claude` acts on the shared copy. Fails at 0363d4a. |
| REQ-cli-016, LIFE-13 | `tests/optional.rs::legacy_claude_copy_is_reported_and_removed_only_explicitly`: a legacy `.claude` copy with one edited file and one known-outdated file is reported as legacy by status and update and stays byte-identical; `remove-legacy` refuses without the shared copy; after install it removes current and known-outdated files and keeps the edited file. Fails at 0363d4a. |
| REQ-cli-016, LIFE-14 | `tests/workflow_authority.rs` checks the `.agents` installed copy and that this repository has no `.claude/`; `tests/package.rs` keeps `.claude/` forbidden in the package. `tests/optional.rs::skills_install_per_host_and_update_only_unmodified_files` keeps its two-directory coverage with codex instead of claude. |

## Cases

- TC-cli-claude-host-shares-agents-copy (LIFE-12) binds `claude_host_installs_the_shared_agents_copy_once`; recorded failed before the fix.
- TC-cli-legacy-claude-copy-removed-only-explicitly (LIFE-13) binds `legacy_claude_copy_is_reported_and_removed_only_explicitly`; recorded failed before the fix.

## Focused and full checks

`cargo test --test optional`, the focused case through `chough cases run`, and `fledge lanes run verify` with hi 0.8.0, fledge 1.8.0 and specsync 6.1.2.
