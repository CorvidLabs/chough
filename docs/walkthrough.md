# A focused session, a failed check, and a resumed task

This walkthrough uses Chough's own durable LIFE criteria and CLI contract. Use a
clean checkout or an isolated copy if you want the exercise to leave your working
session untouched. The broader bug-triage steps are skill-led; the helper records
context and observations rather than performing those decisions for you. This is
the current simple session flow, not evidence that the [planned stage checkpoints
and cross-agent cold resume](lifecycle.md#iteration-optional-side-stages-and-checkpoints)
are implemented or proven.

## Initialize without replacing project work

After installing from source, run inside the Chough repository:

```sh
chough init --objective "Verify safe project initialization" \
  --intent-scope LIFE --module cli
chough context
chough status --environment local-development
```

The session is `.chough/session.json`. The skill and its required references are
installed beneath `.agents/skills/chough-workflow/`. Repeating `init` reports
preserved paths; new command options do not replace the existing session. Edit the
session deliberately when changing the objective. Chough's own checkout already
contains a session; these flags will preserve its existing values. That bootstrap
session may still point at the deferred evidence-reuse proposal, so inspect the
current objective and native state instead of treating a saved suggestion as an
approved new task.

On hosts with compatible skill discovery, ask:

> Use $chough-workflow to verify REQ-cli-001. Start from LIFE-4 and LIFE-6, inspect
> the CLI contract, and preserve the existing project files.

Otherwise give the agent the installed or canonical `SKILL.md` explicitly.
Chough does not install a universal slash command. A supported host can map its
own command to the same skill and native helper calls.

## Observe a real check before recording it

Run the relevant check through the project's task surface. Save the actual output:

```sh
mkdir -p .chough/artifacts
fledge run test > .chough/artifacts/tests.log 2>&1
```

Inspect the exit status and the saved log. Only after observing success, record:

```sh
chough evidence add cli-tests-001 \
  --criterion LIFE-4 --criterion LIFE-7 \
  --requirement REQ-cli-001 \
  --contract specs/cli/cli.spec.md \
  --kind product-test --outcome passed \
  --environment local-development \
  --summary "Observed the CLI test suite pass; see the attached output." \
  --artifact .chough/artifacts/tests.log
chough status --environment local-development --json
```

Use a new evidence ID for each observation. Existing IDs are preserved. A test
failure should be recorded with `--outcome failed`, a faithful summary, and its
actual output. Do not run the success command after a failed or interrupted test.

The helper validates active criterion IDs against native HI export and exact
requirement IDs in the supplied contracts' `requirements.md` companions. Artifacts
must exist. It hashes artifacts and captures the Git revision, dirty state, and
worktree fingerprint; it does not rerun the test.

## Investigate a failure as a bug

If initialization unexpectedly changes a user-edited file, preserve a small
reproduction in a temporary Git repository: initialize, edit the generated file,
initialize again, and compare bytes. Record the real command sequence and output.
Tie the observation to LIFE-4, LIFE-6, and REQ-cli-001.

Use the [bug procedure](../skills/chough-workflow/references/bugs.md) to classify
the observation, search existing local reports and relevant issues, and draft a
report. Distinguish a product defect from an environment failure or unclear intent.
A regression needs the same reproduction failing now and passing at an identified
earlier revision. A plausible explanation remains a hypothesis until verified.

Keep the report local unless filing an issue or posting an update is authorized.
Use a native SpecSync bug-fix change for the repair, obtain the real scope decision,
then show the reproduction pass after the fix and complete adjacent checks. Follow
native status through review and same-PR archival. A repaired local fixture does
not establish that a released or deployed product is fixed.

This example describes how to investigate that failure; it does not claim that
the initial Chough implementation has that defect. The automated fixture tests
exercise preservation and error cases against the actual helper.

## Resume with the next action intact

Edit `.chough/session.json` to reflect the state you actually reached. Keep its
schema and existing fields; for example, update these values:

```json
{
  "schema_version": 1,
  "objective": "Finish the initialization preservation investigation",
  "intent_scope": "LIFE",
  "modules": ["cli"],
  "decisions": ["Use the smallest isolated Git fixture that demonstrates the symptom."],
  "open_questions": [],
  "next_action": "Read the reproduction output, then follow native SpecSync status for the scoped repair."
}
```

In the next agent session:

```sh
chough context --json
chough next --environment local-development --json
```

The packet carries the current objective, relevant native sources, decisions,
questions, and next action. Read any explicit missing or truncated source markers
before deciding the context is sufficient. Native SpecSync output remains the
authority on lifecycle actions.

## Interpret freshness precisely

| Status | Meaning |
|---|---|
| `current-observation` | Recorded inputs and explicit environment match now; outcome and provenance still matter. |
| `stale-revision-or-inputs` | The revision or relevant Git worktree inputs changed. |
| `stale-artifact` | A bound artifact changed or disappeared. |
| `stale-environment` | The requested environment differs from the recorded one. |
| `environment-unchecked` | No environment was supplied for comparison. |
| `unknown-revision` | A current Git snapshot could not be established. |

The first applicable stale reason is reported. A current failed observation remains
a failure. Changing code or making a new commit requires fresh observations even
if older tests passed. Chough's own `.chough/` records and `target/` are excluded
from the worktree fingerprint, while explicitly bound artifacts are hashed
separately. Avoid treating generated or ignored files as verified inputs unless
they are explicitly represented in evidence.

Human approval and delivery remain unknown in helper status. A human-observation
record is not authenticated acceptance, and a delivery-observation record is not a
deployment check executed by Chough.
