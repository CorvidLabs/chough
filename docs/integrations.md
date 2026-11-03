# Integration map

Chough integrates native HI export, SpecSync change status and Git directly, and runs
bound test commands only when a case is explicitly executed. Everything else below is
guided by the skill or kept as a qualified, deferred or unavailable adapter.

## Hosts

| Host | Project skill directory | Observed support |
|---|---|---|
| Claude Code 2.1.316 | `.agents/skills/chough-workflow/` (`--host claude` shares the openai copy) | Two fresh headless sessions discovered a `.claude/skills` copy and did not discover `.agents/skills` alone (answer: NONE). Chough no longer installs `.claude/skills`, so Claude Code reaches the workflow through `CLAUDE.md` and `AGENTS.md` (see the tradeoff below); no run has re-checked a newer version |
| Codex CLI 0.153.4 | `.agents/skills/chough-workflow/` (openai mapping) | A fresh session followed the `AGENTS.md` router to the installed skill and completed a cold start; native discovery of that directory was not tested separately |
| Kimi Code CLI 0.41.0 | `.agents/skills/chough-workflow/` (openai mapping) | One macOS arm64 run on 2026-11-01 auto-discovered and loaded the skill and attempted a cold resume. It could not safely resume the archived work because of BUG-0024's stale archived-action defect; no fresh post-fix Kimi run was performed |
| Google Antigravity | `.agents/skills/chough-workflow/` (openai mapping, the default; a shared skills directory) | One run on 2026-11-01 reported listing the skill from this directory; the report is narrative, written by the agent itself in closed PR #20 with no event log or transcript, and not independently verified (see [the cold-start record](exercises/cold-start.md)) |
| `--host gemini` | `.gemini/skills/chough-workflow/` | Mapping from the CorvidLabs skills installer; no run has exercised it |
| Cursor, Grok | `.cursor/skills`, `.grok/skills` | Mapping from the CorvidLabs skills installer; not exercised |
| Any host | Give the agent `skills/chough-workflow/SKILL.md` | The skill reads as plain Markdown |

### Claude Code and the single shared copy

Chough keeps one skill copy per project, in `.agents/skills`, and `--host claude`
installs or updates that copy instead of a second `.claude/skills` copy. The
tradeoff: Claude Code 2.1.316 was observed not to discover `.agents/skills` on its
own, so in that version the skill is not listed automatically. Claude Code reads
`CLAUDE.md`, which points it at `AGENTS.md` and the skill file, and that is the
supported route until a run of a newer version shows native discovery. If you
need native listing, give the agent the skill file directly. A project that still
has a `.claude/skills/chough-workflow/` copy from an earlier helper sees it reported
as `legacy` by `chough skill status` and `skill update`; neither command writes or
deletes it. `chough skill remove-legacy --host claude` removes only files that are
byte-identical to the current or a known earlier bundle, keeps edited and unknown
files, and refuses, removing nothing, while the shared copy is missing.

`chough init` does not write or edit `CLAUDE.md`. In a project you use with Claude
Code, add this line to `CLAUDE.md` yourself (create the file if it has none):

```markdown
Read and follow `.agents/skills/chough-workflow/SKILL.md` for the Chough workflow.
```

A directory name is a convention, not proof of support. `chough skill status` shows
whether an installed copy is current, edited or outdated.

## Native tools

| Tool | Native ownership | Chough boundary |
|---|---|---|
| [HI](https://github.com/CorvidLabs/hi) 0.8.0 | Human sentences, permanent IDs, scoped export, structure | Reads `hi export`; FRD is derived downstream; HI does not track achievement |
| [SpecSync](https://github.com/CorvidLabs/spec-sync) 6.1.2 | Requirements, contracts, changes, approval, verification, review, archive, lessons | Reads `change status --json` `summary.state`, `approval_valid`, `correction_valid`, `terminal_evidence.validity` and `change.affected_specs` for gates, and the approval ledger and deltas read-only to confirm approved deltas are unchanged; never runs approve, check, review or finalize |
| [Fledge](https://github.com/CorvidLabs/fledge) 1.8.0 | Tasks, lanes, work and release mechanics | Case binding `fledge-task`; Chough's own lanes |
| [CorvidLabs skills](https://github.com/CorvidLabs/skills) | Product-authoritative skills and the host directory mapping | Linked, not copied |
| [Design system](https://github.com/CorvidLabs/design-system) | Tokens and assets | The visual sample pins `tokens.css` at c698ed4; Figma is optional |

## Qualified, deferred or unavailable adapters

| Adapter | Status | Limits to preserve |
|---|---|---|
| APS 1.1.0 | Qualified, deferred (D-010) | Lost 49 of 50 concurrent read-modify-write updates with exit 0; unknown fields and any schema version accepted; deleted value reads as a fresh value; implicit `~/.aps` root; no Windows binary; values only as argv |
| 3md / agent.3md | Evaluated, not used for definitions (D-011) | Single-line string attributes; loaders ignore unknown fields; SpecSync reads `specs/<module>/<module>.3md` as lessons |
| Magpie | Not installed; deferred | Generic mode runs every `tool=` skill with `/bin/sh -c`, judges exit codes, and records skipped skills as passes. Bind only through `command` with `expect_output` |
| Rune | Optional, later | A settled send is not task success; current-agent-first is the model |
| Trust | Proposed gate adapter | Local verify has no JSON report; degraded or baseline results are not proposed-commit provenance |
| Augur | Not integrated | `check` exits zero regardless of verdict; use `gate` to enforce; risk is not behavior |
| Attest | Not integrated | A signature proves key possession, not human authorization; never infer `humanApproved` |
| Atlas | Not integrated | SpecSync compatibility unverified |
| Let | Not integrated | Discovers local agent assets; it does not control agent execution |
| Figma plugin | Optional | Syncs token and component data; Figma authoring is not required for the design stage |
| GitHub plugin | Guidance only | Issue lists are bounded and can hide fetch errors; nothing is posted without a request |
| Bench, deploy, e2e plugins | Not integrated | Baselines are not scoped; deploy scripts simulate (record as `dry-run`); e2e tests Fledge itself |

APS guidance consulted for the qualification: [use cases](https://github.com/0xLeif/aps-cli/blob/main/docs/use-cases.md)
and [agent checkpoints](https://github.com/0xLeif/aps-cli/blob/main/examples/agent-memory/README.md).

No general production-health service is assumed. Monitoring uses project-native
sources wrapped as cases with explicit assertions, and only with authorization.
