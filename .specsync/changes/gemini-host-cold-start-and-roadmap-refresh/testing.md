---
change: gemini-host-cold-start-and-roadmap-refresh
artifact: testing
---

# Testing

## Focused cases

- TC-cli-0105: agent-assisted case verifying that a fresh Gemini host discovers the workflow skill, resumes from project files alone, records honest tool versions and friction, and respects human gates.
- TC-cli-0062 to TC-cli-0065, TC-cli-0090, TC-cli-0091: package and bundle hygiene tests in tests/package.rs verifying that the bundle contains no em or en dash and that earlier bundle digests are recognized as known-outdated.
- chough skill status: verifies that all bundle files across installed hosts are current.
- chough cases check: verifies that case definitions in .chough/cases/cli.cases.toml are valid under chough-testing/1.
- fledge lanes run verify: full verification lane passing formatting, lints, tests, HI checks, and SpecSync strict checks.
