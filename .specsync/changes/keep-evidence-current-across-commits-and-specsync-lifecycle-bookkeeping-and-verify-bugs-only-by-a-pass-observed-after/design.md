---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: design
---

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
