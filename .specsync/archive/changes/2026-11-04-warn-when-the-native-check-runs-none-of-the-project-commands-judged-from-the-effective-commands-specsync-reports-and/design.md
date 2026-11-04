---
change: warn-when-the-native-check-runs-none-of-the-project-commands-judged-from-the-effective-commands-specsync-reports-and
artifact: design
---

# Design

- New `src/policy.rs` owns reading `.specsync/sdd.json` (moved unchanged from
  src/stage.rs) and adds `verification_warning(root, effective)`, where `effective` is
  the `summary.verification_commands` list of the native status read that `next` and
  `status` already made (no new read). It returns a warning when that list is present
  and every entry is one of SpecSync's own commands (`specsync` followed by a space),
  naming them; when `.specsync/sdd.json` lists commands the effective list does not
  contain, the warning also says how many configured commands are not run. It returns
  nothing when the list is absent (native read unknown or older output), when a project
  command is present, when SDD is disabled or with no native configuration.
- `chough next` (text and JSON) and `chough status` carry it as
  `native_verification_warning` (null otherwise); text prints a `Warning:` line under
  the native change. The `stage` report is not changed: `stage show` makes no native
  read.
- `stage::assess`: the plain "Continue" action at test keeps its text, says recorded
  test evidence is needed before accept and names `chough cases run`.
- tests/common/mod.rs: the stub `specsync` reports `summary.verification_commands` from
  `.stub-verification` when that file exists; existing fixtures are unchanged.
