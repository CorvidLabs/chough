# Lesson bundle: fix-435

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 435
- **Kind**: Feature
- **Specs**: records, history, cli
- **Paths**: src/sessions.rs, src/records.rs, tests/no_hard_links.rs, changelog.d
- **Acceptance**: On a volume without hard links (exFAT), chough inbox add writes its entry whole or not at all and succeeds, as init, bug add and journal add do since #260 and #424; the ignored test tests/no_hard_links.rs covers it.

## Evidence

- Verification commit: `07eff875332690cf06e489276e96ccbbd362eed0`
- Base commit: `4a46668220663eb00524cb6990a325b0713eb099`
- Verified by: `specsync check --spec cli --spec history --spec records`

## From the change's context.md

# Context

Found by the 0.6.2 documentation audit and filed as chough#435.
`chough inbox add` still publishes each entry with `fs::hard_link`
(src/sessions.rs `publish_entry`, about line 627). On a volume without
hard links, such as exFAT, it fails the way `chough init` did before
#424. #260 and #424 removed the hard links from new bugs, journal posts
and `.chough/local/.gitignore`; the docs say only those work there.

The existing whole-or-not-at-all writer is `records::write_new_bytes`
(src/records.rs, with `write_aside_then_place` and an
`Unsupported` fallback to create-new in place). `tests/no_hard_links.rs`
mounts an exFAT image with hdiutil (ignored by default) and is the way
this was checked for init/bug add/journal add; it gains inbox add.

## From the change's testing.md

# Testing

The exFAT path: tests/no_hard_links.rs (ignored by default, mounts an
exFAT image with hdiutil; run by hand with `cargo test --locked --test
no_hard_links -- --ignored` when safe) gains `chough inbox add` and
asserts the entry is written whole. The regular lane: the session and
inbox tests that cover publish_entry's exists-and-false contract, plus
fmt and lint. Verification commands: the change's own specsync check
lane plus those tests.

## Where these lessons go

- `specs/records/context.md`
- `specs/history/context.md`
- `specs/cli/cli.3md`
