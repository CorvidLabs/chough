import subprocess, sys, pathlib, json
root = pathlib.Path(sys.argv[1]); logs = pathlib.Path(sys.argv[2]); logs.mkdir(exist_ok=True)
mutants = [
 ("skip-pre-step-diff", "src/direct_writes.rs", "local.pending.extend(before.finding);", "let _ = before.finding;"),
 ("skip-post-step-baseline", "src/direct_writes.rs", "local.baseline = fingerprint(root, &before.change);", "let _ = &before.change;"),
 ("record-from-next", "src/direct_writes.rs", 'json!({"baseline": true, "pending": pending})', '{ let _ = settle(root, Some(change)); json!({"baseline": true, "pending": pending}) }'),
 ("drop-pending-at-move", "src/direct_writes.rs", "let mut findings = local.pending;", "let mut findings: Vec<DirectWrite> = { let _ = local.pending; vec![] };"),
 ("flag-at-first-sight", "src/direct_writes.rs", "let Ok(Some(local)) = read_local(root, change) else {\n        return vec![];\n    };", "let local = read_local(root, change).ok().flatten().unwrap_or(Local { schema_version: 1, ..Local::default() });"),
 ("direct-write-on-any-kind", "src/stage.rs", "        transition.kind == DIRECT_WRITE,\n", "        true,\n"),
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
        r = subprocess.run(["cargo", "test", "--locked", "--no-fail-fast", "--test", "direct_writes", "--test", "change"], cwd=root, capture_output=True, text=True)
        path.write_text(original)
        (logs / f"{name}.log").write_text(r.stdout + r.stderr)
        failed = [l.split()[1] for l in r.stdout.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        compiled = "error[" not in r.stderr and "could not compile" not in r.stderr
        results.append((name, "caught" if r.returncode != 0 and compiled else ("COMPILE-ERROR" if not compiled else "SURVIVED"), failed))
finally:
    for rel, original in originals.items():
        (root / rel).write_text(original)
print(json.dumps(results, indent=1))
