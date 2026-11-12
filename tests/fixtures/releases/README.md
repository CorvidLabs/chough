# Release fixtures

One captured project per released 0.x tag, for the 0.6.0 migration tests
(issue #213, LIFE-21 bar item 3). Each folder holds what that tag's chough
actually wrote when driven through one small scenario; the 0.6.0 migration
test migrates a copy of each and asserts nothing is lost.

## What each fixture holds

Every tag: a tiny neutral project (`README.md`, `artifact.txt`,
`specs/fixture/` companions) and its `.chough/` tree with:

- a session (schema-2 `session.json` in v0.1.0–v0.3.0; `.chough/sessions/` in
  v0.4.0+),
- stage moves (v0.1.0: `transitions.jsonl`; v0.2.0+: `history/cycles/`
  events),
- one case definition, checked and run (`.chough/cases/fixture.cases.toml`
  plus its observation and test-log artifact),
- one evidence observation with a small artifact (`artifact.txt`),
- one bug record.

v0.4.0 and v0.5.0 also hold an archived session under
`.chough/sessions/archive/` (v0.4.0: the first session driven to done and
archived by `session use`; v0.5.0: the second session closed with
`session close`). v0.1.0 predates `history/`; v0.2.0 moved transitions into
history events.

## Regenerating

```sh
regenerate.sh v0.3.0   # one tag
```

The script builds the tag in `/tmp/chough-build/<tag>` (reused if the release
binary exists) with `cargo +1.96.0 build --release --locked` — under a shared lock when
`CHOUGH_HEAVY_LOCK` names a lock file, directly otherwise — drives it in
`/tmp/chough-fixture/<tag>/` with stub `hi` and
`specsync` stand-ins, and rewrites the tag's folder here. Building from
source requires only the repository (for `git worktree add <tag>`) and the
1.96.0 toolchain.

## What varies between runs

Timestamps, the random hex suffixes on observation, bug, event and cycle IDs,
Git commit SHAs inside the fixture project, and content digests that cover
those. Everything else — file layout, record kinds, the scenario — is fixed.

## No leaks

Captures run with HOME, USER and the project under `/tmp/chough-fixture/` and
`USER=fixture`; the script greps the result for the caller's real home, user
name and host name and fails on any hit. No lock path is committed; the lock location comes from the
`CHOUGH_HEAVY_LOCK` environment variable.
