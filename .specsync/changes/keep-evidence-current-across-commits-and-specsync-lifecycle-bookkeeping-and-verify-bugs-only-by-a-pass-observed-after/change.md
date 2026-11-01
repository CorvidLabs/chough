---
id: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
state: draft
type: bug_fix
base_commit: 85650d93ad9a811c2c54e08256620d351754da8d
---

# Keep evidence current across commits and SpecSync lifecycle bookkeeping, and verify bugs only by a pass observed after the failure

## Intent

Keep evidence current across commits and SpecSync lifecycle bookkeeping, and verify bugs only by a pass observed after the failure

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Freshness compares content, not the commit: a new observation stays `current-observation` after committing only `.chough/` records, after an empty commit, and after committing working-tree edits that were already present when it ran; it becomes `stale-revision-or-inputs` when any fingerprinted file changes, whether tracked or untracked (source, tests, docs, `specs/`, `.specsync/` configuration, or a SpecSync change's deltas, requirements or other artifacts, before or after materialization) (BUG-0015, LIFE-2, LIFE-7, REQ-cli-004).
- Only lifecycle bookkeeping is excluded: the content digest leaves out `.chough/`, `target/`, and SpecSync's bookkeeping in each `.specsync/changes/<id>/` and `.specsync/archive/changes/<dated-id>/`: `state.json`, `approvals.json`, `verification.json`, `verification-attempts.json`, `review.json`, `review-attempts.json`, `finalization.json`, `accepted-state.json`, and the `state:` front-matter value of `change.md`. A `specsync change check --commit` that writes only these keeps observations current. Editing a delta, `requirements.md`, `testing.md`, `tasks.md` or any other artifact of an active change makes them stale before any `change check` (BUG-0016, LIFE-6, LIFE-7, REQ-cli-004).
- Finalizing moves the change directory into the archive, and that move alone does not make observations stale: change artifacts are hashed under a key built from the change ID and their path inside the change, not from the directory location. A real artifact content change in the archive, or any new non-bookkeeping file, still makes them stale (BUG-0016, LIFE-6, REQ-cli-004).
- A tracked file deleted in the working tree is hashed as absent, so committing a deletion that was already present when the case ran keeps the observation current, and restoring the file makes it stale (BUG-0015, REQ-cli-004).
- Records keep their revision: each new observation still stores the HEAD revision, shown in status, next and the library index and used by `bug` regression-baseline ancestry, and it adds an optional content digest that is validated as a SHA-256 digest when present. A record without the digest keeps the revision-bound comparison, so it stays stale after any new commit and never silently becomes current; a malformed digest is refused (LIFE-2, LIFE-7, REQ-cli-004, REQ-cli-017).
- Bug verification follows order: every bug verification label (`bug show`, `bug list`, `bug report` and the open backlog in `status` and `next`) reports `reproduced-and-later-passed`, and `bug update --status verified` is accepted, only when a present linked passed observation was observed strictly after the most recent present linked failed observation, compared by `observed_at_unix_seconds`. An older pass, or a pass in the same second as the failure, leaves the bug `reproduced` and refuses `verified` with a message naming the failing observation (BUG-0014, LIFE-5, REQ-cli-012).
- The accept order is documented in `references/accept.md` and `references/lifecycle.md` (canonical bundle, `.claude/skills` and `.agents/skills` copies, byte-identical), and in docs/lifecycle.md. The order is: when the delta adds or modifies requirements, run `change check` first to materialize them; `chough cases run`; record the gate with `chough checkpoint --awaiting-human`; commit the source together with `.chough/`; `specsync change check <id> --commit`; push and hosted CI; present the work; the human decision; `specsync change review`; `specsync change ship` or `finalize`; only then clear the gate and run `chough stage advance`. Between the check and finalize, nothing writes to `.chough/` and nothing is committed. The references say that until SpecSync offers an opt-in verification exclusion (the proposed upstream `verification_ignored_paths`), any Chough write in that window forces another `change check`. `chough skill status` reports the main 2756dc2 copies of both files as known-outdated, not edited (FLOW-13, LIFE-6, REQ-cli-016).
- Contract and documentation agree: REQ-cli-004 is reworded by a MODIFIED delta item `### REQUIREMENT REQ-cli-004`. SpecSync applies that item only to a canonical heading that is exactly `### REQ-cli-004`, so before `change check` the canonical heading `### REQ-cli-004 — Bind observations to content and environment` is normalized to it. Invariant 4, the docs/cli.md freshness table, docs/architecture.md, docs/walkthrough.md, docs/FRD.md, specs/cli/testing.md (the "stale ... by design" note) and the docs/NEXT_CHANGE.md status say that the revision is recorded but not compared. The `stale-revision-or-inputs` label keeps its name for JSON compatibility, and specs/cli/context.md gains the lesson from CorvidLabs/raven#644 (LIFE-7, REQ-cli-004).
- No regression: behavior these rules do not name is unchanged. Before the fix, new cases for BUG-0014, BUG-0015 and BUG-0016 each record a failing reproduction; after it they pass, and those bugs are verified by later passes. The full suite and `fledge lanes run verify` pass, and hosted CI passes on Ubuntu, macOS and Windows (LIFE-5, LIFE-7).

## No-spec Rationale

Not applicable
