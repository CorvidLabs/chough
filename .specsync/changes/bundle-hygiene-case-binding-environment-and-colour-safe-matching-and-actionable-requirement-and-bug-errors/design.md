---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: design
---

# Design

## Bundle hygiene (REQ-cli-019, REQ-cli-016)

Replace the two dashes with colons. Add to `KNOWN_PREVIOUS` in `src/skill.rs`, under
a comment naming #13, the 2756dc2 digests of templates/frd.md and templates/cases.toml
and of each other bundle file this change edits (references/build.md, plan.md,
testing.md). Existing state logic is unchanged: only files byte-identical to a listed
digest are replaced. A new test in `tests/package.rs` walks `bundle_files()` and fails
with `<file>:<line>: em or en dash` on U+2013 or U+2014. `chough skill update`
refreshes `.agents/` and `.claude/` copies in this repository.

## Requirement refusals (REQ-cli-005, REQ-cli-009)

`validate_requirements` keeps its signature's inputs and stays fail-closed. On a miss
it returns a message built from:

1. `requirement <ID> not found in <companion paths searched>`.
2. A hint. Derive a module from the ID by removing `REQ-` and the last `-<part>` with
   `rsplit_once('-')`; accept it only if it is a valid module name and differs from
   the searched modules. If `local_path(root, specs/<module>/requirements.md)` exists
   and its bounded `read_record` text contains the ID under the same token split, the
   hint is cross-module. Otherwise it is the neutral hint: `it may be a typo, or a
   requirement an approved native change adds that is not yet materialized: run
   specsync change check <change> before running or recording cases that link it`.

The cross-module wording differs by caller, so the function returns a small typed
miss (ID, searched paths, owning module if found) that `cases::check_links` and
`evidence::add` render: the former says to move the case to
`.chough/cases/<module>.cases.toml` because a cases file binds one module's contract;
the latter says to add `--contract specs/<module>/<module>.spec.md`. Neither reads the
checkpoint or a delta, because `validate_requirements` has no session access and
`evidence add` does not go through cases; the neutral hint is true whether or not a
native change is active. Exit codes are unchanged.

## Bug flags (REQ-cli-012)

In `bugs::validate`, a classification found in SCOPES appends ``; `<value>` is a
scope: use --scope <value>``, and a scope found in CLASSES appends the reverse. Doc
comments on the `classification` and `scope` clap fields of add and update list the
allowed values for `--help`. No clap `value_parser`: it would move the error to
clap's exit code 2.

## Binding environment (REQ-cli-009, REQ-cli-011)

`Binding` gains `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")] env:
BTreeMap<String, String>`, allowed for every adapter. `validate_binding` bounds it:
names `[A-Za-z_][A-Za-z0-9_]*`, no NUL in values, at most 32 entries, at most 4096
bytes per value. `records::run_bounded` takes an extra `env: &[(String, String)]`
applied with `Command::envs`; native reads pass an empty slice. No `env_clear`: the
command still inherits the environment, and the observation says so. When `env` is
non-empty the observation adds the limitation `declared environment set: NAME, ...;
other inherited variables were not recorded`. Values are not copied into the
observation; they are bound through the case digest and the committed case file.
`Evidence` gains no field, so older binaries still read new observations.

## Colour-safe matching (REQ-cli-009, REQ-cli-011)

A linear helper removes, from the assertion text only: CSI (`ESC [`, parameter and
intermediate bytes, one final byte 0x40 to 0x7E), OSC (`ESC ]` up to BEL or `ESC \`,
or the end of input) and two-byte `ESC <byte>` sequences. `artifact()` keeps the raw
bytes it writes and builds `text` from the cleaned bytes, which also feed
`cargo_counts`. If anything was removed, the limitation `terminal escape sequences
were removed before matching; the artifact keeps the raw output` is added.
`validate_binding` refuses any `expect_output` fragment containing U+001B: after
cleaning, such a fragment could never match, so refusing at `cases check` makes the
problem visible before a run rather than as a confusing failure. This can break an
existing case file that relies on raw escapes; none exists here, and the change log
says so.

## Out of the design

No external artifact paths, no NO_COLOR default, no delta parsing, no repository-wide
dash lint, no edit to native SpecSync records.
