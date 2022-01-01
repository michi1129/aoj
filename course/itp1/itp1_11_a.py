a, b, c, d, e, f = map(int, input().split(" "))
moves = list(input())
vertical = [a, b, f, e]
horizontal = [a, d, f, c]


def v2h():
    horizontal[0] = vertical[0]
    horizontal[2] = vertical[2]


def h2v():
    vertical[0] = horizontal[0]
    vertical[2] = horizontal[2]

for move in moves:
    if move == 'N':
        tmp1 = vertical[0]
        tmp2 = vertical[1:]
        vertical = tmp2 + [tmp1]
        v2h()
    elif move == 'S':
        tmp1 = vertical[-1]
        tmp2 = vertical[:-1]
        vertical = [tmp1] + tmp2
        v2h()
    elif move == 'E':
        tmp1 = horizontal[0]
        tmp2 = horizontal[1:]
        horizontal = tmp2 + [tmp1]
        h2v()
    elif move == 'W':
        tmp1 = horizontal[-1]
        tmp2 = horizontal[:-1]
        horizontal = [tmp1] + tmp2
        h2v()

print(vertical[0])
