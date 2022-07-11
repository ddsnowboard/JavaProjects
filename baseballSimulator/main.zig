const std = @import("std");
const expect = std.testing.expect;

struct Player {
atBats: u32,
            hits: u32,
            doubles: u32, 
            triples: u32,
            homeRuns:u32,
            pub fn singles(self: Player) u32 {
                return self.hits - self.doubles - self.triples - self.homeRuns;
            }
}

const batters = [_]Player{
    Player { .atBats = 135, .hits = 24, .doubles = 4, .triples = 0, .homeRuns = 1},
           Player { .atBats = 315, .hits = 107, .doubles = 28, .triples = 0, .homeRuns = 19},
           Player { .atBats = 331, .hits = 85, .doubles = 12, .triples = 3, .homeRuns = 7},
           Player { .atBats = 77, .hits = 10, .doubles = 3, .triples = 0, .homeRuns = 1},
           Player { .atBats = 316, .hits = 92, .doubles = 20, .triples = 1, .homeRuns = 17},
           Player { .atBats = 166, .hits = 40, .doubles = 6, .triples = 1, .homeRuns = 4},
           Player { .atBats = 246, .hits = 63, .doubles = 7, .triples = 3, .homeRuns = 5},
           Player { .atBats = 238, .hits = 60, .doubles = 17, .triples = 2, .homeRuns = 5},
           Player { .atBats = 126, .hits = 25, .doubles = 5, .triples = 0, .homeRuns = 4}
};

pub fn main() void {
    var iter = circularIterator(batters[0..]);
    var counter: u32 = 0;
    while (counter < 25) {
        std.debug.print("Val is {d}\n", .{iter.next()});
        counter += 1;
    }
}

test "if statement" {
    const a = true;
    var x = @as(u16, 0);
    if (a) {
        x += 1;
    } else {
        x += 2;
    }
    try expect(x == 1);
}

fn CircularIterator(comptime T: type) type {
    return struct {
slice: []const T,
           index: usize = 0,
           pub fn next(self: *@This()) T {
               const out = self.slice[self.index];
               self.index = (self.index + 1) % self.slice.len;
               return out;
           }
    };
}

fn TypeOfSlice(comptime sliceType: type) type {
    return @typeInfo(@typeInfo(sliceType).Pointer.child).Array.child;
}

fn circularIterator(slice: anytype) CircularIterator(TypeOfSlice(@TypeOf(slice))) {
    return CircularIterator(TypeOfSlice(@TypeOf(slice))){ .slice = slice };
}
