n, m, l = list(map(int, input().split()))
a = []
for i in range(n):
    a.append(list(map(int, input().split())))
b = []
for i in range(m):
    b.append(list(map(int, input().split())))

c = []
for i in range(n):
    r = []
    for j in range(l):
        r.append(0)
    c.append(r)

for i in range(n):
    for j in range(l):
        for k in range(m):
            c[i][j] += a[i][k] * b[k][j]

for r in c:
    print(" ".join(map(str, r)))
