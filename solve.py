import re
data = []
for line in open('in','r').read().strip().split('\n'):
    x,y,bx,by=map(int,re.findall(r'-?\d+',line))
    data.append((x,y,bx,by))

def dist(a,b,x,y):
    return abs(a-x)+abs(b-y)

def check(u,v):
    for x,y,bx,by in data:
        if (u,v)==(bx,by) or (u,v)==(x,y):return True
        if dist(u,v,x,y)<=dist(x,y,bx,by):return False
    return True

def intersect(a,b,x,y):
    if a>b:
        # print("?")
        a,b,x,y=x,y,a,b
    if b+1<x:return None
    return [a,max(b,y)]

y=2000000
N=10**7
# N=100
cnt=0
for x in range(-N,N+1):
    if not check(x,y):
        cnt+=1
print("Part 1:",cnt)

N=4000000
for u in range(0,N+1):
    not_ok=[]
    for x,y,bx,by in data:
        dt=abs(x-bx)+abs(y-by)
        r=dt-abs(u-x)
        if r<=0:continue
        not_ok.append([max(0,y-r),min(N,y+r)])
    not_ok.sort()
    # print(1,not_ok)
    i=0
    while i<len(not_ok)-1:
        a,b=not_ok[i]
        x,y=not_ok[i+1]
        p=intersect(a,b,x,y)
        if p is None:
            i+=1
            continue
        not_ok[i]=p
        del not_ok[i+1]
    if not_ok != [[0,N]]:
        print("Part 2:",(not_ok[0][1]+1)+u*N)
        break