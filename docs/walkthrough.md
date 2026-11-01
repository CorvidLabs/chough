# Walkthrough: install, one feature and bug, and resuming

This follows the [shipping sample](../examples/shipping/). To run the whole sequence
with assertions in a disposable directory, use `fledge run exercise` or
`sh examples/feature-bug-loop.sh`; the commands below are the same steps by hand.
Human gates appear as decisions a person makes; the sample script labels its
stand-ins as SIMULATED.

## Install

```sh
cargo install --path . --locked     # from a Chough checkout
hi --version; specsync --version; fledge --version
```

Use the same SpecSync version for every lifecycle writer and CI (this repository uses
6.1.1). In the project:

```sh
chough init --objective "Free shipping from fifty dollars" \
  --intent-scope CHECKOUT --module shipping --host claude
chough skill status
```

Claude Code discovers `.claude/skills/chough-workflow/`; for other hosts add their
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

The agent drafts a native change and presents it:

```sh
specsync change new "Free shipping at the threshold" --spec shipping --path src/lib.rs --path tests/ --json
chough checkpoint --native-change free-shipping-at-the-threshold \
  --awaiting-human "Scope approval of free-shipping-at-the-threshold"
chough stage advance        # refused: refused:native-draft
```

Only after you approve does it run `specsync change approve <id> --actor "<you>"` and
`chough checkpoint --clear-awaiting-human`. Then `chough stage advance` enters build.

## Build, focused tests and a bug

```sh
specsync change check free-shipping-at-the-threshold --commit
chough stage advance                                   # build -> test
chough cases select --requirement REQ-shipping-002 --path src/lib.rs
chough cases run TC-shipping-0001 --environment local  # exit 3: failed (left: 499, right: 0)
chough bug search "order of exactly fifty dollars is charged shipping"
chough bug add --title "Shipping charged at exactly the free threshold" \
  --symptom "…" --trigger "…" --expected "Fee 0" --actual "Fee 499" \
  --environment local --case TC-shipping-0001 --observation <failed-observation> \
  --scope blocking --classification product-defect
chough stage advance                                   # refused: refused:blocking-bugs
chough stage back --to build --reason "BUG-0001 blocks acceptance"
```

After the fix, rerun the focused cases, link the passing observation and mark the bug
verified (`chough bug update BUG-0001 --status verified --observation <passing>`).
`chough bug report BUG-0001` prints a report candidate; nothing is posted unless you
ask.

## Acceptance

```sh
chough stage advance        # test -> accept
chough stage advance        # refused: refused:native-verifying
chough next --environment local
```

Before presenting, the agent records the gate (`chough checkpoint --awaiting-human
"Implementation review of free-shipping-at-the-threshold"`), commits the source
together with `.chough/`, runs `specsync change check free-shipping-at-the-threshold
--commit`, pushes the branch and observes CI. Your implementation review is the
decision; only after it does the agent run `specsync change review` and then `ship` or
`finalize` on the same pull request, writing nothing to `.chough/` in between, and
only then clears the gate and runs `chough stage advance`. See [the closeout
order](lifecycle.md#acceptance-and-native-closeout). Release is a separate, optional
request.

## Resume cold

A new agent, possibly a different host, with none of this conversation:

```sh
chough next --environment local
```

It reports the stage, the gate for the current stage, freshness of the referenced
observations, open and blocking bugs, standing preferences, pending human decisions
and the next valid action. The [cold-start exercise](exercises/cold-start.md) records
an independent Codex session doing exactly this on the sample.

## Read freshness precisely

Any changed fingerprinted file, a changed artifact, a changed case definition or a
different environment makes an observation stale; rerun the case instead of reusing
the result. The revision is recorded with each observation but not compared:
committing only `.chough/` records, an empty commit, committing edits that were
already present when the case ran, and SpecSync's lifecycle bookkeeping (`change
check --commit`, finalize) keep it current. A record from an earlier Chough without a
content identity stays stale after any commit. See the [CLI reference](cli.md) for
every outcome and freshness label.
