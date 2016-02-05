out = 0
for i in range(1, 1000000):
    if str(i) == str(i)[::-1] and "{0:b}".format(i) == "{0:b}".format(i)[::-1]:
        out += i
print(out)
