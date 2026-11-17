---
change: register-the-previous-stages-digest-and-qualify-session-use-reads
artifact: context
---

# Context

PR #300 is green and merge is blocked by two unresolved review threads. One says a pristine install of `references/stages.md` from main `0fd9fd22` (digest `36522cbfa49c0d29b2505c552ff15d3743b9bca7c5985db0fb002d6b61974727`) is classified `edited`, so `chough skill update` returns `preserved-edit` and keeps the old fast path. The other says `docs/architecture.md` claims `session use` reads only the session it activates, which is false for a bare key or alias.

The historical skill-release blobs stay as they are. The previous stages file is a fixture beside the corpus, not a new blob. Session behavior does not change. The architecture sentence is qualified to match `resolve_open`.
