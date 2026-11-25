## Context

The REQ-cli-072 change (archived) edited tests/docs.rs (README lane-list wording),
tests/skill_releases.rs (manifest pin) and tests/fixtures/skill-releases/manifest.json
(fixture digests) without declaring them in affected_paths; the archive audit refused
hosted CI. corvid-agent's review named the gap (Blocking 1) and DRIVE-20 routes the
fix through chough.
