from math import log
from collections import namedtuple, deque
def herosPath(i):
    """
    Given a 16 bit integer (it can be easily generalized, but
    I'd have to change some things), give the hero's path 
    through the integer if it were arranged as a 4x4 matrix, 
    with the LSB at the top left and MSB at the bottom right, 
    from the top left to the bottom right. You can only go 
    in the cardinal directions. Return a string of directions
    (l, r, u, d) that you took to get from the beginning to 
    the end, or an empty string if there is no path.
    Taken from a Jump Trading interview
    """
    assert i == 0 or log(i, 2) < 16 

    right = lambda x: x << 1 if log(x, 2) % 4 != 3 else 1
    left = lambda x: x >> 1 if log(x, 2) % 4 != 0 else 1
    up = lambda x: x >> 4 if x > 8 else 1
    down = lambda x: x << 4 if x <= (1 << 11) else 1

    Path = namedtuple("Path", ["position", "path"])
    visited = 0
    q = deque()
    q.appendleft(Path(1, ""))
    while len(q) != 0:
        hero = q.pop()
        if hero.position & visited != 0:
            continue
        visited |= hero.position
        if hero.position == (1 << 15):
            return hero.path

        if right(hero.position) & i == 0:
            q.appendleft(Path(right(hero.position), hero.path + "r"))
        if left(hero.position) & i == 0:
            q.appendleft(Path(left(hero.position), hero.path + "l"))
        if up(hero.position) & i == 0:
            q.appendleft(Path(up(hero.position), hero.path + "u"))
        if down(hero.position) & i == 0:
            q.appendleft(Path(down(hero.position), hero.path + "d"))
    return ""

print("herosPath({}) = {}".format(0, herosPath(0)))
print("herosPath({}) = {}".format(2, herosPath(2)))
print("herosPath({}) = {}".format(2 + 32 + 64, herosPath(2 + 32 + 64)))
print("herosPath({}) = {}".format(2 + 32 + 16, herosPath(2 + 32 + 16)))
