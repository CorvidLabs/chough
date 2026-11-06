"""Targeted mutations for #153's mapping tests: each must fail its new test while the old test passes."""
import pathlib, subprocess, sys
LOG = pathlib.Path(sys.argv[1]); LOG.mkdir(parents=True, exist_ok=True)
def t(name): return ["cargo", "test", "--locked", "--test", "bugs", "--", "--exact", name]
MUTATIONS = [
    ("evidence-gap-recorded-as-requirement-gap", '                *slot = value;', '                *slot = if value == "evidence-gap" { "requirement-gap".into() } else { value };',
     t("every_classification_scope_and_status_is_accepted_and_others_are_refused"), t("a_scope_given_as_a_classification_and_the_reverse_name_the_right_flag")),
    ("wont-fix-still-open", 'const CLOSED: [&str; 3] = ["verified", "duplicate", "wont-fix"];', 'const CLOSED: [&str; 3] = ["verified", "duplicate", "deferred"];',
     t("only_an_open_blocking_bug_blocks_leaving_test"), t("an_open_blocking_bug_sends_work_back_to_build")),
    ("shared-id-bonus-dropped", "    overlap + if shared { 0.25 } else { 0.0 }", "    let _ = shared;\n    overlap",
     t("search_scores_words_and_shared_ids_and_refuses_only_likely_duplicates"), t("a_reproduced_bug_is_deduplicated_and_keeps_added_evidence")),
    ("duplicate-threshold-lowered", "const DUPLICATE_THRESHOLD: f64 = 0.5;", "const DUPLICATE_THRESHOLD: f64 = 0.25;",
     t("search_scores_words_and_shared_ids_and_refuses_only_likely_duplicates"), t("a_reproduced_bug_is_deduplicated_and_keeps_added_evidence")),
    ("report-fixture-hidden", '        fixture = bug.fixture.as_deref().unwrap_or("none recorded"),', '        fixture = "none recorded",',
     t("the_report_shows_every_stored_field_and_none_recorded_for_empty_ones"), t("unverified_out_of_scope_bugs_stay_visible_without_changing_scope")),
]
path = "src/bugs.rs"
for name, old, new, new_test, old_test in MUTATIONS:
    f = pathlib.Path(path); original = f.read_text()
    assert original.count(old) == 1, f"{name}: target found {original.count(old)} times"
    f.write_text(original.replace(old, new, 1))
    try:
        diff = subprocess.run(["git", "diff", "--", path], capture_output=True, text=True).stdout
        a = subprocess.run(new_test, capture_output=True, text=True)
        b = subprocess.run(old_test, capture_output=True, text=True)
    finally:
        subprocess.run(["git", "checkout", "--", path], check=True)
    (LOG / f"{name}.log").write_text(f"== {name}\n{diff}\n== new: {' '.join(new_test)}\nexit {a.returncode}\n{a.stdout}\n{a.stderr}\n== old: {' '.join(old_test)}\nexit {b.returncode}\n{b.stdout}\n{b.stderr}\n")
    compile_error = "error[E" in a.stderr or "error[E" in b.stderr
    line = f"{name}: new test {'FAILED (caught)' if a.returncode else 'PASSED (survived)'}; old test {'passed (gap shown)' if b.returncode == 0 else 'FAILED'}{' COMPILE ERROR' if compile_error else ''}"
    print(line, flush=True)
    with open(LOG / "summary.txt", "a") as out: out.write(line + "\n")
print("tree:", subprocess.run(["git", "status", "--porcelain", "--untracked-files=no", "--", "src"], capture_output=True, text=True).stdout.strip() or "src clean")
