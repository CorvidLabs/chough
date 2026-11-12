---
id: 2026-11-07-history-flags-direct-specsync-lifecycle-writes-183
title: History flags direct SpecSync lifecycle writes (#183)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-07
time: 2026-11-07T01:29:05Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-07-flag-direct-specsync-lifecycle-writes-in-history-183-drive-21.md
  - .specsync/archive/changes/2026-11-07-flag-direct-specsync-lifecycle-writes-in-history-183-drive-21/
record: .chough/archive/sessions/2026-11-07-flag-direct-specsync-lifecycle-writes-in-history-183-drive-21.md
actor:
  - agent: claude-code
    terminal: t382
---
# History flags direct SpecSync lifecycle writes (#183)

As slice 3 of DRIVE-21 (#183), Chough records direct SpecSync lifecycle writes in its history, and the release note names the oldest Chough that reads the new event. Findings are written only on a successful stage move, because nothing may write Chough's records between check --commit and finalize; next and status only show them. REQ-cli-057 now lets change steps write their untracked baseline locally. Recording from next or status, watching other changes, and refusing on a finding were rejected.

The maintainer approved the definition, asking for it to land before the 0.5.0 cut so 0.5.0 reads the new event, and passed the GitHub review of PR #186. The story closed as done on 7 November 2026.

Read more: [the story](../../.chough/archive/sessions/2026-11-07-flag-direct-specsync-lifecycle-writes-in-history-183-drive-21.md) · [the archived change](../../.specsync/archive/changes/2026-11-07-flag-direct-specsync-lifecycle-writes-in-history-183-drive-21/)
