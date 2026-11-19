---
change: fix-298
artifact: testing
---

# Testing

`cargo test --locked --test change ship_refuses_a_history_with_a_change_both_open_and_archived` builds an accept-stage shipping session whose history holds a commit with `.specsync/changes/old/` and `.specsync/archive/changes/2026-11-01-old/`, later removed. `chough change ship` must refuse as `refused:corrupt-history`, naming the commit and `old`, with no SpecSync call (the stub's call log stays empty) and every file unchanged. It fails on main. Case `TC-cli-ship-refuses-a-corrupt-history` binds it. The existing ship tests keep passing.
