import pathlib,tempfile,subprocess,os,json,hashlib,time
R=pathlib.Path('/tmp/chough-pr51-finish-SLSKxX');O=pathlib.Path(tempfile.mkdtemp(prefix='chough51-checkpoint-writer-before-',dir='/tmp'));E={**os.environ,'GIT_OPTIONAL_LOCKS':'0'}
def sha(b):return hashlib.sha256(b).hexdigest()
files={}
for name in ['src/checkpoint.rs','src/history.rs','tests/cli.rs']:
 b=(R/name).read_bytes();p=O/'inputs'/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b);files[name]=sha(b)
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,env=E,text=True).strip();patch=subprocess.check_output(['git','diff','--','tests/cli.rs','src/checkpoint.rs','src/history.rs'],cwd=R,env=E);(O/'regression.patch').write_bytes(patch)
argv=['fledge','run','test','--','--locked','--test','cli','checkpoint_writer_','--','--nocapture'];metadata={'head':head,'files_sha256':files,'patch_sha256':sha(patch),'argv':argv,'binary_before_sha256':sha((R/'target/debug/chough').read_bytes()),'production_changes_before':subprocess.check_output(['git','diff','--name-only','--','src/checkpoint.rs','src/history.rs'],cwd=R,env=E,text=True).splitlines()};assert not metadata['production_changes_before'];(O/'before.json').write_text(json.dumps(metadata,indent=2)+'\n');pathlib.Path('/tmp/chough51-checkpoint-writer-before-dir.txt').write_text(str(O)+'\n');print('CAPTURE_DIR',O,flush=True)
start=time.monotonic()
with (O/'red.log').open('wb') as f:res=subprocess.run(argv,cwd=R,env=E,stdout=f,stderr=subprocess.STDOUT,timeout=240)
metadata['exit_code']=res.returncode;metadata['duration_seconds']=round(time.monotonic()-start,3);metadata['binary_after_sha256']=sha((R/'target/debug/chough').read_bytes());metadata['log_sha256']=sha((O/'red.log').read_bytes());metadata['files_sha256_after']={name:sha((R/name).read_bytes()) for name in files};assert metadata['files_sha256_after']==files
(O/'result.json').write_text(json.dumps(metadata,indent=2)+'\n');print(json.dumps(metadata,indent=2));print((O/'red.log').read_text()[-10000:])
