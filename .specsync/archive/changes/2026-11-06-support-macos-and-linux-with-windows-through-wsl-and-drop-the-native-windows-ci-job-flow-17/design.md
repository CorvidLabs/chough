---
change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
artifact: design
---

# Design

No visual or interaction design applies.

The product job keeps its name `Product / ${{ matrix.os }}`, so `Product / ubuntu-latest` and `Product / macos-latest` keep the same names. No other job changes.

The new package test reads `.github/workflows/ci.yml`, `fledge.toml`, README.md, docs/ci.md and docs/integrations.md, with CRLF normalized to LF. Each check that fails names its file:

- the product job's matrix is exactly `[ubuntu-latest, macos-latest]`, and no line of ci.yml names Windows;
- the `native-gates` job runs on `ubuntu-latest` and runs `fledge lanes run verify`, the verify lane includes `exercise`, and the exercise task runs `examples/feature-bug-loop.sh`. Together these mean a Linux build runs the exercise on every pull request;
- README's Tested platforms names Ubuntu and macOS for hosted CI and says Windows is supported through WSL 2 with Ubuntu; README, docs/ci.md and docs/integrations.md list no `Product / windows-latest` and claim no Windows CI.
