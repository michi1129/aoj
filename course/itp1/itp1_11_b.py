a, b, c, d, e, f = map(int, input().split(" "))
original_vertical = [a, b, f, e]
original_horizontal = [b, c, e, d]


def reset_dice():
    return Dice(original_vertical[:], original_horizontal[:])


class Dice:
    def __init__(self, v, h):
        self.vertical = v
        self.horizontal = h

    def v2h(self):
        self.horizontal[0] = self.vertical[1]
        self.horizontal[2] = self.vertical[3]

    def h2v(self):
        self.vertical[1] = self.horizontal[0]
        self.vertical[3] = self.horizontal[2]

    def roll(self, direction):
        if direction == 'N':
            tmp1 = self.vertical[0]
            tmp2 = self.vertical[1:]
            self.vertical = tmp2 + [tmp1]
            self.v2h()
        elif direction == 'S':
            tmp1 = self.vertical[-1]
            tmp2 = self.vertical[:-1]
            self.vertical = [tmp1] + tmp2
            self.v2h()
        elif direction == 'E':
            tmp1 = self.horizontal[0]
            tmp2 = self.horizontal[1:]
            self.horizontal = tmp2 + [tmp1]
            self.h2v()
        elif direction == 'W':
            tmp1 = self.horizontal[-1]
            tmp2 = self.horizontal[:-1]
            self.horizontal = [tmp1] + tmp2
            self.h2v()


test_count = int(input())
for i in range(test_count):
    dice = reset_dice()

    top, front = map(int, input().split(" "))

    v = 0
    while dice.vertical[0] != top and v < 4:
        dice.roll("N")
        v += 1

    h = 0
    while dice.horizontal[0] != front and h < 4:
        dice.roll("E")
        h += 1

    # 強制的に一つ回す
    dice.roll("E")

    v = 0
    while dice.vertical[0] != top and v < 4:
        dice.roll("N")
        v += 1

    h = 0
    while dice.horizontal[0] != front and h < 4:
        dice.roll("E")
        h += 1

    print(dice.horizontal[1])
