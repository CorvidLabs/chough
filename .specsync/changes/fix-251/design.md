---
change: fix-251
artifact: design
---

# Design

- `doctor::exclusions` also reads two settings, and notes each one that names the repository root (`.`, `./` or empty) in `whole_repo`:
  - `source_dirs` from `.specsync/config.toml`;
  - `meaningful_paths` from the policy.
- `Exclusions::advice`, which both `chough init` and `chough doctor` print, gives the narrowing advice first while `whole_repo` is not empty and something is to be done. It names the settings and the overlap refusal, and the skill bundle when `source_dirs` covers the repository. It says to narrow `source_dirs` to the code folders, for example `["src"]`, and keep `.` out of `meaningful_paths`, then list the missing entries. With no policy, `init` gives it without the list.
- The `records_in_verification` finding still exists only while the policy does (REQ-history-012). Its `fix` names the narrowing, and its detail lists `whole_repo`.
- Chough writes neither file.
- For the root-code case (#354), `exclusions` notes a root-level file a spec covers in `root_code`, read from the `files:` lists under `specs_dir`. It also notes in `unexcluded` the folders `exclude_dirs` doesn't list yet: `.chough`, and the top folder of each installed skill bundle from `skill::HOSTS`. `advice` and the finding's fix then name excluding them, and the list's need for a folder or spec-sync#1083, instead of narrowing.
