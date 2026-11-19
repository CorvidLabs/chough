# Lesson bundle: fix-294

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 294
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/gates.rs, .chough/cases/cli.gate-summary-change-line.cases.toml, changelog.d/2026-11-18-fix-294.md
- **Acceptance**: A test opens a gate on a session whose native change is recorded and asserts that chough gate list prints the summary line Change: <id> - <title> (<state>) and that the summary holds no em dash; it fails when the em dash returns to src/gates.rs. src/gates.rs is unchanged.

## Evidence

- Verification commit: `65bb8df60f4a80df80370717aa4cc56f79394515`
- Base commit: `66543c2bf509627ec2a165414acae0bc8ab59c4a`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

GitHub #294, a follow-up from #293 (the fix for #278, Grok's review finding 3). #293 changed the gate summary's change line in `src/gates.rs` to `Change: <id> - <title> (<state>)`, and REQ-cli-061's acceptance criteria say: "A gate's summary names its change as `Change: <id> - <title> (<state>)`, with no em dash (#278)." No test checks that separator: the case `TC-history-archived-stories-join-evidence-without-an-em-dash` lists `src/gates.rs` but binds a sessions test. If the em dash came back into `src/gates.rs`, every test would still pass.

This change adds the missing test in `tests/gates.rs`, beside the existing summary tests, and binds it to a case of its own. `src/gates.rs` is unchanged. Leif approved the scope on 2026-11-18, relayed by his planning session (hc t4c), with the A list (#268, #274, #294).

## From the change's testing.md

# Testing

`cargo test --locked --test gates a_gate_summary_names_its_change_with_a_plain_dash` opens a gate on a session whose native change is recorded, and requires `chough gate list` to print `Change: <id> - <title> (` with no em dash anywhere in the summary. The code already does this, so the test passes on main. It is shown to guard the line by failing against `src/gates.rs` with the em dash put back, a local edit that is never committed. Case `TC-cli-gate-summary-names-its-change-with-a-plain-dash` binds it.

## Where these lessons go

- `specs/cli/cli.3md`
