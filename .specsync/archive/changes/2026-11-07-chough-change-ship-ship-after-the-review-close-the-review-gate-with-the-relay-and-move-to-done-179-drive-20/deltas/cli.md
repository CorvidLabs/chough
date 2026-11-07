## ADDED

### REQUIREMENT REQ-cli-059

Ship through Chough. `chough change ship <id> [--close-gate <gate> --decided-by <who> --words <words> --where <where> --recorded-by <agent>]` SHALL, before running anything and with nothing written, refuse unless the active session's native change is `<id>` and its stage is accept, and unless every open human gate is the one `--close-gate` names. The relay values SHALL be checked as `chough checkpoint --close-gate` checks them (REQ-cli-050). Each refusal SHALL name the command that recovers. It SHALL then run `specsync change ship <id>` in the project root with no time bound, and pass SpecSync's standard output and standard error through unchanged. When SpecSync exits non-zero, it SHALL exit with that code and write nothing. When SpecSync succeeds, it SHALL close the named gate as `chough checkpoint --close-gate` would, a relayed claim, and then move the session to done as `chough stage advance --to done` would, printing each step's report after SpecSync's output. When either step refuses after a successful ship, it SHALL exit 1, say that the change was shipped, and name each remaining command. Stopping Chough with SIGINT, SIGTERM or SIGHUP SHALL stop the SpecSync process it started (FLOW-14).

Acceptance Criteria
- With the review gate's relay, `chough change ship` runs `specsync change ship <id>` once, closes the gate with the four values as a relayed claim, and moves the session to done.
- A SpecSync refusal's standard output, standard error and exit code pass through, and nothing under `.chough/` changes.
- An open human gate without its relay, a wrong or incomplete relay, another change or a stage other than accept are each refused before SpecSync runs, with nothing written, and the command each names succeeds.
- When the move into done is refused after a successful ship, the exit is 1, the output says the change was shipped and names `chough stage advance --to done`, and that command then succeeds.
- `chough change ship` completes after the native read bound has passed.
- The walkthrough and the skill's closeout run `chough change new`, `chough change check` and `chough change ship`; review stays `specsync change review`.
