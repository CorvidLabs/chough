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
  references/testing.md `eac153b213914bf71a84cb08cda5f0810730a324b112e3caf96ee3be2672b3e9`,
  references/build.md `e92e5a8fdc2e7b02ac6e01f34e15ee66501f7e3a312970069abb9e409aca167a`,
  references/plan.md `8ecec7edffa34daa0082a026d6e07d4acc1245d6fc3dcba9e87d7296c226b8ab`. Without KNOWN_PREVIOUS entries, existing installs would
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
  fragment.
- Artifacts: absolute, `..` and symlinked paths are refused by design. An artifact
  ignored by Git records its fingerprint, reads current-observation on the recording
  machine and stale-artifact once missing. SpecSync's project-input digest honours
  Git ignore rules but not its own exclude settings (upstream, shared with #12).
