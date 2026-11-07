---
change: flag-direct-specsync-lifecycle-writes-in-history-183-drive-21
artifact: context
---

# Context

Issue #183, slice 3 of "Chough drives SpecSync". DRIVE-21, captured from Leif's interview answer 7 ("Allowed but flagged", 2026-11-06), reads: "Agents may still call specsync directly, and it still works. Chough notes each direct lifecycle write in history, so the need for it can be seen and removed." Slices 1 and 2 (#174, #182) moved new, answer, check, status and ship behind `chough change`. Approve and review stay direct until slice 5 (spec-sync #900 in 6.3.0), so those are the writes this slice will show.

## Decisions that bind this change

- **The slice order.** Approved by Leif, relayed by the orc. When the orc assigned this slice it asked: "name the oldest Chough that reads the new event kind in the release note".
- **When findings are recorded.** Nothing may write `.chough/` between `change check --commit` and finalize. A finding is therefore kept in an untracked local file and recorded with the next successful stage move, which writes history anyway. `next` and `status` only show findings.
- **How Chough knows its own writes.** It must update a baseline after its own steps, and the baseline lives in `.chough/local/`, which Git ignores. REQ-cli-057 promised that the change steps write nothing under `.chough/`, so this change modifies it to allow that one untracked file. Its test then checks that no tracked file changes and that the baseline is the only local write.
- **Compatibility.** `Transition` uses `deny_unknown_fields`, and `validate_payload` refuses unknown kinds. Any Chough before this change, including 0.4.0, therefore refuses a history holding the new event. The change log names the first release that contains it (planned 0.5.0) as the oldest reader.
- **IDs.** REQ-cli-060 and REQ-history-013, announced to t27.
- **Ruled out:**
  - Recording from `next` or `status`: they stay read-only.
  - Recording during the verification window.
  - Watching authored artifacts (`context.md`, design and plan files, deltas): agents write those by hand on purpose.
  - Watching other changes than the active session's.
  - Refusing on a finding: DRIVE-21 says "it still works".
- **Granularity.** Several direct writes between two Chough commands show as one finding listing each changed file. Chough sees the records' state at each of its commands, not each SpecSync call.
