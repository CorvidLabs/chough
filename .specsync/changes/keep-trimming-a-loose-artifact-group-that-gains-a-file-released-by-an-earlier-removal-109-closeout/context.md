---
change: keep-trimming-a-loose-artifact-group-that-gains-a-file-released-by-an-earlier-removal-109-closeout
artifact: context
---

# Context

#109's closeout ran the move into done on Chough's own repository, and the trim was refused partway: "protections changed during removal (artifact:handoff-cold-start: its files changed)". It restored the 172 files it had moved or removed (KNOW-10). Removing the observation records that shared a log inside the `handoff-cold-start` folder made that log loose. The folder's loose group then listed it before its planned files, and the per-file re-check required the first file to be unchanged. That requirement fits an observation set, whose record is removed last, but not a loose group. For a loose group, the re-check now requires only that every planned file still present is still loose and the group still unprotected. Only planned files are ever removed, so the released file stays.
