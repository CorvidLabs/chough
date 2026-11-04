# Lesson bundle: read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Read native SpecSync spec files under specs at any size: Chough's 65536-byte structured-record bound no longer applies to spec companions such as requirements.md, every Chough record keeps its bound, and the workflow skill gains advisory composability guidance
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/records.rs, src/evidence.rs, src/skill.rs, tests/, docs/cli.md, skills/chough-workflow/, .agents/skills/, specs/cli/, CHANGELOG.md, .chough/cases/
- **Acceptance**: Chough reads a native SpecSync spec file under specs/ at any size: `chough evidence add` validates a requirement ID against a requirements.md companion larger than 4 MiB, and still refuses an ID owned by another module with the same message
- **Acceptance**: Every Chough record keeps its bound: a structured record other than the checkpoint over 65,536 bytes and a checkpoint over 131,072 bytes are still refused, and RECORD_LIMIT stays 65,536
- **Acceptance**: The workflow skill build reference gains advisory composability guidance, identical in the bundled and installed copies: split independent behavior, keep coupled behavior together, treat a source file nearing about 1,000 lines as a split candidate, and treat size as a hint, never a gate
- **Acceptance**: `chough skill status` reports main's previous build reference as known-outdated, so `chough skill update` can refresh it
- **Acceptance**: fmt, lint, the full tests, the new case and fledge lanes run verify pass

## Evidence

- Verification commit: `f4c9d29d84c986022295d9aac609f9ff30126199`
- Base commit: `4daf861509b70c14282ac16b5d0547d701e385b2`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

PR #55 (knowledge directory) could not verify after merging main because the cli
requirements companion `specs/cli/requirements.md` grew past Chough's 65,536-byte
structured-record bound. #59 split the cli spec to make room, but that only moved
the limit. The bound is Chough's own: `src/evidence.rs` read `requirements.md`
through `records::read_record` (RECORD_LIMIT 65,536) when `chough evidence add`
checks a requirement ID's owning module. `chough next` also skips a module
`context.md` over 4 MiB (NATIVE_RECORD_LIMIT) when pointing at its Lessons section,
and `chough library snapshot` refuses files over 4 MiB; REQ-cli-034 names both as
known exceptions, and a later native change on a follow-up PR lifts them.
SpecSync itself does not cap spec files.

Leif (0xLeif), 2026-11-03 10:05 AM MT: "Spec files can grow to whatever number they
want. It's mainly just all the artifacts and all of the investigations or evidence
and all the stuff that you're storing. we shouldn't really care about file size for
specs." And: "Ideally, we should have a guidance to try to keep stuff lean and not
try to, you know, just have a single spec file for everything, and we should break
stuff apart and decouple it if possible but if things need to be coupled, that's
fine. and then same thing for files too, we need to make sure that we're not just
making, you know, thousand plus line files and going to ten thousand lines and
everything there, so we need to have some, you know, good sense of composability but
not be crazy about it."

The parent agent relayed both at 10:06 AM MT and asked for this change: remove the
cap for spec files under `specs/`, keep it for Chough's own records, add advisory
guidance that is never a gate.

Ruled out: raising RECORD_LIMIT (it still protects Chough's records); a size gate or
automatic splitting for specs or source files (Leif: "not be crazy about it").

## From the change's design.md

# Design

No visual or interaction design. One reader boundary: `records::read_spec_file` for
native spec files under `specs/`, `records::read_record` and its bounds for
everything Chough writes.

## From the change's testing.md

# Testing

- tests/cli.rs `spec_requirements_companions_are_read_at_any_size`: two
  requirements companions over 4 MiB; `evidence add` accepts an ID in its module and
  refuses one owned by another module with the existing message, never a size
  refusal. It failed first with "structured record exceeds 65536 bytes".
- tests/cli.rs `records_other_than_the_checkpoint_keep_the_65536_byte_bound` and the
  checkpoint capacity tests still pass unchanged.
- tests/cli.rs `build_reference_from_main_4daf861_is_known_outdated`: main's build
  reference reads known-outdated, and `skill update` makes it current with the new
  section. It failed first with `edited`.
- Cases TC-cli-spec-files-read-at-any-size and
  TC-cli-build-reference-from-main-known-outdated on local-linux-x86_64.
- fmt, lint, the full tests and `fledge lanes run verify`.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-034, TC-cli-spec-files-read-at-any-size | `tests/cli.rs::spec_requirements_companions_are_read_at_any_size`: cli and checkout-ui requirements companions padded past 4 MiB; `evidence add` accepts REQ-cli-001 and refuses REQ-checkout-ui-004 with the "belongs to module checkout-ui" hint and no size refusal. Failed first with "structured record exceeds 65536 bytes". |
| REQ-cli-034, REQ-cli-032 | `tests/cli.rs::records_other_than_the_checkpoint_keep_the_65536_byte_bound` and the checkpoint capacity tests, unchanged: Chough's own records keep their bounds. |
| REQ-cli-034, TC-cli-build-reference-from-main-known-outdated | `tests/cli.rs::build_reference_from_main_4daf861_is_known_outdated`: main's build reference reads known-outdated and `skill update` makes it current with "## Keep it composable". Failed first as `edited`. |

## Where these lessons go

- `specs/cli/context.md`
