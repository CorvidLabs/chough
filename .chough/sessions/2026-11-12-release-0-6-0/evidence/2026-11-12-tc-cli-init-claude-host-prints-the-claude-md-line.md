---
kind: evidence
id: 2026-11-12-tc-cli-init-claude-host-prints-the-claude-md-line
aliases: []
claim: "Ran cargo test --locked --test optional -- --exact init_with_the_claude_host_prints_the_claude_md_line_and_writes_none for TC-cli-init-claude-host-prints-the-claude-md-line: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-init-claude-host-prints-the-claude-md-line]
case_digest: [3eac1ceb4d2119359f445c1f9d95150a4644fb4c90fca9be1a964dafedff4106]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - optional
  - "--"
  - "--exact"
  - init_with_the_claude_host_prints_the_claude_md_line_and_writes_none
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: de7893f8aad7c737300c7aa841f8737100d77f77
content: 57a5ce76a35a464f1e318ed9735b9a02f90f91f95eca257ebbe5e57f108f5279
environment: local-macos-arm64
fingerprint: sha256:a2510e0c864634a917a4e218875e3ec67394b7c681e309aec42df50cea158844
criteria: [LIFE-12, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:14:46Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test optional -- --exact init_with_the_claude_host_prints_the_claude_md_line_and_writes_none for TC-cli-init-claude-host-prints-the-claude-md-line: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision de7893f8aad7, observed 2026-11-12T22:14:46Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- init_with_the_claude_host_prints_the_claude_md_line_and_writes_none stdout ----
- thread 'init_with_the_claude_host_prints_the_claude_md_line_and_writes_none' (925663015) panicked at tests/optional.rs:1106:5:
- assertion `left == right` failed: {
-   "created": [
-     ".chough/sessions/2026-11-12-a-claude-project/session.md",
-     ".chough/README.md",
-     ".agents/skills/chough-workflow/SKILL.md",
-     ".agents/skills/chough-workflow/agents/openai.yaml",
-     ".agents/skills/chough-workflow/references/accept.md",
-     ".agents/skills/chough-workflow/references/bugs.md",
-     ".agents/skills/chough-workflow/references/build.md",
-     ".agents/skills/chough-workflow/references/checkpoint.md",
-     ".agents/skills/chough-workflow/references/design.md",
-     ".agents/skills/chough-workflow/references/intent.md",
-     ".agents/skills/chough-workflow/references/librarian.md",
-     ".agents/skills/chough-workflow/references/lifecycle.md",
-     ".agents/skills/chough-workflow/references/plan.md",
-     ".agents/skills/chough-workflow/references/release.md",
-     ".agents/skills/chough-workflow/references/roles.md",
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.67s
