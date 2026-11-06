"""Apply one targeted mutation per new C6 assertion, run its test, expect failure, restore.

Run from the repository root: python3 -I <this file> <log-dir>
Every mutation is an exact, asserted string replacement in one file, restored with
`git checkout -- <file>` whatever happens.
"""
import pathlib
import subprocess
import sys

LOG_DIR = pathlib.Path(sys.argv[1])
LOG_DIR.mkdir(parents=True, exist_ok=True)

MUTATIONS = [
    ("req-cli-002-outranks", "skills/chough-workflow/references/accept.md",
     "always\noutranks a lesson", "usually\nmatters more than a lesson",
     ["cargo", "test", "--locked", "--test", "cli", "--", "--exact",
      "the_skill_states_lesson_labels_sources_and_limits_and_that_decisions_outrank_lessons"]),
    ("req-cli-002-lesson-store", "src/vault.rs",
     'notes.push((format!("{dir}/modules/{module}.md"), content));',
     'notes.push((format!("{dir}/modules/{module}.md"), content + &read(root, &format!("{base}/context.md")).unwrap_or_default()));',
     ["cargo", "test", "--locked", "--test", "cli", "--", "--exact",
      "chough_points_to_lessons_and_keeps_no_lesson_store_index_or_ledger"]),
    ("req-cli-007-second-store", "src/checkpoint.rs",
     "    lock_checkpoint(root)?;\n    remove_orphaned_session_temps(root)?;\n",
     "    lock_checkpoint(root)?;\n    remove_orphaned_session_temps(root)?;\n    std::fs::write(root.join(\".chough/session.mirror.json\"), serialize_checkpoint(session)?)?;\n",
     ["cargo", "test", "--locked", "--test", "stages", "--", "--exact",
      "one_checkpoint_store_and_no_aps_adapter_in_either_mode"]),
    ("req-cli-012-upgrade-note", "docs/cli.md",
     "Older Chough versions that accept only `BUG-NNNN` must be upgraded before",
     "Older Chough versions that accept only `BUG-NNNN` keep working before",
     ["cargo", "test", "--locked", "--test", "bugs", "--", "--exact",
      "the_cli_documentation_tells_older_versions_to_upgrade_for_new_form_bug_ids"]),
    ("req-cli-029-authority-order", "skills/chough-workflow/references/plan.md",
     None, None,
     ["cargo", "test", "--locked", "--test", "workflow_authority", "--", "--exact",
      "the_skill_records_authority_before_approval_and_labels_never_decide"]),
    ("req-cli-029-missing-verdict", "src/cases.rs",
     "        results.len() == case.expected.len(),\n        \"give a verdict for every expected outcome",
     "        results.len() <= case.expected.len(),\n        \"give a verdict for every expected outcome",
     ["cargo", "test", "--locked", "--test", "cases", "--", "--exact",
      "a_skipped_cold_start_is_never_recorded_or_read_as_a_pass"]),
    ("req-cli-029-roadmap-cold-start", "docs/ROADMAP.md",
     "No passing cold-start result is claimed", "The cold-start proof passed",
     ["cargo", "test", "--locked", "--test", "workflow_authority", "--", "--exact",
      "the_roadmap_claims_no_passing_cold_start_for_delivered_work"]),
    ("req-cli-031-walkthrough-order", "docs/walkthrough.md",
     None, None,
     ["cargo", "test", "--locked", "--test", "cli", "--", "--exact",
      "the_walkthrough_begins_with_the_new_project_setup_path"]),
    ("req-cli-045-librarian-off", "src/stage.rs",
     '        Ok((value, _)) if value != "on" => {\n            return json!({"status": "skipped", "reason": "the librarian is off"});\n        }\n        Err(error) => return json!({"status": "refused", "reason": format!("{error:#}")}),\n        Ok(_) => {}\n    }\n    let start = current_cycle_start(root, session);',
     '        Err(error) => return json!({"status": "refused", "reason": format!("{error:#}")}),\n        Ok(_) => {}\n    }\n    let start = current_cycle_start(root, session);',
     ["cargo", "test", "--locked", "--test", "reduce", "--", "--exact",
      "with_the_librarian_off_done_leaves_the_references_and_reports_skipped"]),
    ("req-cli-045-no-reset", "src/stage.rs",
     "    let start = current_cycle_start(root, session);",
     "    let start = current_cycle_start(root, session).or(Some(u64::MAX));",
     ["cargo", "test", "--locked", "--test", "reduce", "--", "--exact",
      "with_no_reset_time_done_drops_no_observation_reference"]),
    ("req-reporting-003-timeout-counted", "examples/measure-report.sh",
     "        open=$(grep -vc ' state=observed ' \"$work/native\" || true)",
     "        open=0",
     ["cargo", "test", "--locked", "--test", "timing", "--", "--exact",
      "measure_report_pairs_two_binaries_on_one_root_and_keeps_native_wait_apart"]),
]


def move_section(text, heading, after):
    """Move the `## heading` section so it follows the `## after` section."""
    parts = text.split("\n## ")
    head, sections = parts[0], parts[1:]
    moving = next(s for s in sections if s.startswith(heading + "\n"))
    sections.remove(moving)
    index = next(i for i, s in enumerate(sections) if s.startswith(after + "\n"))
    sections.insert(index + 1, moving)
    return "\n## ".join([head] + sections)


def special(name, text):
    if name == "req-cli-029-authority-order":
        import re
        words = ("If an actual prior or standing human instruction covers this scope, that "
                 "authorization already exists: record its source, scope and limits in the "
                 "checkpoint and native context, then run the native operation yourself.").split(" ")
        pattern = re.compile(r"\s+".join(re.escape(word) for word in words))
        found = pattern.findall(text)
        assert len(found) == 1, f"authority sentence found {len(found)} times"
        sentence = found[0]
        text = text.replace(sentence, "", 1)
        block_end = text.index("```", text.index("specsync change approve <id> --actor") ) + 3
        return text[:block_end] + "\n\n" + sentence + text[block_end:]
    if name == "req-cli-031-walkthrough-order":
        return move_section(text, "New project", "Install")
    raise AssertionError(name)


summary = []
for name, path, old, new, command in MUTATIONS:
    file = pathlib.Path(path)
    original = file.read_text()
    if old is None:
        mutated = special(name, original)
    else:
        assert original.count(old) == 1, f"{name}: target found {original.count(old)} times"
        mutated = original.replace(old, new, 1)
    assert mutated != original, name
    file.write_text(mutated)
    try:
        diff = subprocess.run(["git", "diff", "--", path], capture_output=True, text=True).stdout
        run = subprocess.run(command, capture_output=True, text=True)
    finally:
        subprocess.run(["git", "checkout", "--", path], check=True)
    killed = run.returncode != 0
    log = LOG_DIR / f"{name}.log"
    log.write_text(f"== mutation {name} in {path}\n{diff}\n== {' '.join(command)}\nexit {run.returncode}\n"
                   f"--- stdout ---\n{run.stdout}\n--- stderr ---\n{run.stderr}\n")
    summary.append(f"{name}: test {'FAILED (mutation caught)' if killed else 'PASSED (mutation SURVIVED)'} exit {run.returncode}")
    print(summary[-1], flush=True)

clean = subprocess.run(["git", "status", "--porcelain", "--untracked-files=no"], capture_output=True, text=True).stdout
print("tree after restore:", "clean" if not clean.strip() else clean)
(LOG_DIR / "summary.txt").write_text("\n".join(summary) + "\n")
