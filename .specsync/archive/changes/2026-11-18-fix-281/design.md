---
change: fix-281
artifact: design
---

# Design

Nothing visual changes. One new flag on an existing command:

```sh
chough checkpoint --no-native-change "Planning only: hi criteria, docs and issues"
```

The session's front matter gains `no_native_change: <reason>`. A stage move's
report and its Timeline entry give the gate as `ungated:no-native-change-required`.
