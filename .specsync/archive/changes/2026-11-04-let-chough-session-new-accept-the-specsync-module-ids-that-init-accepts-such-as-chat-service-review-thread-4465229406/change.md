---
id: let-chough-session-new-accept-the-specsync-module-ids-that-init-accepts-such-as-chat-service-review-thread-4465229406
state: archived
type: bug_fix
base_commit: 290cce3a024425cfd35600a43b8e458beaa89436
---

# Let chough session new accept the SpecSync module IDs that init accepts, such as chat.service (review thread 4465229406 on #77)

## Intent

Let chough session new accept the SpecSync module IDs that init accepts, such as chat.service (review thread 4465229406 on #77)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- Source: review thread 4465229406 on #77 (Leif's agent): chough session new --module chat.service fails while chough init --module chat.service works. Outcomes: session new accepts every module ID init accepts, such as chat.service and identity-wipe.service, and stores them in the session; module IDs SpecSync refuses, such as a/b, .., a trailing dot or a leading space, are refused with the same invalid module name message as init and no session file written; a failing test comes first; fledge lanes run verify passes

## No-spec Rationale

Code-only fix: session new checked --module with the session-key rule instead of the SpecSync module-ID rule that init, checkpoint, bugs, evidence and cases already use (#56); no requirement text names a module-ID rule, so no canonical spec changes
