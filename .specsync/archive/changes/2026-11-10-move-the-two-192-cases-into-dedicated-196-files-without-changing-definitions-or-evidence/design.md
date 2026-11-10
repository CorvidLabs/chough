---
change: move-the-two-192-cases-into-dedicated-196-files-without-changing-definitions-or-evidence
artifact: design
---

# Design

## File mapping

| Existing case ID | Old file | New file |
| --- | --- | --- |
| TC-cli-change-new-no-spec-change | .chough/cases/cli.change-steps.cases.toml | .chough/cases/cli.case-split-196.cases.toml |
| TC-history-timing-skips-direct-writes | .chough/cases/history.direct-writes.cases.toml | .chough/cases/history.case-split-196.cases.toml |

Each new file retains profile chough-testing/1 and its original module. Every original case ID appears exactly once globally. The unchanged parser/digest code continues to load definitions; source filename changes while semantic case digests, bindings and evidence links remain unchanged. Canonical requirements are already satisfied by the same cases; no delta or permanent HI/REQ ID is created. This is nonvisual, so no visual-design artifact/approval is needed. No public CLI/API or persisted case schema changes.

Knowledge/INDEX.md may be regenerated using the supported command after own bookkeeping; all actual generated paths must be enumerated. Only this change's session and cycle plus native workspace are written. Inherited source/records remain exact.
