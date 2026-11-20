---
change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
artifact: design
---

# Design

1. **The pin, 6.3.0 to 6.4.0, everywhere Chough names it**: the five SpecSync Action
   steps in `.github/workflows/ci.yml` (`version:` and the Action commit), the version the
   archive check and its test expect (`.github/scripts/`), `.specsync/config.toml`'s
   header, `AGENTS.md`, the README's install line and its Rust note, `docs/ci.md`,
   `docs/cli.md`, `docs/integrations.md`, `docs/walkthrough.md`, and the install hint in
   `src/change.rs` and `src/ready.rs`. The Action commit is the candidate's until 6.4.0
   publishes, then the published tag's: a swap of one commit hash, before review.
2. **Ready asks SpecSync's dry run** (`src/ready.rs`). It runs
   `specsync --json change approve <id> --dry-run`, with no `--actor`. Each refused check
   is a finding, SpecSync's message shown as given with control characters escaped; its
   fix is SpecSync's `fix` when it gives one, shown and never run, otherwise
   `chough change ready <id>` once the message's problem is fixed. A skipped check is not
   a finding: it waits on a refused one, which is. A `specsync` that cannot start, or
   prints no dry-run document, gives one finding naming
   `cargo install --locked specsync@6.4.0`, never "Nothing is missing.".
3. **The delta-set rule goes** (Q1, A): the dry run's `deltas` refusal names the same
   missing and extra deltas. The stale-text, exports, cases, paths, interview and
   artifact rules stay: they are Chough's own checks, or list every item where the dry
   run gives each check's first problem.
4. **Paths outside the declaration** leave out SpecSync's own untracked
   `.specsync/change.lock` and `.specsync/workflow-v2-baseline.json`, and a path an
   affected path of another active change covers (Q2, A).
5. **`.specsync/hashes.json`**: ready no longer makes or restores it. `Cache`,
   `make_cache`, `restore_cache` and `open_cache` go, with their stderr line; the exports
   check just runs `specsync check --spec <module> --format json`.
6. **History fragments**: `.gitattributes` loses `specs/**/*.spec.md merge=specsync`;
   `specs/**/requirements.md` keeps SpecSync's driver. `docs/cli.md` and `AGENTS.md` drop
   the advice to rebuild a spec's Change Log by hand. `.specsync/sdd.json` loses the
   inert `specs/` entry.

## Open questions

Q1. Ready's own delta-set rule: **(A) drop it (recommended)**, since the dry run names
the same missing and extra deltas, one rule fewer, though in one SpecSync line with
`chough change ready <id>` as the fix rather than one `write` or `remove` command per
file; or **(B) keep it beside the dry run**, with per-file fixes, each such delta then
named twice.

Q2. Another active change's work on the same branch (#406's run on 66 real changes found
hundreds of such lines per change): **(A) leave out a path that another active change's
affected path covers (recommended)**, one exception in the paths rule; or **(B) leave it
for a later batch**.
