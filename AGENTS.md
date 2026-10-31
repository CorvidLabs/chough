# Chough

Chough (pronounced chuff) is a small Rust helper plus portable agent skills. Read
[INTENT.md](INTENT.md), the relevant `hi/` criteria, and affected module contracts
and companions under `specs/` before changing behavior. Product requirements live
in `docs/FRD.md`; proposed additions are not human commitments.

Use Fledge first: `fledge run fmt`, `fledge run test`, `fledge run lint`, and
`fledge lanes run pre-push`. HI owns human intent and permanent criterion IDs;
SpecSync owns contracts, companions, changes, review, and archives. Chough owns
only its focused session and revision/environment-bound observations. A passing
structural check never proves product behavior, authentic human approval, or
deployment. Never create human approval/review records from an agent label.

Read `skills/chough-workflow/SKILL.md` for the product workflow. Native tool help
and `specsync change status` take precedence over cached command sequences.
Bootstrap is an explicitly authorized first implementation using native structural
checks; the native change lifecycle is optional by upstream policy. Do not claim
that a bootstrap push is a fully approved/archived lifecycle. Later scoped changes
should use the full native lifecycle with genuine human decisions.

Keep changes small. Preserve existing files during initialization. Do not add a
model client, custom runtime, hosted service, or duplicate lifecycle state machine.
Rune and other tools are optional adapters until implemented and tested.

Multiple agents may be working here. Own only assigned files, preserve others'
edits, and coordinate interface changes. Source belongs in `src/`, meaningful
integration tests in `tests/`, and agent/doc assets beside their owners. Do not
commit generated binaries or claim unobserved CI/release/deployment success.
