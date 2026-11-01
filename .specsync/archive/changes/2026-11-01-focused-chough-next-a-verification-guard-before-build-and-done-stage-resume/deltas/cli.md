## ADDED

### REQUIREMENT REQ-cli-021

Focused next action. `chough next` SHALL print a compact action view, separate from the full `chough status` report: the objective; the current stage with its status, iteration and side-stage callers; the recorded native change with its read state, summary state, approval and correction validity, terminal evidence validity, native next action and approved-delta result; the next valid action with its reason and, when one applies, its command; the gate for the next main stage; awaiting-human gates; blockers; open acceptance-blocking bug IDs with the count of other open bugs; and only the referenced observations that are not current, with counts for the rest. It SHALL print text by default and the same fields as JSON with `--json`, and each form SHALL be at most 16384 bytes, with every list bounded, its total stated and any truncation marked. `chough next` and `chough status` SHALL derive the next valid action from the same rules and SHALL each make at most one native status read. In stage done, when the recorded native change is archived with authenticated terminal evidence or no native change is recorded, the next valid action SHALL recommend a new cycle, a stage reset that carries a reason followed by a new native change, rather than repeat the checkpoint's next action; a stage reset SHALL replace the checkpoint's next action. Trace: LIFE-3, FLOW-13, FLOW-2, LIFE-7.

Acceptance Criteria
- With 500 referenced stale observations, 40 awaiting-human gates and 40 blockers of 4 KiB each, and a native response near the parse bound, the text and JSON forms each stay within 16384 bytes, list at most 10 entries per list and state each total.
- The view lists referenced observations that are historical, pruned or unavailable and counts referenced current and unreferenced ones; it carries no raw native output and no observation record.
- `chough next` and `chough status` report the same next valid action for the same records, and each runs `specsync change status` at most once.
- In stage done with an archived, authenticated change, the next valid action recommends `chough stage reset` with `--reason` and `specsync change new`, and the suggested reset succeeds; after a reset the checkpoint's next action is the new-cycle text, and back leaves it unchanged.

### REQUIREMENT REQ-cli-022

Approved-delta guard. Before a move to build, test or accept on a native change that is approved, implementing or verifying with a valid definition approval, the helper SHALL also confirm that every affected semantic delta matches the digest recorded by the latest native definition approval, computed as SpecSync defines it, reading the native approval ledger and the deltas read-only within a bound. A changed, missing or unapproved delta SHALL refuse the move as delta-changed, and a comparison that cannot be made SHALL refuse it as delta-unverifiable; each refusal SHALL name the next native action. The guard SHALL start no process and SHALL NOT run native verification. Trace: FLOW-5, LIFE-7, LIFE-1.

Acceptance Criteria
- Digests computed for archived changes match the digests their approvals recorded.
- An edited delta, a module added after approval and a deleted delta refuse the move with `refused:native-delta-changed`; a missing, oversized or malformed ledger and an approval without delta digests refuse it with `refused:native-delta-unverifiable`; a refused move leaves the session and transitions byte-identical.
- A change whose deltas match, including after a line-ending-only edit, passes, and a no-spec-change approval with no deltas passes.
