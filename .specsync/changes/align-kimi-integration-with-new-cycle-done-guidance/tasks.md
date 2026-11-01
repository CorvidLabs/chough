---
change: align-kimi-integration-with-new-cycle-done-guidance
artifact: tasks
---

# Tasks

- [ ] Update TC-cli-0105 and the regression expectations for REQ-cli-021.
- [ ] Record the TC-cli-0105 failure at the pre-fix integrated tip and link it to BUG-0025.
- [ ] Remove the native-routing override and preserve main's focused next, gates and approved-delta behavior.
- [ ] Correct the affected Kimi docs and testing companions without changing the canonical contract or frozen archives.
- [ ] Run focused tests, TC-cli-0105, TC-cli-0121 and TC-cli-0122, then the full verify and pre-push lanes with CI-matching tools.
- [ ] Commit source and `.chough/` records, run native `change check --commit`, push and observe hosted checks.

Human gate: 0xLeif reviews the implementation. Record the native review only after that decision.

Human gate: finalize only on 0xLeif's explicit instruction, advance the checkpoint to done, push and observe checks. Merge under the existing PR instruction only after archive and passing gates. No release or deployment follows automatically.
