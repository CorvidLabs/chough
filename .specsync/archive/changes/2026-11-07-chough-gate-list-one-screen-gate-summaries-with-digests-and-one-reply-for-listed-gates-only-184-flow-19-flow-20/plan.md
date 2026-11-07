---
change: chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20
artifact: plan
---

# Plan

## Approach

Add `src/gates.rs` with `gate list` and `gate record`. It builds the summary from files and Git, and saves the list under `.chough/local/gate-lists/`. `gate record` closes the gates through the same path as `checkpoint --close-gate`, adding the summary digest and the list ID to each relay. `chough change ship` resolves `--list` and `--gate` to the gate's text and checks the digest before it runs anything.

## Out of scope

- Native approval and review records (slice 5).
- Lists across repositories.
- Push notifications.
- Changing which gates exist.

## Steps

1. Add the tests in `tests/gates.rs`, and the ship-with-list test in `tests/change.rs`. They fail first.
2. Add `src/gates.rs` and the `gate` command (`src/main.rs`). Add the optional relay fields and their validation (`src/stage.rs`, `src/history.rs`), and the close-with-digest path (`src/checkpoint.rs`). Add `--list` and `--gate` to ship (`src/change.rs`).
3. Update `docs/cli.md`, the README row, the accept and lifecycle references (both copies) and the CHANGELOG, naming the oldest reader.
4. Materialize, record the cases, run a mutation round, and run `fledge run linux-verify`.

## Risks

- A summary is only as good as its inputs. A change with no `base_commit`, no `plan.md` Risks section or no observations shows "unknown" or "none recorded" for that part.
- The diff link is a GitHub compare URL only when `origin` is GitHub. Otherwise it is a `git diff` command.
- Once a relay with a summary digest is recorded, older Chough can't read the history. The change log says so.
- Summaries change whenever HEAD moves. A reply to an old list is refused as stale, and the agent lists again.

## Constraints consulted

- FLOW-19 and FLOW-20.
- FLOW-16 as amended, and FLOW-10: only the relayed decision given is recorded.
- DRIVE-2: no native read.
- DRIVE-4: no write in the verification window.
- DRIVE-12: append-only events.
- Leif's rule: each refusal names a recovery, and a test runs it.
