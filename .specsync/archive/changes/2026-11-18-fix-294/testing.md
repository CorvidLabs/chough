---
change: fix-294
artifact: testing
---

# Testing

`cargo test --locked --test gates a_gate_summary_names_its_change_with_a_plain_dash` opens a gate on a session whose native change is recorded, and requires `chough gate list` to print `Change: <id> - <title> (` with no em dash anywhere in the summary. The code already does this, so the test passes on main. It is shown to guard the line by failing against `src/gates.rs` with the em dash put back, a local edit that is never committed. Case `TC-cli-gate-summary-names-its-change-with-a-plain-dash` binds it.
