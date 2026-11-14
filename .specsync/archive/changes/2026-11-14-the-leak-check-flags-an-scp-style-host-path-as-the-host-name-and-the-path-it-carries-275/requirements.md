---
change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
artifact: requirements
---

# Requirements

REQ-records-006 gains the scp-style form: a value holding `<host>:/<absolute path>` is checked like the path it carries; a home or system root after the colon refuses as a path leak, and the host part counts as the local host name when it matches. One acceptance criterion is added. See `deltas/records.md`.
