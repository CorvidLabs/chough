## MODIFIED

### REQUIREMENT REQ-cli-045

Closed-cycle references at done. When `chough stage advance` moves into done and its save succeeds, with the librarian on, the helper SHALL, before the trim of REQ-cli-041, drop from the checkpoint's `observations` every ID whose record was observed before the current cycle began (the newest recorded reset) or that has no record, keeping every observation reference from the current cycle. Done closes the cycle, so the same step SHALL drop every entry of the checkpoint's file and design references, whether or not a reset time can be read (KNOW-21). It SHALL report the result as `applied.checkpoint_references` with status `dropped` and the dropped observation IDs, file references and design references as `dropped`, `dropped_files` and `dropped_design`, `unchanged`, `skipped` (librarian off, or nothing to drop and no reset time can be read), or `refused` with its reason. When no reset time can be read, no observation reference SHALL be dropped. A referenced observation record that exists but cannot be read SHALL keep every reference and refuse the step. A refusal SHALL NOT undo or refuse the move into done. Retention SHALL ignore the file and design references of an archived session, which is done, and SHALL NOT rewrite it. The archives, the retention ledger and the knowledge directory keep the dropped IDs, and the native archive keeps the paths the change touched. Trace: KNOW-16, KNOW-21, KNOW-13.

Acceptance Criteria
- With the current cycle begun 15 days ago, `stage advance` into done drops a reference to a record observed before it, keeps one observed after it, reports the dropped ID, and the trim may then remove the dropped record.
- With no reset recorded, `stage advance` into done drops every file and design reference, reports them as `dropped_files` and `dropped_design`, keeps the observation references, and a cold loose folder a dropped file reference held is then trimmed.
- With the librarian off, the references are unchanged and reported `skipped`.
- A cold loose folder named only by an archived session's file and design references is trimmed, and the archived session file is unchanged.
