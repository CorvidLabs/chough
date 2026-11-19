---
change: fix-283
artifact: testing
---

# Testing

`tests/cases.rs::a_retired_case_keeps_a_binding_to_a_removed_fledge_task`, red on main:
- A retired case bound to a task missing from `fledge.toml` passes `cases check`, with its file unchanged.
- The same binding on an active case is refused, naming the task.
- `cases run` on the retired case still refuses it.

Case `TC-cli-a-retired-case-keeps-its-fledge-task-binding` binds it.
