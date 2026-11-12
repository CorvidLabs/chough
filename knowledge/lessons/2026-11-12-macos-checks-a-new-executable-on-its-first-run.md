---
id: 2026-11-12-macos-checks-a-new-executable-on-its-first-run
title: macOS checks a new executable on its first run, which can take seconds under load
type: lesson
tags: [lesson, macos, tests, flaky-tests]
date: 2026-11-12
applies_to: [macOS, tests that write stand-in executables, timed waits]
source: .chough/bugs/2026-11-11-verify-lane-test-step-fails-intermittently-under-load-green-on.md
---
# macOS checks a new executable on its first run, which can take seconds under load

macOS assesses every newly written executable the first time it runs, one at a time
across the machine, and remembers the result for that file. On a busy Mac, where
builds and test suites keep writing and running new executables, that first run can
start seconds after `spawn()` returns. The second run of the same file starts in
milliseconds, and so does a hard link or a rename of a file that has already run; a
new file with the same bytes pays the check again.

A test that writes a stand-in script and then times its first call can therefore fail
only under load, with symptoms such as "the command never started" or a run past its
deadline. Run each freshly written stand-in once during setup, outside every timed
wait: a guard line such as `[ -z "${WARM_UP:-}" ] || exit 0` lets that run exit before
the body, so it changes no call log.

Seen in Chough 0.6.0: a fresh stand-in's first run took 3.9 s at the median under
load (its second, 12 ms). The timing native-budget test failed 17 of 20 runs and the
verify lane's test step 6 of 12 under load, and both passed every run once stand-ins
were warmed at install. The bugs' logs and evidence record the reruns.
