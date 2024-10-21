from math import atan2, pi
from itertools import permutations, starmap
from pdb import set_trace as breakpoint

FILE = "triangles.txt"

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __repr__(self):
        return "({}, {})".format(self.x, self.y)

    def __str__(self):
        return repr(self)

class Triangle:
    def __init__(self, a, b, c):
        self.a = a
        self.b = b
        self.c = c

    def __repr__(self):
        return "Triangle({}, {}, {})".format(self.a, self.b, self.c)

    def __str__(self):
        return repr(self)

    def contains_zero(self):
        def angle_sum(l, m, r):
            l_angle = min(abs(l - m), 2 * pi - abs(l - m))
            r_angle = min(abs(r - m), 2 * pi - abs(r - m))
            return l_angle + r_angle >= pi

        angles = (atan2(p.y, p.x) for p in [self.a, self.b, self.c])
        return all(starmap(angle_sum, permutations(angles)))

    @classmethod
    def from_line(cls, s):
        (ax, ay, bx, by, cx, cy) = s.strip().split(",")
        return cls(Point(int(ax), int(ay)), Point(int(bx), int(by)), Point(int(cx), int(cy)))

with open(FILE) as f:
    triangles = map(Triangle.from_line, f)
    triangles_containing_zero = (t for t in triangles if t.contains_zero())
    print(sum(1 for _ in triangles_containing_zero))
