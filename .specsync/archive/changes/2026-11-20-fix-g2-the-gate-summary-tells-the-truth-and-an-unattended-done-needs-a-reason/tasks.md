---
change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
artifact: tasks
---

# Tasks

- [x] Failing tests first, each red on main: F4 in `tests/gates.rs`; F14's uncommitted
      diff and missing base in `tests/gates.rs`; F5's journal post in `tests/doctor.rs`;
      #390's refusal, warning and Timeline reason in `tests/stages.rs`, with a leak-sample
      test for `--reason`.
- [x] The code in `src/gates.rs`, `src/doctor.rs`, `src/stage.rs` and the flag in
      `src/main.rs`.
- [x] `docs/cli.md`, the case definitions and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
