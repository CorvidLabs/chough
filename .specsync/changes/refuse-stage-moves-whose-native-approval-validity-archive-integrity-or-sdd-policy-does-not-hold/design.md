---
change: refuse-stage-moves-whose-native-approval-validity-archive-integrity-or-sdd-policy-does-not-hold
artifact: design
---

# Design

One destination check replaces the two purpose-specific gates in `src/stage.rs`.

| Destination | Prerequisite (SDD enabled, change recorded) |
|---|---|
| intent, design, plan | none: backtracking and reset stay free |
| build | state approved, implementing or verifying; `approval_valid` true; `correction_valid` not false |
| test, accept | the build prerequisite, or the done prerequisite |
| done | state archived; `terminal_evidence.validity` authenticated-history, exact or successor-covered |

- The check runs for advance, skip, back, reset and release -> done, so every route to
  a stage meets the same prerequisite.
- `approval_valid` must be present and true; a missing field is unknown and refuses.
  `correction_valid` refuses only when present and false.
- Refusal labels: `refused:native-<state>` (unchanged), `refused:native-approval-invalid`,
  `refused:native-correction-invalid`, `refused:native-archive-<validity>`,
  `refused:sdd-policy-invalid`, `refused:no-native-change` and `refused:native-unknown`
  (unchanged). Each detail names the native next action and status command.
- SDD policy, mirroring SpecSync: no file is `NotAdopted` (label `ungated:sdd-disabled`,
  unchanged); a JSON object whose known fields have SpecSync's types is read with
  `enabled` defaulting to true; anything else is `Invalid(reason)` and refuses. Unknown
  fields are accepted, as SpecSync accepts them.
- No new dependency, no native write, no change to how native status is read.
