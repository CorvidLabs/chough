# Final report read comparison

This comparison uses the final typed-error-preserving debug binary and a new unchanged checkout of main825a380. All inputs,1-second native deadlines and3-second timing-native budgets are identical between each old/new pair. First process does not imply a purged OS cache. Version order is reversed for repeated runs. Heavy root checks were quiet.

| Command | Before first s | After first s | Before repeated s | After repeated s |
|---|---:|---:|---:|---:|
| next | 4.281 | 2.937 | 3.487 | 2.019 |
| status | 7.909 | 2.929 | 6.345 | 2.621 |
| timing-filtered | 1.159 | 1.112 | 1.121 | 1.223 |
| timing-all | 3.162 | 3.231 | 3.178 | 3.191 |
| retention-preview | 1.114 | 1.173 | 1.179 | 1.133 |

All20CLIcommands completed. Every native attempt timed out: one read per next/status/filtered timing/retention preview and three per full timing under the timing budget. Native completed cost is null/unknown, not zero. These are capped local-work measurements, not successful native qualification or release performance guarantees. The earlier direct unwrapped native read completed in22.912seconds with authenticated history and remains separate evidence.

Next/status stdout was byte-identical in both version orders. All1160managedfiles remained byte-identical and Git stayed clean. The new traces report one suite load for432statuslookups130IDs and138nextlookups57IDs. Existing source/artifact/environment checks and native gates remain intact. The first comparison and intermediate201-test runs preceded the cached-error formatting fix and are retained as historical results; final qualification is recorded separately.

Raw stdout/stderr, native invocation logs, reproduction script, exact source patch and binary identities are in report-reads-final-comparison.tar.gz. Metadata/results/preservation JSON files expose the raw figures without extracting the bundle. The after binary SHA-256 is `3bd7aec0ec39101e00ceabf288245cc5ce5fe4a84e88d8684a7d012279b71acb`. The raw production patch SHA-256 is `5ae58c8c3a62daae88de92c0ff096508f5c124ba058f715a96be807edbe7fdf6`. No archive, acceptance, merge, adoption or cleanup is implied.
