# Chough

Chough (pronounced chuff) is a small Rust helper plus portable agent skills.

To resume work here, run `chough next --environment <env>` (or `cargo run -- next`)
and read [the workflow skill](skills/chough-workflow/SKILL.md). The checkpoint is
`.chough/session.json`; native SpecSync status decides lifecycle gates.

Before changing behavior, read [INTENT.md](INTENT.md), the relevant `hi/` criteria,
[docs/FRD.md](docs/FRD.md) and the affected contract and companions under `specs/`.
Use Fledge first: `fledge run fmt`, `fledge run test`, `fledge run lint`, `fledge
lanes run verify`. Match tool versions to CI: hi 0.8.0, fledge 1.8.0, specsync 6.1.2.
One command installs all three; see "Install the toolchain" in [README.md](README.md).

HI owns human wording and criterion IDs. SpecSync owns contracts, companions,
changes, approval, verification, review and archives; meaningful files need an active
native change. Chough owns only its checkpoint, transitions, preferences, case
definitions, observations and local bugs. Never create approval, review, release or
deployment records from an agent label, and never treat a structural pass as product
proof.

Keep changes small. Preserve existing files and others' edits; own only assigned
files. Do not add a model client, custom runtime, hosted service or duplicate
lifecycle state. Rune, APS, Magpie and other adapters stay optional until
implemented and tested. Do not commit generated binaries or claim unobserved CI,
release or deployment results.
