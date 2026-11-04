---
change: say-when-native-verification-runs-no-project-commands-and-point-the-test-stage-to-recorded-test-evidence-before-accept
artifact: design
---

# Design

- New `src/policy.rs` owns reading `.specsync/sdd.json`: the `PolicyFile` and `Policy`
  types and `sdd_policy` move there unchanged from src/stage.rs, which keeps using them.
  It adds `verification_warning(root)`: when a native SpecSync configuration exists and
  the policy is enabled with an empty `verification_commands` list, it returns one plain
  sentence saying native verification runs none of the project's commands, so a passing
  `specsync change check` does not show the project's tests ran, and naming
  `verification_commands` in `.specsync/sdd.json` as where to list them. It reads only
  that local file: no native read and no subprocess.
- `chough next` adds `native_verification_warning` to its JSON view and prints it as a
  `Warning:` line under the native change line; `chough status` and the `chough stage`
  report add the same key (null when there is nothing to say).
- In `stage::assess`, the plain "Continue" action at the test stage keeps its text and
  gets a reason saying recorded test evidence is needed before accept, and the command
  `chough cases run <case-id> --environment <env>`. Every earlier branch (paused,
  awaiting human, blocking bugs, unlinked failure, gate refusals) is unchanged.
- Gates are untouched: the same moves are allowed and refused with and without the
  warning.
