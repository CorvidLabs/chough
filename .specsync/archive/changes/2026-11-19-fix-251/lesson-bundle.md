# Lesson bundle: fix-251

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 251
- **Kind**: BugFix
- **Specs**: records, cli
- **Paths**: src/doctor.rs, tests/doctor.rs, docs/cli.md, .chough/cases/records.whole-repo-specsync.cases.toml, changelog.d/2026-11-18-fix-251.md
- **Acceptance**: With source_dirs = ["."] in .specsync/config.toml, chough init prints advice that names it, says SpecSync refuses the record folders in verification_ignored_paths as overlapping and counts the installed skill bundle as uncovered source, and advises narrowing source_dirs to the code folders such as ["src"]; with a policy whose meaningful_paths holds "." too, chough doctor's records_in_verification finding names both settings, advises narrowing and keeping "." out of meaningful_paths, then listing the missing entries; once both are narrowed the advice is the list again. Neither file is ever written.

## Evidence

- Verification commit: `4582f2345d44cef3f82951640aad66933c71b2b0`
- Base commit: `66efea4dd777077ef9bbd412cf0fac8f7ea93b41`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

GitHub #251 and #279, from CorvidLabs/arcron and NFTMix adopting Chough 0.6.0. Many CorvidLabs repos set SpecSync's `source_dirs = ["."]`.

- **#251:** `chough doctor`'s `records_in_verification` advised listing Chough's record folders in `verification_ignored_paths`. SpecSync refuses any entry that overlaps `source_dirs`, so the advice could not be followed and the finding stayed forever. The arcron follow-up found that SpecSync refuses the list over `meaningful_paths` holding `.` too.
- **#279:** with `source_dirs = ["."]`, the skill bundle `chough init` installs counts as uncovered source, and a 100% coverage gate goes red on adoption.

Leif's decision (2026-11-18, planning interview, on both issues, relayed by hc t4c as the scope approval) is that Chough advises narrowing. When SpecSync's `source_dirs` is `["."]`, `chough init` and `chough doctor` advise narrowing it to the real code folders, for example `["src"]`, and say why, instead of advice SpecSync refuses as overlapping. That fixes both issues together without changing SpecSync's overlap rule.

Asked whether the advice also covers `.` in `meaningful_paths`, he answered "Yes, both" (relayed by hc t4c, 2026-11-18).

The tester's check of #354 passed the arcron shape and blocked on NFTMix's. There, every specified file sits at the repository root, so there is no folder to narrow `source_dirs` to. The literal example `["src"]` leaves coverage at 0/0, and no SpecSync setting satisfies both the coverage gate and the ignored list. Leif chose "Both: better advice + SpecSync" (relayed by hc t4c, 2026-11-18). When the specified code sits at the root, init and doctor advise adding the skill bundle's and Chough's folders to `exclude_dirs`, which fixes #279's coverage. They also say the ignored list needs the code moved into a folder, or SpecSync's fix, filed as CorvidLabs/spec-sync#1083. The folder-repo advice stays as it is.

## From the change's design.md

# Design

- `doctor::exclusions` also reads two settings, and notes each one that names the repository root (`.`, `./` or empty) in `whole_repo`:
  - `source_dirs` from `.specsync/config.toml`;
  - `meaningful_paths` from the policy.
- `Exclusions::advice`, which both `chough init` and `chough doctor` print, gives the narrowing advice first while `whole_repo` is not empty and something is to be done. It names the settings and the overlap refusal, and the skill bundle when `source_dirs` covers the repository. It says to narrow `source_dirs` to the code folders, for example `["src"]`, and keep `.` out of `meaningful_paths`, then list the missing entries. With no policy, `init` gives it without the list.
- The `records_in_verification` finding still exists only while the policy does (REQ-history-012). Its `fix` names the narrowing, and its detail lists `whole_repo`.
- Chough writes neither file.
- For the root-code case (#354), `exclusions` notes a root-level file a spec covers in `root_code`, read from the `files:` lists under `specs_dir`. It also notes in `unexcluded` the folders `exclude_dirs` doesn't list yet: `.chough`, and the top folder of each installed skill bundle from `skill::HOSTS`. `advice` and the finding's fix then name excluding them, and the list's need for a folder or spec-sync#1083, instead of narrowing.

## From the change's testing.md

# Testing

`tests/doctor.rs::a_whole_repository_specsync_gets_the_narrowing_advice`, red on main, has three steps:
1. With `source_dirs = ["."]` and no policy, `chough init` prints the narrowing advice, naming the setting, the overlap refusal, the skill bundle and `["src"]`. `doctor` reports no finding.
2. With a policy whose `meaningful_paths` holds `.`, the finding's fix starts with the narrowing, and its advice names both settings and then the list.
3. Once both are narrowed, the advice is the list again.

Both files stay byte-identical throughout. Case `TC-records-a-whole-repository-specsync-gets-the-narrowing-advice` binds the test.

`tests/doctor.rs::root_level_code_gets_the_exclude_advice_instead_of_narrowing` (#354), red before this addition: a spec covering `generate.py` and `config.yaml` at the root gets the exclude advice and spec-sync#1083, with no folder to narrow to. Once those folders are excluded, only the list's part is left.

## Where these lessons go

- `specs/records/context.md`
- `specs/cli/cli.3md`
