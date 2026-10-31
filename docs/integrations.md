# Integration map

Chough's first slice integrates native HI export and SpecSync status and uses Fledge
for its own project tasks. The workflow skill can guide the remaining native tools.
The optional adapters below are design seams, not implemented capabilities.

| Tool | Native ownership | Chough boundary |
|---|---|---|
| [HI](https://github.com/CorvidLabs/hi/blob/main/README.md) | Human sentences, permanent IDs, scoped JSON export, structural checks. | Use exported intent; derive FRD downstream. HI does not bind evidence or track achievement. |
| [SpecSync](https://github.com/CorvidLabs/spec-sync/blob/main/README.md) | Requirements, module contracts, companions, changes, approvals, verification, review, archives, lessons. | Surface native status and relevant sources. Structural validation is separate from product testing. |
| [Fledge](https://github.com/CorvidLabs/fledge/blob/main/README.md) | Task lanes, work/PR/release flows, plugins, execution adapters. | Prefer the supported task surface; do not recreate its task runner. |
| [Rune](https://github.com/CorvidLabs/rune/blob/main/README.md) | Ruby PTY runner, live watch, structured output, named project sessions, send-and-settle, attach. | Future optional terminal/session adapter. A settled process is not proof a task succeeded. |
| [Trust](https://github.com/CorvidLabs/trust/blob/main/README.md) | Composes Fledge, SpecSync, Augur, and Attest through native local/CI policy. | Future gate adapter; reuse the composition rather than duplicate gate orchestration. |
| [Augur](https://github.com/CorvidLabs/augur/blob/main/docs/cli.md) | Heuristic risk analysis and policy enforcement. | Future risk input; retain verdict and command semantics. It does not prove product behavior. |
| [Attest](https://github.com/CorvidLabs/attest/blob/main/README.md) | Assertions at exact commits, signing and verification. | Future provenance input; assertions do not execute tests or establish human authorization. |
| [Atlas](https://github.com/CorvidLabs/fledge-plugin-atlas/blob/main/README.md) | Read-only context and gap exploration. | Future context source; current SpecSync compatibility remains to be validated. |

## Limits adapters must preserve

- **Trust:** local `verify` does not produce a JSON report; `status` and `doctor`
  do. Its Action distinguishes passed, degraded, and failed layers and overall
  results. Standard mode can succeed with degraded provenance. Baseline scope
  checks the protected base, not the proposed commit's provenance.
- **Augur:** `check` can emit JSON but exits zero regardless of verdict. Use the
  native `gate` command for enforcement. An adapter must not interpret exit zero
  from `check` as an approved risk verdict.
- **Attest:** a signed record proves possession of the signing key. It does not
  prove that the signer was a human or that the human authorized the action. Never
  fill `humanApproved` from an agent's inference. Signer pinning and policy choices
  need a deliberate project decision.
- **Rune:** `run` buffers one invocation, `watch` supports live human passthrough
  and NDJSON events, and `session` retains a named agent CLI across invocations.
  Per-send settled output and rendered screen help observe a process; task success
  still needs explicit verification. Human attach and bounded logs matter before
  adding session coordination.
- **Atlas:** legacy Fledge spec documentation makes compatibility a question to
  answer with a real fixture, not an assumption.

None of these replaces bug reproduction, product acceptance testing, a genuine
human decision, or an observation from the promised delivery environment.
