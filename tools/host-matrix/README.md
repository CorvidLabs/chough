# Host-matrix harness

One scripted Chough cycle per agent host, judged from the files it leaves behind
(REQ-cli-066). It serves bar item 4 of the 1.0 bar (LIFE-21): before each release, the
cycle passes on every supported host, on macOS and Linux. Each host runs as plain
command-line runs, one per turn, and needs no Herdr (LIFE-27).

```sh
fledge run host-matrix-selftest          # the stand-in host, no model: proves the harness
fledge run host-matrix -- <host>         # one real host, through integrations/<host>/adapter.sh
sh tools/host-matrix/run.sh <host> --adapter <file>   # any CLI agent, from your own adapter
```

`CHOUGH` names the chough under test; the Fledge tasks build this checkout's and use
it. The harness also needs `hi`, `specsync`, `cargo`, `git` and `jq` on `PATH`.

## The cycle

`run.sh` copies `examples/shipping` into a new, empty Git repository and removes its
`patches/`, which hold the solution. It then runs five turns. Each turn is a fresh
run of the host, given the fixed prompt in `prompts/` (`common.md` plus
`turn-<n>.md`):

| Turn | The host | Then the harness checks, and stands in for the human |
|---|---|---|
| 1 | runs `chough init`, captures the intent and stops at its gate | an open session at intent, an intent gate, the skill installed and read; clears the gate with a SIMULATED decision |
| 2 | plans the native change and stops at the scope gate | stage plan, the change in draft, a gate that drives approval; approves the change natively as `SIMULATED-host-matrix-not-a-human` |
| 3 | builds, runs the focused cases and stops at the review gate | stage accept, the change verifying, a gate that drives review, only `src/lib.rs` and `tests/threshold.rs` changed, and the harness's own acceptance test passes; reviews and ships the change as the SIMULATED actor |
| 4 | finishes the session | no open session, one archived story, the change archived |
| 5 | a cold resume: changes nothing and says where the work stands | the same commit as before, the skill read, `STAGE: done`, and `NEXT` naming `chough session new` |

The host commits its own work and ends every reply with `STAGE:` and `NEXT:` lines; each
turn also checks that the tree is committed. The harness never records a decision in a
person's name: every stand-in decision says SIMULATED (FLOW-10, FLOW-16).

`acceptance.rs` is never compiled as part of Chough. After turn 3, `run.sh` copies it
into a clone of the sample as `tests/host_matrix_acceptance.rs` and runs it there with
`cargo test`, so the host's build is judged by a test the host did not write.

## The verdict

The verdict comes only from the sample's files, Git and native status, never from what
the host says it did. After turn 5, `shape.sh` reduces the records to a shape that leaves
out IDs, times, digests, agent names and wording: the stages reached in order, the gates
opened and closed, decisions in a person's name, the native state with who approved and
reviewed it, the cases passed and the product files changed. `run.sh` compares that shape
with `expected-shape.json`, so every host must leave the same records. It also runs
`chough doctor --leaks`.

`run.sh` prints one JSON result and exits with its code:

- `PASS` (0);
- `FAIL` (1), listing each failed check;
- `UNAVAILABLE` (2), when the host cannot run here. That still counts as a missing leg.

Exit 3 is a harness error, such as a missing tool or a bad argument. The result is the
receipt: the host and its version, the platform, the chough version and commit, the
command, the SHA-256 of the turn logs and the key failing lines. The raw logs stay in
`--logs` (by default a new temporary directory), never in the repository. A turn that
runs past its time limit fails, and its whole process group is ended.

Each turn runs without the starting agent's or terminal's identity in its environment
(`CLAUDECODE`, the `CLAUDE_CODE_*` session variables, `HC_*`). `CHOUGH_ACTOR` names the
host, so the sample's records say which host wrote them.

## Adapters

An adapter is a shell file that `run.sh` sources. The real hosts' adapters will live in
`integrations/<host>/adapter.sh`, beside each host's `hc-template.md`. It defines:

| Name | Meaning |
|---|---|
| `host_check` | exit 0 when the host can run here; otherwise print why (the leg reads UNAVAILABLE) |
| `host_version` | one line naming the host and its version |
| `host_turn <prompt> <log> <reply>` | one fresh run in the current directory: read the prompt file, write the full output to the log and the final reply to the reply file |
| `HOST_LABEL` | the name records and results use (default: the host argument) |
| `HOST_INIT_ARGS` | the `chough init` host flags (default: `--host openai`) |
| `HOST_SKILL` | where the skill copy must be after turn 1 |
| `HOST_SKILL_READ` | an extended regular expression the turn log must match to show the skill was read |
| `HOST_TURN_TIMEOUT` | seconds per turn (default: 1800) |

The harness never calls hc, Herdr or Rune (LIFE-24). Hosts run without permission
prompts, but only inside the throwaway folder.

## The self-test

`selftest.sh` runs the harness with the scripted stand-in host in `stand-in/`. The
stand-in runs no model: it takes each turn from the project's own records and copies the
sample's patches. The self-test passes only when every one of these holds:

- the good run is PASS;
- each known-bad run (`HOST_MATRIX_FAULT`) fails on the check that names it:
  - uncommitted work;
  - a skipped scope gate;
  - an approval by the host itself;
  - an extra product file;
  - a wrong cold resume;
  - a hung turn;
- a host that cannot run reads UNAVAILABLE.

Hosted CI runs it on Ubuntu and macOS ([docs/ci.md](../../docs/ci.md)).
