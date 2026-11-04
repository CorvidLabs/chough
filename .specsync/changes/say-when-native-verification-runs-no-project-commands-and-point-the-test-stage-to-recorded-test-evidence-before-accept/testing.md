---
change: say-when-native-verification-runs-no-project-commands-and-point-the-test-stage-to-recorded-test-evidence-before-accept
artifact: testing
---

# Testing

- `tests/verification.rs`, with stub `hi` and `specsync` and an approved change at
  test:
  - with `{"enabled": true}` (no `verification_commands`), `chough next` text and
    `--json`, `chough status` and `chough stage show` carry the warning naming
    `verification_commands`;
  - with `"verification_commands": ["cargo test"]`, and with SDD disabled, none does;
  - at test with nothing else stopping it, `next` names `chough cases run` and says
    recorded test evidence is needed before accept;
  - `stage advance` from test is allowed with and without the warning, and the stub
    `specsync` is called the same number of times either way.
- Each new test fails on main 91a1fdd. `fledge lanes run verify` passes on Linux.
