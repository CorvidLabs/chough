## MODIFIED

### REQUIREMENT REQ-cli-061

Gate lists. `chough gate list [--json]` SHALL number every open human gate of every open session in the project from 1, with the stage it holds and the native step it drives (REQ-cli-030). It SHALL give each a one-screen summary of at most 24 lines and 4,096 bytes, with:
- what changed: the files, insertions and deletions since the change's base, counted in the working tree so that uncommitted and untracked files count, as a change's own files are before they are committed, leaving out `.chough/`, the knowledge directory and SpecSync lifecycle bookkeeping, and naming the five files with the most changed lines among the rest (#190). At a gate that drives approval, the change's own definition, the Markdown files under `.specsync/changes/<id>/` (its `change.md`, deltas, plan and other artifacts), is what the human approves, so the summary SHALL count those files and name them first, its deltas before the change's other files with the requirement IDs each delta adds and those it modifies, each said as such, never leaving them out as bookkeeping. At a gate held at intent, the summary SHALL give the intent being approved in place of what changed when the session records no native change: each criterion the session references, by ID with its sentence from the HI file that holds it, and no diff line, since there is no base. The base is the change's own `base_commit` (from its `state.json`), so an earlier change on the same branch is not counted again. The base is instead the merge base of HEAD with the default branch (`origin/HEAD`, else `main`) in two cases: when that merge base is not an ancestor of the `base_commit`, meaning default-branch work was merged or rebased in after the change began (#190), and when the change records no `base_commit`. A merge base that is HEAD itself, as on the default branch, is not used. With no usable base, the summary says the base is unknown;
- the tests: how many cases passed, failed and were not run, by the latest observation of each case that links a requirement the change's deltas add or modify, or that the session's observations name, with the failing case IDs;
- the risks: the first 8 lines of the change's `plan.md` section "Risks", or "none recorded";
- a diff link: the GitHub compare URL from the base to HEAD when `origin` is a GitHub remote, otherwise `git diff <base>..<HEAD>`.

Each summary's digest SHALL be the SHA-256 of its text. The list SHALL be saved as `.chough/local/gate-lists/<list>.json`, where `<list>` is the first 12 hex digits of the SHA-256 of the list. Listing SHALL make no native read and write no tracked file. The list SHALL end with how to reply: `chough gate record` for the gates whose change is not verifying, and `chough change ship <id> --list <list> --gate <n>` for each gate whose change is verifying.

`chough gate record <list> --gates <n>[,<n>...] --decided-by <who> --words <words> --where <where> --recorded-by <agent>` SHALL close each named gate as `chough checkpoint --close-gate` would with the four values (REQ-cli-050), and its relay SHALL carry that gate's `summary_digest` and the `list` (REQ-history-014). It SHALL close all the named gates or none. Before writing anything, it SHALL refuse:
- an unknown list, a number not on it, a listed gate that is no longer open, or a gate whose summary's digest has changed, each naming `chough gate list`;
- a gate whose change's `state.json` state is `verifying`, naming `chough change ship <id> --list <list> --gate <n>` with the relay, and giving as the reason that such a gate closes through the ship, which runs the review as the human and closes the gate after the native ship, never that Chough's records may not be written;
- more than one named gate that drives a native step, naming the gates to record one at a time.

A named gate that drives a native step SHALL run it as the decided human (REQ-records-008) before any gate is closed.

Acceptance Criteria
- `gate list` numbers each open gate with a summary naming what changed, the tests, the risks and a diff link, at most 24 lines, and the SHA-256 digest of its text; the list is saved under `.chough/local/gate-lists/` and no tracked file changes.
- One `gate record` reply naming two listed gates closes both, each relay keeping the same words and its own summary digest; a gate left off the reply stays open.
- An unknown list, a number not listed, a gate no longer open and a summary changed since listing are each refused with nothing written, and listing again then recording succeeds.
- A gate whose change is verifying is refused, naming `chough change ship` with `--list` and `--gate`, which then ships and closes it with its digest.
- Two named gates that both drive a native step are refused with nothing run or written; one such gate with gates that drive nothing runs its step once and closes all of them.
- #190: with a one-line product edit, 300 changed records under `.chough/` and `knowledge/`, and 200 files merged from main after the base, the summary counts one file and names it.
- For a second native change on one branch, the summary's what changed counts only that change's own files since its `base_commit`, not the earlier change's; with default-branch work merged in after it began, it counts from the merge base, as #190's criterion requires.
- At a scope gate, the summary's what changed counts and names the change's deltas and plan under `.specsync/changes/<id>/` first, while its `state.json` and `approvals.json` stay left out.
- With a verifying change's review gate open, `gate list` ends naming `chough change ship <id> --list <list> --gate <n>` for it and offers no `gate record` for it, and the `gate record` refusal gives the ship as its reason, with no claim that Chough's records may not be written.
- At a scope gate whose change has more than five Markdown files, the summary names the delta and the requirement IDs it adds or modifies before the other files.
- At an intent gate with no native change recorded, the summary names the session's criterion IDs and their HI files instead of an unknown base.
- With a change's delta and plan written but not committed, the scope gate's summary counts them and names the delta first, as it does after the commit.
- At an intent gate with no native change, the summary gives each referenced criterion's ID and sentence and no diff line; a delta that only adds says adds, and one that adds and modifies names each.
