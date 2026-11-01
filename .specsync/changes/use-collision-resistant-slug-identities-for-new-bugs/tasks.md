---
change: use-collision-resistant-slug-identities-for-new-bugs
artifact: tasks
---

# Tasks

- [x] Add TC-cli-0125 and the independent-root regression, record `tc-cli-0125-collision-pre-fix` with both manual and automatic collisions, and link BUG-0026 before changing `src/bugs.rs`.
- [x] Implement the shared creation-time generator, dual-form validation, 71-byte slug bound, canonical SHA-256 payload, immutable updates and atomic collision refusal without numeric fallback.
- [x] Cover slug and digest vectors, legacy compatibility, cross-form links, update immutability and concurrent different-content creation with the required collision diagnostic and byte preservation.
- [x] Apply the public and companion updates other than scoped REQ-cli-012 materialization, and pass focused bug and optional tests, formatting and warnings-denied Clippy after the final source edit.

Materialization and final evidence: REQ-cli-012 is materialized, `tc-cli-0125-collision-post-fix` passed and is linked to verified BUG-0026, and the companions name both actual observations. The full Rust suite and both Fledge lanes remain to be run with CI-matching tools before acceptance.

Human gates and finalization (not tasks): scope approval is recorded for 0xLeif. Human implementation review, native finalization and archive, and the merge decision remain separate human gates. No checked task or local structural pass approves, finalizes, merges, releases or deploys the change.
