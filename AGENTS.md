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
On Linux, `fledge run linux-image` builds the test images from `tools/linux/` with those
pins, and `fledge run linux-verify` runs the verify lane in them; rebuild the images
with `linux-image` after a Docker reset.
One command installs all three; see "Install the toolchain" in [README.md](README.md).

Each pull request records its work in its own session, so parallel branches never write
the same checkpoint (DRIVE-3, DRIVE-12):

- Start a branch's work with `chough session new <change-id>`. It becomes the active
  session in that worktree, as the folder `.chough/sessions/<date>-<slug>/`, whose
  `session.md` front matter is the checkpoint. In another worktree, `chough session use
  <key>` selects one. Never edit another pull request's session; a subagent reports
  with `chough inbox add`.
- Transitions are entries of the session's `## Timeline`, appended in the same save as
  the checkpoint change. Never edit, reorder or remove one; `chough correct` appends a
  correction. At its move into done a session becomes one story,
  `.chough/archive/sessions/<id>.md`: commit it, then run `chough doctor --fix` to
  remove the committed session folder and commit that.
- `.chough/sessions/`, `.chough/archive/` and `.chough/bugs/` are listed in
  `verification_ignored_paths` (`.specsync/sdd.json`), so Chough's records can be
  written and committed at any point of the native order; `chough next` says so.
  `CHANGELOG.md` merges with Git's union driver.
- Add a change's cases in a file of its own,
  `.chough/cases/<module>.<change-topic>.cases.toml`, not in the shared module file.
- Set up Chough's bug merge driver once per clone:
  `git config --local merge.chough-bug.driver "chough bug merge %O %A %B"`. Sessions need
  no merge driver: each is its own folder.
- After merging main into a branch, run `chough library index` and commit `knowledge/`.
  It union-merges, and the verify lane's `library index --check` fails until it is
  rebuilt.
- Still conflicting, until SpecSync fixes it: a module's spec change log and
  `requirements.md` (CorvidLabs/spec-sync#862). Keep both sides' rows, renumber yours
  after main's, set `version:` to the last row, and keep requirement IDs unique.

HI owns human wording and criterion IDs. SpecSync owns contracts, companions,
changes, approval, verification, review and archives; meaningful files need an active
native change. Chough owns only its records under `.chough/` (sessions with their
checkpoint, story and Timeline, evidence, local bugs, case definitions and
preferences) and the generated knowledge index and journal. An approval or review
in a human's name comes only from that human's decision at a Chough gate
(`chough checkpoint --close-gate`); on authority given in advance the agent records
it as itself, `<agent>-<terminal>`, quoting the human. Never create approval, review,
release or deployment records from an agent label, and never treat a structural pass
as product proof.

Keep changes small. Preserve existing files and others' edits; own only assigned
files. Do not add a model client, custom runtime, hosted service or duplicate
lifecycle state. Rune, APS, Magpie and other adapters stay optional until
implemented and tested. Do not commit generated binaries or claim unobserved CI,
release or deployment results.
