# CLI reference

Run `chough --help` and each subcommand's `--help` for the installed executable. All
commands accept global `--root PATH` (default: the current directory). Recording
evidence or running cases requires the Git repository root and an existing commit.
Output is JSON unless stated otherwise.

## Commands

| Command | Behavior |
|---|---|
| `init` | `--objective`, `--intent-scope`, repeated `--module`, repeated `--host openai\|claude\|codex\|cursor\|gemini\|grok` (default openai). Creates a schema 2 checkpoint and the skill bundle; preserves existing files. |
| `context [--json]` | Focused packet: instructions, intent, scoped HI export, module sources, recent transitions, valid transitions, preferences, open bugs. Markdown by default. |
| `status`, `next` `[--environment NAME]` | Checkpoint, gate for the current stage, valid transitions, `next_valid_action`, native status, snapshot, observations with freshness, open and blocking bugs. |
| `migrate [--stage S]` | Converts a schema 1 session after saving its bytes to `.chough/session.v1.json`; refuses if that backup exists. |
| `checkpoint` | Show or update: `--objective` (clears run consents), `--next-action`, `--native-change`, `--decision`, `--question`, `--resolve-question`, `--completed`, `--blocker`, `--clear-blockers`, `--awaiting-human`, `--clear-awaiting-human`, `--criterion`, `--requirement`, `--case`, `--design`, `--file`, `--observation`, `--bug`. |
| `stage show\|advance\|skip\|iterate\|back\|reset\|pause\|resume\|enter\|return` | Transitions with reasons and an append-only log. Every move meets its destination's native prerequisite, whichever command makes it (advance, skip, back, reset, return, release → done): build needs state approved, implementing or verifying with `approval_valid` true (refused as `refused:native-approval-invalid` otherwise, or `refused:native-correction-invalid`); test and accept need that or an archived change; done needs archived with authenticated `terminal_evidence` (`refused:native-archive-<validity>` otherwise). Intent, design and plan stay reachable. A malformed or ill-typed `.specsync/sdd.json` refuses gated moves as `refused:sdd-policy-invalid`; a valid `"enabled": false` or no policy file leaves them ungated, and a missing `enabled` means enabled, as in SpecSync. Leaving test needs no open blocking bug; entering validate or release needs consent or a standing preference. |
| `cases check\|list\|select\|run\|record` | Validate definitions; list digests; select focused cases (`--criterion`, `--requirement`, `--path`, `--track copy-localization\|design-qa`); run an automated binding; record a manual or agent-assisted run with `--verdict N=pass\|fail` for every expected outcome. `run` and `record` exit 3 when the recorded outcome is not passed. |
| `evidence add ID`, `evidence prune [--apply]` | `add`: criterion, requirement, contract, kind, outcome, environment, summary; optional artifacts (spelled canonically: relative, `/`-separated), `--case`, `--method`, `--track`, `--limitation`. Never replaces an ID. `prune`: preview, or with `--apply` remove, eligible observation sets; see [Retention](#retention). |
| `bug add\|update\|list\|search\|show\|report` | Local backlog. `add` refuses likely duplicates unless `--distinct`; regression needs `--regression-baseline`; `verified` needs a passed observation; `report` prints Markdown and posts nothing. |
| `pref show\|set\|unset` | Standing preferences with `--source`: repeat-validation, monitoring, release, monitor-initiation, librarian, library-dir, required-lane, retention. |
| `consent show\|grant\|revoke` | Run consent for the current objective: repeat-validation, monitoring, release, retention. |
| `validate run\|stop\|resume\|status` | Optional repeated runs of one case with stored limits and local capture of failures. A stop request (state `stopped`) or a paused checkpoint stage (state `paused`) is checked immediately before every attempt, including after the interval, which is waited in slices of at most 200 ms; both states resume with `validate resume`. |
| `library init\|snapshot\|index` | Optional librarian; requires `pref set librarian on`. |
| `skill status\|update [--host H]` | Compare installed copies with the bundle; update only missing and known-outdated files. |

## Records and limits

Managed JSON is bounded to 64 KiB, case files to 256 KiB, captured output to 1 MiB,
context to 16 KiB per section and 128 KiB in total, and sessions to 16 modules.
Native SpecSync status is parsed from its complete output up to 4 MiB and displayed
up to 16 KiB; `status` and `next` carry the parsed state and next action in
`native_summary`. Larger output is reported as `oversized` and refuses gated
transitions.
Unknown fields and unsupported schema versions are rejected. Paths are
project-relative; traversal and symlinks are rejected. Native tools and bound
commands receive direct arguments, never shell-evaluated text.

| Record | Schema |
|---|---|
| Checkpoint | 2 (schema 1 read-only until migrated) |
| Observation | 2 (schema 1 still read) |
| Case file | `profile = "chough-testing/1"` |
| Bug, preferences, transition, validation run | 1 |

## Execution bounds

Case bindings and native reads (`specsync change status`, `hi export`) run through
one bounded runner. Both streams are read concurrently; each keeps at most its cap
while reading and keeps draining beyond it, counting every byte.

| Run | Stdout kept | Stderr kept | Deadline |
|---|---|---|---|
| Case binding | 1 MiB | 1 MiB | `timeout_seconds` (default 600 s) |
| Native read | 4 MiB + 1 byte (the parse bound) | 16 KiB | 120 s; `CHOUGH_NATIVE_TIMEOUT_SECONDS` sets 1 to 3600 |

At the deadline, or when output stays open more than 2 s after the command exits
(cut short at the deadline, but at least 200 ms after the exit), Chough stops the
command and its descendants: on Unix the process group the command leads, on Windows
its process tree with `taskkill /T /F`. It then waits at most 1 s for the output to
close and abandons any reader still blocked, so the run returns on time and its
capture is reported incomplete. On Windows, `taskkill` cannot find descendants whose
parent has already exited; such a run returns on time with an incomplete capture,
and those descendants may keep running.

A case artifact keeps both streams within 1 MiB in total and ends with a note for
each stream that did not fit: `[truncated: stdout wrote N bytes; M discarded]`. Case
limitations name a timeout (`timed out after N s; the command and its descendants
were stopped`), descendants that held output after exit (`... and were stopped` or
`... and could not be stopped`) and `output capture incomplete`. A run whose capture
is incomplete is never `passed`.

Native section states are `observed`; `failed` for a non-zero exit or output that
stayed open and could not be read completely; `unavailable` when the command cannot
start or `CHOUGH_NATIVE_TIMEOUT_SECONDS` is not a whole number from 1 to 3600 (the
command is then not run); `timed-out`; and, for JSON status, `oversized` and
`invalid-output`. Each state other than `observed` leaves lifecycle state unknown,
and gated transitions refuse it as `refused:native-unknown`. Without a SpecSync
configuration, status reports `not-configured` and transitions are not natively gated.

## Observation outcomes and freshness

| Outcome | Meaning |
|---|---|
| `passed` | The process exited 0, its output was captured completely and the binding's assertion held, or every verdict passed |
| `failed` | A reported test or declared assertion failed, a verdict failed, the run timed out, or its output capture was incomplete |
| `not-run` | Nothing was asserted: no test matched, the binding was ambiguous, or cargo failed before reporting results |

| Freshness | Meaning |
|---|---|
| `current-observation` | Inputs, artifacts, case definition and environment match now |
| `stale-revision-or-inputs` | A fingerprinted file changed; for a record without a content identity, also any new revision |
| `stale-artifact` | A bound artifact changed or disappeared |
| `stale-case-definition` | What the case asserts changed since the run |
| `stale-environment` | A different environment was requested |
| `environment-unchecked` | No environment was supplied |
| `unknown-revision` | No Git snapshot could be established |

The label keeps its name for JSON compatibility, but the revision alone no longer
stales an observation. Each record still stores the HEAD revision, shown in status,
next and the library index and used for regression-baseline ancestry; freshness
compares a working content identity (`snapshot.content`, a SHA-256 digest). It covers
every tracked or untracked non-ignored file except `.chough/`, `target/` and SpecSync
lifecycle bookkeeping: the top-level `state.json`, `approvals.json`,
`verification.json`, `verification-attempts.json`, `review.json`,
`review-attempts.json`, `finalization.json`, `accepted-state.json` and
`lesson-bundle.md` of each `.specsync/changes/<id>/` and
`.specsync/archive/changes/<date>-<id>/`, and the value of the front-matter `state:`
line of its `change.md`. Change artifacts are keyed by change ID and their path in the
change, so finalize moving a change into the archive keeps observations current. A
deleted tracked file counts as absent. So committing only Chough records, an empty
commit, committing edits that were already present, `specsync change check --commit`
and finalize keep observations current; a change to source, tests, docs, `specs/`,
`.specsync/` configuration or a change's deltas and other artifacts makes them stale.
A record written without a content identity, by an earlier Chough, keeps the
revision-bound comparison and stays stale after any new commit, and so does a schema 1
record, whose schema has no content identity, even if one was added to it by hand; a
malformed one is refused. Older Chough binaries refuse records with a content identity as invalid.

A bug reads `reproduced-and-later-passed`, and accepts `bug update --status verified`,
only when a present linked passed observation was observed strictly after its most
recent present linked failed observation (by `observed_at_unix_seconds`). An older
pass, or one in the same second, leaves it `reproduced`; with no linked failure it
reads `unverified` and `verified` is refused.

Environment comparison uses the label you supply; it does not detect changes in the
real environment. A native status response that decodes as JSON is usable output, not
evidence that a lifecycle gate passed.

Kinds: `product-test`, `structural`, `human-observation`, `delivery-observation`,
`publication-observation`, `installation-observation`, `deployment-observation`,
`live-health-observation`, `dry-run`. Human approval and delivery stay unknown in
helper output; an observation is an unauthenticated claim by whoever recorded it.

## Retention

`chough evidence prune` previews by default and changes no file. An observation set is
one evidence record plus the artifacts under `.chough/artifacts/` that no other record
references; artifacts elsewhere in the project are never removed. Each set is eligible
or protected, and the preview lists eligible sets (cases, environment, age, outcome,
bytes, files), protected sets with every reason that applies, totals, the bytes that
would be reclaimed and a warning when evidence and artifacts exceed 50 MiB.

| Protected because | Reason shown |
|---|---|
| Listed in the checkpoint, as an observation or as a `--design` or `--file` path naming the record or one of its artifacts | `checkpoint` |
| Linked by a bug of any status (observation, note or regression baseline), or named by its fixture path | `BUG-NNNN <status>` |
| Listed by an active native change's acceptance manifest | `native active <id>` |
| Listed by the most recent archived change's manifest (highest finalization timestamp) | `native archive <id>` |
| The native read failed, or its output had no acceptance entries | `native unavailable` |
| Not a routine automated run: a non-test kind, or a method other than `automated` | the kind, the method, or `unspecified-method` |
| Among the newest 3 routine runs for any case it lists, in its environment | `newest` |
| The newest routine failure for any case it lists, in its environment | `last failure` |
| A failure, or the first or last pass, of a stored validation run | `validation <run>` |
| Younger than 14 days | `recent` |

A record bound to several cases counts in the group of each; one with no case forms its
own group per environment. Paths are compared by one key per file: the spelling rebuilt
from its components joined with `/`, with case folded, so `a//b`, `a/b/` and differently
cased spellings of one file count as one artifact. An artifact is removed only when one
record uses it and no bug fixture or checkpoint path names it.

Manifests are read with `specsync change status <id> --json` for each directory in
`.specsync/changes/` and for the latest archive. Without a SpecSync configuration no
manifest protects a set; when any read is unavailable, fails or lacks
`acceptance_entries`, every set stays protected.

`--apply` removes eligible sets only under a run consent (`chough consent grant
retention --source ...`) or the standing preference `retention` (`off`, the default, or
`auto`, set with `chough pref set retention auto --source ...`). With `auto`, `cases
run`, `validate run` and `validate resume` apply the same rules after recording and
report the result in a `retention` field; a failure there is reported, and the recorded
outcome stands. Before each set is removed, one JSON line is appended to
`.chough/pruned.jsonl`: ID, cases, environment, outcome, kind, revision, observed time,
artifact digests, record digest, removed files, reason, policy and authority, HEAD at
pruning and the pruning time. Files leave the working tree only; committed bytes stay
in Git history, and native archives and ignore rules are untouched. The ledger is read
up to 16 MiB; an unreadable ledger leaves absent observations `unavailable`.

Referenced observations carry a label in `status`, `next` (`observation_references`
and each entry in `observations`), `bug show` (`observation_labels`) and `validate
status` (each run's `observation_labels`):

| Label | Meaning |
|---|---|
| `current` | Present and its inputs, artifacts and case definition match now (and the requested environment, when one is given) |
| `historical` | Present but stale |
| `pruned` | Absent and listed in `.chough/pruned.jsonl` |
| `unavailable` | Absent and not in the ledger |

Pruned and unavailable observations never count as current, passed or verifying
evidence: bug verification ignores them, `bug update --status verified` needs a present
passed observation, and validation runs keep their recorded counts.

## Test-case adapters

| Adapter | Runs | Passes only when |
|---|---|---|
| `cargo-test` | `cargo test --locked --test <target> -- --exact <test>` | Exit 0 and exactly one test reported and passed |
| `fledge-task` | `fledge run <task>` | Exit 0 and every `expect_output` fragment appears |
| `command` | The argv as written | Exit 0 and every `expect_output` fragment appears |

### Binding environment

Any binding may declare `env`, a table of variable names and string values that are
set for the executed command on top of the inherited environment. Chough never clears
the inherited environment and sets no variable of its own, not even `NO_COLOR`; add
`NO_COLOR = "1"` yourself when a tool should print plain text. Native reads are
unaffected.

```toml
[case.bind]
adapter = "command"
argv = ["npx", "vitest", "run"]
expect_output = ["Test Files  1 passed (1)"]
env = { NO_COLOR = "1", PORT = "4173" }
```

`cases check` refuses, with the case location: a name that does not match
`[A-Za-z_][A-Za-z0-9_]*` or is longer than 128 bytes; two names equal ignoring ASCII
case (Windows treats them as one variable); a value containing NUL or longer than
4 KiB; and more than 32 entries. A `cargo-test` binding also cannot declare, in any
ASCII case, a name that chooses what cargo builds or runs: `PATH`, `CARGO`,
`CARGO_HOME`, `RUSTC`, `RUSTDOC`, `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`,
`RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, `RUSTDOCFLAGS`,
`CARGO_ENCODED_RUSTDOCFLAGS`, `RUSTC_BOOTSTRAP`, and any name starting with
`RUSTUP_` (the toolchain), `CARGO_BUILD_`, `CARGO_TARGET_` (a per-target runner,
linker or flags, which can carry `-C linker=...`), `CARGO_HOST_`, `CARGO_PROFILE_`
or `CARGO_UNSTABLE_`. `CARGO_TARGET_DIR`, which only chooses where cargo writes, is
allowed. For `command` and `fledge-task`, which name their
program in argv, a declared `PATH` can change which program that name resolves to.

`env` lives in the committed case file, so it is not a place for secrets. Its names
and values are part of the case digest: adding or editing `env` makes earlier
observations `stale-case-definition`, while an omitted or empty `env` leaves the
digest of an existing case unchanged. An observation names the declared variables,
`declared environment set: NAME, ...`, never their values. Every observation of an
executed case, with or without `env`, carries the limitation `inherited environment
variables were not recorded`: the inherited environment can still change what a
binding does, and the record cannot show it.

### Output matching

`expect_output` fragments and the `cargo-test` count of `test result:` lines are
matched against the captured text with complete terminal escape sequences removed;
the artifact keeps the raw captured bytes. Removed are CSI sequences (`ESC [`,
parameter bytes 0x30 to 0x3F, intermediate bytes 0x20 to 0x2F, one final byte 0x40 to
0x7E), OSC sequences (`ESC ]` ended by BEL or `ESC \` before any line break) and
two-byte sequences of `ESC` and one byte from 0x40 to 0x5F. A sequence that does not
complete under these rules, including one cut by a line break or the end of the
output, is left unchanged. Removal never removes a line break, so it can never lower
the number of `test result:` lines. When anything was removed, the observation carries
the limitation `terminal escape sequences were removed before matching; the artifact
keeps the raw output`.

Changed from earlier releases: an `expect_output` fragment that contains ESC
(U+001B) is now refused by `cases check`, because it could never match the cleaned
text. A case file that matched raw escape codes must match the visible text instead.

### Error hints

A requirement link that no searched companion holds is refused as before, with the
same exit code, and the message names the companions searched,
`requirement <ID> not found in specs/<module>/requirements.md`, then a likely cause:

- When the ID has the form `REQ-<module>-NNN` (the module is what remains after
  removing `REQ-` and the last `-NNN`, so `REQ-checkout-ui-004` names `checkout-ui`)
  and that module's `requirements.md` holds it, the message names the module and the
  fix. A cases file binds one module's contract, so `cases check`, `run` and `record`
  say to move the case to `.chough/cases/<module>.cases.toml`; `evidence add` says to
  add `--contract specs/<module>/<module>.spec.md`.
- Otherwise the ID may be a typo, or a requirement an approved native change adds
  that `specsync change check` has not yet materialized: run that check before running
  or recording cases that link it, as the [accept order](../skills/chough-workflow/references/accept.md)
  does first when the delta adds or modifies requirements.

The hint reads neither the checkpoint nor any native delta, and no unresolved link is
accepted.

`bug add` and `bug update` refuse a scope value given as `--classification` with
``; `<value>` is a scope: use --scope <value>``, and a classification given as
`--scope` with the reverse hint; the exit code and the bug record are unchanged.
`--help` for both commands lists the values: scope `blocking`, `in-scope` or
`out-of-scope`; classification `unclassified`, `product-defect`, `regression`,
`environment`, `requirement-gap` or `evidence-gap`.

See the [walkthrough](walkthrough.md) and [exercises](exercises/feature-bug-loop.md).
