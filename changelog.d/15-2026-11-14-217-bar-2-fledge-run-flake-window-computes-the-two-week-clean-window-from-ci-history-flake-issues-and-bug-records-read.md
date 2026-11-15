- `fledge run flake-window` computes the 1.0 flake window from CI history (REQ-cli-062,
  #217). It reads the CI workflow's runs and jobs, the issues labelled `flake` and
  `.chough/bugs/`, writes nothing, and prints a JSON receipt: `clean`, `running` (with
  the latest restart and when the window can first be clean) or `blocked`. A red or
  partly skipped main run, any rerun, a filed flake and a day without a green main run
  on both macOS and Linux restart the window. It exits 2 when GitHub cannot be read in
  full. Its fixtures run in the `CI gate fixtures` job.
