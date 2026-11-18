---
change: fix-274
artifact: context
---

# Context

GitHub #274, found by t37f while fixing #247 (#258). `examples/feature-bug-loop.sh`, the verify lane's feature-and-bug exercise, runs under `set -eu` without `pipefail`, and pipes the JSON of `chough` and `specsync` commands straight into `jq`. A pipeline's exit status is its last command's, so when such a command is refused, `jq` reads nothing, exits 0 and the script carries on. The failure surfaces later, at a step that reads the result. CI run 43357893936 (#247) failed this way: the refused `chough bug update … --status verified` was reported one step late.

`set -o pipefail` is not available in every `sh` (Debian's `dash` lacks it), so the fix captures each such command's output first, where `set -e` stops the script on a failure with the command's own error, and pipes the captured text to `jq`.

Leif approved the scope on 2026-11-18, relayed by his planning session (hc t4c), with the A list (#268, #274, #294). No requirement covers how the example fails, so no requirement text changes.
