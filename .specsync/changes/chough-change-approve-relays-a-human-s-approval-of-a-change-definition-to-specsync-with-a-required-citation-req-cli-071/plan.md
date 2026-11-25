---
change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
artifact: plan
---

# Plan

## Approach

Add `Approve` to `ChangeCommand` in `src/change.rs`, mirroring the way `Ship`
carries a relay of a human decision. `chough change approve <id>` composes the
relay note from `--actor`, `--words`, `--where`, `--recorded-by` (defaulting to
`chough <version>`) and the optional `--note`, refuses before anything runs when
a relay field is absent or blank, checks the id against `refused:unshowable-argument`
and the actor and note against the written-value leak guard, keeps the change's
direct-write baseline current, then runs `specsync change approve <id> --actor
<actor> --note <note>` and passes its standard output, standard error and exit
code through unchanged. `src/main.rs` dispatches it like every other change step.

## Out of scope

No gate, session, stage or human-identity handling: the command is a passthrough,
and a gate decision keeps its own path (`checkpoint --close-gate`, `gate record`,
`change ship`). No SpecSync flags beyond `--actor` and `--note`. Release and push
are not part of this change.

## Steps

1. Write the delta (`deltas/cli.md`), this change's artifacts, the changelog
   fragment and the case definitions.
2. Add `ApproveArgs` and `approve` to `src/change.rs`, dispatch it in `run`,
   and update the `Change` doc in `src/main.rs`.
3. Add the tests in `tests/change.rs` on the existing stand-in `specsync`
   pattern: exact arguments, the note's shape, the version default, the
   refusals, the passthrough and the leak checks.
4. Update `docs/cli.md` and `README.md` where the change steps are listed.
5. Run `fledge run fmt`, `fledge run test` and `fledge run lint`, then
   `chough cases run` and `chough change check`.

## Risks

The note is free text SpecSync stores in the approval record; a future SpecSync
that takes the relay fields as its own flags would make the composed note
redundant, but until then it is the only place every field survives. The
definition approval of this change itself is the first use of the new command's
pattern and stays a human decision.

## Constraints consulted

`specs/cli/cli.3md`: write `tasks.md` without unchecked boxes, because `change
check` refuses them; treat the delta as frozen once approved.
