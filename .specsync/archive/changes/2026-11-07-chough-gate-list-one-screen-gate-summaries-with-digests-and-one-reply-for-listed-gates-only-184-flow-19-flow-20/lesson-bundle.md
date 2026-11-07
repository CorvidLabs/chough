# Lesson bundle: chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Chough gate list: one-screen gate summaries with digests, and one reply for listed gates only (184, FLOW-19, FLOW-20)
- **Kind**: Feature
- **Specs**: cli, history
- **Paths**: src/gates.rs, src/main.rs, src/change.rs, src/checkpoint.rs, src/stage.rs, src/history.rs, src/skill.rs, tests/gates.rs, tests/change.rs, tests/package.rs, tests/docs.rs, docs/cli.md, README.md, CHANGELOG.md, skills/chough-workflow/references/accept.md, skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/accept.md, .agents/skills/chough-workflow/references/lifecycle.md
- **Acceptance**: REQ-cli-061 and REQ-history-014 (new), REQ-cli-059 (modified): chough gate list numbers every open human gate with a one-screen summary (what changed since the change's base, the tests, the risks, a diff link) and its SHA-256 digest, saved untracked, with no native read; chough gate record closes only listed gates with one relay, each keeping its own summary digest and the list, all or none, refusing an unknown list, an unlisted number, a closed gate, a changed summary, or a verifying change (each naming a recovery a test runs); chough change ship takes --list and --gate for the review gate. The change log names the oldest Chough that reads summary digests. Six tests in tests/gates.rs fail first, each mutant in testing.md fails one, and fledge run linux-verify passes.

## Evidence

- Verification commit: `882e4f6bc0337f3d2a36c20031fdf6b7341d1933`
- Base commit: `0a20d934a08a5c92a14ca6b0016dfb520daa0815`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

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

## From the change's design.md

# Design

- **`chough gate list [--json]`** works over every open session (`.chough/sessions/*.json`, or the legacy checkpoint) and each of its `awaiting_human` gates, in session-key order then gate order. They are numbered 1..N.
- **Summary text** has at most 24 lines and 4,096 bytes, cut with a "… (cut)" line if longer. Its lines are:
  - `Gate: <gate>`
  - `Change: <id> — <title> (<state>)`
  - `What changed: <files> files, +<ins> −<del> since <base8>`, then up to five `  <path> +a −d` lines (from `git diff --numstat <base>..HEAD`);
  - `Tests: <p> passed, <f> failed, <n> not run`, plus `Failing: <ids>`. The cases are those in `.chough/cases/*.cases.toml` whose `requirements` include a `REQUIREMENT` heading in the change's `deltas/*.md`, plus any case the session's observations name. Each one's latest observation comes from `.chough/evidence/`, by `observed_at_unix_seconds`.
  - `Risks:` and the first 8 lines of `plan.md`'s Risks section, or `none recorded`;
  - `Diff: <url or command>`.

  The digest is the SHA-256 of the text.
- **List file.** `.chough/local/gate-lists/<list>.json` holds `{schema_version: 1, head, gates: [{number, session, change, gate, digest, summary}]}`. `<list>` is the first 12 hex digits of its SHA-256. It is written only once `.chough/local/` is ignored, as the direct-write baseline is.
- **`chough gate record <list> --gates 1,3 …`** checks everything first. The list must exist. Each number must be on it. Each gate must still be open in its session. Each summary rebuilt now must have the listed digest. No gate's change may be verifying. The relay values are checked by `checkpoint::check_relay`. Then it closes each gate as `checkpoint --close-gate` does, through the same save, with `relayed_by` carrying `summary_digest` and `list`.
- **Relay fields.** `RelayedBy` gains `summary_digest` and `list`, each `Option`, skipped when `None`. Validation accepts both or neither: `summary_digest` is 64 lowercase hex digits and `list` is 12.
- **Ship with a list.** `chough change ship <id> --list <l> --gate <n> --decided-by …` resolves the gate's text from the list and checks its digest before anything runs. The follow-up close then passes the digest and list through the hidden `checkpoint --close-gate … --summary-digest <d> --list <l>`, so the record is the same as `gate record`'s.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
