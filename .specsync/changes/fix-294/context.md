---
change: fix-294
artifact: context
---

# Context

GitHub #294, a follow-up from #293 (the fix for #278, Grok's review finding 3). #293 changed the gate summary's change line in `src/gates.rs` to `Change: <id> - <title> (<state>)`, and REQ-cli-061's acceptance criteria say: "A gate's summary names its change as `Change: <id> - <title> (<state>)`, with no em dash (#278)." No test checks that separator: the case `TC-history-archived-stories-join-evidence-without-an-em-dash` lists `src/gates.rs` but binds a sessions test. If the em dash came back into `src/gates.rs`, every test would still pass.

This change adds the missing test in `tests/gates.rs`, beside the existing summary tests, and binds it to a case of its own. `src/gates.rs` is unchanged. Leif approved the scope on 2026-11-18, relayed by his planning session (hc t4c), with the A list (#268, #274, #294).
