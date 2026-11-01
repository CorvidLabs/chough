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
  - `.specsync/changes/<id>/<rest>` and `.specsync/archive/changes/<YYYY-MM-DD>-<id>/<rest>`, where `<rest>` is a top-level bookkeeping name (`state.json`, `approvals.json`, `verification.json`, `verification-attempts.json`, `review.json`, `review-attempts.json`, `finalization.json`, `accepted-state.json`): skip.
  - Other files in a change directory: key `specsync-change/<id>/<rest>`, which does not depend on location. `change.md` is hashed with the value of its `state:` front-matter line blanked. Nothing else is normalized.
  - Everything else, including `.specsync/` configuration, the registry and `specs/`: key = path.
- `content` sorts entries by key, then hashes key length, key, kind (file, symlink), mode bits, length and bytes, as today. A listed tracked path that is absent from the working tree contributes nothing, so its digest is the same as after the deletion is committed. `fingerprint` keeps its `missing` marker for legacy parity.
- `validate_record` accepts `content` only as a SHA-256 hex digest.
- `freshness()` compares `content` when the record has it. Otherwise it compares the legacy `fingerprint`, so a legacy record still goes stale on any commit. The label stays `stale-revision-or-inputs`.
- The schema version stays 2, and REQ-cli-017 already allows added fields. Older binaries fail closed on new records ("invalid evidence record"). See the open question in plan.md.

## Bug verification order (src/bugs.rs)

- `verification()` takes the present linked observations. `latest_failure` is the maximum `observed_at_unix_seconds` among failed ones. With no failure the result is `unverified`. With a passed observation whose time is greater than `latest_failure`, it is `reproduced-and-later-passed`. Otherwise it is `reproduced`.
- `bug update --status verified` requires a present passed observation later than every present linked failure. With no linked failure, today's rule is kept: any present pass. The refusal names the latest failing observation.
- Pruned and unavailable observations stay ignored, as today.

## Bundle (src/skill.rs and references)

- Rewrite the "Prepare the review" and "After the human decides" steps in accept.md, and the shape and verification sections in lifecycle.md, to follow the order in the acceptance criteria. bugs.md gets one line saying that `verified` needs a pass observed after the latest failure.
- Copy the canonical files byte-for-byte to `.claude/skills/chough-workflow/` and `.agents/skills/chough-workflow/`.
- Add the 2756dc2 digests of accept.md and lifecycle.md (and bugs.md) to `KNOWN_PREVIOUS`, so that unedited installs report known-outdated.
