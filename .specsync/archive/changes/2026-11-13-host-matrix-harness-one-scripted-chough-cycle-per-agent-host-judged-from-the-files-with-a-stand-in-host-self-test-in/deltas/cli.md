## ADDED

### REQUIREMENT REQ-cli-066

Host-matrix harness. The repository SHALL hold an end-to-end host-matrix harness under `tools/host-matrix/` (LIFE-21 item 4, LIFE-22, LIFE-24, LIFE-27).
- `sh tools/host-matrix/run.sh <host>` SHALL drive one fixed Chough cycle in a new Git copy of `examples/shipping` without its `patches/`, one fresh host run per turn: init and intent; plan; build, test and accept; done; and a cold resume that changes nothing and states stage `done` and `chough session new`.
- A host SHALL be an adapter: a shell file that defines `host_check`, `host_version` and `host_turn`, read from `integrations/<host>/adapter.sh` or from the file `--adapter` names. The harness SHALL NOT call hc, Herdr or Rune.
- Between turns the harness SHALL stand in for each human gate only as `SIMULATED-host-matrix-not-a-human`, with notes that say SIMULATED, and SHALL NOT record a decision in a person's name (FLOW-10, FLOW-16).
- Each host run SHALL run without the starting agent's or terminal's identity in its environment, with `CHOUGH_ACTOR` set to the host.
- The verdict SHALL come only from the sample's files, Git and native status: the checks after each turn, the harness's own acceptance test of the shipping fee, `chough doctor --leaks`, and a host-independent shape of the records compared with `tools/host-matrix/expected-shape.json`.
- It SHALL print one JSON result, PASS (exit 0), FAIL naming each failed check (exit 1) or UNAVAILABLE when the host cannot run (exit 2), with the host and its version, the platform, the chough version and commit, the command, the SHA-256 of the turn logs and the key failing lines. Raw logs SHALL stay outside the repository. A turn that runs past its time limit SHALL end with its whole process group and fail.
- `sh tools/host-matrix/selftest.sh` SHALL run the harness with the scripted stand-in host, which runs no model. It SHALL fail unless the good run is PASS, each known-bad run fails on the check that names its fault, and a host that cannot run reads UNAVAILABLE. The known-bad runs are uncommitted work, a skipped scope gate, an approval by the host itself, an extra product file, a wrong cold resume and a hung turn. Hosted CI SHALL run it on Ubuntu and macOS (FLOW-21).

Acceptance Criteria
- `sh tools/host-matrix/selftest.sh` prints `HOST MATRIX SELFTEST PASSED` on macOS and Linux.
- The `Host matrix self-test` CI job runs it on `ubuntu-latest` and `macos-latest`.
- Every stand-in gate decision, native approval and native review in the sample names `SIMULATED-host-matrix-not-a-human`; none names a person.
