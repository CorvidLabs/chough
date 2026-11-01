---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-019 | `tests/package.rs`: no U+2013 or U+2014 in any bundle file; dashed 2756dc2 copies of frd.md and cases.toml, and 2756dc2 copies of build.md, plan.md and testing.md, read known-outdated and update to current; an edited copy stays edited |
| REQ-cli-005 | `tests/cli.rs`: `evidence add` with a cross-module ID names the module and the `--contract` fix; a typo gets the neutral hint; both refuse with the old exit code |
| REQ-cli-009 | `tests/cases.rs`: cross-module case names `.chough/cases/<module>.cases.toml`; unmaterialized delta ID gets the `specsync change check` hint; hyphenated module names derive correctly; `env` bounds refused with location; ESC fragment refused; a case without `env` keeps its 2756dc2 digest; editing `env` changes the digest |
| REQ-cli-011 | `tests/cases.rs`: a declared variable reaches the command (portable `sh` or `cmd` printer per platform) and the limitation names it without its value; vitest-style coloured output passes its fragment with the escape limitation and a raw byte-exact artifact; an absent fragment still fails; unit tests of the cleaner for CSI, OSC (BEL and ESC backslash), two-byte and truncated sequences |
| REQ-cli-012 | `tests/bugs.rs`: `--classification blocking` and `--scope product-defect` name the other flag; `--help` lists both value sets; exit code unchanged |

Regression: full `cargo test`, `fledge lanes run verify` (including the
feature-and-bug exercise) and native structural checks on all three CI platforms.
Real behavior: rerun the triage reproductions of #13 (`chough init --host claude
--host openai` then a dash search), #14 items 1, 2, 4 and 5 and #11 on the built
binary, and record cases linked to BUG-0017, BUG-0018 and BUG-0019 after `change
check`.
