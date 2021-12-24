s = input()
q = int(input())
for i in range(q):
    c = input().split()
    n1 = int(c[1])
    n2 = int(c[2])
    if c[0] == "print":
        print(s[n1 : n2 + 1])
    elif c[0] == "reverse":
        l = s[:n1]
        m = "".join(reversed(s[n1 : n2 + 1]))
        r = s[n2 + 1 :]
        s = l + m + r
    elif c[0] == "replace":
        l = s[:n1]
        r = s[n2 + 1 :]
        s = l + c[3] + r
