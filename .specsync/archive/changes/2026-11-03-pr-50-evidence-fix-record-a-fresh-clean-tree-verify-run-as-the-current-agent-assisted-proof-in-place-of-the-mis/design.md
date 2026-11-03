---
change: pr-50-evidence-fix-record-a-fresh-clean-tree-verify-run-as-the-current-agent-assisted-proof-in-place-of-the-mis
artifact: design
---

# Design

1. The change definition is committed first, so the tree is clean at a named revision R.
2. `fledge lanes run verify` runs on that clean tree, with its output written outside the repository. Git status is checked as clean before and after the run.
3. A small log goes into `.chough/artifacts/`. Its header names R, the clean tree and the command; the body is the lane's step summary.
4. `chough evidence add` (product-test, agent-assisted) records that log. Paths under `.chough/` are excluded from the snapshot, so the snapshot is R and clean. Its limitation names OBS-product-test-035959f28aa76d052d48fe78 as replaced. No existing record is edited.
