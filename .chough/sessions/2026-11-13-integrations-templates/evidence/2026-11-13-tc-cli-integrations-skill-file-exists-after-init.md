---
kind: evidence
id: 2026-11-13-tc-cli-integrations-skill-file-exists-after-init
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-skill-file-exists-after-init]
case_digest: [23f650f2098c747e712edc8684d3bda735c787475ceb29c5d44401c9fe1ad543]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - each_templates_skill_file_exists_after_its_own_chough_init
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 2cf5facb7c792081f8f73104eb458f4cda4408b9
content: e87d689143e165f16ea94afa6e1b6bb1edd9f5a40749321661f272dfb48435ef
environment: local-macos-arm64
fingerprint: sha256:62b609932de40b316cf52cb653524b467359093b602061934e21ced09153877b
criteria: [LIFE-22, LIFE-24]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:07:17Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 2cf5facb7c79, observed 2026-11-13T06:07:17Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- each_templates_skill_file_exists_after_its_own_chough_init stdout ----
- thread 'each_templates_skill_file_exists_after_its_own_chough_init' (960887693) panicked at tests/integrations.rs:312:9:
- grok's template loads .agents/skills/chough-workflow/SKILL.md, which chough init --host grok does not create
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out; finished in 2.01s
