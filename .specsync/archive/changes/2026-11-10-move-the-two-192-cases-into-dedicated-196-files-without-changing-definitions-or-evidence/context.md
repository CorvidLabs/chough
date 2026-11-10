---
change: move-the-two-192-cases-into-dedicated-196-files-without-changing-definitions-or-evidence
artifact: context
---

# Context

## Concrete problem and source

Live Chough issue196, filed by0xLeif and explicitly assigned by coordinator t19f, asks to move two cases appended by192 into files of their own under existing main AGENTS.md/DRIVE-12. The exact live base is4bbd5a6373d26c0ad069878f8f543842a4564a6f; GitHub has14 open issues and0 open PRs at preparation. PR198 is merged and is not recreated.

TC-cli-change-new-no-spec-change currently lives in cli.change-steps.cases.toml. TC-history-timing-skips-direct-writes currently lives in history.direct-writes.cases.toml. This is file ownership hygiene, not a test-semantic or runtime repair.

## Authority and ownership

Actor E/t221 prepares and later implements only196 in the isolated ch-queue-196-case-files worktree. Authority source is t19f NEW AUTHORIZED ASSIGNMENT relaying Leif's request to work through all Chough PRs/issues. This relay is recorded as its source, not invented verbatim human acceptance. Root retains native approval/review/finalize/merge; author waits for the exact scope receipt before any case edit. Scope authorization does not record genuine human implementation acceptance/review or delivery. No additional permission is requested from Leif for already-authorized bounded work.

One writer per change; lifecycle ownership transfers explicitly for root approval. Existing de31/153/147d, archives, inherited sessions/evidence/native changes and all other workers are preserved. No compile/full/site without a separate root lease; no GitHub mutation/push/merge/install.

## Current evidence and uncertainty

The unchanged actual pinned Chough cases list and cases check pass before implementation. The full catalog/digests and source snapshots are external under program/evidence/chough-queue-work. The native forwarding route is an exact shell exec wrapper to immutable SpecSync e971, not a symlink; initial pre-write guard failure is retained. Own session creation moved no inherited sessions; preservation guard passed. No implementation pass, Rust binding execution or human gate is inferred from this preparation.
