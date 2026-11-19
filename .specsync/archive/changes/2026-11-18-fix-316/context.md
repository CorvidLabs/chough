---
change: fix-316
artifact: context
---

# Context

GitHub #316, a regression from #303 (the fix for #267) and a release blocker for 0.6.1. #303 made a hyphen a token boundary before every token prefix. For `sk-`, the model-provider form, that catches ordinary slugs: any value with `-sk-` followed by 20 or more letters, digits and hyphens reads as a key, including IDs Chough generates from a title. So `chough bug add --title "Fix flaky sk-learn tests in CI pipeline"` refuses with `refused:leak: id holds token`, even with `--redact`. And a bug that 0.6.0 committed as `2026-11-18-use-sk-learn-for-the-classifier-model` can no longer be updated, and `chough doctor --leaks` fails on it. The macOS swarm tester found it, and the planning session reproduced it on main.

Leif's decision (2026-11-18, relayed by hc t4c): "Narrow it: not for sk-". Keep the hyphen boundary for the distinctive prefixes (the GitHub forms, `glpat-`, `xox*-`, `AKIA`/`ASIA`), and go back to 0.6.0's rule for `sk-` only, where a hyphen before the prefix continues a word. `AIza` keeps #303's boundary, since only `sk-` is narrowed.

REQ-records-006 names model-provider token forms without fixing the byte before a prefix, so no requirement text changes. Not in scope, and filed in #316 as a follow-up: `doctor --fix --redact` should never rewrite a record's ID into one that no longer matches its file name.
