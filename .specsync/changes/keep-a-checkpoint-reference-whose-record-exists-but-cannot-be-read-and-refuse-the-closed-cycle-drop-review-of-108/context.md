---
change: keep-a-checkpoint-reference-whose-record-exists-but-cannot-be-read-and-refuse-the-closed-cycle-drop-review-of-108
artifact: context
---

# Context

Reviews of #108 (corvid-agent 5780146055, and Leif's review agent at src/stage.rs:946) found that `evidence::observed_at` turned a read or parse error into None. The closed-cycle drop treats None as "no record" and drops the reference, so a current-cycle record that exists but cannot be read lost its checkpoint reference. Trim and index both refuse that record correctly. REQ-cli-045 allows dropping only a missing record. `observed_at` now separates absence from a read error, and any unreadable referenced record makes the step keep every reference and report `refused` naming it. Done is still recorded.
