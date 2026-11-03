---
change: pr-50-evidence-fix-record-a-fresh-clean-tree-verify-run-as-the-current-agent-assisted-proof-in-place-of-the-mis
artifact: testing
---

# Testing

- The new observation snapshot shows `dirty: false` at the logged revision.
- `cmp` against HEAD shows the old records unchanged.
- `fledge lanes run verify` passes.
