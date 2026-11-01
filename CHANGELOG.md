# Changelog

All notable changes to Chough are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Fresh-agent handoff blocks in `chough next`, read from existing records and within
  the same 16384-byte cap: the 5 newest checkpoint decisions with the kind and reason
  of entries written as `decided:`, `rejected:` or `failed:` `<text> because
  <reason>` (other entries shown whole; no checkpoint schema change), the latest
  failed observation by recorded time with the status of each bug that links it, and
  a pointer to each module's lesson source with the newest archived SpecSync lesson
  bundles. Output is ordered by recorded fields and names only, so the same records
  give byte-identical text and JSON.
- Skill guidance: lessons carry an `observation`, `hypothesis` or `verified` label
  with their source and limits, a project decision always outranks a lesson, a
  failure fixed in the same pull request still gets a bug or lesson, and closeout is
  one quiet pass inside the open native change.

### Fixed

- `chough next` and `chough status` no longer say `Continue <stage>`, or give the
  build gate refusal, at intent to accept once the recorded native change is
  archived with authenticated terminal evidence; they name the archive and the new
  cycle, recording done first at accept.

### Changed

- The skill now says SpecSync 6.1.2 ships `verification_ignored_paths` and why
  Chough's own repository has not adopted it.

## [0.1.0] - 2026-11-01

First release. Nothing in this entry is tagged or published yet; the release steps
are in [docs/releasing.md](docs/releasing.md).

### Added

- Resumable stage workflow: intent, design, plan, build, test, accept and optional
  release, with side stages for bugs, repeated validation and the librarian;
  checkpoint schema 2 in `.chough/session.json`, focused cases (`chough-testing/1`),
  local bugs, preferences and run consents, and per-host skill installation
  ([#2](https://github.com/CorvidLabs/chough/pull/2)).
- Native archive CI checks: CI fails while a native SpecSync change is active or an
  archive fails to authenticate ([#3](https://github.com/CorvidLabs/chough/pull/3)),
  and the archive gate now runs before every other CI job
  ([#18](https://github.com/CorvidLabs/chough/pull/18)). No check is required to
  merge yet; see [docs/ci.md](docs/ci.md).
- Fail-closed stage gates on native approval validity, archive integrity and the SDD
  policy ([#7](https://github.com/CorvidLabs/chough/pull/7)).
- Bounded external commands: time and output limits for case bindings and native
  reads; validation honors a stop request after waiting
  ([#8](https://github.com/CorvidLabs/chough/pull/8)).
- Reference-aware evidence retention (`chough evidence prune`), current guidance and
  an installable package that carries no Chough records
  ([#9](https://github.com/CorvidLabs/chough/pull/9)).
- `chough timing`: how long stages, tests, bug fixes and human decisions took, from
  existing records ([#15](https://github.com/CorvidLabs/chough/pull/15)).
- Case binding `env`, matching on visible text with terminal escape sequences removed
  (an `expect_output` fragment containing ESC is refused), and actionable
  requirement and bug errors ([#16](https://github.com/CorvidLabs/chough/pull/16)).
- A compact `chough next` view, text or `--json`, each form within 16384 bytes with
  bounded lists and stated totals; an approved-delta guard that refuses moves to
  build, test or accept when an affected native delta no longer matches its
  approval digest, since SpecSync 6.1.1 status does not report it (BUG-0002); and done-stage resume that recommends a new
  cycle once the native change is archived
  ([#23](https://github.com/CorvidLabs/chough/pull/23)).
- Release preparation: pinned HI 0.8.0, SpecSync 6.1.2 and Fledge 1.8.0 with one
  install command and a test that keeps the pins in agreement, a minimum Rust
  version of 1.88 checked in CI, skill guidance corrected against the helper, and
  this change log.

### Changed

- Native status is parsed beyond the display bound, and checkpoint clears apply
  before additions ([#4](https://github.com/CorvidLabs/chough/pull/4),
  recorded in [#6](https://github.com/CorvidLabs/chough/pull/6)).
- Evidence stays current across commits that change only Chough records or SpecSync
  bookkeeping, and a bug is verified only by a pass observed after its latest
  failure ([#17](https://github.com/CorvidLabs/chough/pull/17)).
- The skill bundle holds no em or en dashes
  ([#16](https://github.com/CorvidLabs/chough/pull/16)); the README says how to
  pronounce the name ([#19](https://github.com/CorvidLabs/chough/pull/19)).
- Documentation checked against the helper: stage gates, CLI reference, CI, roadmap
  and diagrams ([#21](https://github.com/CorvidLabs/chough/pull/21)).

### Known limitations

See [Known limitations](README.md#known-limitations) in the README.

[0.1.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.1.0
