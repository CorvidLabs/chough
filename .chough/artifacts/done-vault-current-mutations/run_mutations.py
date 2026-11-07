import subprocess, sys, pathlib, json
root = pathlib.Path(sys.argv[1]); logs = pathlib.Path(sys.argv[2]); logs.mkdir(exist_ok=True)
mutants = [
 ("link-from-disk", "src/vault.rs", ".filter(|change| archived.contains(change))", '.filter(|change| { let _ = archived; exists(root, &format!("{dir}/archives/{change}.md")) })'),
 ("no-final-regeneration", "src/stage.rs", "            applied[\"librarian_index\"] = crate::library::refresh_at_done(root);\n        }\n", "            let _ = ();\n        }\n"),
 ("regenerate-only-on-rotation", "src/stage.rs", "                || applied[\"session_archive\"][\"status\"] == \"archived\")", "                || false)"),
]
originals = {}
results = []
try:
    for name, rel, old, new in mutants:
        path = root / rel
        original = originals.setdefault(rel, path.read_text())
        if original.count(old) != 1:
            results.append((name, f"NOT-APPLIED ({original.count(old)})")); continue
        path.write_text(original.replace(old, new))
        r = subprocess.run(["cargo", "test", "--locked", "--no-fail-fast", "--test", "reduce"], cwd=root, capture_output=True, text=True)
        path.write_text(original)
        (logs / f"{name}.log").write_text(r.stdout + r.stderr)
        failed = [l.split()[1] for l in r.stdout.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        compiled = "error[" not in r.stderr and "could not compile" not in r.stderr
        results.append((name, "caught" if r.returncode != 0 and compiled else ("COMPILE-ERROR" if not compiled else "SURVIVED"), failed))
finally:
    for rel, original in originals.items():
        (root / rel).write_text(original)
print(json.dumps(results, indent=1))
