from itertools import combinations

while True:
    n, x = list(map(int, input().split()))
    if n == 0 and x == 0:
        break
    ans = 0
    for nums in combinations(range(1, n + 1), 3):
        s = sum(nums)
        if s == x:
            ans += 1
    print(ans)
