n, m = list(map(int, input().split()))
v = []
for i in range(n):
    ns = list(map(int, input().split()))
    v.append(ns)
v2 = []
for i in range(m):
    d = int(input())
    v2.append(d)

for i in range(n):
    s = 0
    for j in range(m):
        s += v[i][j] * v2[j]
    print(s)
