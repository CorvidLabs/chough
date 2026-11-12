# Walkthrough: install, one feature and bug, and resuming

This follows the [shipping sample](../examples/shipping/). To run the whole sequence
with assertions in a disposable directory, use `fledge run exercise` or
`sh examples/feature-bug-loop.sh`; the commands below show the agent's operations.
Human gates appear as decisions a person makes; the sample script labels its
stand-ins as SIMULATED.

Throughout, the division of labor stays the same: the agent runs the tools —
HI, SpecSync, Fledge and the `chough` helper — captures criteria, fills the native
change's technical interview fields from your intent and the repository, builds,
tests and moves the stages. You make the product decisions: the intent as a whole,
any rendered design, scope where your prior or standing authorization does not
already cover it, and the implementation review. `chough next` only recommends the
next action; the agent verifies native state before acting on it.

## New project

For a Git project that has no HI or SpecSync files yet. These steps were observed on
Linux x86_64 with hi 0.8.0, specsync 6.1.2 and fledge 1.8.0 (issue #40). Install the
tools first as in [Install](#install).

1. Write the intent as a minimal HI file, for example `hi/checkout.md`:

   ```markdown
   ---
   hi: 1
   families: [CHECKOUT]
   ---

   # Checkout

   ## Intent

   Shoppers should not pay shipping on larger orders.

   ## Criteria

   - **CHECKOUT-1**  An order of fifty dollars or more ships free.
   ```

   `hi check` validates it and `hi ls` lists the criteria. The IDs never move.

2. Create the module contracts:

   ```sh
   specsync init               # .specsync/config.toml and sdd.json, SDD off
   specsync scaffold shipping  # specs/shipping/: shipping.spec.md, context, requirements, tasks, testing
   ```

   Fill in the scaffolded files (source files, requirements and testing) before
   you rely on them. `specsync check` reports what is still missing.

3. Turn on the SpecSync change workflow, which `specsync init` leaves off:

   ```sh
   specsync change adopt
   ```

   Until you do, stage moves pass as `ungated:sdd-disabled`. They are allowed, but
   no native lifecycle gates them, and `chough next` reports "not checked: native SDD
   is disabled; run `specsync change adopt` to gate stages". Once SDD is adopted, gated
   moves need a native change recorded with `chough checkpoint --native-change <id>`.

4. Continue with `chough init` as below.

## Install

```sh
cargo install --path . --locked     # from a Chough checkout
hi --version; specsync --version; fledge --version
```

Use the same SpecSync version for every lifecycle writer and CI (this repository uses
6.3.0). In the project:

```sh
chough init --objective "Free shipping from fifty dollars" \
  --intent-scope CHECKOUT --module shipping --host claude
chough skill status
```

`--host claude` shares the `.agents/skills/chough-workflow/` copy; Claude Code 2.1.316
did not discover that directory alone, so add a line to the project's `CLAUDE.md` that
points at `.agents/skills/chough-workflow/SKILL.md` (`init` does not write it, and with `--host claude` prints it as `claude_md`; see
[integrations](integrations.md)). Kimi Code CLI users should
initialize with `--host openai`, which installs `.agents/skills/chough-workflow/`,
the directory Kimi discovered in the observed run. For other hosts, add their
`--host` or point the agent at the skill file.

## Intent and design

Ask the agent in plain language, for example: "Use the Chough workflow: orders of
fifty dollars or more should ship free." It captures agreed criteria with `hi`,
derives the FRD, runs the role passes and asks you one question about the intent as
a whole. After you approve:

```sh
chough stage advance                                  # intent -> design
chough stage skip --reason "Fee calculation only; nothing visual"   # design -> plan
```

A visual change instead goes through the
[design stage](exercises/visual-design.md) and waits for your approval of the
rendered prototype and its companion.

## One scope approval

The agent drafts a native change and checks whether actual prior or standing human
authority covers its concrete scope. When that authority is missing, it presents
the decision and records the pending gate as shown here:

```sh
chough change new "Free shipping at the threshold" --spec shipping --path src/lib.rs --path tests/
chough checkpoint --native-change free-shipping-at-the-threshold \
  --awaiting-human "Scope approval of free-shipping-at-the-threshold"
chough stage advance        # refused: refused:awaiting-human (refused:native-draft once cleared)
```

When you decide, the agent records your decision on the gate. At plan the gate
drives the native approval, so Chough first runs `specsync change approve` in your
name, from the identity you configured (see the [README](../README.md#start-here)), and
only then closes the gate with your words, a relayed claim, not authenticated:

```sh
chough checkpoint --close-gate "Scope approval of free-shipping-at-the-threshold" \
  --decided-by "<you>" --words "<your words>" --where "<where and when>" \
  --recorded-by "<agent>-<terminal>"
```

When earlier permission you gave already covers the definition, nobody waits on a
gate: the agent records the approval in its own name and quotes you,
`specsync change approve <id> --actor "<agent>-<terminal>" --note "Approved under <your words> (<where>); covered scope: <scope>; limits: <limits>. Recorded by <agent> (<model>, hc <terminal>)."`.
Applicable existing permission is used without another tool-specific question. Then
`chough stage advance` enters build. Your later implementation acceptance stays a
separate decision.

## Build, focused tests and a bug

```sh
chough change check free-shipping-at-the-threshold
chough stage advance                                   # build -> test
chough cases select --requirement REQ-shipping-002 --path src/lib.rs
chough cases run TC-shipping-0001 --environment local  # exit 3: failed (left: 499, right: 0)
chough bug search "order of exactly fifty dollars is charged shipping"
chough bug add --title "Shipping charged at exactly the free threshold" \
  --symptom "…" --trigger "…" --expected "Fee 0" --actual "Fee 499" \
  --environment local --case TC-shipping-0001 --observation <failed-observation> \
  --scope blocking --classification product-defect
chough stage advance                                   # refused: refused:blocking-bugs
chough stage back --to build --reason "<bug-id> blocks acceptance"
```

After the fix, rerun the focused cases, link the passing observation and mark the bug
verified (`chough bug update <bug-id> --status verified --observation <passing>`).
`bug add` names the new bug by its date and title, here
`.chough/bugs/<date>-shipping-charged-at-exactly-the-free-threshold.md`, and prints that
ID.
`chough bug report <bug-id>` prints a report candidate; nothing is posted unless you
ask.

## Acceptance

```sh
chough stage advance        # test -> accept
chough stage advance        # refused: refused:native-verifying
chough next --environment local
```

Before presenting, the agent opens the review gate (`chough checkpoint
--awaiting-human "Implementation review of free-shipping-at-the-threshold"`, which
holds accept and drives the native review), commits the source together with
`.chough/`, runs `chough change check free-shipping-at-the-threshold` and presents the
work. Your implementation review is the decision. When you accept, the agent runs one
command on the same pull request. It records `specsync change review` in your name,
ships, closes the review gate with your decision as a relayed claim, and moves to
done:

```sh
chough change ship free-shipping-at-the-threshold \
  --close-gate "Implementation review of free-shipping-at-the-threshold" \
  --decided-by "<you>" --words "<your words>" --where "<where and when>" \
  --recorded-by "<agent>-<terminal>"
```

It checks the session, the stage and the relay before it runs anything. If the move
into done is refused after the ship, it says the change is shipped and names the
command left to run. The move into done turns the session into one story in
`.chough/archive/sessions/` and writes a journal post. The agent commits those, then
runs `chough doctor --fix`, which removes the session folder now that it is committed,
and commits that. Pushing and observing CI come last, because the hosted Archive gate
refuses while the change is open. See [the
closeout order](lifecycle.md#acceptance-and-native-closeout). Release is a separate,
optional request.

## Resume cold

A new agent, possibly a different host, with none of this conversation:

```sh
chough next --environment local
```

It prints a focused view of at most 16 KiB: the stage, the native change and the gate
for the next stage, the referenced observations that are not current, blocking bugs
and a count of the others, pending human decisions and the next valid action with
the command to run. `chough status` is the full JSON report. The
[cold-start exercise](exercises/cold-start.md) records an independent Codex session
doing exactly this on the sample. Separately, it records one Kimi cold start in this
repository, not that sample. Kimi discovered the skill and reached the archived
native state, but could not safely cold-resume because `next` and `status` gave stale
review and ship guidance, recorded as BUG-0024. The earlier native-routing fix was
superseded during merge by main's newer REQ-cli-021 new-cycle behavior. BUG-0025 records
that distinct integration defect and is verified by the later linked TC-cli-0105 pass
under the approved alignment change. TC-cli-0105 covers the new-cycle behavior as an
automated regression; Kimi did not return to validate a corrected build. See the exercise
for the full observation and limits.

## Read freshness precisely

Any changed fingerprinted file, a changed artifact, a changed case definition or a
different environment makes an observation stale; rerun the case instead of reusing
the result. The revision is recorded with each observation but not compared:
committing only `.chough/` records, an empty commit, committing edits that were
already present when the case ran, and SpecSync's lifecycle bookkeeping (`change
check --commit`, finalize) keep it current. A receipt migrated from an earlier Chough
without a content identity stays stale after any commit. See the [CLI reference](cli.md) for
every outcome and freshness label.
