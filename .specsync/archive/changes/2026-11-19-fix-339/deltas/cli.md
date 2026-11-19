# cli delta

## ADDED

(none)

## MODIFIED

### REQUIREMENT REQ-cli-009

Validated test definitions. The helper SHALL validate test definitions in `.chough/cases/<module>.cases.toml` against profile `chough-testing/1`: unique stable case IDs, active HI criteria, native requirement IDs, track, method, environments, artifact types, explicit expected outcomes, setup and steps for manual or agent-assisted cases, and a binding for automated cases. A binding MAY declare `env`, a bounded table of environment variable names and values; for the `cargo-test` adapter it SHALL NOT declare a variable that chooses what cargo builds or runs, and no binding SHALL declare an expected-output fragment containing the escape character. Unknown fields and unknown profiles SHALL be rejected. A definition file that does not parse as TOML and holds a Git conflict marker (REQ-records-001) SHALL be refused naming the file, the line and the marker; a definition that parses may quote a marker inside a string (#339). Each case SHALL have a definition digest that is independent of its position in the file, and an omitted or empty `env` SHALL leave that digest unchanged. New case definitions SHALL use curated IDs in the form `TC-<module>-<title-slug>`, where the title slug consists of nonempty lowercase ASCII alphanumeric runs separated by single hyphens. The complete ID SHALL remain within the existing 100-byte record ID bound using a module-aware slug limit. Case IDs SHALL be globally unique across all modules, including legacy and new forms. Exact legacy `TC-<module>-NNNN` definitions SHALL remain valid with unchanged definition digests and without renumbering or migration. New regression case definitions introduced for this change SHALL use slug IDs. Documentation SHALL state that older Chough versions require an upgrade to use slug case IDs. A requirement ID the companion lacks SHALL be accepted only when the checkpoint's native change, on one native read per command, passes the build gate checks of REQ-cli-022 and its delta for the case module adds or modifies that ID; otherwise it SHALL be refused, naming why a consulted delta did not resolve it. The shipped skill SHALL say so, SHALL show the test stage linking failing-first evidence, and SHALL tell agents not to list running or recording cases as a `tasks.md` checkbox. Trace: FLOW-7, LIFE-2, LIFE-6, DRIVE-6.

Acceptance Criteria
- Reordering cases leaves their IDs and digests unchanged; editing expected outcomes, steps or a binding, including its `env`, changes the digest.
- Duplicate IDs, unknown criteria or requirements, unknown fields, missing expected outcomes and automated cases without a binding are refused with the offending location.
- An invalid `env` name, a name over 128 bytes, two names equal ignoring ASCII case, a value with NUL, more than 32 entries or a value over 4 KiB, and an expected-output fragment containing ESC are refused with the offending location.
- A `cargo-test` binding that declares PATH, CARGO, CARGO_HOME, RUSTC, RUSTDOC, RUSTC_WRAPPER, RUSTC_WORKSPACE_WRAPPER, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, RUSTDOCFLAGS, CARGO_ENCODED_RUSTDOCFLAGS, RUSTC_BOOTSTRAP, or a name starting with RUSTUP_, CARGO_BUILD_, CARGO_TARGET_ (other than CARGO_TARGET_DIR), CARGO_HOST_, CARGO_PROFILE_ or CARGO_UNSTABLE_, in any ASCII case, is refused with the offending location.
- Curated slug IDs accept lowercase ASCII normalization and enforce the module-aware 100-byte bound; empty, uppercase, repeated-separator and leading or trailing separator forms are refused. A duplicate ID in another module is refused globally.
- Exact legacy numeric cases remain readable and executable with unchanged definition digests and no renumbering or migration. The new regression case IDs for this change use slugs rather than allocating legacy numbers.
- A case linking a requirement only the approved, unchanged delta adds passes `cases check`, `run` and `record` on one native read and is listed under `approved_delta_requirements`; an edited delta, invalid approval, wrong state or module, no change or a failed read refuses with the reason.
- The plan, testing and accept references describe this and failing-first evidence in the test stage.
- A case definition file half-merged by Git is refused by `cases check` naming the file, the line and the marker, not only as an invalid definition; a valid definition quoting a marker line in a string is accepted.

### REQUIREMENT REQ-cli-055

Chough doctor. `chough doctor` SHALL report the Chough debt of REQ-history-012 and each committed record, or case definition that does not parse as TOML, that holds a Git conflict marker (REQ-records-001, #339) as JSON with `schema_version` 1, `mode` (`report`, `dry-run` or `fix`) and `findings`, each with `kind`, `target`, `fix` (the Chough command that fixes it), `decision` (whether the fix is a decision) and `status` (`found`, `fixed` or `skipped`, with a `reason` when skipped). With no flag, or with `--dry-run`, it SHALL change nothing and report each finding as `found`. With `--fix` it SHALL apply each fix that is not a decision. A lone `=======` outside a conflict in a committed record, or in a case definition that does not parse as TOML, SHALL be a `possible_conflict_separator` finding naming its line, "possible leftover conflict separator at line N", marked `advisory`: a warning only, which SHALL never make `chough doctor` exit 1 (#339). A Git conflict marker line inside a closed code fence of a committed record SHALL be a `possible_fenced_conflict` finding naming its line and marker, "possible conflict markers inside a code fence at line N", also marked `advisory`, since a real conflict there cannot be told from a quoted one (#339). A fix that would leave a record unreadable, such as redacting an ID (REQ-history-012), SHALL NOT be applied: its finding SHALL be `skipped`, with the reason and the recovery, and the record SHALL stay byte for byte as it was on that line. It SHALL apply a decision fix only when `--reason <text>` is given, and SHALL otherwise report it as `skipped` with a fix that names `--reason`. It SHALL exit 0 when no finding is left `found` or `skipped`, and 1 otherwise. It SHALL make no native read, and SHALL never record a human decision, approval, review or consent. Its per-session findings (`finished_session`, `stale_gates`) SHALL come from one session check that `chough migrate`'s plan also runs on the planned records (REQ-records-009), so a migration's follow-ups name what the doctor will report. Trace: DRIVE-17.

Acceptance Criteria
- With no flag and with `--dry-run`, the doctor reports the same findings, each with a `chough` fix, exits 1, and leaves every file under `.chough/` unchanged.
- `--fix` without `--reason` applies the fixes that are not decisions and reports each decision fix as `skipped`, naming `--reason`; a project with no debt reports no findings and exits 0.
- No doctor run calls SpecSync.
- On a tree just migrated from 0.5.x, `chough doctor` reports exactly the `finished_session` and `stale_gates` sessions the migration's follow-ups named.
- `--fix --redact` on a committed session record whose ID reference holds a token leaves that line unchanged and the session readable, and reports the finding `skipped`, naming `chough session shelve <id>`.
- A committed record whose body holds a conflict gives a `conflict_markers` finding with no fix, and `--fix` reports it `skipped`.
- A committed case definition holding a conflict gives a `conflict_markers` finding for each marker line, naming the file, the line and the marker; a valid definition quoting a marker in a string gives none.
- A lone `=======` in a committed record gives one advisory `possible_conflict_separator` finding naming its line, and doctor's exit status is what it is without it.
- A Git conflict left inside a code fence of a committed session gives one advisory `possible_fenced_conflict` finding per marker line, naming it, and doctor's exit status is what it is without it.

## REMOVED

(none)
