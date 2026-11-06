## ADDED

### REQUIREMENT REQ-cli-055

Chough doctor. `chough doctor` SHALL report the Chough debt of REQ-history-012 as JSON with `schema_version` 1, `mode` (`report`, `dry-run` or `fix`) and `findings`, each with `kind`, `target`, `fix` (the Chough command that fixes it), `decision` (whether the fix is a decision) and `status` (`found`, `fixed` or `skipped`, with a `reason` when skipped). With no flag, or with `--dry-run`, it SHALL change nothing and report each finding as `found`. With `--fix` it SHALL apply each fix that is not a decision. It SHALL apply a decision fix only when `--reason <text>` is given, and SHALL otherwise report it as `skipped` with a fix that names `--reason`. It SHALL exit 0 when no finding is left `found` or `skipped`, and 1 otherwise. It SHALL make no native read, and SHALL never record a human decision, approval, review or consent. Trace: DRIVE-17.

Acceptance Criteria
- With no flag and with `--dry-run`, the doctor reports the same findings, each with a `chough` fix, exits 1, and leaves every file under `.chough/` unchanged.
- `--fix` without `--reason` applies the fixes that are not decisions and reports each decision fix as `skipped`, naming `--reason`; a project with no debt reports no findings and exits 0.
- No doctor run calls SpecSync.
