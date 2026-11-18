---
change: fix-274
artifact: testing
---

# Testing

`cargo test --locked --test cli exercise_stops_at_a_refused_bug_update` runs the script's own helpers and its "rerun after the fix" lines, cut out between their markers, with a stand-in `chough` whose `bug update` is refused. The script must exit non-zero with the refusal and run no later line. On main it carries on and exits 0. `cargo test --locked --test cli exercise_pipes_no_command_output_straight_into_jq` reads the script and requires that no line pipes a `$chough` or `specsync` command's output directly into another command. On main several do. Case `TC-cli-exercise-stops-at-a-refused-command` binds the first. The verify lane's `exercise` step runs the whole script.
