---
change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
artifact: design
---

# Design

Nothing visual or interactive changes. The harness is a set of shell scripts:

- `run.sh` drives one host through five turns and judges them;
- `shape.sh` reduces a finished sample to a host-independent shape;
- `selftest.sh` runs the stand-in host correctly and with each fault;
- `stand-in/adapter.sh` and `stand-in/agent.sh` are the scripted host.

An adapter is a sourced shell file that defines `host_check`, `host_version` and
`host_turn <prompt> <log> <reply>`. It may also set `HOST_LABEL`, `HOST_INIT_ARGS`,
`HOST_SKILL`, `HOST_SKILL_READ` and `HOST_TURN_TIMEOUT`. The result is one JSON object:
the verdict, each failed check and a receipt. The receipt holds the host and its
version, the platform, the chough version and commit, the command, the SHA-256 of the
turn logs and the key failing lines. Raw logs stay in a temporary directory outside
the repository.
