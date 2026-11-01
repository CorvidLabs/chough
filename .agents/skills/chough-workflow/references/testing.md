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
- A binding may declare `env`, variables set on top of the inherited environment,
  for example `env = { NO_COLOR = "1" }` when a tool should print plain text. Chough
  sets none by default. `env` is committed with the case, so never put secrets in
  it; a `cargo-test` binding cannot set names that choose what cargo builds or runs,
  such as `PATH` or `RUSTFLAGS`. Observations name the declared variables, not their
  values, and always state that inherited variables were not recorded.
- `expect_output` and the `cargo-test` count read the output with complete terminal
  escape sequences (colours, hyperlinks) removed, while the artifact keeps the raw
  bytes. Write fragments as the visible text; a fragment containing ESC is refused.
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

## Artifacts

Artifacts stay inside the project; there are no external artifact paths. Keep
screenshots and other large files small. One may be ignored by Git: the observation
keeps its fingerprint, and a clone without the file reads `stale-artifact`, never
passed. Do not use Git LFS for them while SpecSync `change check` refuses tracked LFS
files (CorvidLabs/spec-sync#789).

## Retention

Routine reruns are prunable; proof that something still references is not.
`chough evidence prune` previews eligible and protected observation sets with reasons
and bytes, and changes nothing. It protects what the checkpoint, a bug, an active
native change or the most recent archived one lists, non-routine records, the newest
3 automated runs and the newest failure per case and environment, validation failures
and boundary passes, and anything younger than 14 days. Apply only with `--apply` under
`chough consent grant retention --source "<their words>"`, or when a human set
`chough pref set retention auto --source …` (default `off`), which also prunes after
`cases run` and `validate run`. Removals are logged in `.chough/pruned.jsonl`. Never
delete evidence by hand, and never report a `pruned` or `unavailable` observation as
passed.
