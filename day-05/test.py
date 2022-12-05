def transpose(arr):
    # assumes arr is rectangular
    return [[arr[j][i] for j in range(len(arr))] for i in range(len(arr[0]))]
for part in [1,2]:
    lines=open('input','r').read()
    diagram,moves=map(lambda x: x.split('\n'),lines.split('\n\n'))
    diagram = [''.join(line).strip() for line in transpose(diagram[::-1])][1::4]
    for line in moves:
        n,s,t=map(int,line.split(' ')[1::2])
        s-=1
        t-=1
        diagram[t]+=(lambda t:t[::-1] if part==1 else t)(diagram[s][-n:])
        diagram[s]=diagram[s][:-n]
    print(f"Part {part}:",''.join(l[-1]for l in diagram))