# Report read comparison, 2026-11-02 UTC

The baseline is clean main 825a380. Both freshly built debug binaries were run
against one frozen checkout of that same revision, with identical source and
managed input bytes. The after binary's exact dirty production patch and SHA-256
are retained in report-reads-comparison.tar.gz and its metadata. The original
baseline binary remains byte-identical. Platform: macOS 26.5.2 arm64, Rust/Cargo
1.95.0, HI 0.8.0, Fledge 1.8.0 and SpecSync 6.1.2.

This separate controlled comparison uses a 1-second native read deadline and a
3-second timing-native budget; the earlier 10/30-second baseline is preserved in
report-reads-before.tar.gz. First-process is a new process, with no OS cache purge.
Version order is reversed for repeated runs. Heavy audit/build jobs were quiet
for this comparison. These are two paired observations, not a release-performance
or statistical latency guarantee.

| Command | Before first s | After first s | Before repeated s | After repeated s |
|---|---:|---:|---:|---:|
| next | 3.889 | 2.912 | 3.096 | 1.851 |
| status | 7.111 | 2.780 | 6.285 | 2.187 |
| filtered timing | 1.119 | 1.154 | 1.128 | 1.106 |
| full timing | 3.163 | 3.094 | 3.124 | 3.092 |
| retention preview | 1.145 | 1.093 | 1.106 | 1.125 |

All 20 reporting commands completed with exit zero, but all attempted native
reads timed out. Each next/status/filtered-timing/preview request started one
native read; each full timing started three under its 3-second budget. Completed
native cost is unknown, not zero. These reports did not obtain successful native
qualification. A separate unwrapped SpecSync archived-status read completed in
22.912 seconds and reported archived authenticated-history; its result and raw
response are preserved with the earlier baseline.

The new status trace measured one suite load for 432 digest lookups and 130 IDs;
next measured one load for 138 lookups and 57 IDs. Before-load estimates came from
the source call graph, not file-open instrumentation. Local next/status work fell
in these paired measurements. Timing and retention stayed near their native caps;
no meaningful improvement is claimed for them, and native authentication latency
is not fixed or bypassed by this change.

Next and status stdout were byte-identical before/after in both run orders.
All 1,160 managed fixture files remained byte-identical, and Git stayed clean.
The existing source/artifact/environment checks and native gates are separately
covered by focused regressions and the final mandatory verification lane.

Files: report-reads-comparison-metadata.json and -results.json expose the raw
figures; -preservation.json records input preservation; the tar bundle contains
all stdout/stderr, native start/end logs, reproduction script and source patch.
The original raw recorder's zero native_seconds is an empty sum of completed calls;
report-reads-before-interpretation.json corrects that interpretation without editing
its raw source. No native approval, review, archive, merge, adoption or cleanup
result is implied by a diagnostic run.
