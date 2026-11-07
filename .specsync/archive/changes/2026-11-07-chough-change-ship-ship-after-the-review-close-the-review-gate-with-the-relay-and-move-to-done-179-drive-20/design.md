---
change: chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20
artifact: design
---

# Design

- **`ChangeCommand::Ship`** takes `<id>` and the optional `--close-gate`, `--decided-by`, `--words`, `--where` and `--recorded-by`.
- **Pre-checks**, reads only, before any process starts:
  - The checkpoint's active session records native change `<id>`. If not, it refuses: "the active session's native change is <x>, not <id>; run `chough session use <id>` or `chough change ship <x>`".
  - The stage is accept. If not, it refuses naming `chough stage advance --to accept`.
  - Each open `awaiting_human` gate equals `--close-gate`. Otherwise it refuses, naming the full `chough change ship <id> --close-gate "<gate>" --decided-by … --recorded-by …` command.
  - With `--close-gate` given, the four values are checked by the function extracted from `checkpoint --close-gate`. That function takes the command a refusal tells you to run again. For `checkpoint`, its incomplete, gate-not-open and too-long refusals keep their exact text. For `ship`, they name `chough change ship <id> --close-gate …`, because closing the gate with `checkpoint` before ship would write `.chough/` inside the verification window.
- **Ship** runs `specsync change ship <id>` through `records::run_owned`: working directory `--root`, no deadline, under the stop watch. Its stdout and stderr bytes are copied through. A non-zero exit returns that code with nothing written.
- **After a successful ship**, Chough runs `std::env::current_exe()` with `--root <root> checkpoint --close-gate <gate> --decided-by … --recorded-by …` (only when a gate was named), then with `--root <root> stage advance --to done`, each through `run_owned`. Each child's stdout and stderr are copied through after SpecSync's. If a child exits non-zero, Chough exits 1. On stderr it says: "specsync change ship <id> succeeded, so the change is shipped; <step> was refused. Run `<that step>`, then `<any step after it>`."
- No other file changes behavior. `checkpoint --close-gate` keeps its output and refusals.
