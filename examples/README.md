# Examples

Disposable samples for exercising the Chough workflow with the real tools. Their
human intent is sample input written for the exercises, and any human gate the
scripts pass is labelled SIMULATED. Nothing is pushed, posted, released or deployed.

| Sample | Script | Shows |
|---|---|---|
| [shipping](shipping/) | [feature-bug-loop.sh](feature-bug-loop.sh) | Design skip, native scope gate, focused cases, a failing assertion, a local blocking bug, fix and retest, acceptance left to a human; `prepare-cold-start` mode for resuming |
| [checkout-ui](checkout-ui/) | [visual-design.sh](visual-design.sh) | One canonical HTML/CSS prototype on CorvidLabs tokens, rendered checks with assertions, agent-assisted verdicts, design approval left pending |

Each script copies its sample into an empty directory (a new temporary one by
default) and prints where it ran. See [docs/exercises](../docs/exercises/) for
observed results.
