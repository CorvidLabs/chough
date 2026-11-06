## MODIFIED

### REQUIREMENT REQ-cli-013

Preferences and run consent. The helper SHALL keep standing project preferences in `.chough/preferences.json`, each with value, source and scope, separately from run consents stored in the checkpoint for the current objective. Repeated validation, monitoring, release routing and monitor initiation SHALL default to ask, ask, on-request and capture-only respectively; the librarian SHALL be on by default, as REQ-cli-015 states, and off only by a recorded preference. A run consent SHALL never be written as a standing preference, and changing the objective SHALL clear run consents. Trace: FLOW-9, FLOW-11, FLOW-12.

Acceptance Criteria
- Granting a run consent leaves `.chough/preferences.json` byte-identical; setting a standing preference requires a source.
- Repeated validation is refused without a consent or standing preference, and a new objective clears the consent.
- Failures found by repeated validation create local bug records that remain discoverable after restart; monitor initiation proposes a native change command only when configured and otherwise captures locally.
- With no preference recorded, `chough pref show` reports the librarian `on` with origin `default`, and no requirement gives it another default.
