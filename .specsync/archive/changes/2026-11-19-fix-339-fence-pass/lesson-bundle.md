# Lesson bundle: fix-339-fence-pass

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 339 fence pass
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/records.rs, tests/records.rs, changelog.d/2026-11-18-fix-339.md, .chough/cases/records.conflict-marker-follow-ups.cases.toml
- **Acceptance**: With a session body holding many code fence openers that never close, a record write and chough doctor finish in time that grows linearly with the record, not quadratically: each line is visited a bounded number of times. A conflict after those openers is still refused, since an unclosed fence hides nothing, and closed fences, Chough's own sections and the advisory findings behave as before.

## Evidence

- Verification commit: `163d7eac2868fb21ced1c407cc13210c1de88cef`
- Base commit: `082f1734397527910603c96624c5faf58f34bae0`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

A follow-up to fix-339 (#356), from Leif's P2 review comment on #356: when a code fence opener has no later matching close, `fenced` walked every remaining line looking for one, and the outer loop repeated that walk for the next opener. Repeated unclosed fence openers therefore made the scan O(n²) with no conflict at all. His measurement: the optimized CLI took 1.61 s to checkpoint a 128,732-byte session with 16,000 openers, and more than 2.0 s at 256,732 bytes with 32,000, far below the 1 MiB session bound. Every record write and doctor's two scans repeat the cost.

The standing rule Leif gave for 2026-11-18, "Yes, for today", covers a real gap his review finds in an already-approved PR: a small follow-up change inside the approved scope, a tester's glance, then hc t4c resolves the thread.

## From the change's design.md

# Design

`fenced` keeps its rule and changes how it finds each opener's close. One right-to-left pass answers, for every opener line, which later line closes it: the first later line that closes that fence character with a run at least the opener's length, before the next heading of Chough's own sections. For each fence character it keeps a stack of candidate closes, nearest first, whose run lengths grow with depth: a close is dropped once a nearer one is at least as long, a section heading of Chough's empties the stacks, and an opener's close is found by a binary search on the stack. Each line is pushed and popped at most once, so the pass is O(n log n). The forward pass then walks the lines as before, and each opener looks its close up instead of searching for it. An opener with no close still hides nothing.

## From the change's testing.md

# Testing

`tests/records.rs::many_unclosed_fence_openers_are_scanned_in_bounded_time`, red on fix-339 before this change: a session whose Goal holds 40,000 unclosed fence openers saves within a generous time bound, and with a conflict after them the next write is still refused, naming the conflict's line. The existing fence, section and advisory tests still pass unchanged.

## Where these lessons go

- `specs/cli/cli.3md`
