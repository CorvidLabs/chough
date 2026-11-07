import subprocess, sys, pathlib, json
root = pathlib.Path(sys.argv[1]); logs = pathlib.Path(sys.argv[2]); logs.mkdir(exist_ok=True)
mutants = [
 ("count-direct-write-as-unrecognised", "src/timing.rs", "        if kind == stage::DIRECT_WRITE {\n            continue;\n        }\n", "", ["direct_writes"]),
 ("drop-rationale-from-argv", "src/change.rs", "            if let Some(rationale) = rationale {\n                arguments.extend([\"--rationale\".into(), rationale]);\n            }\n", "            let _ = rationale;\n", ["change"]),
]
originals = {}; results = []
try:
    for name, rel, old, new, targets in mutants:
        path = root / rel
        original = originals.setdefault(rel, path.read_text())
        if original.count(old) != 1:
            results.append((name, f"NOT-APPLIED ({original.count(old)})")); continue
        path.write_text(original.replace(old, new))
        args = ["cargo", "test", "--locked", "--no-fail-fast"] + [a for t in targets for a in ("--test", t)]
        r = subprocess.run(args, cwd=root, capture_output=True, text=True)
        path.write_text(original)
        (logs / f"{name}.log").write_text(r.stdout + r.stderr)
        failed = [l.split()[1] for l in r.stdout.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        compiled = "error[" not in r.stderr and "could not compile" not in r.stderr
        results.append((name, "caught" if r.returncode != 0 and compiled else ("COMPILE-ERROR" if not compiled else "SURVIVED"), failed))
finally:
    for rel, original in originals.items():
        (root / rel).write_text(original)
print(json.dumps(results))
