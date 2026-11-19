- A host-matrix harness in `tools/host-matrix/` drives one scripted Chough cycle per
  agent host, one fresh host run per turn, in a throwaway copy of the shipping sample.
  It stands in for every human gate only as `SIMULATED-host-matrix-not-a-human` and
  judges the result only from the files, Git and native status. Its self-test runs a
  scripted stand-in host with no model, and hosted CI runs it on Ubuntu and macOS. Real
  host adapters follow (REQ-cli-066, #218).
