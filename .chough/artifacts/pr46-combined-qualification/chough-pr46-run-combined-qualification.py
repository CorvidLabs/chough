import pathlib,sys,subprocess,json,os,time,datetime,hashlib,re
root=pathlib.Path('/tmp/chough-tickets-20261102-JAIhPp');os.chdir(root)
kind=sys.argv[1];assert kind in ('verify','msrv')
env=dict(os.environ);env['PATH']='/Users/leif/.cargo/bin:'+env['PATH']
if kind=='msrv':env['CARGO_TARGET_DIR']='/tmp/chough-pr46-final-combined-msrv-target'
command=['fledge','lanes','run','verify'] if kind=='verify' else ['cargo','+1.88.0','test','--locked','--offline']
log=pathlib.Path('/tmp/chough-pr46-combined-'+kind+'.log');result=pathlib.Path('/tmp/chough-pr46-combined-'+kind+'-result.json')
def output(args):return subprocess.check_output(args,env=env,text=True).strip()
identity={'head':output(['git','rev-parse','HEAD']),'origin_main':output(['git','rev-parse','origin/main']),'branch':output(['git','branch','--show-current']),'source_sha256':{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for d in ['src','tests','skills/chough-workflow'] for p in sorted(pathlib.Path(d).rglob('*')) if p.is_file()},'cases_sha256':hashlib.sha256(pathlib.Path('.chough/cases/cli.cases.toml').read_bytes()).hexdigest(),'binary_sha256':hashlib.sha256(pathlib.Path('target/debug/chough').read_bytes()).hexdigest()}
versions={name:output(args) for name,args in [('rust', ['rustc','--version'] if kind=='verify' else ['rustc','+1.88.0','--version']),('cargo',['cargo','--version'] if kind=='verify' else ['cargo','+1.88.0','--version']),('hi',['hi','--version']),('fledge',['fledge','--version']),('specsync',['specsync','--version'])]}
started=datetime.datetime.now(datetime.timezone.utc).isoformat();clock=time.monotonic()
with log.open('wb') as f:
 f.write((json.dumps({'command':command,'versions':versions,'started_utc':started,'target':env.get('CARGO_TARGET_DIR'),'input':identity},indent=2)+'\n').encode());f.flush()
 try:p=subprocess.run(command,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=1200);code=p.returncode
 except subprocess.TimeoutExpired:code=124;f.write(b'\nQualification command exceeded1200seconds.\n')
s=log.read_text();test_text=s
if kind=='verify':
 a=s.find('Running task: test');b=s.find('Step 3 done',a)
 test_text=s[a:b] if a>=0 and b>=0 else ''
rows=[{'passed':int(a),'failed':int(b),'ignored':int(c)} for a,b,c in re.findall(r'^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',test_text,re.M)]
data={'command':command,'versions':versions,'started_utc':started,'finished_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'elapsed_seconds':round(time.monotonic()-clock,3),'environment':'local-macos-arm64' if kind=='verify' else 'local-macos-arm64-rust-1.88.0','target':env.get('CARGO_TARGET_DIR'),'exit_code':code,'log':str(log),'input':identity,'test_suites':rows,'observed_passed_tests':sum(x['passed'] for x in rows),'observed_failed_tests':sum(x['failed'] for x in rows),'observed_ignored_tests':sum(x['ignored'] for x in rows),'lane_completed':'Lane verify completed (9 steps' in s if kind=='verify' else None}
result.write_text(json.dumps(data,indent=2)+'\n');print(json.dumps({k:data[k] for k in ['command','exit_code','elapsed_seconds','log','observed_passed_tests','observed_failed_tests','observed_ignored_tests','lane_completed']},indent=2),flush=True)
raise SystemExit(code)
