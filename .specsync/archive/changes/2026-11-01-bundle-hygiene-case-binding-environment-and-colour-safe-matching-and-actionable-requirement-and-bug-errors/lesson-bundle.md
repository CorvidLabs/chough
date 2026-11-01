# Lesson bundle — bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Bundle hygiene, case binding environment and colour-safe matching, and actionable requirement and bug errors
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/cases.rs, src/evidence.rs, src/bugs.rs, src/records.rs, src/skill.rs, tests/, docs/, skills/chough-workflow/, .agents/skills/chough-workflow/, .claude/skills/chough-workflow/
- **Acceptance**: Bundle hygiene: no file in the shipped skill bundle (skills/chough-workflow) contains U+2014 or U+2013; templates/frd.md and templates/cases.toml use colons in their place, and a test in tests/package.rs walks every bundle file and fails on either character. KNOWN_PREVIOUS lists the dashed digests (templates/frd.md 5647b56e..., templates/cases.toml 6c7ebaa1...) and the 2756dc2 digest of references/testing.md, the only other bundle file this change edits, so `chough skill status` reports unedited copies from earlier builds as known-outdated, `chough skill update` replaces them, and edited copies stay edited. Chough's own .agents/ and .claude/ copies are refreshed and read current (BUG-0018, LIFE-4, LIFE-9, REQ-cli-016, REQ-cli-019).
- **Acceptance**: Actionable requirement refusals: a requirement link that cannot be resolved is still refused with the same exit code, and the message names the companion files searched. When the ID has the form REQ-<module>-NNN (module found by removing `REQ-` and the last `-NNN`, so hyphenated modules such as checkout-ui work) and that module's bounded requirements.md contains it under the same token match the refusal uses, the message names that module and says what to do: a cases file binds one module's contract, so move the case to .chough/cases/<module>.cases.toml (cases check, run and record), or pass --contract specs/<module>/<module>.spec.md (evidence add). Otherwise it says the ID may be a typo or a requirement that an approved native change adds but `specsync change check` has not yet materialized, and to run that check before running or recording cases that link it. The hint reads neither the checkpoint nor any delta, and no unresolved link is accepted. This is the error half of BUG-0017; the documented order is the evidence-freshness change's (BUG-0017, BUG-0019, FLOW-7, LIFE-5, REQ-cli-005, REQ-cli-009).
- **Acceptance**: Actionable bug flag errors: `chough bug add` and `chough bug update` given a scope value in --classification say that the value is a scope and to use --scope, and given a classification value in --scope say the reverse; `--help` for both commands lists the allowed values of both flags. Validation stays in Chough rather than moving to clap, so exit codes and bug record shapes are unchanged (BUG-0019, LIFE-5, REQ-cli-012).
- **Acceptance**: Declared binding environment: a case binding for any adapter may declare `env`, a table of variable names to string values. Names match [A-Za-z_][A-Za-z0-9_]* and are at most 128 bytes, two names equal ignoring ASCII case are refused (Windows treats them as one), values contain no NUL, and a binding holds at most 32 entries of at most 4 KiB each. For the cargo-test adapter, names that choose what cargo builds or runs are refused, compared ignoring ASCII case: PATH, CARGO, CARGO_HOME, RUSTC, RUSTDOC, RUSTC_WRAPPER, RUSTC_WORKSPACE_WRAPPER, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, RUSTDOCFLAGS, CARGO_ENCODED_RUSTDOCFLAGS, RUSTC_BOOTSTRAP, and any name starting with RUSTUP_, CARGO_BUILD_, CARGO_TARGET_ (except CARGO_TARGET_DIR, which stays allowed), CARGO_HOST_, CARGO_PROFILE_ or CARGO_UNSTABLE_. Violations are refused by `cases check` with the case location. The declared variables are set for the executed command on top of the inherited environment; native reads are unchanged. An empty `env` is not serialized, so the digests of existing cases are unchanged and their observations keep their freshness, while adding or editing `env` changes the digest. Every observation of an executed case, with or without `env`, carries the limitation `inherited environment variables were not recorded`; when the binding declares `env`, the observation also names the declared variable names, not their values. Docs say `env` lives in committed case files and is not for secrets, and that for the command and fledge-task adapters a declared PATH can change which program the argv resolves to (FLOW-7, REQ-cli-009, REQ-cli-011).
- **Acceptance**: Colour-safe output matching: expect_output fragments and the cargo-test count are matched against the captured text with complete terminal escape sequences removed, while the stored artifact stays the raw captured bytes. Removed are CSI sequences (ESC [, parameter bytes 0x30 to 0x3F, intermediate bytes 0x20 to 0x2F, one final byte 0x40 to 0x7E), OSC sequences (ESC ] ended by BEL or ESC backslash before any line break) and two-byte sequences of ESC and one byte from 0x40 to 0x5F. A sequence that does not complete under these rules, including one interrupted by a line break or the end of the output, is left unchanged. Cleaning never removes a line break, so it never lowers the number of lines that start with `test result:`, and a test shows this. When any sequence was removed the observation carries the limitation `terminal escape sequences were removed before matching; the artifact keeps the raw output`. An expect_output fragment that contains ESC (U+001B) is refused by `cases check`, because it could never match the cleaned text. Vitest-style coloured output whose visible text holds `Test Files  1 passed (1)` passes that fragment, and a fragment absent from the visible text still fails. Chough sets no NO_COLOR or other variable by default (FLOW-7, REQ-cli-009, REQ-cli-011).
- **Acceptance**: Guidance: docs/cli.md documents `env` (not for secrets, the refused cargo-test names, the PATH caution), the standing inherited-environment limitation, escape removal, the new error hints, and, as a change from earlier releases, that an expect_output fragment containing ESC is now refused; the pull request description calls out the same refusal as the one change that can break an existing case file. docs/architecture.md and the skill's references/testing.md say artifacts stay inside the project (no external artifact paths; REQ-cli-006 is unchanged) and that large files such as screenshots should be kept small and may be ignored by Git: the observation keeps the fingerprint, and a clone without the file reads stale-artifact, never passed. They do not recommend Git LFS, because SpecSync `change check` currently refuses tracked LFS files (CorvidLabs/spec-sync#789). The order of cases run, commit, change check, review and finalize is documented by the evidence-freshness change, not this one, so this change does not edit references/build.md, references/plan.md, references/accept.md or references/lifecycle.md (LIFE-4, FLOW-7).
- **Acceptance**: Bugs are verified in order: before each fix, a new case for each of BUG-0017 (error half), BUG-0018 and BUG-0019 runs at the unfixed tip and records a failed observation, which is linked to its bug first; after the fix the same unchanged case runs again, and that later passed observation is linked. BUG-0018 and BUG-0019 are then set verified. BUG-0017 stays open with a note that its error half is fixed here and its documented-order half belongs to the evidence-freshness change. Behavior these rules do not name is unchanged. The full suite, the feature-and-bug exercise and native structural checks pass on Ubuntu, macOS and Windows CI, and regression tests cover each rule above (LIFE-5, FLOW-7).

## Evidence

- Verification commit: `bf8562043a8bb8a78da85c7aaa0a9aa28a3dea0b`
- Base commit: `85650d93ad9a811c2c54e08256620d351754da8d`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

A Raven run with Chough reported five usability problems (CorvidLabs/chough #10 to
#14). Triage reproduced them on main 2756dc2 and the human (0xLeif) agreed to split
them into two packages. This change is package 2: bundle hygiene and clearer case and
bug errors. It covers:

- #13 (BUG-0018): the shipped bundle has two em dashes (templates/frd.md line 23,
  templates/cases.toml line 1). `chough init --host` copies them into every host
  directory, and Raven's docs lint rejects the `.agents/` copy of frd.md.
- #14 items 4 and 5 (BUG-0019): `bug add --classification blocking` and a case
  linking another module's requirement fail with errors that do not say how to fix
  them.
- #11, error half (BUG-0017): a case linking a requirement that an approved delta
  adds fails with the same bare `requirement not found in contract companions` as a
  typo. Its other half, a documented order without double runs, is the
  evidence-freshness change's: it documents the accept order, and the repeat runs left
  after that depend on #10 and #12. BUG-0017 therefore stays open after this change.
- #14 item 1: a `command` binding has no environment; NO_COLOR or a port had to go
  through `/usr/bin/env` in argv.
- #14 item 2: `expect_output` matches raw output, so colour codes split fragments
  such as `Test Files  1 passed (1)`.
- #14 item 3: documentation only. External artifact paths stay refused (REQ-cli-006).

Ruled out during triage:

- Resolving requirement links against unmaterialized deltas: Chough would parse and
  own SpecSync's delta format, cannot confirm the delta is still the approved one
  (the approval digest is not a plain file hash, and BUG-0002 shows status keeps
  approval_valid true after an edit), and the runs would go stale at `change check
  --commit` anyway. Deferred until SpecSync publishes approved delta requirement IDs.
- Setting NO_COLOR by default: it silently changes the environment under test.
- External artifact paths: they break REQ-cli-006 and are neither reviewable nor
  portable.
- Rewriting em dashes in SpecSync-generated archives: native records stay untouched;
  the lesson-bundle heading is an upstream spec-sync report.

The staleness issues #10 and #12 (BUG-0015, BUG-0016), BUG-0014 and the documented
accept order are package 1, the evidence-freshness change. In the grouping the human
agreed, this package owns only #11's error hint, so it does not edit references/
build.md, plan.md, accept.md or lifecycle.md. Both packages edit `src/evidence.rs`,
`src/bugs.rs`, `src/skill.rs` (each adds `KNOWN_PREVIOUS` digests for different
files), `tests/bugs.rs`, `tests/cli.rs`, docs/cli.md and docs/architecture.md, so the
second to merge rebases; no bundle file is edited by both.

`.chough/cases/cli.cases.toml` gains the case definitions for BUG-0017 to BUG-0019,
but `.chough/` is not listed in the affected paths: it is Chough's own record
directory, it is not one of the SDD meaningful paths in .specsync/sdd.json, so listing
it adds no coverage, and this workflow's lesson is that listing it stales verification.
The cases are declared in tasks.md and testing.md instead.

The native Intent repeats the title. SpecSync 6.1.1 derives the title, the Intent and
the change ID from the one `change new` description, and no `change answer` question
sets the Intent alone (an `intent` answer is stored as an unrelated key); every
archived change in this repository has the same shape. The intent is this context:
fix the reported usability problems in #11 (error half), #13 and #14 without weakening
any fail-closed rule.

## From the change's design.md

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

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-019 | `tests/package.rs`: no U+2013 or U+2014 in any bundle file; dashed 2756dc2 copies of frd.md and cases.toml, and the 2756dc2 copy of references/testing.md, read known-outdated and update to current; an edited copy stays edited |
| REQ-cli-005 | `tests/cli.rs`: `evidence add` with a cross-module ID names the module and the `--contract` fix; a typo gets the neutral hint; both refuse with the old exit code |
| REQ-cli-009 | `tests/cases.rs`: cross-module case names `.chough/cases/<module>.cases.toml`; unmaterialized delta ID gets the `specsync change check` hint; hyphenated module names derive correctly; `env` bounds refused with location (name pattern and 128-byte length, case-insensitive duplicate, NUL, 33 entries, 4 KiB value, each refused cargo-test name in mixed case); ESC fragment refused; a case without `env` keeps its 2756dc2 digest; editing `env` changes the digest |
| REQ-cli-011 | `tests/cases.rs`: a declared variable reaches the command (portable `sh` or `cmd` printer per platform) and the limitation names it without its value; a case without `env` carries the inherited-environment limitation; vitest-style coloured output passes its fragment with the escape limitation and a raw byte-exact artifact; an absent fragment still fails; unit tests of the cleaner for CSI, OSC (BEL and ESC backslash), two-byte sequences limited to 0x40 to 0x5F, ESC before a line break, an OSC or CSI cut by a line break or the end of input (left unchanged); a property test that cleaning keeps every line break and never lowers the count of `test result:` lines; a cargo-test case whose filter matches 2 tests while a test prints an unterminated OSC still does not pass |
| REQ-cli-012 | `tests/bugs.rs`: `--classification blocking` and `--scope product-defect` name the other flag; `--help` lists both value sets; exit code unchanged |

Regression: full `cargo test`, `fledge lanes run verify` (including the
feature-and-bug exercise) and native structural checks on all three CI platforms.
## Bug verification order

references/bugs.md treats a bug without a linked failed observation as unverified,
and BUG-0019, BUG-0018 and BUG-0017 have none yet. Open BUG-0014 means Chough itself
does not yet check that a pass came after the failure, so the order is kept here:

1. Red, before any fix: each new case in `.chough/cases/cli.cases.toml` (BUG-0017
   error half, BUG-0018, BUG-0019) runs at the unfixed tip and records failed; that
   observation is linked to its bug first. These cases link only existing requirement
   IDs, so they validate before `change check`.
2. Green, after verification: the same unchanged case runs again, in a later second,
   and records passed; the pass is linked. BUG-0018 and BUG-0019 are set verified.
   BUG-0017 gets the pass and a note, and stays open for its documented-order half,
   which the evidence-freshness change owns.

Real behavior: rerun the triage reproductions of #13 (`chough init --host claude
--host openai` then a dash search), #14 items 1, 2, 4 and 5 and #11 on the built
binary.

## Where these lessons go

- `specs/cli/context.md`
