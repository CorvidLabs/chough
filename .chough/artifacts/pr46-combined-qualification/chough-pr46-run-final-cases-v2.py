import pathlib,subprocess,json,hashlib,os,tomllib,time
root=pathlib.Path('/tmp/chough-tickets-20261102-JAIhPp');os.chdir(root);env=dict(os.environ);env['PATH']='/Users/leif/.cargo/bin:'+env['PATH']
cli=str(root/'target/debug/chough');dest=root/'.chough/artifacts/pr46-combined-qualification';assert dest.exists()
def freeze(path,data):
 if path.exists():assert path.read_bytes()==data,str(path)
 else:path.write_bytes(data)
for name in ['verify','msrv']:
 for suffix in ['.log','-result.json']:freeze(dest/(name+suffix),pathlib.Path('/tmp/chough-pr46-combined-'+name+suffix).read_bytes())
versions=json.loads((dest/'verify-result.json').read_text())['versions'];binary={'rust1.95':{'path':str(root/'target/debug/chough'),'sha256':hashlib.sha256((root/'target/debug/chough').read_bytes()).hexdigest()},'rust1.88':{'path':'/tmp/chough-pr46-final-combined-msrv-target/debug/chough','sha256':hashlib.sha256(pathlib.Path('/tmp/chough-pr46-final-combined-msrv-target/debug/chough').read_bytes()).hexdigest()},'note':'Each cargo test suite used its version-specific CARGO_BIN_EXE_chough fixture binary. The qualification runner input.binary_sha256 describes the ambient1.95CLI; this manifest separately identifies the MSRV fixture binary.'};freeze(dest/'binary-identities.json',(json.dumps(binary,indent=2)+'\n').encode())
for p in ['/tmp/chough-pr46-run-combined-qualification.py','/tmp/chough-pr46-run-final-cases-v2.py']:freeze(dest/pathlib.Path(p).name,pathlib.Path(p).read_bytes())
# Static agent-assisted ordering check: actual source text and installed copies only.
paths=['skills/chough-workflow/references/plan.md','skills/chough-workflow/references/testing.md','skills/chough-workflow/references/accept.md'];text={p:' '.join(pathlib.Path(p).read_text().split()) for p in paths}
assert 'list human gates, and running or recording Chough cases, as plain text rather than checkboxes' in text[paths[0]]
assert 'A requirement the delta adds reaches the companion only when `change check` materializes it' in text[paths[0]]
assert 'Finish the implementation tasks, run that check, then add, run and record those cases' in text[paths[1]]
assert 'Running or recording cases is never a `tasks.md` checkbox' in text[paths[1]]
assert 'It refuses while `tasks.md` has an unchecked box, so the cases in step 2 are never a box there' in text[paths[2]]
assert all('\u2013' not in pathlib.Path(p).read_text() and '\u2014' not in pathlib.Path(p).read_text() for p in paths)
def records():return {str(p):hashlib.sha256(p.read_bytes()).hexdigest() for d in ['.chough','.specsync'] for p in sorted(pathlib.Path(d).rglob('*')) if p.is_file()}
before=records();args=[cli,'skill','status','--host','openai','--host','claude'];r=subprocess.run(args,env=env,capture_output=True,timeout=60);assert r.returncode==0,r.stderr.decode();after=records();assert before==after
status=json.loads(r.stdout);assert len(status['hosts'])==2 and all(f['state']=='current' for h in status['hosts'] for f in h['files']);(dest/'ordering-skill-status.json').write_bytes(r.stdout)
for h in status['hosts']:
 for f in h['files']:assert (root/h['directory']/f['file']).read_bytes()==(root/'skills/chough-workflow'/f['file']).read_bytes()
note={'method':'agent-assisted','environment':'repository-text','case':'TC-cli-skill-orders-the-change-check-before-new-requirement-cases','checks':[{'expected_index':1,'verdict':'pass','observed':'Plan keeps case execution/recording out of task checkboxes and describes requirement materialization.'},{'expected_index':2,'verdict':'pass','observed':'Testing orders completed implementation tasks, scoped native check, then new-requirement cases.'},{'expected_index':3,'verdict':'pass','observed':'Acceptance step1 names unchecked-task refusal and excludes case recording from task boxes.'},{'expected_index':4,'verdict':'pass','observed':'The three affected guidance files contain neither em nor en dash.'},{'expected_index':5,'verdict':'pass','observed':'Real skill status reports every file in both installed host copies current; exact file comparisons agree.'}],'canonical_sha256':{p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest() for p in paths},'managed_records_unchanged_by_read':True,'managed_record_count':len(before),'limits':['Static repository-text and installed-copy consistency only.','No external agent or cold-start run; no human approval or compliance result inferred.']};(dest/'ordering-review.json').write_text(json.dumps(note,indent=2)+'\n')
cs=tomllib.loads(pathlib.Path('.chough/cases/cli.cases.toml').read_text())['case'];selected=[c for c in cs if c['id'].startswith(('TC-cli-history-','TC-cli-retention-revalidation','TC-cli-report-')) or c['id'] in ['TC-cli-agent-native-workflow-upgrade','TC-cli-native-workflow-bundle-consistency','TC-cli-skill-orders-the-change-check-before-new-requirement-cases']];assert len(selected)==16
results=[]
for c in selected:
 ident=c['id']
 if c['method']=='automated':args=[cli,'cases','run',ident,'--environment','local-macos-arm64']
 else:
  assert ident==note['case'];args=[cli,'cases','record',ident,'--environment','repository-text','--method','agent-assisted','--summary','Static ordering review passed all5expected outcomes; real skill status and byte comparisons show both24-file host copies current; no managed record changed by the read.']
  for n in range(1,6):args+=['--verdict',str(n)+'=pass']
  args+=['--artifact','.chough/artifacts/pr46-combined-qualification/ordering-review.json','--artifact','.chough/artifacts/pr46-combined-qualification/ordering-skill-status.json','--limitation','Static repository-text and installed-copy review only. No external-agent or cold-start run; no human acceptance or compliance result is inferred.']
 start=time.monotonic();r=subprocess.run(args,env=env,capture_output=True,timeout=300);path=pathlib.Path('/tmp/chough-pr46-combined-'+ident+'.json');path.write_bytes(r.stdout);path.with_suffix('.stderr').write_bytes(r.stderr)
 assert r.returncode==0,(ident,r.returncode,r.stderr.decode(),r.stdout.decode())
 result=json.loads(r.stdout);assert result['outcome']=='passed',result
 results.append({'case':ident,'environment':'local-macos-arm64' if c['method']=='automated' else 'repository-text','method':c['method'],'command':args,'exit_code':r.returncode,'elapsed_seconds':round(time.monotonic()-start,3),'result':result})
 pathlib.Path('/tmp/chough-pr46-combined-cases.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps({'case':ident,'outcome':result['outcome'],'observation':result['observation']}),flush=True)
(dest/'cases.json').write_text(json.dumps(results,indent=2)+'\n')
