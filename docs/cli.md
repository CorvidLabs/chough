# CLI reference

Run `chough --help` and each subcommand's `--help` for the installed executable.
All commands accept global `--root PATH`, defaulting to the current directory.
The project root must exist. Recording evidence requires the Git repository root
and an existing commit.

| Command | Options and behavior |
|---|---|
| `init` | `--objective TEXT`, `--intent-scope SCOPE`, repeated `--module NAME`. Creates missing managed files and reports preserved paths. |
| `context` | `--json`. Emits focused Markdown by default or a structured packet. |
| `status` | `--environment NAME`, `--json`. Reports snapshot, observations and freshness, native status, session guidance, and unknown approval/delivery. |
| `next` | Same options and report as `status`; keeps session guidance beside native next-action authority. |
| `evidence add ID` | Required criterion, requirement, contract, kind, outcome, environment, and summary. Optional repeated artifact paths. Appends a new observation; never replaces an ID. |

`--criterion ID`, `--requirement ID`, `--contract PATH`, and `--artifact PATH`
are repeatable on `evidence add`. Kinds are `product-test`, `structural`,
`human-observation`, and `delivery-observation`. Outcomes are `passed` and `failed`.
Contract paths refer to native `specs/**/*.spec.md` files and their native
`requirements.md` companions. Criterion IDs must exist and be active in HI.

Managed JSON uses `schema_version: 1`; unknown fields and unsupported versions
are rejected. Records are limited to 64 KiB, sessions select at most 16 modules,
and source content is limited to 16 KiB per section and 128 KiB in aggregate.
Visible truncation markers direct the agent back to the source when needed.
Native command output is bounded for display; the helper is not a sandbox or a
resource-isolation layer for installed executables.

Paths for managed files, contracts, and artifacts are project-relative. Traversal
and symlink paths are rejected. Ancestor instruction files are intentionally read
from their applicable locations. The helper invokes native tools with direct
arguments, not by evaluating the session text as a shell command.

`context` uses native `hi export` for the selected scope. `status` uses native
`specsync change status --json` when native configuration exists. Unavailable,
failed, not-configured, and invalid-output results remain distinct. Successful
process output must decode as a complete JSON object or array before it is usable;
a successful response does not itself mean the product or lifecycle is complete.

## Current limitations versus planned workflow

`next` is currently the same report as `status`, not an autonomous stage router.
The version-1 session holds objective, intent scope, modules, decisions, open
questions and next action; it has no stage, preference, monitor or librarian schema.
`init` preserves existing files, including old session values; new flags do not
rewrite them. Unsupported/extra JSON fields are rejected.

The input fingerprint includes HEAD, tracked and untracked nonignored worktree
inputs and relevant permissions; `.chough/` and `target/` working files are excluded.
A new commit still stales old observations. Metadata-commit reuse is deferred and
unapproved. Environment comparison uses the supplied label, not automatic detection
of all real environment changes. Native output validity means usable JSON, not
that a lifecycle gate passed.

Full checkpoint/cold-resume coordination, 3md testing profiles, APS persistence,
monitoring/preferences and a librarian remain [planned design](lifecycle.md).
See the [walkthrough](walkthrough.md) for the implemented observation/session flow.
