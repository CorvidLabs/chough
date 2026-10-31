---
id: prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package
state: implementing
type: feature
base_commit: f19823d4287debe65cf8c5a78fa4a2f72d83b9b7
---

# Prepare a first release candidate with reference-aware evidence retention, current guidance and an installable package

## Intent

Prepare a first release candidate with reference-aware evidence retention, current guidance and an installable package

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Retention: `chough evidence prune` previews by default and changes nothing: it lists each eligible observation set (record plus artifacts only it references) with case, environment, age, outcome, bytes and reason, every protected record with its protecting reference, and the bytes reclaimed. Protected: references from the checkpoint, any bug (observations or regression baseline, whatever its status), an active native change acceptance manifest, the most recent archived native change manifest, non-routine records (manual, agent-assisted, human, delivery, publication, installation, deployment or live-health), the newest 3 automated runs per case and environment, the newest failure per case and environment, and the failures and first and last passes of each validation run. Defaults: eligible only when older than 14 days and outside those sets; a warning above 50 MiB of evidence and artifacts (LIFE-8, REQ-cli-018).
- Pruning applies only with --apply under a run consent or the standing preference retention auto that a human set; with auto, cases run and validate run prune eligible sets after recording without asking again. Each pruned set appends one line to .chough/pruned.jsonl (ID, case, environment, outcome, revision, time, artifact digests, reason, policy, HEAD at pruning); no frozen native archive, ignore rule or Git history is changed, and pruned bytes stay recoverable from Git history (LIFE-8, REQ-cli-018).
- Status, next, bug show and validation status label each referenced observation current, historical (stale but present), pruned (listed in the ledger) or unavailable (missing and not listed); pruned and unavailable observations are never current, passed or verifying evidence, and a validation run keeps its recorded counts (LIFE-7, FLOW-13, REQ-cli-018).
- The crate package contains only the helper source, the canonical skill bundle, Cargo files, README and LICENSE; cargo install from it and chough init in a new project give the helper and skills without Chough development records, and chough skill status reports copies installed from earlier main builds as known-outdated rather than edited (LIFE-9, LIFE-4, REQ-cli-019).
- README, ROADMAP and docs/architecture.md describe the public repository, completed work and current next steps accurately while keeping dated decisions, and one concise records table explains current working records versus historical evidence, examples versus Chough self-test records, frozen native archives, canonical skills versus installed copies, ignored build output, and what is normally committed, kept local or kept as a CI artifact.
- A demonstrated release candidate: one real lifecycle of this change with a failing test, local bug, fix and retest, genuine human implementation review, native archive and Chough reaching done; a fresh-context resumption from files only; retention verified in a disposable copy without losing protected evidence or misleading the resumed agent; package contents, install instructions, tested platforms and known limitations presented. Nothing is published or released.

## No-spec Rationale

Not applicable
