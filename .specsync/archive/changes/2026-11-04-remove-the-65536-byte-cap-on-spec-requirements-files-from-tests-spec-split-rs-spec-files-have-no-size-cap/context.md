---
change: remove-the-65536-byte-cap-on-spec-requirements-files-from-tests-spec-split-rs-spec-files-have-no-size-cap
artifact: context
---

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
