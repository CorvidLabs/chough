---
change: chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20
artifact: testing
---

# Testing

New tests in `tests/gates.rs`. The fixture is a Git project with a session for change `g1`. The session's `state.json` holds a title and a `base_commit`. A delta adds REQ-cli-001, `plan.md` has a Risks section, and a passing and a failing observation exist for two cases that link REQ-cli-001; nothing links them to the session. Gates are open. One more test covers a listed gate of another open session.

- `gate_list_shows_each_open_gate_with_a_one_screen_summary_and_its_digest`: the numbers; the summary's parts (changed files, tests with the failing case, risks, the diff command); at most 24 lines; the digest is the SHA-256 of the text; the list file exists; no tracked file changes.
- `one_reply_closes_only_the_listed_gates_each_with_its_own_digest`: with three gates open, recording numbers 1 and 3 closes both with the same words and their own digests; gate 2 stays open.
- `gate_record_refuses_unknown_unlisted_closed_or_changed_gates_with_nothing_written`: an unknown list, a number not listed, a gate closed since listing, and a commit since listing are each refused, naming `chough gate list`, with nothing written. Listing again and recording then succeeds.
- `a_verifying_gate_is_refused_and_closes_through_ship_with_the_list`: the refusal names `chough change ship g1 --list`; ship with `--list` and `--gate` then reaches done, with the digest in the relay.
- `relay_summary_fields_are_both_or_neither`: a legacy-log relay with both fields reads back; one alone, or a malformed value, is an invalid transition.
- `the_change_log_names_the_oldest_chough_that_reads_summary_digests`.

They fail first because no `gate` command or relay fields exist.

Mutation round, each mutant failing at least one new test:
- skip the digest check;
- skip the listed-number check;
- close every open gate instead of the listed ones;
- drop `summary_digest` from the relay;
- allow a verifying gate;
- accept one relay field alone.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-061 | tests/gates.rs |
| REQ-history-014 | tests/gates.rs |
| REQ-cli-059 (modified) | tests/gates.rs (ship with `--list`) |

## Lanes

`fledge run linux-verify` passes. Before it, the local spec check runs with `--require-coverage 100`.
