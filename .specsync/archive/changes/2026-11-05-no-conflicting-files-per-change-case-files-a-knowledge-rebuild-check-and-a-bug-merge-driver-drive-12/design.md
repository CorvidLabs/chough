---
change: no-conflicting-files-per-change-case-files-a-knowledge-rebuild-check-and-a-bug-merge-driver-drive-12
artifact: design
---

# Design

- **Cases:** `load` accepts `<module>.cases.toml` and `<module>.<part>.cases.toml`. The module is the stem's first dot-separated segment and must equal the file's `module` field. `<part>` is a valid module-style name. Case IDs stay unique across files, as now. Each change adds its cases in a file of its own, for example `cli.no-conflicting-files.cases.toml`.
- **Knowledge:** `chough library index --check` builds every generated file in memory, writes nothing, and fails, naming each file that is missing or differs from the rebuild, with CRLF read as LF. The verify and ci lanes run it as a `knowledge` task. `.gitattributes` gives `knowledge/**` the union driver: a merge never stops on the vault, and the check makes sure it is rebuilt with `chough library index` before CI passes.
- **Bugs:** `chough bug merge <base> <ours> <theirs>` is a three-way merge over the record:
  - A field only one side changed takes that change.
  - Lists (`notes`, `observations`, `cases`, `criteria`, `requirements`, `steps`, `related`) keep base's entries, then ours' additions, then theirs', without duplicates. An entry either side removed is removed.
  - `updated_at_unix_seconds` is the later one.
  - Any other field that both sides changed differently, such as status or scope, is a conflict. `ours` is left unchanged and the command exits nonzero for a human to resolve.
  - The result must parse as a bug record, and it is written the way `bug` commands write one.
- `.gitattributes` declares `.chough/bugs/*.json merge=chough-bug`. AGENTS.md and docs/cli.md show the one-time `git config` for both Chough drivers.
