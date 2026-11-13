---
kind: evidence
id: 2026-11-13-tc-cli-the-readme-init-paragraph-names-the-host-directories
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly]
case_digest: [f5023a12e5493bcca87a2014014e9a2988bd9a4186712dae93134e87e305b229]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: f4cfb57bd145d15ca613b2747d73c5372137538d
content: 7b5512beec4b9a2f80ee505be47bcd1cb09ac81b65b09d909d878177e022d9ed
environment: local-macos-arm64
fingerprint: sha256:90c14ff5045120938ed975b48349568cf46a09895ebe3850bc80d7d08f4c2dba
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:11:33Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical for TC-cli-the-readme-init-paragraph-names-the-host-directories-briefly: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision f4cfb57bd145, observed 2026-11-13T03:11:33Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical stdout ----
- thread 'the_readme_init_paragraph_names_the_host_directories_without_a_long_parenthetical' (932807804) panicked at tests/docs.rs:1935:5:
- a parenthetical of 509 bytes in README's init paragraph: `init` never overwrites existing files. With `--objective` it creates the first
- session, `.chough/sessions/<date>-<slug>/session.md`, whose front matter is the
- checkpoint; it writes the reading guide `.chough/README.md`, creates the librarian's
- `knowledge/` directory (on by default) and copies the skill bundle into each requested host's project skill directory
- (`openai`, the default, → `.agents/skills`, a shared skills directory, which `claude`
- shares (Claude Code 2.1.316 did not discover `.agents/skills` alone, so add a
- `CLAUDE.md` line pointing at `.agents/skills/chough-workflow/SKILL.md`; `init --host
- claude` prints that line as `claude_md` and does not write it, see
- [integrations](docs/integrations.md)); `gemini` → `.gemini/skills`, which
- no run has exercised; also `codex` → `.codex/skills`, `cursor` → `.cursor/skills` and
- `grok` → `.grok/skills`). Start each later change with
- `chough session new <change-id>`. Kimi Code CLI uses the existing `openai`
- installer value and discovered `.agents/skills/chough-workflow/SKILL.md` in one
- observed run. Google Antigravity read the skill from `.agents/skills` in one run
- reported narratively in closed PR #20 and not independently verified. `chough next`
- prints the focused next action as text (`--json` for compact JSON); `chough status`
- is the full JSON report. Then ask your agent in plain language, for example "Use the
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 19 filtered out; finished in 0.00s
