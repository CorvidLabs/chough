# Chough

Chough (pronounced chuff) is a small Rust helper plus portable agent skills.

To resume work here, run `chough next --environment <env>` (or `cargo run -- next`)
and read [the workflow skill](skills/chough-workflow/SKILL.md). The checkpoint is the
active session under `.chough/sessions/` (`chough session list`); native SpecSync status
decides lifecycle gates.

Before changing behavior, read [INTENT.md](INTENT.md), the relevant `hi/` criteria,
[docs/FRD.md](docs/FRD.md) and the affected contract and companions under `specs/`.
Use Fledge first: `fledge run fmt`, `fledge run test`, `fledge run lint`, `fledge
lanes run verify`. Match tool versions to CI: hi 0.8.0, fledge 1.8.0, specsync 6.4.0.
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
- Add a change's changelog entry as a fragment of its own,
  `changelog.d/<YYYY-MM-DD>-<change-id>.md`, not in `CHANGELOG.md`; a release assembles
  the fragments into `CHANGELOG.md` with `fledge run changelog` (#290).

For a local or small-context agent, including Flint, Nemotron, Ripple, and every host
template under `integrations/`:

- Before any edit, run `chough session list`, then `chough next --environment local`, and do the `Run:` line.
- For a bug fix or small feature in one module, read only the fast path in `skills/chough-workflow/references/stages.md` and the one reference `chough next` names.
- The session key is `fix-<issue>`. The change description is `fix <issue>`, not a sentence. Use an existing HI id. Do not invent one. A `REQ-` id goes on `--requirement` only.
- Use `chough change new`, `answer`, `check`, `status` and `ship`. Do not run `specsync change approve`, `review`, `reopen` or `send-back`. Declare every path, including tests, before the first product edit.
- Stop at intent, scope approval and implementation review. Open a gate with `chough checkpoint --awaiting-human`. Close it with `--close-gate` only with words the human gave you.
- Before the review: `chough change check <id> --no-commit` when the delta changes requirements, `chough cases run`, open the review gate, commit the source with `.chough/`, then `chough change check <id>`, which commits the verification. Add the changelog fragment in that same work. End every file with a trailing newline.
- A `refused:` message is the answer. Run the command it names, or stop and report it. Do not hand-edit a session, the Timeline, an archive or an approval. If `chough <command> --help` does not show a flag, it does not exist.
- Do not push, open or merge a pull request, or release, unless the task says so. Wait on another agent with the harness's wait. Do not sleep in a loop. A review ends with a line that is exactly `Verdict: pass` or `Verdict: block`.
- Add a change's cases in a file of its own,
  `.chough/cases/<module>.<change-topic>.cases.toml`, not in the shared module file.
- Set up Chough's two merge drivers once per clone:
  `git config --local merge.chough-bug.driver "chough bug merge %O %A %B"` and
  `git config --local merge.chough-generated.driver "chough library merge %O %A %B"`.
  Also run `specsync merge --install` once, so `specs/**/requirements.md` merges
  through SpecSync's driver (#291). A module's spec file needs none: SpecSync 6.4.0
  records each change in its own history fragment, `specs/<module>/history/<id>.md`,
  in place of a Change Log row and a `version:` bump (#404).
  Sessions need no merge driver: each is its own folder.
- Rebuild the generated `knowledge/index.md` and its feed with `chough library index`
  and commit `knowledge/` when preparing a release, before the version bump is tagged;
  the release task then runs `chough library index --check` before that branch merges.
- Still conflicting, until SpecSync fixes it: `requirements.md` (CorvidLabs/spec-sync#862).
  Keep both sides' requirements and keep requirement IDs unique.

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
