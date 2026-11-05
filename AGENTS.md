# Chough

Chough (pronounced chuff) is a small Rust helper plus portable agent skills.

To resume work here, run `chough next --environment <env>` (or `cargo run -- next`)
and read [the workflow skill](skills/chough-workflow/SKILL.md). The checkpoint is the
active session under `.chough/sessions/` (`chough session list`); native SpecSync status
decides lifecycle gates.

Before changing behavior, read [INTENT.md](INTENT.md), the relevant `hi/` criteria,
[docs/FRD.md](docs/FRD.md) and the affected contract and companions under `specs/`.
Use Fledge first: `fledge run fmt`, `fledge run test`, `fledge run lint`, `fledge
lanes run verify`. Match tool versions to CI: hi 0.8.0, fledge 1.8.0, specsync 6.2.0.
One command installs all three; see "Install the toolchain" in [README.md](README.md).

Each pull request records its work in its own session, so parallel branches never write
the same checkpoint (DRIVE-3, DRIVE-12):

- Start a branch's work with `chough session new <change-id>`. It becomes the active
  session in that worktree, and finished sessions move to `.chough/sessions/archive/`.
  In another worktree, `chough session use <key>` selects one. Never edit another
  pull request's session; a subagent reports with `chough inbox add`.
- Transitions are immutable event files. The legacy `.chough/transitions.jsonl` is
  frozen behind a digest anchor; never edit it. The retention ledger and
  `CHANGELOG.md` merge with Git's union driver.
- Add a change's cases in a file of its own,
  `.chough/cases/<module>.<change-topic>.cases.toml`, not in the shared module file.
- Set up both Chough merge drivers once per clone:
  `git config --local merge.chough-session.driver "chough checkpoint --merge %O %A %B"`
  and `git config --local merge.chough-bug.driver "chough bug merge %O %A %B"`.
- After merging main into a branch, run `chough library index` and commit `knowledge/`.
  It union-merges, and the verify lane's `library index --check` fails until it is
  rebuilt.
- Still conflicting, until SpecSync fixes it: a module's spec change log and
  `requirements.md` (CorvidLabs/spec-sync#862). Keep both sides' rows, renumber yours
  after main's, set `version:` to the last row, and keep requirement IDs unique.

HI owns human wording and criterion IDs. SpecSync owns contracts, companions,
changes, approval, verification, review and archives; meaningful files need an active
native change. Chough owns only its checkpoint, transitions, preferences, case
definitions, observations and local bugs. Never create approval, review, release or
deployment records from an agent label, and never treat a structural pass as product
proof.

Keep changes small. Preserve existing files and others' edits; own only assigned
files. Do not add a model client, custom runtime, hosted service or duplicate
lifecycle state. Rune, APS, Magpie and other adapters stay optional until
implemented and tested. Do not commit generated binaries or claim unobserved CI,
release or deployment results.
