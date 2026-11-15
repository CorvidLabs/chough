## MODIFIED

### REQUIREMENT REQ-cli-001

The helper SHALL initialize a resumable session and discoverable workflow skill
without replacing existing files. Trace: LIFE-4, LIFE-6.

Acceptance: repeated init preserves edited session and skill bytes; missing bundle
files can be added without overwriting neighbors. A session whose files were
edited since init is left byte-identical on a later init, and init never removes
or rewrites a file the user or a previous run created.

Acceptance Criteria
- repeated init preserves edited session and skill bytes
- missing bundle files can be added without overwriting neighbors
- a session whose files were edited since init stays byte-identical on later init
