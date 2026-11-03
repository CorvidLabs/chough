import pathlib,tempfile,subprocess,os,json,hashlib,time,shutil
R=pathlib.Path('/tmp/chough-pr51-finish-SLSKxX');O=pathlib.Path(tempfile.mkdtemp(prefix='chough51-checkpoint-writer-final-',dir='/tmp'));E={**os.environ,'PATH':'/Users/leif/.cargo/bin:'+os.environ.get('PATH',''),'GIT_OPTIONAL_LOCKS':'0'}
def sha(b):return hashlib.sha256(b).hexdigest()
def write(n,b):
 p=O/n;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
def info():return {p:sha((R/p).read_bytes()) for p in ['src/checkpoint.rs','src/history.rs','tests/cli.rs']}
fixed=(R/'src/checkpoint.rs').read_bytes();old=pathlib.Path('/tmp/chough51-checkpoint-writer-before-0jsk7fvc/inputs/src/checkpoint.rs').read_bytes();test=(R/'tests/cli.rs').read_bytes();history=(R/'src/history.rs').read_bytes();assert sha(fixed)=='b7a0aebc142380a47e38dca540bac9e2d1f17d8efeaf697271a0b92da5abde32';assert sha(old)=='f4ccbd705196680da028af17b21a63dcef29ed2fdc931e4d76a9cfe43cadc750'
versions={}
for name,argv in [('rustc',['rustc','--version']),('cargo',['cargo','--version']),('clippy',['cargo','clippy','--version']),('fledge',['fledge','--version'])]:
 versions[name]={'resolved':shutil.which(argv[0],path=E['PATH']),'version':subprocess.check_output(argv,cwd=R,env=E,text=True).strip()}
assert '1.95.0' in versions['rustc']['version'] and '1.95.0' in versions['cargo']['version'] and '0.1.95' in versions['clippy']['version'] and versions['fledge']['version']=='fledge 1.8.0'
argv=['fledge','run','test','--','--locked','--test','cli','checkpoint_writer_','--','--nocapture'];meta={'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,env=E,text=True).strip(),'tool_versions':versions,'PATH_prefix':'/Users/leif/.cargo/bin','focused_argv':argv,'final_test_sha256':sha(test),'history_sha256':sha(history),'red_source_sha256':sha(old),'green_source_sha256':sha(fixed),'phases':[]}
pathlib.Path('/tmp/chough51-checkpoint-writer-final-dir.txt').write_text(str(O)+'\n');write('setup.json',(json.dumps(meta,indent=2)+'\n').encode());print('CAPTURE_DIR',O,flush=True)
def run(phase,name,cmd):
 for p in ['src/checkpoint.rs','src/history.rs','tests/cli.rs']:write(phase+'/inputs/'+p,(R/p).read_bytes())
 before=info();start=time.monotonic();logpath=O/phase/(name+'.log')
 with logpath.open('wb') as f:r=subprocess.run(cmd,cwd=R,env=E,stdout=f,stderr=subprocess.STDOUT,timeout=240)
 after=info();assert before==after;assert (R/'tests/cli.rs').read_bytes()==test and (R/'src/history.rs').read_bytes()==history
 out={'phase':phase,'name':name,'argv':cmd,'exit_code':r.returncode,'seconds':round(time.monotonic()-start,3),'files_sha256':before,'files_unchanged_during_run':True,'binary_sha256_after':sha((R/'target/debug/chough').read_bytes()),'log_sha256':sha(logpath.read_bytes())};meta['phases'].append(out);write('result.json',(json.dumps(meta,indent=2)+'\n').encode());print(json.dumps(out),flush=True);return r.returncode,logpath.read_text()
try:
 (R/'src/checkpoint.rs').write_bytes(old)
 rc,log=run('before','focused',argv)
 assert rc!=0 and '0 passed; 3 failed; 0 ignored' in log and 'refused checkpoint changed managed records' in log
finally:
 (R/'src/checkpoint.rs').write_bytes(fixed)
 meta['fixed_source_restored_in_finally']=sha((R/'src/checkpoint.rs').read_bytes())==sha(fixed);write('source-restored.json',(json.dumps({'restored':meta['fixed_source_restored_in_finally'],'files_sha256':info()},indent=2)+'\n').encode());assert meta['fixed_source_restored_in_finally'];print('FIXED_SOURCE_RESTORED',flush=True)
rc,log=run('after','focused',argv);assert rc==0 and '3 passed; 0 failed; 0 ignored' in log
for name in ['fmt','check','lint']:
 rc,_=run('after',name,['fledge','run',name]);assert rc==0
assert (O/'before/inputs/tests/cli.rs').read_bytes()==(O/'after/inputs/tests/cli.rs').read_bytes()==test
meta['identical_test_bytes_red_and_green']=True;meta['new_test_count']=3;meta['case_scope']='Observation and serialized-byte refusal with clear/open gates across empty/selected/legacy cursors. Final-cursor exactly131072success,131073refusal; no-event null-head exactly131072success. All managed record paths/bytes preserved on refused updates. Red first variants stop before selected/legacy; green covers all.';meta['limits']='No broader I/O/collision/cycle-capacity transaction guarantee. No native/evidence/checkpoint/archive edits or commits by worker.'
write('result.json',(json.dumps(meta,indent=2)+'\n').encode());write('production.patch',subprocess.check_output(['git','diff','--','src/checkpoint.rs','src/history.rs','tests/cli.rs'],cwd=R,env=E));shutil.copyfile('/tmp/chough51-checkpoint-writer-final-pair.py',O/'capture.py');print('FINAL_PAIR_COMPLETE',json.dumps({'directory':str(O),'red_failed':3,'green_passed':3,'tests_sha256':sha(test),'source_sha256':sha(fixed),'result_sha256':sha((O/'result.json').read_bytes())}),flush=True)
