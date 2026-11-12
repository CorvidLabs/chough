---
id: 2026-11-12-build-test-targets-before-a-recorded-run
title: Build test targets before a recorded test run, so its log holds no local path
type: lesson
tags: [lesson, cargo, evidence]
date: 2026-11-12
applies_to: [cargo, recorded test logs, chough cases run]
source: .chough/bugs/2026-11-12-a-project-as-chough-init-leaves-it-conflicts-on-knowledge.md
---
# Build test targets before a recorded test run, so its log holds no local path

A `cargo test` that has to build first prints `Compiling <crate> v<version> (<absolute
path>)`. When that run is recorded as evidence, the line goes into the committed log,
and with it the absolute path of the checkout, which is a local path leak.

Build the targets first with `cargo test --locked --no-run --test <target>`, then record
the run: the recorded output then holds only the test run. Build with the same `cargo`
the recorder runs, or it builds again.

Seen in Chough 0.6.0: a red run at 6ee5ec37 committed a scratch path from a Compiling
line. The path was redacted in place, and the run was repeated with its target built
first, as the bug's log records.
