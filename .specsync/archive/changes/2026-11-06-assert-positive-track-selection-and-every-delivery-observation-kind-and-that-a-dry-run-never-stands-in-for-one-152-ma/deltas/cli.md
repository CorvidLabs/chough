## MODIFIED

### REQUIREMENT REQ-cli-014

Optional release routing. The helper SHALL advance from accept to done and never to release, and SHALL allow entering release only with a release run consent or a standing release preference. Release observations SHALL use distinct kinds for publication, installation, deployment, live health and dry runs, and a dry run SHALL never satisfy a delivery kind. Trace: FLOW-11, LIFE-7.

Acceptance Criteria
- Advancing from accept never enters release, and entering release without authorization is refused.
- A dry-run observation is reported as a dry run, not as deployment or live health.
- Publication, installation, deployment and live-health observations are each accepted and reported with their own kind, and each is key-item proof: retention protects it with its kind as the reason and moves it into the vault when its archive turns cold. A dry run's kind is never a protection, and a superseded dry run is trimmed with no vault copy.
