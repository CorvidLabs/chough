import pathlib,subprocess,os,json,tempfile,hashlib,re
root=pathlib.Path('/tmp/chough-tickets-20261102-JAIhPp');os.chdir(root);env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
def git(*a):return subprocess.check_output(['git',*a],env=env)
assert git('rev-parse','HEAD').decode().strip()=='b04a8d517837c28e0912bb8af832845fc9a5bfaa'
for kind in ['verify','msrv']:
 d=json.loads(pathlib.Path('/tmp/chough-pr46-combined-'+kind+'-result.json').read_text());assert d['exit_code']==0
 for p,digest in d['input']['source_sha256'].items():assert hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()==digest,p
 assert hashlib.sha256(pathlib.Path('.chough/cases/cli.cases.toml').read_bytes()).hexdigest()==d['input']['cases_sha256']
prefix='.specsync/changes/integrate-accepted-history-and-workflow-changes-into-pr-46-report-reads/'
allowed={prefix+x for x in ['change.md','verification.json','verification-attempts.json','state.json']}
status=git('status','--porcelain','--untracked-files=all').decode().splitlines();assert all(x[3:] in allowed for x in status),status
before=git('show','HEAD:'+prefix+'change.md').decode();after=(root/prefix/'change.md').read_text();normalize=lambda s:re.sub(r'^state: [^\n]+$', 'state: native-lifecycle',s,count=1,flags=re.M);assert normalize(before)==normalize(after)
subprocess.run(['git','fetch','origin'],check=True)
actual='16fe55da9ee900edf34a4d3ded517f36854a46d9';archived='122b8e9e8b913b49e618c59c2906e9e26613fd7e';assert git('rev-parse','origin/main').decode().strip()==actual
assert git('rev-parse',actual+'^{tree}')==git('rev-parse',archived+'^{tree}')
assert not git('diff','--name-only',actual,archived).strip()
p=pathlib.Path(tempfile.mkdtemp(prefix='chough-pr46-actual44-main-'));pathlib.Path('/tmp/chough-pr46-actual44-main-dir.txt').write_text(str(p)+'\n')
base=git('merge-base','HEAD',actual).decode().strip();refs={'base':base,'ours':git('rev-parse','HEAD').decode().strip(),'actual44main':actual,'archived44':archived}
for name,ref in refs.items():
 for path in ['.chough/session.json','.chough/transitions.jsonl','specs/cli/cli.spec.md']:(p/(name+'.'+path.rsplit('/',1)[-1])).write_bytes(git('show',ref+':'+path))
 lines=git('ls-tree','-r',ref).decode().splitlines();proof=[x for x in lines if any(x.split('\t',1)[1].startswith(q) for q in ('.specsync/archive/','.chough/evidence/','.chough/artifacts/','docs/exercises/handoff-cold-start/'))];(p/(name+'.proof-inventory.txt')).write_text('\n'.join(proof)+'\n')
(p/'refs.json').write_text(json.dumps(refs,indent=2)+'\n');(p/'whole-tree-identity.json').write_text(json.dumps({'actual44main':actual,'archived44':archived,'actual_tree':git('rev-parse',actual+'^{tree}').decode().strip(),'archived_tree':git('rev-parse',archived+'^{tree}').decode().strip(),'identical':True,'diff_paths':[]},indent=2)+'\n')
print('WHOLE_TREE_EQUAL',git('rev-parse',actual+'^{tree}').decode().strip(),'RAW_OPERANDS',p,flush=True)
r=subprocess.run(['git','merge','--no-commit','--no-ff',actual],stdout=subprocess.PIPE,stderr=subprocess.STDOUT);(p/'merge.log').write_bytes(r.stdout);print(r.stdout.decode(errors='replace'));assert r.returncode in (0,1)
print('CONFLICTS\n'+git('diff','--name-only','--diff-filter=U').decode())
