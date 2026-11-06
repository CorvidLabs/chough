---
id: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
state: approved
type: documentation
base_commit: b535b92fb0e7cf5679449c18860e136bb27397e6
---

# Place the #77 sessions entry under 0.4.0 and add #111's legacy-anchor session fix to the change log (LIFE-16)

## Intent

Place the #77 sessions entry under 0.4.0 and add #111's legacy-anchor session fix to the change log (LIFE-16)

## Affected Canonical Specs

- None

## Acceptance Criteria

- In CHANGELOG.md the #77 sessions entry ('Sessions per objective or native change') sits under [0.4.0], not [0.3.0], with its text unchanged except that it now names #77; [0.4.0] gains one entry for #111's fix, so a session started with chough session new after a project adopted its legacy history writes events instead of refusing with 'legacy history anchor changed'; no other entry in a released section changes. The new tests in tests/changelog.rs fail first on main, and fledge lanes run verify passes.

## No-spec Rationale

Change-log corrections for LIFE-16: #77 and #111 first shipped in v0.4.0, the #77 entry sits under 0.3.0 and #111 has none; no requirement text changes.
