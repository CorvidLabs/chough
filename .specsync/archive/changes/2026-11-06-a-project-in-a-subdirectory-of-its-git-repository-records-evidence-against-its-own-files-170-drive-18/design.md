---
change: a-project-in-a-subdirectory-of-its-git-repository-records-evidence-against-its-own-files-170-drive-18
artifact: design
---

# Design

## Approach

In `evidence::snapshot` (`src/evidence.rs`), instead of refusing when `git rev-parse --show-toplevel` differs from the project root:
- **Prefix:** take `git rev-parse --show-prefix`, which is empty at the top level, and require the project root to be the top level joined with that prefix.
- **Files:** `git ls-files --cached --others --exclude-standard -z`, run in the project root, already lists only the files under it, relative to it, so the hashing and the `.chough/` and `target/` filters work unchanged.
- **Dirty:** `git status --porcelain --untracked-files=all -- .` limits the dirty check to the project. Its paths are relative to the repository root, so the prefix is stripped before `fingerprint_key`; a path outside the prefix is ignored.
- **Revision:** the repository's `HEAD`, as before.

At the top level the prefix is empty, so every command and byte is as before.

## Risks

- **Renamed paths** in porcelain output (`R  old -> new`) keep today's handling: the text after the status code is checked as a whole, as before.
- **Paths outside the project** (for example `../x`) never appear, because `ls-files` and the pathspec `.` are scoped to the project.
- **Out of scope:** a project outside any Git repository is unchanged; Git's own error still applies.

## Constraints consulted

DRIVE-18, REQ-cli-056 (new), SpecSync D2.
