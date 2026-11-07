import subprocess, sys, pathlib, json
root = pathlib.Path(sys.argv[1]); logs = pathlib.Path(sys.argv[2]); logs.mkdir(exist_ok=True)
mutants = [
 ("skip-digest-check", "src/gates.rs", "if sha256(&summary(root, &session, &listed.gate)) != listed.digest {", "if false && sha256(&summary(root, &session, &listed.gate)) != listed.digest {"),
 ("skip-listed-number-check", "src/gates.rs", "let Some(listed) = gates.iter().find(|gate| gate.number == number).cloned() else {", "let Some(listed) = gates.iter().find(|_| true).cloned() else {"),
 ("close-every-listed-gate", "src/gates.rs", "    for (index, listed) in checked.iter().enumerate() {", "    let checked: Vec<Listed> = load(root, &args.list).unwrap_or_default();\n    for (index, listed) in checked.iter().enumerate() {"),
 ("drop-summary-digest", "src/gates.rs", '            .args(["--summary-digest", &listed.digest, "--list", &args.list])\n', ""),
 ("allow-verifying-gate", "src/gates.rs", "        if verifying {", "        if false && verifying {"),
 ("one-relay-field-alone", "src/stage.rs", '        _ => anyhow::bail!(\n            "invalid transition payload: relayed_by.summary_digest and relayed_by.list go together"\n        ),', "        _ => {}"),
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
        r = subprocess.run(["cargo", "test", "--locked", "--no-fail-fast", "--test", "gates"], cwd=root, capture_output=True, text=True)
        path.write_text(original)
        (logs / f"{name}.log").write_text(r.stdout + r.stderr)
        failed = [l.split()[1] for l in r.stdout.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        compiled = "error[" not in r.stderr and "could not compile" not in r.stderr
        results.append((name, "caught" if r.returncode != 0 and compiled else ("COMPILE-ERROR" if not compiled else "SURVIVED"), failed))
finally:
    for rel, original in originals.items():
        (root / rel).write_text(original)
print(json.dumps(results, indent=1))
