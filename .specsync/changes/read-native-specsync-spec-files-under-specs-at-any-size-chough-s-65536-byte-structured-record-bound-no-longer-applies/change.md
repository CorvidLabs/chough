---
id: read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies
state: approved
type: feature
base_commit: 4daf861509b70c14282ac16b5d0547d701e385b2
---

# Read native SpecSync spec files under specs at any size: Chough's 65536-byte structured-record bound no longer applies to spec companions such as requirements.md, every Chough record keeps its bound, and the workflow skill gains advisory composability guidance

## Intent

Read native SpecSync spec files under specs at any size: Chough's 65536-byte structured-record bound no longer applies to spec companions such as requirements.md, every Chough record keeps its bound, and the workflow skill gains advisory composability guidance

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Chough reads a native SpecSync spec file under specs/ at any size: `chough evidence add` validates a requirement ID against a requirements.md companion larger than 4 MiB, and still refuses an ID owned by another module with the same message
- Every Chough record keeps its bound: a structured record other than the checkpoint over 65,536 bytes and a checkpoint over 131,072 bytes are still refused, and RECORD_LIMIT stays 65,536
- The workflow skill build reference gains advisory composability guidance, identical in the bundled and installed copies: split independent behavior, keep coupled behavior together, treat a source file nearing about 1,000 lines as a split candidate, and treat size as a hint, never a gate
- `chough skill status` reports main's previous build reference as known-outdated, so `chough skill update` can refresh it
- fmt, lint, the full tests, the new case and fledge lanes run verify pass

## No-spec Rationale

Not applicable
