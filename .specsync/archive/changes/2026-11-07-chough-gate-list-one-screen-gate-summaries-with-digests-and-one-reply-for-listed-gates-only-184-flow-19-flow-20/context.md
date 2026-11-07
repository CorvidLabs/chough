---
change: chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20
artifact: context
---

# Context

Issue #184, slice 4 of "Chough drives SpecSync". The orc relayed Leif's interview answers 2 and 3 on 2026-11-06, and they were captured as FLOW-19 and FLOW-20:

- FLOW-19: "Chough first shows me a one-screen summary of what I'm deciding: what changed, the tests, the risks and a diff link … tied to that summary's digest".
- FLOW-20: "One reply of mine can decide several gates only when Chough listed each of them to me. It covers only the gates on that list, and each record keeps my words and its own summary digest".

Answer 6 adds: the orc gathers every open gate into one numbered list, and Leif replies by number.

This slice builds the list, the summaries and the one-reply record in Chough's own gate records. Writing the same reply into SpecSync's native approval and review records is slice 5, after SpecSync 6.3.0 ships spec-sync #900.

## Decisions that bind this change

- **The slice order.** Approved by Leif, relayed by the orc. Slices 1 to 3 (#174, #182, #186) are on main or in review.
- **No native read.** The summary is built from files and Git only: the change's `state.json` for its title, state and `base_commit`; `plan.md` for its Risks section; the session's observations; and `git diff`.
- **The list is untracked.** It lives in `.chough/local/gate-lists/`. The record that matters, the closed gate's relay, keeps the summary digest and the list ID.
- **The review gate closes through ship.** Nothing may write `.chough/` between check and finalize. So a gate whose change is verifying is refused by `gate record`, and `chough change ship` takes `--list` and `--gate` instead (REQ-cli-059, modified).
- **Compatibility.** `RelayedBy` uses `deny_unknown_fields`, so a relay with `summary_digest` and `list` is unreadable to older Chough. The change log names the first release with it as the oldest reader.
- **IDs.** REQ-cli-061 and REQ-history-014; t27 confirmed it holds none.
- **Redefined 2026-11-07 (Leif's option A, send-back recorded as 0xLeif by the orc).** The build found that nothing fills a session's observations: `chough cases run` does not link them, and real sessions show 0. As first defined, the tests line would have read "0 passed, 0 failed, 0 not run" while tests existed. The line now counts the cases that link a requirement the change's deltas add or modify, plus any the session's observations name. Everything else is unchanged.
- **Ruled out:**
  - native records (slice 5);
  - a list across repositories, which is the orc's job;
  - pushing to Moshi;
  - storing summaries in tracked files;
  - recording a decision without the four relay values.
