---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: research
---

# Research

All observed on main 2756dc2 with specsync 6.1.1 (triage journal for #11, #13, #14
and the verifier corrections).

- Dashes: `git grep -nI` for U+2014 and U+2013 under `skills/` finds only
  templates/cases.toml:1 and templates/frd.md:23. The rest of the repository has more
  (fledge.toml, docs/exercises/intent.md, examples/checkout-ui, SpecSync-written
  archive headings); a repository-wide rule is out of scope. Digests on main:
  frd.md `5647b56e21f6667d1ed97381bcbd67a7baf2f60dd8332abcb269e72d4759fd23`,
  cases.toml `6c7ebaa10b099829bc0db4529e28286ded721b0dfa9b5635a3ae0f37d9d91d4a`,
  references/testing.md `eac153b213914bf71a84cb08cda5f0810730a324b112e3caf96ee3be2672b3e9`
  (the three bundle files this change edits). Without KNOWN_PREVIOUS entries, existing installs would
  read `edited` and never update. Raven's lint checks only `*.md` and excludes
  `.claude/`, so only the `.agents/` frd.md copy fails there; the bundle rule still
  covers every file.
- Requirement refusal: `evidence::validate_requirements(root, contracts,
  requirements)` concatenates the contracts' requirements.md and token-matches IDs.
  It has no session access and is called from `cases::check_links` (cases check, run
  and record) and `evidence::add`. A cross-module ID, a typo and an unmaterialized
  delta ID give word-for-word identical messages. Native `change status|show --json`
  does not list a delta's requirement IDs.
- Bug flags: `bugs::validate` checks SCOPES and CLASSES separately; the clap fields
  are free strings with no help text, so `--help` shows no values.
- Environment: `Binding` is `deny_unknown_fields`; `env` gives an unknown-field error.
  `records::run_bounded` inherits the parent environment, and nothing in the record
  says so: the same case passed with `FOO=bar` set and failed without it.
  `Located::digest` serializes the whole case, so a new field must skip serialization
  when empty. Evidence parsing is strict, so a new observation field would make older
  binaries reject new records; the design uses a limitation string instead.
- Colour: `cases::artifact` builds `text` with `from_utf8_lossy` of the raw bytes, and
  both `expect_output` and `cargo_counts` use it. Vitest-style output failed on
  `Test Files  1 passed (1)`. No case in this repository or examples/ has ESC in a
  fragment. `cargo_counts` sums every line that starts with `test result:` and the
  adapter passes only when exactly one test ran, so a cleaner that joins lines or
  swallows the rest of the output could turn a 2-test match into a pass; the design
  never removes a line break.
- Artifacts: absolute, `..` and symlinked paths are refused by design. An artifact
  ignored by Git records its fingerprint, reads current-observation on the recording
  machine and stale-artifact once missing. SpecSync's project-input digest honours
  Git ignore rules but not its own exclude settings (upstream, shared with #12).
  Git LFS was not tested here; open CorvidLabs/spec-sync#789 reports that `change
  check` refuses tracked LFS files ('Git `filter` attribute is not supported for
  canonical evidence') even under ignored_paths, so LFS is not recommended.
- Bugs: BUG-0017 to BUG-0019 have no linked observations yet, so under
  references/bugs.md they are unverified reports, and open BUG-0014 means Chough does
  not check that a pass came after the failure. BUG-0019's recorded revision was
  57d331e, an amended-away commit (same source as 2756dc2 plus bug records); it is
  corrected to 2756dc2 with a note.
