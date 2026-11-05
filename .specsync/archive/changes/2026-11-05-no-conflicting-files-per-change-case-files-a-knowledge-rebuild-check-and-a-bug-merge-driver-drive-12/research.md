---
change: no-conflicting-files-per-change-case-files-a-knowledge-rebuild-check-and-a-bug-merge-driver-drive-12
artifact: research
---

# Research

- `cases::load` requires each file's stem to equal its `module` field, so all of a module's cases share one file.
- `library::build` already produces every generated file in memory before `write_all` writes it. A check can compare without writing.
- `chough checkpoint --merge %O %A %B` is the existing pattern for a Chough merge driver: it writes the result into `%A` and exits nonzero, leaving `%A` alone, when it can't merge.
- Git's built-in `union` driver needs no configuration. A custom driver needs `git config merge.<name>.driver` in each clone; when it isn't configured, Git falls back to its normal text merge. So the knowledge files use union plus a rebuild check, and bug records need a driver, because union breaks JSON.
