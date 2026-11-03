---
change: bound-transition-history-and-preserve-referenced-proof
artifact: research
---

# Research

## Observed baseline

Read-only inventory at 825a380: 785 tracked .chough files, 8,030,046 bytes; 323 observation files, 424 artifact files; transitions.jsonl is 41,596 bytes and 130 lines. These are file sizes, not evidence of excessive disk usage or runtime cost.

src/stage.rs appends schema-1 transition lines to one .chough/transitions.jsonl. Recent reporting and reached_by_reset read that shared file. src/timing.rs reads up to 16 MiB and derives one sequence of cycles and gates. The current-cycle checkpoint merge is coherent, but it does not identify which transition branch belongs to the chosen checkpoint. A same-time sort alone cannot resolve that ambiguity.

src/evidence.rs already protects checkpoint and bug references, active/latest native manifests, non-routine observations, newest three routine runs, newest failures, validation boundaries and sets younger than 14 days. It previews exact candidate files and bytes. Apply rebuilds classification, records a ledger entry first and then removes files. This change preserves those rules and makes changed/missing inputs and interrupted deletion explicit; it does not lower their protection thresholds.

## Alternatives considered by the agent

- Keep the shared log and add rotation: fewer bytes per file, but normal branch appends still edit the same path and a combined timeline still lacks lineage. Rejected for the proposed new format.
- One database, hosted coordinator or separate writable history-head file: introduces another authority or runtime. Rejected.
- Rewrite or migrate historical log bytes into the new format: needlessly changes proof and risks duplicate attribution. Rejected; retain a digest-bound legacy anchor.
- Sort independent events only by timestamps or random IDs: clock ties and branch forks would manufacture a sequential story. Rejected; use parent links and explicit current-checkpoint selection.
- Chosen proposal: immutable event files and a cursor in the existing checkpoint, with explicit adoption and whole-cycle retention. This is an agent recommendation pending scope approval.

## Backlog reconciliation

| Item | Delivered work | Remaining tracking boundary |
|---|---|---|
| #30 | PR #33 supplies slug cases and random OBS/VAL/SNAP IDs; independent-root regressions exist | Keep open pending criterion-by-criterion integration evidence; this pass does not claim the issue's real Git and controlled same-clock acceptance has been established |
| #32 | PR #34 deduplicates timing reads, skips excluded native history, bounds total native wait and adds tracing | Focused next/status latency, full cold/warm measurements and any upstream native diagnosis remain outside that slice |
| #31 | Reference-aware evidence pruning already exists; PR #29 handles checkpoint merging | New transition storage, lineage-aware consumers and retention integration are proposed here |
| Roadmap | PRs #24, #33, #34, #35 and #36 merged | Replace stale in-progress wording while retaining failed/not-run proof limits |

Tool paths are /Users/leif/.cargo/bin/{hi,fledge,specsync}. Installed versions are HI 0.8.0, Fledge 1.8.0 and SpecSync 6.1.2. GitHub latest stable release queries matched these pins. No tool upgrade was performed.
