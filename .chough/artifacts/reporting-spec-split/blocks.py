import re,sys,hashlib,json
def blocks(path):
    t=open(path,encoding='utf-8').read()
    out={}
    for p in re.split(r'(?m)^(?=### REQ-)',t)[1:]:
        head,_,body=p.partition('\n')
        rid=head.split()[1]
        out[rid]={'heading_suffix':head[len('### '+rid):],'body':body.strip(),'sha256':hashlib.sha256(body.strip().encode()).hexdigest()}
    return out
if __name__=='__main__':
    res={}
    for p in sys.argv[1:]: res[p]={k:{'sha256':v['sha256'],'heading_suffix':v['heading_suffix']} for k,v in blocks(p).items()}
    print(json.dumps(res,indent=1))
