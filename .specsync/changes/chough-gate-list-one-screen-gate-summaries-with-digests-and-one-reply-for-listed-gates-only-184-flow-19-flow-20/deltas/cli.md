## ADDED

### REQUIREMENT REQ-cli-061

Gate lists. `chough gate list [--json]` SHALL number every open human gate of every open session in the project from 1. It SHALL give each a one-screen summary of at most 24 lines and 4,096 bytes, with:
- what changed: the files, insertions and deletions since the change's `base_commit` (from its `state.json`), naming the five files with the most changed lines;
- the tests: how many cases passed, failed and were not run, by the latest observation of each case that the session's observations name, with the failing case IDs;
- the risks: the first 8 lines of the change's `plan.md` section "Risks", or "none recorded";
- a diff link: the GitHub compare URL from the base to HEAD when `origin` is a GitHub remote, otherwise `git diff <base>..<HEAD>`.

Each summary's digest SHALL be the SHA-256 of its text. The list SHALL be saved as `.chough/local/gate-lists/<list>.json`, where `<list>` is the first 12 hex digits of the SHA-256 of the list. Listing SHALL make no native read and write no tracked file.

`chough gate record <list> --gates <n>[,<n>...] --decided-by <who> --words <words> --where <where> --recorded-by <agent>` SHALL close each named gate as `chough checkpoint --close-gate` would with the four values (REQ-cli-050), and its relay SHALL carry that gate's `summary_digest` and the `list` (REQ-history-014). It SHALL close all the named gates or none. Before writing anything, it SHALL refuse:
- an unknown list, a number not on it, a listed gate that is no longer open, or a gate whose summary's digest has changed, each naming `chough gate list`;
- a gate whose change's `state.json` state is `verifying`, naming `chough change ship <id> --list <list> --gate <n>` with the relay.

Acceptance Criteria
- `gate list` numbers each open gate with a summary naming what changed, the tests, the risks and a diff link, at most 24 lines, and the SHA-256 digest of its text; the list is saved under `.chough/local/gate-lists/` and no tracked file changes.
- One `gate record` reply naming two listed gates closes both, each relay keeping the same words and its own summary digest; a gate left off the reply stays open.
- An unknown list, a number not listed, a gate no longer open and a summary changed since listing are each refused with nothing written, and listing again then recording succeeds.
- A gate whose change is verifying is refused, naming `chough change ship` with `--list` and `--gate`, which then ships and closes it with its digest.

## MODIFIED

### REQUIREMENT REQ-cli-059

Ship through Chough. `chough change ship <id> [--close-gate <gate> --decided-by <who> --words <words> --where <where> --recorded-by <agent>]` SHALL, before running anything and with nothing written, refuse unless the active session's native change is `<id>` and its stage is accept, and unless every open human gate is the one `--close-gate` names. The relay values SHALL be checked as `chough checkpoint --close-gate` checks them (REQ-cli-050). `--list <list> --gate <n>` MAY stand in for `--close-gate <gate>`: the gate SHALL be number `<n>` of that `chough gate list` list (REQ-cli-061), and its summary SHALL still have the listed digest, or the ship SHALL refuse before anything runs, naming `chough gate list`. The closed gate's relay SHALL then carry the summary digest and the list (REQ-history-014). Each refusal SHALL name the command that recovers. It SHALL then run `specsync change ship <id>` in the project root with no time bound, and pass SpecSync's standard output and standard error through unchanged. When SpecSync exits non-zero, it SHALL exit with that code and write nothing. When SpecSync succeeds, it SHALL close the named gate as `chough checkpoint --close-gate` would, a relayed claim, and then move the session to done as `chough stage advance --to done` would, printing each step's report after SpecSync's output. When either step refuses after a successful ship, it SHALL exit 1, say that the change was shipped, and name each remaining command. Stopping Chough with SIGINT, SIGTERM or SIGHUP SHALL stop the SpecSync process it started (FLOW-14).

Acceptance Criteria
- With the review gate's relay, `chough change ship` runs `specsync change ship <id>` once, closes the gate with the four values as a relayed claim, and moves the session to done.
- A SpecSync refusal's standard output, standard error and exit code pass through, and nothing under `.chough/` changes.
- An open human gate without its relay, a wrong or incomplete relay, another change or a stage other than accept are each refused before SpecSync runs, with nothing written, and the command each names succeeds.
- When the move into done is refused after a successful ship, the exit is 1, the output says the change was shipped and names `chough stage advance --to done`, and that command then succeeds.
- `chough change ship` completes after the native read bound has passed.
- With `--list` and `--gate` for a listed review gate, the ship closes that gate with the summary digest and the list in its relay, and a summary changed since it was listed is refused before SpecSync runs, naming `chough gate list`.
- The walkthrough and the skill's closeout run `chough change new`, `chough change check` and `chough change ship`; review stays `specsync change review`.
