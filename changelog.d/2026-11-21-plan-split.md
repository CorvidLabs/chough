- The workflow skill now ships an optional plan-stage fork and the bounded
  test-stage grouping loop, with tests pinning the wording. The plan reference asks
  one question: can the slice be cut into a left half and a right half with no shared
  file? One worker does a slice that cannot. A slice that can gets one manager and two
  workers, each in its own clone and branch. A worker that finishes stops, and neither
  worker imports the other half. The manager leaves out any path outside that worker's
  list, and then merges the two reports. The test reference names the bounded
  loop, the bug grouping and the validate consent. The stages and build references
  agree with the same wording.
