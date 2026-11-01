# Lesson bundle — keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep evidence current across commits and SpecSync lifecycle bookkeeping, and verify bugs only by a pass observed after the failure
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, src/bugs.rs, src/skill.rs, tests/cli.rs, tests/bugs.rs, tests/common/mod.rs, specs/cli/requirements.md, specs/cli/cli.spec.md, specs/cli/testing.md, specs/cli/context.md, skills/chough-workflow/references/accept.md, skills/chough-workflow/references/lifecycle.md, skills/chough-workflow/references/bugs.md, .claude/skills/chough-workflow/references/accept.md, .claude/skills/chough-workflow/references/lifecycle.md, .claude/skills/chough-workflow/references/bugs.md, .agents/skills/chough-workflow/references/accept.md, .agents/skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/bugs.md, docs/cli.md, docs/architecture.md, docs/walkthrough.md, docs/FRD.md, docs/lifecycle.md, docs/NEXT_CHANGE.md, docs/DECISIONS.md, docs/exercises/cold-start.md, README.md
- **Acceptance**: Freshness compares content, not the commit: a new observation stays `current-observation` after committing only `.chough/` records, after an empty commit, and after committing working-tree edits that were already present when it ran; it becomes `stale-revision-or-inputs` when any fingerprinted file changes, whether tracked or untracked (source, tests, docs, `specs/`, `.specsync/` configuration, or a SpecSync change's deltas, requirements or other artifacts, before or after materialization) (BUG-0015, LIFE-2, LIFE-7, REQ-cli-004).
- **Acceptance**: Only lifecycle bookkeeping is excluded: the content digest leaves out `.chough/`, `target/`, and exactly these top-level files in each `.specsync/changes/<id>/` and `.specsync/archive/changes/<dated-id>/`: `state.json`, `approvals.json`, `verification.json`, `verification-attempts.json`, `review.json`, `review-attempts.json`, `finalization.json`, `accepted-state.json` and `lesson-bundle.md`, plus the value of the `state:` line in the front matter of `change.md` (a `state:` line in its body is still hashed). A `specsync change check --commit` that writes only these keeps observations current. Editing a delta, `requirements.md`, `testing.md`, `tasks.md` or any other artifact of an active change makes them stale before any `change check` (BUG-0016, LIFE-6, LIFE-7, REQ-cli-004).
- **Acceptance**: Finalizing moves the change directory into the archive and writes only files from that list, so it does not make observations stale: change artifacts are hashed under a key built from the change ID and their path inside the change, not from the directory location. A real artifact content change in the archive, or a file in the change or archive directory that is not on that list, makes them stale (BUG-0016, LIFE-6, REQ-cli-004).
- **Acceptance**: A tracked file deleted in the working tree is hashed as absent, so committing a deletion that was already present when the case ran keeps the observation current, and restoring the file makes it stale (BUG-0015, REQ-cli-004).
- **Acceptance**: Records keep their revision: each new observation still stores the HEAD revision, shown in status, next and the library index and used by `bug` regression-baseline ancestry, and evidence schema 2 gains an optional content digest that is validated as a SHA-256 digest when present. A record without the digest keeps the revision-bound comparison, so it stays stale after any new commit and never silently becomes current; a malformed digest is refused (LIFE-2, LIFE-7, REQ-cli-004, REQ-cli-017).
- **Acceptance**: Bug verification follows order: every bug verification label (`bug show`, `bug list`, `bug report` and the open backlog in `status` and `next`) reports `reproduced-and-later-passed`, and `bug update --status verified` is accepted, only when a present linked passed observation was observed strictly after the most recent present linked failed observation, compared by `observed_at_unix_seconds`. An older pass, or a pass in the same second as the failure, leaves the bug `reproduced` and refuses `verified` with a message naming the failing observation. A bug with no present linked failure reads `unverified` and refuses `verified` (BUG-0014, LIFE-5, REQ-cli-012).
- **Acceptance**: The accept order is documented in `references/accept.md` and `references/lifecycle.md` (canonical bundle, `.claude/skills` and `.agents/skills` copies, byte-identical), and in docs/lifecycle.md. The order is: when the delta adds or modifies requirements, run `change check` first to materialize them; `chough cases run`; record the gate with `chough checkpoint --awaiting-human`; commit the source together with `.chough/`; `specsync change check <id> --commit`; push and hosted CI; present the work; the human decision; `specsync change review`; `specsync change ship` or `finalize`; only then clear the gate and run `chough stage advance`. Between the check and finalize, nothing writes to `.chough/` and nothing is committed. The references say that until SpecSync offers an opt-in verification exclusion (the proposed upstream `verification_ignored_paths`), any Chough write in that window forces another `change check`. `references/bugs.md` says that `verified` needs a pass observed after the latest linked failure. `chough skill status` reports the main 2756dc2 copies of accept.md, lifecycle.md and bugs.md as known-outdated, not edited (FLOW-13, LIFE-6, REQ-cli-016).
- **Acceptance**: Contract and documentation agree: REQ-cli-004, REQ-cli-012 and REQ-cli-017 are reworded by MODIFIED delta items in the `### REQUIREMENT REQ-cli-NNN` form, applied to canonical headings that are exactly `### REQ-cli-NNN`; after approval and before `change check`, the REQ-cli-004 heading is normalized to `### REQ-cli-004` with its title moved into the body, exactly as requirements.md shows. Invariant 4, the docs/cli.md freshness table, docs/architecture.md, docs/walkthrough.md, docs/FRD.md (its REQ-cli-004 section and its excluded list), README.md, specs/cli/testing.md (the "stale ... by design" note), docs/lifecycle.md and the docs/NEXT_CHANGE.md status say that the revision is recorded but not compared; docs/exercises/cold-start.md gains a dated note that its stale reading came from the earlier rule. docs/DECISIONS.md records the human's decision on this scope as a new decision that supersedes D-016. The `stale-revision-or-inputs` label keeps its name for JSON compatibility, and specs/cli/context.md gains the lesson from CorvidLabs/raven#644 (LIFE-7, REQ-cli-004).
- **Acceptance**: No regression: before the fix, new cases for BUG-0014, BUG-0015 and BUG-0016 each record a failing reproduction linked to its bug; after it they pass, and those bugs are verified by later passes. Existing tests pass, and an existing expectation changes only where testing.md names it as following from these criteria. The full suite and `fledge lanes run verify` pass, and hosted CI passes on Ubuntu, macOS and Windows (LIFE-5, LIFE-7).

## Evidence

- Verification commit: `e844d6fed7d92035b4476bcf4ce322d7aaa80854`
- Base commit: `85650d93ad9a811c2c54e08256620d351754da8d`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The Raven run (CorvidLabs/raven#644) reported two problems, filed as CorvidLabs/chough#10 and #12. During triage both were reproduced on main 2756dc2 in scratch clones, and they were then recorded as bug records BUG-0015 and BUG-0016. Those records link no observations yet, so their verification label is `unverified`; the linked failing reproductions are plan step 1. BUG-0014 was already open from the release-candidate retention verifier and has not been reproduced in this workflow.

- BUG-0015 (#10): `snapshot()` in `src/evidence.rs` hashes `git rev-parse HEAD` into the fingerprint before the file list, and `freshness()` compares the whole fingerprint. Any commit stales every observation. That includes the commit that saves the observation, and even `git commit --allow-empty`. `git reset --soft HEAD~1` makes it current again. specs/cli/testing.md describes the resulting stale reading as "by design", docs/FRD.md says a new HEAD "remains stale under the current implementation", and docs/exercises/cold-start.md calls it the freshness rule "working as designed". docs/DECISIONS.md D-016 keeps reuse after metadata-only commits unapproved and deferred, so approving this change supersedes it.
- BUG-0016 (#12, Chough side): the fingerprint also covers SpecSync's per-change bookkeeping. So `specsync change check --commit` stales the observations it is meant to accompany, and so does `finalize`, which moves the change into the archive. On the SpecSync side, the verification project-input digest hashes every non-ignored file, including `.chough/`. SDD `ignored_paths` does not feed it, so any Chough write after a check forces another check. That part is upstream and is not changed here.
- BUG-0014: `bugs::verification` reports `reproduced-and-later-passed` when any linked observation passed, whatever its order. `bug update --status verified` only needs a pass. This fix makes durable evidence stay current for longer, so ordering matters more.

Verifier corrections adopted:

- Do not exclude all of `.specsync/changes/` and `.specsync/archive/`. Before materialization, a change's deltas and artifacts are the only record of its scope. A prototype that excluded the whole directory let an observation stay current after a delta rewrite.
- A committed deletion that was already in the working tree changed the digest. The prototype hashed a `missing` marker for the deleted path, and after the commit the path was no longer listed at all.
- Older binaries reject records with the new field, because `Snapshot` uses `deny_unknown_fields`. That rejection is fail-closed.

Draft review corrections adopted: D-016 is superseded explicitly; `lesson-bundle.md` is bookkeeping, so finalize does not stale observations; REQ-cli-012 and REQ-cli-017 are modified in the delta rather than narrowed silently; `verified` needs a linked failure; the exact REQ-cli-004 heading edit is shown in requirements.md (it cannot be made before approval, because the audit refuses canonical edits under a draft); overlap with the bundle-usability and timing-report drafts is recorded in design.md.

Ruled out:

- Recomputing H(record revision || current content) per distinct revision. Its cost is unbounded in the number of revisions.
- Reusing SDD `ignored_paths`. It has a different meaning.
- Git-ignoring `.chough/`. Chough's records are meant to be committed.
- Renaming the `stale-revision-or-inputs` label. It breaks JSON consumers.

Out of this change: the upstream SpecSync exclusion (an issue to file, proposing `verification_ignored_paths`), #11, #13 and #14.

## From the change's design.md

# Design

## Content identity (src/evidence.rs)

```text
Snapshot { revision, dirty, fingerprint, content: Option<String> }   // content: serde default, skipped when None
```

- `snapshot()` reads `git ls-files --cached --others --exclude-standard -z` once and computes two digests from it:
  - `fingerprint`: unchanged legacy meaning (revision + file stream with the current exclusions).
  - `content`: no revision, built from the entries described below.
- One helper, `fingerprint_key(name) -> Key`, classifies each listed path. The same helper filters the `dirty` porcelain lines.
  - `.chough/…` and `target/…`: skip.
  - `.specsync/changes/<id>/<rest>` and `.specsync/archive/changes/<YYYY-MM-DD>-<id>/<rest>`, where `<rest>` is a top-level bookkeeping name (`state.json`, `approvals.json`, `verification.json`, `verification-attempts.json`, `review.json`, `review-attempts.json`, `finalization.json`, `accepted-state.json`, `lesson-bundle.md`): skip. `lesson-bundle.md` is on the list because finalize generates it from records already hashed (archive commit 8b18498 adds it); leaving it hashed would make every finalize stale the observations. The same name in a subdirectory, such as `deltas/`, is hashed.
  - Other files in a change directory: key `specsync-change/<id>/<rest>`, which does not depend on location. `change.md` is hashed with the value of the `state:` line inside its leading `---` front matter blanked. A `state:` line in the body is hashed as written. Nothing else is normalized.
  - Everything else, including `.specsync/` configuration, the registry and `specs/`: key = path.
- `content` sorts entries by key, then hashes key length, key, kind (file, symlink), mode bits, length and bytes, as today. A listed tracked path that is absent from the working tree contributes nothing, so its digest is the same as after the deletion is committed. `fingerprint` keeps its `missing` marker for legacy parity.
- `validate_record` accepts `content` only as a SHA-256 hex digest.
- `freshness()` compares `content` when the record has it. Otherwise it compares the legacy `fingerprint`, so a legacy record still goes stale on any commit. The label stays `stale-revision-or-inputs`.
- The schema version stays 2. REQ-cli-017 lists the fields schema 2 may add and has no general allowance, so the delta modifies REQ-cli-017 to add the content identity. Older binaries fail closed on new records ("invalid evidence record"), because `Snapshot` uses `deny_unknown_fields`. See open question 4 in plan.md for the alternative of schema 3.

## Bug verification order (src/bugs.rs)

- `verification()` takes the present linked observations. `latest_failure` is the maximum `observed_at_unix_seconds` among failed ones. With no failure the result is `unverified`. With a passed observation whose time is greater than `latest_failure`, it is `reproduced-and-later-passed`. Otherwise it is `reproduced`.
- `bug update --status verified` requires a present linked failure and a present passed observation later than the latest one, the same rule as the label. With no linked failure it is refused, which follows REQ-cli-012 ("unverified until a failed observation is linked"); today any present pass is enough. Every bug recorded `verified` in this repository (BUG-0004 to BUG-0013) links a failure, so none of them depends on the old rule. Existing records are not rewritten; their label is computed when read. The refusal names the latest failing observation, or says that no failure is linked.
- Pruned and unavailable observations stay ignored, as today.

## Bundle (src/skill.rs and references)

- Rewrite the "Prepare the review" and "After the human decides" steps in accept.md, and the shape and verification sections in lifecycle.md, to follow the order in the acceptance criteria. lifecycle.md's "Evidence records" paragraph ("bind an observation to the Git revision, working-tree fingerprint ...") is reworded: the revision is recorded, freshness compares content. bugs.md gets one line saying that `verified` needs a pass observed after the latest linked failure.
- Copy the canonical files byte-for-byte to `.claude/skills/chough-workflow/` and `.agents/skills/chough-workflow/`.
- Add the 2756dc2 digests of accept.md and lifecycle.md (and bugs.md) to `KNOWN_PREVIOUS`, so that unedited installs report known-outdated.

## Coordination with other drafts

Two other drafts branch from the same base, and both touch files this change edits. None of them is approved.

- `bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors` (branch `0xleif/feat/bundle-usability`, #11, #13 and #14) lists src/evidence.rs, src/bugs.rs, src/skill.rs and all three skill directories. Its delta also MODIFIES REQ-cli-012, adding a sentence about classification and scope flags. A MODIFIED item replaces the whole requirement, so whichever change is approved second must carry the other's sentence in its REQ-cli-012 text, or its `change check` would drop it. Its `env` field is on the case `Binding`, not on `Evidence` ("`Evidence` gains no field"), so it raises no schema question for evidence records. Its references/testing.md line says to run cases for new requirement IDs only after `specsync change check <id> --commit`. This change's accept order runs a plain `change check` first and `check --commit` after committing, so the two sentences must be made the same before the second lands. Both add `KNOWN_PREVIOUS` digests in src/skill.rs.
- `report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records` (branch `0xleif/feat/timing-report`) adds `pub(super)` accessors in src/evidence.rs and src/bugs.rs and keeps record schemas. It reads `observed_at_unix_seconds`, which this change does not alter. The overlap is textual, so a rebase is expected but no contract conflict.

See open question 9 in plan.md for the order.

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-004 | `tests/cli.rs`, in a temporary Git repository with a local identity. A new observation stays current after committing only `.chough/`, after an empty commit, after committing an edit that was already present, and after committing a deletion that was already present. It goes stale after a tracked edit, a new untracked file, an exec-bit change (unix only, since mode bits are hashed only under `cfg(unix)` and CI also runs on Windows), and edits to `specs/`, `.specsync/config.toml`, `.specsync/changes/<id>/deltas/*.md`, `requirements.md` and `testing.md`. It stays current after writes to each bookkeeping file (including `lesson-bundle.md`) and after a `state:` change in the front matter of `change.md`, and goes stale after any other `change.md` edit, including a `state:` line in its body, and after a bookkeeping name written inside `deltas/`. Moving a stub change directory to `.specsync/archive/changes/<date>-<id>/` keeps it current, and an artifact edit after that move stales it. |
| REQ-cli-004, REQ-cli-017 | A legacy record without `content` stays stale after an empty commit. A record with a malformed `content` is refused. `revision` is still recorded and shown. |
| REQ-cli-012 | `tests/bugs.rs`: pass then fail gives `reproduced` and refuses `verified`. Fail then a later pass gives `reproduced-and-later-passed` and allows `verified`. A pass in the same second gives `reproduced`. A pass with no linked failure gives `unverified` and refuses `verified`; this changes the existing expectation that any pass allows `verified`. A regression-baseline ancestry check still uses the revision. `now()` is not injectable, so these tests write evidence records with chosen `observed_at_unix_seconds` values directly instead of relying on two runs landing in different seconds. |
| REQ-cli-016 | The existing skill status test is extended: an unedited 2756dc2 accept.md, lifecycle.md or bugs.md is known-outdated and is updated. |

## Cases and bugs

New `.chough/cases/cli.cases.toml` entries cover BUG-0014, BUG-0015 and BUG-0016, and each has a failing observation before the fix and a passing one after it. Their requirement links validate only after REQ-cli-004 is materialized, so the scoped check runs first.

## End to end

In a disposable copy of examples/shipping, built with examples/feature-bug-loop.sh, with a local Git identity and `GIT_CONFIG_GLOBAL=/dev/null`, follow the documented order: check, cases run, gate, commit with `.chough/`, check --commit, review (SIMULATED label, not a human decision), finalize. `chough next` reports the observations as `current-observation` after every step, including finalize (whose `lesson-bundle.md` is bookkeeping), and SpecSync needs no further check. This is recorded as an agent-assisted observation with its limits. It has not been run against the narrow bookkeeping list: the triage run used a broader, rejected exclusion.

## Regression

The full `cargo test`, `fledge lanes run verify` and `fledge lanes run pre-push`, and hosted CI on three platforms. Existing tests keep their expectations, with one known exception: the bug lifecycle test in `tests/bugs.rs` records `repro-2` (failed) and then `fixed-1` (passed) and expects `verified`. Both can land in the same second, which the strict-later rule refuses, so that test must give `fixed-1` a later `observed_at_unix_seconds` (written directly, as above) or it will flake. Its "passed observation" refusal text is kept in the new message. No existing test asserts staleness after a commit.

## Where these lessons go

- `specs/cli/context.md`
