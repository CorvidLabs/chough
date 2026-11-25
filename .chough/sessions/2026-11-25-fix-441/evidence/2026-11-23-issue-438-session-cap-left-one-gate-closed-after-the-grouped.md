---
kind: evidence
id: 2026-11-23-issue-438-session-cap-left-one-gate-closed-after-the-grouped
aliases: []
claim: Issue 438 session cap left one gate closed after the grouped command failed above 1048576 bytes
result: failed
evidence_kind: product-test
revision: 0d5988f804155e7b3c285cace6cc0c52f540e035
content: 7e5b85682efb33171650644809014f8be254f13ed98cdd621719266068a6bd41
environment: macos-local
fingerprint: sha256:1cd066cef5b67609b7f7c0e6181034997d81964c6af592c9dc56ae5b139f1627
external: ext-58c867
criteria: [FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [Disposable synthetic gates drove no native step.]
observed: 2026-11-23T14:45:37Z
actor:
  - terminal: t14v
---
**Issue 438 session cap left one gate closed after the grouped command failed above 1048576 bytes.**

Recorded as failed in macos-local at revision 0d5988f80415, observed 2026-11-23T14:45:37Z.

## Key failing lines

- Started at 1047831 bytes with both gates open. Gate A closed at 1048334 bytes. Gate B remained pending after its candidate reached 1048819 bytes.
