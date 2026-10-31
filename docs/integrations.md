# Integration map

Chough's first slice integrates native HI export and SpecSync status and uses Fledge
for its own project tasks. The workflow skill can guide the remaining native tools.
The other tools below are guidance or adapter candidates, not implemented helper
capabilities. The agreed [lifecycle design](lifecycle.md) does not make every adapter
a mandatory dependency or replace the current small Rust helper.

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

## Planned testing, storage, design and delivery seams

| Tool | Candidate role and limit |
|---|---|
| [Magpie](https://github.com/CorvidLabs/magpie) | Real-command targets/artifacts. Generic external-repo mode judges command exit status; project frameworks must supply actual product assertions. No automatic arbitrary AC interpretation. |
| [agent.3md](https://github.com/CorvidLabs/agent-3md) / [3md](https://github.com/CorvidLabs/3md) | Progressive skill routing, typed inputs/optional command binding and layered portable records. A small versioned Chough test profile is a preferred candidate, not adopted. Unknown profile metadata is not a stock test oracle; case IDs must survive plane ordering. |
| [APS](https://github.com/0xLeif/aps-cli) | Candidate for typed project-local FileState checkpoints/preferences/notes. Qualify migration and one authority for the existing session; one writer per key or isolated agent roots/keys. Not a distributed lock, Rust rewrite or second writable checkpoint store. macOS/Linux release binaries exist; Windows builds/tests from source. |
| [CorvidLabs skills](https://github.com/CorvidLabs/skills) / [Let](https://github.com/CorvidLabs/let) | Reuse product-authoritative tool skills and local asset discovery. Let discovers assets; it does not control agent execution. Host support still needs qualification. |
| [Design system](https://github.com/CorvidLabs/design-system) / [Figma plugin](https://github.com/CorvidLabs/fledge-plugin-figma) | Use target-relevant tokens/assets with the canonical local HTML/CSS prototype. The plugin syncs token/component data; Figma authoring is not required. |
| [GitHub plugin](https://github.com/CorvidLabs/fledge-plugin-github) | Issues/PR/CI operations, not reproduction or semantic deduplication. Poll is a bounded one-shot creation snapshot and may hide fetch errors as empty arrays; not production-health monitoring. Local-first reports are published only when authorized. |
| [Bench](https://github.com/CorvidLabs/fledge-plugin-bench) | Optional performance checks. Current baselines are not repo/commit/environment scoped; empty parsed data can warn/pass. Require expected measurements rather than treating command success as sufficient. |
| [Fledge E2E plugin](https://github.com/CorvidLabs/fledge-plugin-e2e) | Tests Fledge itself, not arbitrary application E2E. Canary/native/WASM probes concern plugin boundaries rather than live production canary health. |
| [Deploy plugin](https://github.com/CorvidLabs/fledge-plugin-deploy) | Current scripts simulate uploads/versions/health. Use actual project delivery commands when release/deployment is requested or explicitly configured; simulated success is not delivery evidence. |

APS guidance: [use cases](https://github.com/0xLeif/aps-cli/blob/main/docs/use-cases.md)
and [agent checkpoints](https://github.com/0xLeif/aps-cli/blob/main/examples/agent-memory/README.md).
Prefer FileState over process State or platform UserDefaults for portable local
persistence. Qualification must preserve user state and avoid two competing sessions.

No general CorvidLabs production-health monitor was established by the integration
audit. Use project-native health/log/error sources behind thin optional adapters;
no hosted service or blanket autonomous remediation is implied. Monitoring findings
must persist locally; whether they may start an investigation/proposal is an open
project policy, defaulting to capture. Release itself is optional, separately gated
by user request or an explicit project setting.

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
