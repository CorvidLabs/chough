"""Targeted mutations for #152: each must fail its new test while the old test passes."""
import pathlib, subprocess, sys
LOG = pathlib.Path(sys.argv[1]); LOG.mkdir(parents=True, exist_ok=True)
NEW_SEL = ["cargo", "test", "--locked", "--test", "cases", "--", "--exact", "requesting_the_copy_localization_track_selects_its_linked_cases"]
OLD_SEL = ["cargo", "test", "--locked", "--test", "cases", "--", "--exact", "selection_is_focused_on_the_affected_scope_and_tracks"]
NEW_KIND = ["cargo", "test", "--locked", "--test", "cases", "--", "--exact", "every_delivery_kind_is_recorded_as_itself_and_a_dry_run_as_a_dry_run"]
OLD_KIND = ["cargo", "test", "--locked", "--test", "cases", "--", "--exact", "schema_2_evidence_accepts_delivery_kinds_and_keeps_dry_runs_distinct"]
NEW_RET = ["cargo", "test", "--locked", "--test", "reduce", "--", "--exact", "delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not"]
MUTATIONS = [
    ("copy-localization-never-selected", "src/cases.rs",
     '} else if case.track != "functional" && !tracks.contains(&case.track) {',
     '} else if case.track != "functional" && (case.track == "copy-localization" || !tracks.contains(&case.track)) {',
     NEW_SEL, OLD_SEL),
    ("live-health-kind-not-accepted", "src/evidence.rs",
     "    LiveHealthObservation,\n    DryRun,",
     "    #[value(skip)]\n    LiveHealthObservation,\n    DryRun,",
     NEW_KIND, OLD_KIND),
    ("deployment-counted-routine", "src/evidence.rs",
     "        Kind::ProductTest | Kind::Structural | Kind::DryRun\n",
     "        Kind::ProductTest | Kind::Structural | Kind::DryRun | Kind::DeploymentObservation\n",
     NEW_RET, OLD_KIND),
    ("dry-run-counted-non-routine", "src/evidence.rs",
     "        Kind::ProductTest | Kind::Structural | Kind::DryRun\n",
     "        Kind::ProductTest | Kind::Structural\n",
     NEW_RET, OLD_KIND),
]
for name, path, old, new, new_test, old_test in MUTATIONS:
    f = pathlib.Path(path); original = f.read_text()
    assert original.count(old) == 1, f"{name}: target found {original.count(old)} times"
    f.write_text(original.replace(old, new, 1))
    try:
        diff = subprocess.run(["git", "diff", "--", path], capture_output=True, text=True).stdout
        a = subprocess.run(new_test, capture_output=True, text=True)
        b = subprocess.run(old_test, capture_output=True, text=True)
    finally:
        subprocess.run(["git", "checkout", "--", path], check=True)
    (LOG / f"{name}.log").write_text(f"== {name} in {path}\n{diff}\n== new: {' '.join(new_test)}\nexit {a.returncode}\n{a.stdout}\n{a.stderr}\n== old: {' '.join(old_test)}\nexit {b.returncode}\n{b.stdout}\n{b.stderr}\n")
    line = f"{name}: new test {'FAILED (caught)' if a.returncode else 'PASSED (survived)'}; old test {'passed (gap shown)' if b.returncode == 0 else 'FAILED'}"
    print(line, flush=True)
    with open(LOG / "summary.txt", "a") as out: out.write(line + "\n")
print("tree:", subprocess.run(["git", "status", "--porcelain", "--untracked-files=no"], capture_output=True, text=True).stdout.strip() or "clean")
