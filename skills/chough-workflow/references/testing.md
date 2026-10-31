# Focused testing

Test what the change affects, not every criterion the product has.

## Choose the checks

Run `chough cases select --requirement <REQ> --criterion <ID> --path <changed path>`
for the affected scope. Add `--track copy-localization` when visible text,
terminology, comprehension or locale behavior changed, and `--track design-qa` when
visuals or interactions changed. The output lists selected cases, omitted cases with
reasons, and the project-required lane if one is configured
(`chough pref set required-lane <lane> --source …`). Also run proportionate
regressions for nearby behavior and the project's mandatory gates. Explain any
meaningful omission and record test debt in the native `testing.md`.

| Track | When | Typical method |
|---|---|---|
| functional | Always | Automated unit, integration or UI tests; manual checks where needed |
| copy-localization | Visible text or locale behavior changed | Deterministic string assertions plus reviewer notes |
| design-qa | Visuals or interactions changed | Rendered comparison with the canonical prototype at the target viewports and states |

## Definitions are not observations

Definitions live in `.chough/cases/<module>.cases.toml` (profile `chough-testing/1`,
see [the template](../templates/cases.toml)). Each case has a stable ID
`TC-<module>-NNNN`, links to HI criteria and requirements, setup, steps, explicit
expected outcomes, method, environments and expected artifact types. Several cases
can cover one criterion. Never edit an expected outcome to match a failure; retire a
case (`status = "retired"`) instead of deleting or reusing its ID.

`chough cases check` validates every definition, including its HI and requirement
links. The digest of a case ignores its position and title, so reordering never
changes identity, while editing what it asserts makes earlier observations
`stale-case-definition`.

## Execute honestly

- Automated: `chough cases run <ID> --environment <env>` runs the binding without a
  shell and records the observation with the captured log. `cargo-test` passes only
  when exactly one test ran and passed; a filter matching nothing is `not-run`.
  `fledge-task` and `command` pass only with exit 0 and every `expect_output`
  fragment present. The command exits 3 when the outcome is not passed.
- Manual or agent-assisted: `chough cases record <ID> --method manual --verdict 1=pass
  --verdict 2=fail … --summary "…" --artifact <screenshot>` gives a verdict for every
  expected outcome; any fail makes the observation failed.

A failing case stays failed even when every other case passes. Do not rerun until
it passes and report only the pass. Capture the failure with
[the bug procedure](bugs.md).

AI judgment is never the only oracle when an assertion or a reproducible procedure
can decide. Keep rendered, copy and human judgments as observations with method and
limits. Magpie can run real commands, but its generic mode judges exit codes and
records skipped steps as passes; bind a Magpie run only through `command` with an
`expect_output` assertion, and only where it is installed.
