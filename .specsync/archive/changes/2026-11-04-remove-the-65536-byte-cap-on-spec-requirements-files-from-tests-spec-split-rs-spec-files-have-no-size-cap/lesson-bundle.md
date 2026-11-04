# Lesson bundle: remove-the-65536-byte-cap-on-spec-requirements-files-from-tests-spec-split-rs-spec-files-have-no-size-cap

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Remove the 65536-byte cap on spec requirements files from tests/spec_split.rs (spec files have no size cap)
- **Kind**: BugFix
- **Specs**: history
- **Paths**: tests/spec_split.rs
- **Acceptance**: tests/spec_split.rs no longer fails when a spec requirements file is over 65536 bytes; its wording and mapping tests still pass; the two split cases expect 4 passing tests; fledge lanes run verify passes on Linux with specs/cli/requirements.md over 65536 bytes.

## Evidence

- Verification commit: `d596976a188e69381fdf5f0bac245f8877873ede`
- Base commit: `270ca0cbadea844d6239c4b48da8c29bdb439713`
- Verified by: `specsync check --spec history`

## From the change's context.md

# Context

Merging #92 and #91 makes `specs/cli/requirements.md` 66,330 bytes. Hosted CI on #91
then failed one test, `every_requirements_companion_fits_the_record_bound` in
tests/spec_split.rs, which caps every spec requirements file at 65,536 bytes. The cap
came from the 2026-11-03 and 2026-11-04 cli spec splits, from before Chough stopped
applying its bounds to native files.

Chough's size bounds now cover only its own records under `.chough/` and what it
displays: specs are read at any size (REQ-cli-034) and native sources at full size
(REQ-cli-042, LIFE-15). Leif's brief says spec files have no size cap. On 2026-11-04
Leif chose "Remove the size test" and said: "The size cap is only there for the amount
of files that you have inside of the .chough folder. It's not for anything".

Only that test is removed. The split's wording digests and the ID mapping test stay,
and no other test, gate or bound changes. A search of tests/ found no other cap on a
file outside `.chough/`.

## From the change's testing.md

# Testing

`cargo test --locked --test spec_split` runs 4 tests and passes with
specs/cli/requirements.md at 66,330 bytes. Before the change the same command fails
`every_requirements_companion_fits_the_record_bound`, as hosted CI did on #91.
TC-history-split-keeps-requirement-wording and
TC-reporting-split-keeps-requirement-wording run on local-linux-aarch64, and
`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/history/context.md`
