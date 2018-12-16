def inverseCaptcha(l):
    out = 0
    for idx, d in enumerate(l):
        if idx == len(l) - 1:
            nextIdx = 0
        else:
            nextIdx = idx + 1
        if d == l[nextIdx]:
            out += d
    return out

def inverseCaptcha2(s):
    l = [int(i) for i in s]
    delta = len(l) // 2
    l += l
    out = 0
    for i in range(delta * 2):
        if l[i] == l[i + delta]:
            out += l[i]
    return out
