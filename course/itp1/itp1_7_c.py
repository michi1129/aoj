data = []
r, c = list(map(int, input().split()))
for i in range(r):
    s = input()
    cols = list(map(int, s.split()))
    cols.append(sum(cols))
    data.append(cols)
    print(" ".join(map(str, cols)))

row_of_sum = []
for i in range(c + 1):
    row_of_sum.append(0)

    for j in range(r):
        row_of_sum[i] += data[j][i]

print(" ".join(map(str, row_of_sum)))
