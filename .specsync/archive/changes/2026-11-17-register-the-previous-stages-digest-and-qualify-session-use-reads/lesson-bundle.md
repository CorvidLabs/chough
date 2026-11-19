# Lesson bundle: register-the-previous-stages-digest-and-qualify-session-use-reads

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Register the previous stages digest and qualify session-use reads
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/skill.rs, tests/docs.rs, tests/fixtures/stages-at-0fd9fd22.md, docs/architecture.md, changelog.d/2026-11-17-register-previous-stages.md, .chough/cases/cli.previous-stages.cases.toml
- **Acceptance**: A pristine copy of the workflow stages file from main 0fd9fd22 is known-outdated and chough skill update replaces it with the current bundle. An edited copy of that file stays byte-identical. docs/architecture.md says an exact open session ID is returned without reading another session, a bare key or alias reads each open session, and a name that matches none also reads archived stories.

## Evidence

- Verification commit: `1044263846f31c785db1bda4a32d0a8c4b3d206b`
- Base commit: `ab0d421149f12d9ad2984c833cfe1d9346d3989e`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

PR #300 is green and merge is blocked by two unresolved review threads. One says a pristine install of `references/stages.md` from main `0fd9fd22` (digest `36522cbfa49c0d29b2505c552ff15d3743b9bca7c5985db0fb002d6b61974727`) is classified `edited`, so `chough skill update` returns `preserved-edit` and keeps the old fast path. The other says `docs/architecture.md` claims `session use` reads only the session it activates, which is false for a bare key or alias.

The historical skill-release blobs stay as they are. The previous stages file is a fixture beside the corpus, not a new blob. Session behavior does not change. The architecture sentence is qualified to match `resolve_open`.

## From the change's design.md

# Design

No visual or interaction change.

`KNOWN_PREVIOUS` gains the SHA-256 of `references/stages.md` at main `0fd9fd22`. The regression reads that file from `tests/fixtures/stages-at-0fd9fd22.md`, outside the skill-release blob corpus. A pristine copy becomes current on `chough skill update`. An edited copy stays `preserved-edit`.

`docs/architecture.md` keeps the sentence that starting or switching never moves or rewrites another session. It replaces the claim that `session use` reads only the activated session with the three reads `resolve_open` does: an exact open ID returns without reading another session, a bare key or alias reads each open session, and a name that matches none also reads archived stories.

## From the change's testing.md

# Testing

`cargo test --locked --test docs` passed, 21 tests, including `stages_from_main_0fd9fd22_update_and_an_edited_copy_stays`. `chough cases run TC-cli-previous-stages-0fd9fd22 --environment local-macos-arm64` passed. `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` passed.

The regression checks three things: the fixture hashes to `36522cbfa49c0d29b2505c552ff15d3743b9bca7c5985db0fb002d6b61974727`, a pristine copy is known-outdated and `chough skill update` replaces it with the current bundle, and an edited copy stays byte-identical with action `preserved-edit`. The same test checks that `docs/architecture.md` no longer says `session use` reads only the session it activates, and that it names the exact-ID, bare-key, and archived-story reads.

## Where these lessons go

- `specs/cli/cli.3md`
