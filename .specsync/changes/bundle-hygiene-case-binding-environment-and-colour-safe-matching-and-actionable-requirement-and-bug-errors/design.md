---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: design
---

# Design

## Bundle hygiene (REQ-cli-019, REQ-cli-016)

Replace the two dashes with colons. Add to `KNOWN_PREVIOUS` in `src/skill.rs`, under
a comment naming #13, the 2756dc2 digests of templates/frd.md and templates/cases.toml
and of references/testing.md, the only other bundle file this change edits.
references/build.md, plan.md, accept.md and lifecycle.md are not edited: the accept
order is the evidence-freshness change's. Existing state logic is unchanged: only files byte-identical to a listed
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
bytes per value, at most 128 bytes per name, and no two names equal ignoring ASCII
case (Windows would merge them). For the `cargo-test` adapter it also refuses, ignoring
ASCII case, the names that choose what cargo builds or runs: PATH, CARGO, CARGO_HOME
(its config.toml can set a runner), RUSTC, RUSTDOC, RUSTC_WRAPPER,
RUSTC_WORKSPACE_WRAPPER, RUSTFLAGS and CARGO_ENCODED_RUSTFLAGS (a linker flag),
RUSTDOCFLAGS, CARGO_ENCODED_RUSTDOCFLAGS, RUSTC_BOOTSTRAP, and any name starting
with `RUSTUP_` (the toolchain), `CARGO_BUILD_`, `CARGO_TARGET_` (a per-target
runner, linker or flags), `CARGO_HOST_`, `CARGO_PROFILE_` or `CARGO_UNSTABLE_`;
`CARGO_TARGET_DIR` only chooses where cargo writes and stays allowed. This keeps the adapter's claim that
`cargo test` ran the named test from being redirected by the case file. The command
and fledge-task adapters name their program in argv; docs/cli.md says a declared PATH
can change what that name resolves to. `records::run_bounded` takes an extra `env:
&[(String, String)]` applied with `Command::envs`; native reads pass an empty slice.
No `env_clear`: the command still inherits the environment. Every executed case's
observation, with or without `env`, adds the standing limitation `inherited
environment variables were not recorded`, because the triage showed `FOO=bar` flipping
a case without `env` from failed to passed with nothing in either record. When `env`
is non-empty it also adds `declared environment set: NAME, ...`. Values are not copied
into the observation; they are bound through the case digest and the committed case
file. Limitations are not part of the case digest or the fingerprint, so existing
observations keep their freshness. `Evidence` gains no field, so older binaries still
read new observations. The inherited environment can still redirect any adapter;
the standing limitation is what keeps that visible (Invariant 4).

## Colour-safe matching (REQ-cli-009, REQ-cli-011)

A linear helper removes, from the assertion text only, complete sequences: CSI
(`ESC [`, parameter bytes 0x30 to 0x3F, intermediate bytes 0x20 to 0x2F, one final
byte 0x40 to 0x7E), OSC (`ESC ]` up to BEL or `ESC \`, found before any line break)
and two-byte sequences of ESC and one byte from 0x40 to 0x5F other than `[` and `]`.
Anything else is copied unchanged: an ESC followed by another byte, a CSI or OSC that
reaches a line break or the end of input first, and a lone trailing ESC. None of the
accepted bytes is a line break, so cleaning never joins two lines or drops the rest of
the output, and the prefix of a line before its first ESC is unchanged. That is why it
can never lower the number of lines starting with `test result:` that `cargo_counts`
(src/cases.rs) sums: a cleaner that joined lines or ran an unterminated OSC to the end
could hide one and turn a filter that matched 2 tests into a pass counted as 1. A
property-style unit test over generated inputs (random bytes, ESC followed by every
byte, unterminated OSC and CSI before `\ntest result:` lines) checks that the count of
line breaks is unchanged and the `test result:` line count never drops. `artifact()` keeps the raw
bytes it writes and builds `text` from the cleaned bytes, which also feed
`cargo_counts`. If anything was removed, the limitation `terminal escape sequences
were removed before matching; the artifact keeps the raw output` is added.
`validate_binding` refuses any `expect_output` fragment containing U+001B: after
cleaning, such a fragment could never match, so refusing at `cases check` makes the
problem visible before a run rather than as a confusing failure. This can break an
existing case file that relies on raw escapes; none exists in this repository or
examples/. The repository has no change log, so docs/cli.md states the refusal as a
change from earlier releases and the pull request description calls it out.

## Out of the design

No external artifact paths, no NO_COLOR default, no delta parsing, no repository-wide
dash lint, no edit to native SpecSync records, no Git LFS guidance (SpecSync `change
check` refuses tracked LFS files, CorvidLabs/spec-sync#789), and no accept-order text
(the evidence-freshness change owns it).
