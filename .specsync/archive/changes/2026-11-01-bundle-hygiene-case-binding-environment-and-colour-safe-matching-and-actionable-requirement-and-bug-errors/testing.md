---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-019 | `tests/package.rs`: no U+2013 or U+2014 in any bundle file; dashed 2756dc2 copies of frd.md and cases.toml, and the 2756dc2 copy of references/testing.md, read known-outdated and update to current; an edited copy stays edited |
| REQ-cli-005 | `tests/cli.rs`: `evidence add` with a cross-module ID names the module and the `--contract` fix; a typo gets the neutral hint; both refuse with the old exit code |
| REQ-cli-009 | `tests/cases.rs`: cross-module case names `.chough/cases/<module>.cases.toml`; unmaterialized delta ID gets the `specsync change check` hint; hyphenated module names derive correctly; `env` bounds refused with location (name pattern and 128-byte length, case-insensitive duplicate, NUL, 33 entries, 4 KiB value, each refused cargo-test name in mixed case); ESC fragment refused; a case without `env` keeps its 2756dc2 digest; editing `env` changes the digest |
| REQ-cli-011 | `tests/cases.rs`: a declared variable reaches the command (portable `sh` or `cmd` printer per platform) and the limitation names it without its value; a case without `env` carries the inherited-environment limitation; vitest-style coloured output passes its fragment with the escape limitation and a raw byte-exact artifact; an absent fragment still fails; unit tests of the cleaner for CSI, OSC (BEL and ESC backslash), two-byte sequences limited to 0x40 to 0x5F, ESC before a line break, an OSC or CSI cut by a line break or the end of input (left unchanged); a property test that cleaning keeps every line break and never lowers the count of `test result:` lines; a cargo-test case whose filter matches 2 tests while a test prints an unterminated OSC still does not pass |
| REQ-cli-012 | `tests/bugs.rs`: `--classification blocking` and `--scope product-defect` name the other flag; `--help` lists both value sets; exit code unchanged |

Regression: full `cargo test`, `fledge lanes run verify` (including the
feature-and-bug exercise) and native structural checks on all three CI platforms.
## Bug verification order

references/bugs.md treats a bug without a linked failed observation as unverified,
and BUG-0019, BUG-0018 and BUG-0017 have none yet. Open BUG-0014 means Chough itself
does not yet check that a pass came after the failure, so the order is kept here:

1. Red, before any fix: each new case in `.chough/cases/cli.cases.toml` (BUG-0017
   error half, BUG-0018, BUG-0019) runs at the unfixed tip and records failed; that
   observation is linked to its bug first. These cases link only existing requirement
   IDs, so they validate before `change check`.
2. Green, after verification: the same unchanged case runs again, in a later second,
   and records passed; the pass is linked. BUG-0018 and BUG-0019 are set verified.
   BUG-0017 gets the pass and a note, and stays open for its documented-order half,
   which the evidence-freshness change owns.

Real behavior: rerun the triage reproductions of #13 (`chough init --host claude
--host openai` then a dash search), #14 items 1, 2, 4 and 5 and #11 on the built
binary.
