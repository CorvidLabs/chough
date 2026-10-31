# Lesson bundle — prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Prepare a first release candidate with reference-aware evidence retention, current guidance and an installable package
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/evidence.rs, src/bugs.rs, src/prefs.rs, src/main.rs, src/cases.rs, src/validate.rs, src/skill.rs, Cargo.toml, tests/, docs/, README.md, skills/chough-workflow/, .agents/skills/chough-workflow/, .claude/skills/chough-workflow/, src/checkpoint.rs, src/records.rs
- **Acceptance**: Retention: `chough evidence prune` previews by default and changes nothing: it lists each eligible observation set (record plus artifacts only it references) with case, environment, age, outcome, bytes and reason, every protected record with its protecting reference, and the bytes reclaimed. Protected: references from the checkpoint, any bug (observations or regression baseline, whatever its status), an active native change acceptance manifest, the most recent archived native change manifest, non-routine records (manual, agent-assisted, human, delivery, publication, installation, deployment or live-health), the newest 3 automated runs per case and environment, the newest failure per case and environment, and the failures and first and last passes of each validation run. Defaults: eligible only when older than 14 days and outside those sets; a warning above 50 MiB of evidence and artifacts (LIFE-8, REQ-cli-018).
- **Acceptance**: Pruning applies only with --apply under a run consent or the standing preference retention auto that a human set; with auto, cases run and validate run prune eligible sets after recording without asking again. Each pruned set appends one line to .chough/pruned.jsonl (ID, case, environment, outcome, revision, time, artifact digests, reason, policy, HEAD at pruning); no frozen native archive, ignore rule or Git history is changed, and pruned bytes stay recoverable from Git history (LIFE-8, REQ-cli-018).
- **Acceptance**: Status, next, bug show and validation status label each referenced observation current, historical (stale but present), pruned (listed in the ledger) or unavailable (missing and not listed); pruned and unavailable observations are never current, passed or verifying evidence, and a validation run keeps its recorded counts (LIFE-7, FLOW-13, REQ-cli-018).
- **Acceptance**: The crate package contains only the helper source, the canonical skill bundle, Cargo files, README and LICENSE; cargo install from it and chough init in a new project give the helper and skills without Chough development records, and chough skill status reports copies installed from earlier main builds as known-outdated rather than edited (LIFE-9, LIFE-4, REQ-cli-019).
- **Acceptance**: README, ROADMAP and docs/architecture.md describe the public repository, completed work and current next steps accurately while keeping dated decisions, and one concise records table explains current working records versus historical evidence, examples versus Chough self-test records, frozen native archives, canonical skills versus installed copies, ignored build output, and what is normally committed, kept local or kept as a CI artifact.
- **Acceptance**: A demonstrated release candidate: one real lifecycle of this change with a failing test, local bug, fix and retest, genuine human implementation review, native archive and Chough reaching done; a fresh-context resumption from files only; retention verified in a disposable copy without losing protected evidence or misleading the resumed agent; package contents, install instructions, tested platforms and known limitations presented. Nothing is published or released.
- **Acceptance**: chough checkpoint accepts a native change ID up to 255 bytes made of the characters SpecSync uses, as SpecSync mints IDs up to 120 bytes and reads up to 255, and still refuses longer or malformed IDs; a regression test records this release candidate's own 117-byte ID (BUG-0013, REQ-cli-007, REQ-cli-008).

## Evidence

- Verification commit: `0f8e869c0cfea8b24a83ac1a53e4c7cee01cff8c`
- Base commit: `f19823d4287debe65cf8c5a78fa4a2f72d83b9b7`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The human asked for a usable first release they can review, acceptance-test, install
and use in another project, with no new integrations or runtime. Queued behind the
approved fixes on PRs #3, #7 and #8; implementation starts after those merge, because
it touches the same files.

Observed on main `f19823d` (2026-10-31): 45 observations and 46 artifacts (about 87 KB
of content) in `.chough/`; every case has one run per environment, so today's volume
comes from one run per case per change plus bug evidence, not from reruns. Nothing
prunes, and repeated validation (`validate run --times N`) writes N observation and
log pairs. `.specsync/archive` holds 520 KB of frozen native records, which this change
does not touch. A disposable test showed that removing a manifest-listed observation
keeps the archive `authenticated-history` and `change audit` green, so retention can act
on Chough's working records without altering native archives.

Live guidance drift: README calls the public repository private; ROADMAP says the
stage workflow awaits review although it merged and archived; `cargo package` would
ship 300 files including Chough's own backlog, evidence and archives.

## From the change's design.md

# Design

## Retention (REQ-cli-018)

An observation set is one evidence record plus the artifacts no other record
references. `chough evidence prune [--apply]` classifies every set:

| Protected because | Reason shown |
|---|---|
| Listed in the checkpoint | `checkpoint` |
| Linked by any bug (observation or baseline), any status | `BUG-NNNN <status>` |
| Listed by an active native change manifest | `native active <id>` |
| Listed by the most recent archived native change manifest | `native archive <id>` |
| Not a routine run (manual, agent-assisted, human, delivery, publication, installation, deployment, live-health) | `<kind>` |
| Among the newest 3 automated runs for its case and environment | `newest` |
| Newest failure for its case and environment | `last failure` |
| Failure, first or last pass of a validation run | `validation <run>` |
| Younger than 14 days | `recent` |

Everything else is eligible. The preview prints eligible sets (case, environment,
age, outcome, bytes), protected sets with reasons, totals and a warning above 50 MiB.
Native manifests are read with `specsync change status <id> --json`; if that read is
unavailable, every set it could protect stays protected.

`--apply` needs a run consent `retention` or the standing preference `retention auto`
(values `off`, the default, and `auto`). With `auto`, `cases run` and `validate run`
apply the same rules after recording. Each pruned set appends a line to
`.chough/pruned.jsonl`; files are removed from the working tree only.

Reading references: present and current → `current`; present but stale → `historical`;
absent and in the ledger → `pruned`; absent and not in it → `unavailable`. Pruned and
unavailable never count as current, passed or verifying evidence.

## Package and install (REQ-cli-019)

`Cargo.toml` gains `include = ["/src", "/skills/chough-workflow", "/Cargo.toml",
"/Cargo.lock", "/README.md", "/LICENSE"]`. `KNOWN_PREVIOUS` gains the main-build digests
of bundle files changed by PRs #7 and #8 and this change.

## Guidance

README (public repository, install, platforms, limits), ROADMAP (current states) and
the `docs/architecture.md` records table (one row group per folder: what it is, who
owns it, committed, local or CI artifact, prunable or frozen).

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-018 | `tests/evidence.rs` (new file in tests/): preview changes nothing; every protection reason; age and count defaults; apply needs consent or `retention auto`; ledger line; current, historical, pruned and unavailable labels; native read failure keeps sets protected |
| REQ-cli-019 | `tests/cli.rs`: `cargo package --list` contents; `chough init` in an empty project installs no development records; known-outdated skill copies |

Exercises: a disposable copy with repeated validation runs (retention preview, apply
under synthetic consent, protected sets kept, resumed agent sees `pruned`, never
`passed`); a fresh agent context resuming from files; `cargo install --path` into a
temporary root and `chough init` in a new project. Platforms: hosted CI on Ubuntu,
macOS and Windows; local macOS arm64.

## Where these lessons go

- `specs/cli/context.md`
