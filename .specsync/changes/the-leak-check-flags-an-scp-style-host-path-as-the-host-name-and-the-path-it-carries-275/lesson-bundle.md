# Lesson bundle: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: The leak check flags an scp-style host path as the host name and the path it carries (#275)
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/leaks.rs, specs/records/requirements.md, specs/records/records.spec.md, docs/cli.md, CHANGELOG.md, .chough/cases/records.scp-host-path.cases.toml, .chough/cases/records.leaks.cases.toml
- **Acceptance**: A committed value holding an scp-style remote path <host>:/<absolute path> is refused as a path leak when a home or system root follows the colon, and its host part is refused as a host-name leak when it is the local host, in the write guard, --redact and doctor's scan alike.

## Evidence

- Verification commit: `40db11b291c1ce4e4201589a384269142f040302`
- Base commit: `9b729e908adf30b6511531776f6976b1afd01395`
- Verified by: `specsync check --spec records`

## From the change's context.md

# Context

Issue #275: the leak check for committed records (REQ-records-006) flagged a home or system path, the local host name in a path or as `<user>@<host>`, emails and credentials, but not an scp-style remote path `host:/path` such as `buildbox:/Users/name/project`. Found by t384 in the post-merge review of #241.

The code change and its test were written by Flint 40B (local, Ollama) through Pi in a rerun of the local-model bake-off, from the workflow skill's new fast path (#285); Claude Code brought the commit onto main, opened this session and native change, and moved the case into its own file.

## From the change's design.md

# Design

`names_a_host` in `src/leaks.rs` gains one context: a name followed by `:` and then `/` or `\` is a host in context, like a name inside a path or after `<user>@`. The path after the colon was already checked as a path. No visual or interaction change.

## From the change's testing.md

# Testing

`leaks::tests::an_scp_style_remote_path_is_checked_like_the_path_it_carries` uses a synthetic host and user. Before the fix, `brp:/Users/SYNTHETIC-person/project` yielded only `home-path`; after it yields `host-name` and `home-path`, and the refusal text, the `--redact` replacement and doctor's per-line scan agree. Case `TC-records-an-scp-style-host-path-is-checked-like-the-path-it-carries` binds it.

## Where these lessons go

- `specs/records/context.md`
