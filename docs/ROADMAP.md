# Roadmap and implementation boundary

[HI](../hi/life.md) holds durable product intent. The current implementation is a
working foundation; the full lifecycle below is agreed direction, not a completed
stage engine or an approved native implementation definition.

| State | Outcome | Evidence / remaining work |
|---|---|---|
| Implemented foundation | Starter workflow skill; Rust init/context/status/next/evidence add; preservation, focused session and scoped observations. | Baseline `d38aa195` passed 14 local integration tests and [hosted Linux/macOS/Windows plus native checks](https://github.com/CorvidLabs/chough/actions/runs/41218954725). This historical run is not evidence for a later implementation tip. |
| Agreed next evolution | Natural stage loops: intent/FRD review, conditional canonical HTML/CSS design, coherent native technical scope, build, focused testing/local bugs, human acceptance and native closeout. | Concrete native scope preparation/real initial approval, implementation, affected AC-to-test evidence, real scoped implementation review before eligible same-PR archive. See [lifecycle](lifecycle.md). |
| Core in that design | Local checkpoints and progressive discovery preserve scope, decisions, preferences, tests, bugs, blockers and next action across fresh sessions. | Demonstrate a fresh agent with no original conversation resuming under the same native state/gates; the bootstrap fixture does not establish this evolved cold-start target. |
| Optional project capabilities | Extended/background validation, project-local librarian callable at any stage, and release only when requested/configured. | Explicit applicability/preferences; persistent local findings; safe repeatable exercises. No automatic live job, external reporting, publication or remediation is implied. |
| Candidates to qualify | Small versioned 3md testing profile; APS FileState checkpoint/preferences adapter; target-native assertions through Fledge/Magpie. | Stable test IDs and definition/result separation; real assertions; compatible session migration and single authority; portability and writer ownership. None is adopted merely by naming it. |
| Later optional adapters | Rune session coordination, Trust gate reuse, read-only Atlas context/gaps and target-native delivery/health sources. | Compatibility, ownership and actual result semantics verified before claiming support; no new runtime/hosted service required. |

Host-neutral/current-agent-first is settled. Host discovery and supported command
mappings still need real host-specific exercises; sequential role passes must work
without subagents. Relevant tests are focused on affected HI/AC/FRD, not a full
retest of every product criterion on each change.

## Open choices

- Test-definition format/profile and the precise adapter semantics to execute it;
  3md is a preferred candidate, not a locked implemented format.
- APS qualification, migration/authority model, concurrency and platform fallback.
- Project preferences for optional validation/monitoring, including whether a
  monitor may initiate an investigation/proposal; default unresolved policy to
  local capture. Any remediation retains scope/gates.
- Project-specific release and trust/signer policies when those optional paths
  are actually requested or configured.

The earlier [metadata-commit observation-reuse proposal](NEXT_CHANGE.md) is
**unapproved and deferred in favor of the lifecycle work**.
It remains useful design history, not an approved assignment or a rejected idea.
The current helper still conservatively marks observations stale on any new HEAD.
