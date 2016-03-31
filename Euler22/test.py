from functools import reduce
l = []
with open("input.txt") as f:
    l = f.readline().replace('"', "").split(",")
out = 0
for i, j in enumerate(sorted(l)):
    score = (i + 1) * reduce(lambda x, y: x + list("ABCDEFGHIJKLMNOPQRSTUVWXYZ").index(y) + 1, j, 0)
    out += score
    print("{} gets {}".format(j, score))
print("Output was {}".format(out))
