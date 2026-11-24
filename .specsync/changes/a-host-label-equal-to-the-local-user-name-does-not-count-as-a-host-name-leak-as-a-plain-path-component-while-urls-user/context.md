---
change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
artifact: context
---

# Context

On this Mac the host name is `Tob.hsd1.co.comcast.net` with LocalHostName `Tob`, and the
local user is `tob`. `chough doctor` on the tob repository reported six `host-name`
findings, all on the same lines: the case-definition path component
`Sources/tob/` in `.chough/cases/image.comfy-backend.cases.toml`. The host rule counts
a short all-letter label as a host name inside a path, and the folder `tob` there is
the product's source directory, not a machine. The user-name rule deliberately does not
count such a folder ("a relative path … holds no user name"), so the host rule re-flags
exactly what the user rule let through. The same name still leaks meaningfully in a URL,
as `<user>@<host>`, or as the host of an scp-style remote path, and the full dotted and
`.local` names keep matching as they do now.
